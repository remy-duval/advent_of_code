use commons::eyre::{eyre, Result, WrapErr};

pub const TITLE: &str = "Day 18: Snailfish";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!("1. Magnitude of the sum: {}", first_part(data.clone()));
    println!("2. Highest magnitude sum: {}", second_part(data));

    Ok(())
}

fn parse(s: &str) -> Result<Vec<Number>> {
    s.lines()
        .map(|line| Number::parse(line).map(|(n, _)| n))
        .collect()
}

fn first_part(numbers: Vec<Number>) -> u64 {
    numbers
        .into_iter()
        .reduce(|a, b| a.add(b))
        .map_or(0, |n| n.magnitude())
}

fn second_part(numbers: Vec<Number>) -> u64 {
    let mut max = 0;
    for i in 0..numbers.len() {
        for j in (0..numbers.len()).filter(|&j| j != i) {
            let first = numbers[i].clone().add(numbers[j].clone()).magnitude();
            let second = numbers[j].clone().add(numbers[i].clone()).magnitude();
            max = max.max(first).max(second)
        }
    }
    max
}

/// A number formed of pairs of numbers stored as a binary tree
/// This could probably be made more efficient by being flattened as a vec ?
#[derive(Debug, Clone, Eq, PartialEq)]
enum Number {
    Pair(Box<(Number, Number)>),
    Regular(u64),
}

impl Number {
    /// Recursively compute the number's magnitude
    fn magnitude(&self) -> u64 {
        match self {
            Number::Pair(pair) => {
                let (a, b) = pair.as_ref();
                3 * a.magnitude() + 2 * b.magnitude()
            }
            Number::Regular(n) => *n,
        }
    }

    /// Add two numbers together
    /// - Concatenate the two in a tree
    /// - Apply reduction (explode or split in this order) repeatedly until stable
    fn add(self, other: Self) -> Self {
        let mut current = Self::Pair(Box::new((self, other)));
        while current.explode_once(0).is_some() || current.split_once() {}
        current
    }

    /// Split the first 10+ number in this sub-tree into a pair and stop
    /// Returns whether such a split was made
    fn split_once(&mut self) -> bool {
        match self {
            Self::Regular(n) => {
                // Split numbers >= 10 in pairs of its halves
                if *n >= 10 {
                    let left = *n / 2;
                    let right = left + *n % 2;
                    *self = Self::Pair(Box::new((Self::Regular(left), Self::Regular(right))));
                    true
                } else {
                    false
                }
            }
            Self::Pair(pair) => {
                let (a, b) = pair.as_mut();
                a.split_once() || b.split_once()
            }
        }
    }

    /// Explode the leftmost 4+ deep pair and stop
    ///
    /// - Returns `None` if it did nothing
    /// - Returns `Some((left, right))` if it exploded something, with values to add to its:
    ///   - leftmost neighbour number
    ///   - rightmost neighbour number
    fn explode_once(&mut self, depth: usize) -> Option<(u64, u64)> {
        if let Self::Pair(pair) = self {
            let (a, b) = pair.as_mut();
            if depth >= 4 {
                let left = match a {
                    Number::Regular(n) => *n,
                    Number::Pair(_) => 0,
                };
                let right = match b {
                    Number::Regular(n) => *n,
                    Number::Pair(_) => 0,
                };
                *self = Self::Regular(0);
                // When going up, we will need to dispatch those values to the left and right
                Some((left, right))
            } else if let Some((left, mut right)) = a.explode_once(depth + 1) {
                if right != 0 {
                    // Add the right explosion value to the left-most of the right element
                    b.increase(right, false);
                    right = 0;
                }
                Some((left, right))
            } else if let Some((mut left, right)) = b.explode_once(depth + 1) {
                if left != 0 {
                    // Add the left explosion value to the right-most of the left element
                    a.increase(left, true);
                    left = 0;
                }
                Some((left, right))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Propagate a value increase to either the left most or right most element in this
    fn increase(&mut self, value: u64, right_most: bool) {
        match self {
            Number::Regular(n) => *n += value,
            Number::Pair(pair) => {
                let (a, b) = pair.as_mut();
                if right_most {
                    b.increase(value, right_most)
                } else {
                    a.increase(value, right_most)
                }
            }
        }
    }

    /// Parse a number tree from a string, returning the parsed number and the rest of the string
    fn parse(s: &str) -> Result<(Self, &str)> {
        match s.strip_prefix('[') {
            None => s
                .char_indices()
                .take_while(|(_, c)| c.is_digit(10))
                .map(|(i, _)| i)
                .last()
                .map(|last| s.split_at(last + 1))
                .ok_or_else(|| eyre!("Expecting number in {}", s))
                .and_then(|(num, rest)| {
                    let num = num
                        .parse()
                        .wrap_err_with(|| format!("Expecting number in {}", num))?;
                    Ok((Self::Regular(num), rest))
                }),
            Some(pair) => {
                let (a, rest) = Self::parse(pair)?;
                let rest = rest
                    .strip_prefix(',')
                    .ok_or_else(|| eyre!("Expecting ',' in the middle of pair {}", s))?;
                let (b, rest) = Self::parse(rest)?;
                let rest = rest
                    .strip_prefix(']')
                    .ok_or_else(|| eyre!("Expecting ']' at the end of pair {}", s))?;
                Ok((Self::Pair(Box::new((a, b))), rest))
            }
        }
    }
}

#[cfg(test)]
mod tests;
