use itertools::Itertools;
use std::collections::HashMap;

use commons::eyre::{ensure, eyre, Result};
use commons::parse::sep_by_empty_lines;

pub const TITLE: &str = "Day 14: Extended Polymerization";

pub fn run(raw: String) -> Result<()> {
    let polymer = parse(&raw)?;
    let one = min_max_rates(&polymer.initial, &polymer.rules, 10);
    println!("1. After 10 steps: {}", one.1 - one.0);

    let two = min_max_rates(&polymer.initial, &polymer.rules, 40);
    println!("2. After 40 steps: {}", two.1 - two.0);

    Ok(())
}

fn parse(s: &str) -> Result<Polymer> {
    fn alpha_index(b: u8) -> Result<u8> {
        b.to_ascii_uppercase()
            .checked_sub(b'A')
            .ok_or_else(|| eyre!("Bad character {}", b))
    }

    let (initial, rules) = sep_by_empty_lines(s)
        .collect_tuple::<(_, _)>()
        .ok_or_else(|| eyre!("Missing empty line between polymer and rules in {}", s))?;

    let initial = initial.bytes().map(alpha_index).collect::<Result<_>>()?;
    let rules = rules
        .lines()
        .map(|r| -> Result<_> {
            let (from, to) = r
                .split_once("->")
                .ok_or_else(|| eyre!("Missing '->' in {}", r))?;
            let from = from.trim().as_bytes();
            let to = to.trim().as_bytes();
            ensure!(from.len() == 2, "Not 2 elements left of {}", r);
            ensure!(to.len() == 1, "Not 1 element right of {}", r);
            Ok((
                (alpha_index(from[0])?, alpha_index(from[1])?),
                alpha_index(to[0])?,
            ))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    Ok(Polymer { initial, rules })
}

/// The puzzle input for the day
struct Polymer {
    /// Initial sequence of polymer, from 0 to 25
    initial: Vec<u8>,
    /// What the next step should insert between a pair of polymer
    rules: HashMap<(u8, u8), u8>,
}

/// Find the min and maximum counts of polymers after the given number of steps
///
/// Uses a far more optimized solution using pair counting proposed by a colleague
/// See previous commit for initial solution, using dynamic programming
/// This solution is at least 2 - 3 times faster than the original one
fn min_max_rates(initial: &[u8], rules: &HashMap<(u8, u8), u8>, steps: usize) -> (u64, u64) {
    // Number of element (indexed by the character)
    let mut rates: [u64; 26] = [0; 26];
    initial.iter().for_each(|&i| rates[i as usize] += 1);

    // Current pairs of elements
    let mut pairs: HashMap<(u8, u8), u64> = HashMap::with_capacity(rules.len());
    for window in initial.windows(2) {
        if let [from, to] = window {
            *pairs.entry((*from, *to)).or_default() += 1;
        }
    }

    let mut next_pairs = HashMap::with_capacity(rules.len());
    for _ in 0..steps {
        for (pair, count) in pairs.drain() {
            if let Some(&mid) = rules.get(&pair) {
                rates[mid as usize] += count;
                *next_pairs.entry((pair.0, mid)).or_default() += count;
                *next_pairs.entry((mid, pair.1)).or_default() += count;
            } else {
                *next_pairs.entry(pair).or_default() += count;
            }
        }

        std::mem::swap(&mut pairs, &mut next_pairs);
    }

    rates
        .into_iter()
        .filter(|&c| c > 0) // Exclude characters that are not present
        .minmax()
        .into_option()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests;
