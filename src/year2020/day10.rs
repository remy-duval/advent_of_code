use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;

use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Adapters;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 10: Adapter Array";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let (ones, threes) = first_part(&data.0)
            .ok_or_else(|| anyhow::anyhow!("Could not find the full adapter chain"))?;

        println!(
            "The full adapter chain has {ones} (1V difference) * {threes} (3V difference) = {sum}",
            ones = ones,
            threes = threes,
            sum = ones * threes
        );

        println!(
            "The number of arrangements of adapters for reaching the wanted value is: {total}",
            total = second_part(&data.0)
        );

        Ok(())
    }
}

pub struct Adapters(HashSet<usize>);

impl FromStr for Adapters {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Adapters(
            s.lines().map(|line| line.trim().parse()).try_collect()?,
        ))
    }
}

/// The maximum value of all the given adapters
/// ### Arguments
/// * `adapters` - All the adapters that are considered
///
/// ### Returns
/// The maximum value of the adapters (or 0 if there are none)
fn max(adapters: &HashSet<usize>) -> usize {
    adapters.iter().copied().max().unwrap_or(0)
}

/// Find a chain of all the adapters that connect 0 to max + 3
/// Then return the number of 1V differences and the number of 3V differences in it
///
/// ### Arguments
/// * `adapters` - All the adapters that are considered
///
/// ### Returns
/// Maybe (ones, threes), the counts of 1V diff and 3V diff in the chain of adapters found
fn first_part(adapters: &HashSet<usize>) -> Option<(u32, u32)> {
    let all = adapters.len();
    let maximum = max(adapters);
    let mut stack = vec![(0, Chain::default())];
    while let Some((next, chain)) = stack.pop() {
        if next == maximum {
            if chain.steps == all {
                return Some((chain.ones, chain.threes + 1));
            }
        } else if adapters.contains(&next) || next == 0 {
            stack.push((next + 3, chain.next(3)));
            stack.push((next + 2, chain.next(2)));
            stack.push((next + 1, chain.next(1)));
        }
    }

    None
}

/// Find the possibilities for arranging adapters to get to the maximum value in V
///
/// ### Arguments
/// * `adapters` - All the adapters that are considered
///
/// ### Returns
/// The number of arrangement of adapters that can chain to reach their maximum V value
fn second_part(adapters: &HashSet<usize>) -> usize {
    let maximum = max(adapters) + 1;
    let mut first = 0; // The arrangements of V - 3
    let mut second = 0; // The arrangements of V - 2
    let mut third = 1; // The arrangements of V - 1 (The base of 0 V is 1 arrangement)
    let mut possibilities = 0; // The arrangements of V
    for voltage in 1..maximum {
        if adapters.contains(&voltage) {
            // If V is an adapter, its arrangements are the addition of the 3 previous ones
            possibilities = third + second + first;
        } else {
            // If V is not an adapter, it can't have any arrangements
            possibilities = 0;
        }
        first = second;
        second = third;
        third = possibilities;
    }

    possibilities
}

/// Hold the information about an adapter chain for the first part
#[derive(Debug, Default)]
struct Chain {
    steps: usize,
    ones: u32,
    threes: u32,
}

impl Chain {
    /// Get the chain for the given next step
    /// If it is 1, increase the `one`
    /// If it is 3, increase the `three`
    /// In any case increase the `step`
    fn next(&self, next: u8) -> Self {
        Chain {
            steps: self.steps + 1,
            ones: self.ones + if next == 1 { 1 } else { 0 },
            threes: self.threes + if next == 3 { 1 } else { 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = include_str!("test_resources/10-A.txt");
    const B: &str = include_str!("test_resources/10-B.txt");
    const C: &str = include_str!("test_resources/10-C.txt");

    #[test]
    fn first_part_test_a() {
        let adapters = Day::parse(A).unwrap().0;
        let (ones, threes) = first_part(&adapters).unwrap();
        assert_eq!(7, ones);
        assert_eq!(5, threes);
    }

    #[test]
    fn first_part_test_b() {
        let adapters = Day::parse(B).unwrap().0;
        let (ones, threes) = first_part(&adapters).unwrap();
        assert_eq!(22, ones);
        assert_eq!(10, threes);
    }

    #[test]
    fn first_part_test_c() {
        let adapters = Day::parse(C).unwrap().0;
        let (ones, threes) = first_part(&adapters).unwrap();
        assert_eq!(66, ones);
        assert_eq!(39, threes);
    }

    #[test]
    fn second_part_test_a() {
        let adapters = Day::parse(A).unwrap().0;
        assert_eq!(8, second_part(&adapters));
    }

    #[test]
    fn second_part_test_b() {
        let adapters = Day::parse(B).unwrap().0;
        assert_eq!(19_208, second_part(&adapters));
    }

    #[test]
    fn second_part_test_c() {
        let adapters = Day::parse(C).unwrap().0;
        assert_eq!(2_644_613_988_352, second_part(&adapters))
    }
}
