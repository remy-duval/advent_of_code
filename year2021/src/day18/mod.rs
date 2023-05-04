use commons::{bail, Result};

pub const TITLE: &str = "Day 18: Snailfish";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!("1. Magnitude of the sum: {}", first_part(&data));
    println!("2. Highest magnitude sum: {}", second_part(&data));

    Ok(())
}

/// Represent a number in the binary tree by storing its depth and value
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Part {
    /// Always between 1 and 5
    depth: u8,
    value: u8,
}

fn parse(s: &str) -> Result<Vec<Vec<Part>>> {
    s.lines().map(parse_one).collect()
}

/// Add up all the numbers together and compute the final magnitude
fn first_part(numbers: &[Vec<Part>]) -> u64 {
    add_all(numbers).map_or(0, |result| magnitude(&result))
}

/// Find the highest magnitude of the numbers two-sums
fn second_part(numbers: &[Vec<Part>]) -> u64 {
    let mut max = 0;
    let mut current = Vec::with_capacity(2 * numbers.first().map_or(0, Vec::capacity));
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                add(&numbers[i], &numbers[j], &mut current);
                let first = magnitude(&current);
                add(&numbers[j], &numbers[i], &mut current);
                max = max.max(magnitude(&current)).max(first)
            }
        }
    }

    max
}

/// Parse a number from a line
fn parse_one(s: &str) -> Result<Vec<Part>> {
    let mut depth = 0;
    let mut result = Vec::new();
    for c in s.trim().chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => (),
            d => match d.to_digit(10).map(|d| d as u8) {
                Some(value) => result.push(Part { depth, value }),
                None => bail!("Expected '[', ']', ',' or digit, got {d}"),
            },
        }
    }

    Ok(result)
}

/// Compute the magnitude of this number
///
/// This is the trickiest part with this data structure, since we don't have the actual tree
fn magnitude(number: &[Part]) -> u64 {
    // To be able to compute the magnitude without rebuilding the full tree:
    // Remember whether we are on the first or second value for this depth level, and for:
    // false -> first, 3x factor, switch to true
    // true -> second, 2x factor, switch to false, bubble up this process to previous depths
    //
    // The full factor to apply to each value is the product of all the factors until its depth
    //
    // E.g.: [[a, [b, c]], d]
    // The factors should be:
    // - a: [3, 3]        (array is [false, false, false, false] using depth 0 - 1)
    // - b: [3, 2, 3]     (array is [false, true, false, false] using depth 0 - 2)
    // - c: [3, 2, 2],    (array is [false, true, true, false]  using depth 0 - 2)
    // - d: [2]           (array is [true, false, false, false] using depth 0)
    //
    // This way of doing breaks for depths of more than five
    let mut second_value = [false; 5];
    number
        .iter()
        .copied()
        .fold(0, |acc, Part { depth, value }| {
            let mut carry_over = true;
            let mut current = value as u64;
            // Add the value x the factors to the total
            for v in second_value.iter_mut().take(depth as usize).rev() {
                current *= if *v { 2 } else { 3 };
                if carry_over {
                    carry_over = *v; // Bubble up if this was the second value
                    *v = !*v;
                }
            }

            acc + current
        })
}

/// Add all numbers together in order
fn add_all(numbers: &[Vec<Part>]) -> Option<Vec<Part>> {
    let mut numbers = numbers.iter();
    let mut acc = numbers.next()?.clone();
    let mut swap = Vec::with_capacity(acc.capacity());
    numbers.for_each(|number| {
        add(&acc, number, &mut swap);
        std::mem::swap(&mut acc, &mut swap);
    });
    Some(acc)
}

/// Add two numbers into the given buffer
fn add(first: &[Part], second: &[Part], into: &mut Vec<Part>) {
    into.clear();
    into.reserve(first.len() + second.len());
    first.iter().chain(second.iter()).for_each(|p| {
        into.push(Part {
            depth: p.depth + 1,
            value: p.value,
        });
    });
    while explode(into) || split(into) {}
}

/// Split the first number >= 10 into a pair. Returns true if changed something.
fn split(number: &mut Vec<Part>) -> bool {
    number
        .iter()
        .position(|p| p.value >= 10)
        .map_or(false, |i| {
            let current = &mut number[i];
            let depth = current.depth + 1;
            let l = current.value / 2;
            let r = l + current.value % 2;
            *current = Part { depth, value: l };
            number.insert(i + 1, Part { depth, value: r });
            true
        })
}

/// Explode the first pair at depth 5 into the nearest elements. Returns true if changed something.
fn explode(number: &mut Vec<Part>) -> bool {
    if let Some(i) = number.iter().position(|p| p.depth > 4) {
        let right = number.remove(i + 1);
        let left = std::mem::replace(&mut number[i], Part { depth: 4, value: 0 });
        if let Some(before) = i.checked_sub(1).and_then(|b| number.get_mut(b)) {
            before.value += left.value;
        }
        if let Some(after) = number.get_mut(i + 1) {
            after.value += right.value;
        }

        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests;
