use color_eyre::eyre::Result;

use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = String;
    const TITLE: &'static str = "Day 5: Alchemical Reduction";

    fn solve(data: Self::Input) -> Result<()> {
        println!("After the basic reaction, {} units remain", first(&data));
        println!("After the best reaction, {} units remain", second(&data));
        Ok(())
    }
}

fn first(polymer: &str) -> usize {
    reaction(polymer, |_| true).len()
}

fn second(polymer: &str) -> usize {
    ('a'..'z')
        .map(|lower| {
            let higher = lower.to_ascii_uppercase();
            reaction(polymer, move |c: &char| *c != lower && *c != higher).len()
        })
        .min()
        .unwrap_or_default()
}

/// React the given polymer, removing pairs of opposite polarity
///
/// ### Arguments
/// * `polymer` - The polymer to react
/// * `keep_only` - A closure that is used to filter the accepted units, if false a unit is removed
///
/// ### Return
/// The polymer after full reaction, removing all elements that don't satisfy `keep_only` first
fn reaction(polymer: &str, keep_only: impl Fn(&char) -> bool) -> String {
    fn reverse_polarity(c: char) -> char {
        if c.is_ascii_uppercase() {
            c.to_ascii_lowercase()
        } else {
            c.to_ascii_uppercase()
        }
    }

    let mut units = polymer.chars().filter(keep_only);
    let mut current = units.next(); // The current unit that can react
    let mut builder = String::with_capacity(polymer.len());
    units.for_each(|next| match current.take() {
        None => current = Some(next),
        Some(curr) if reverse_polarity(next) == curr => {
            // The reaction destroyed the last two, take the previous part of the chain as current
            current = builder.pop();
        }
        Some(curr) => {
            // No reaction happened, append the current to the chain
            current = Some(next);
            builder.push(curr);
        }
    });

    builder.extend(current);
    builder
}

#[cfg(test)]
mod tests;
