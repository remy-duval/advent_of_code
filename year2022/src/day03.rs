use std::str::FromStr;

use commons::error::Result;
use commons::parse::LineSep;
use commons::{err, Report};

pub const TITLE: &str = "Day 3: Rucksack Reorganization";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The sum of priorities of the item shared between halves is {first}");
    let second = second_part(&data);
    println!("2. The sum of priorities of the item shared between groups is {second}");

    Ok(())
}

fn first_part(sacks: &[Rucksack]) -> u64 {
    sacks
        .iter()
        .map(|s| s.first.intersection(&s.second).max_item_priority() as u64)
        .sum()
}

fn second_part(sacks: &[Rucksack]) -> u64 {
    sacks
        .chunks_exact(3)
        .filter_map(|group| {
            group
                .iter()
                .map(|s| s.first.union(&s.second))
                .reduce(|a, b| a.intersection(&b))
                .map(|shared| shared.max_item_priority() as u64)
        })
        .sum()
}

struct Compartment(u64);

impl Compartment {
    fn put(&mut self, item: u8) {
        self.0 |= 1u64 << item as u64;
    }

    fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }

    fn max_item_priority(&self) -> u8 {
        let first_one = u64::BITS - self.0.leading_zeros();
        first_one.saturating_sub(1) as u8
    }
}

struct Rucksack {
    first: Compartment,
    second: Compartment,
}

impl FromStr for Rucksack {
    type Err = Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let line = s.trim();
        let len = line.len();
        let mid = len / 2;
        let mut first = Compartment(0);
        let mut second = Compartment(0);
        for (i, character) in line.chars().enumerate() {
            let priority = match character as u8 {
                c @ b'A'..=b'Z' => c - b'A' + 27,
                c @ b'a'..=b'z' => c - b'a' + 1,
                _ => return Err(err!("Unknown item in sack '{character}'")),
            };

            if i < mid {
                first.put(priority);
            } else {
                second.put(priority);
            }
        }

        Ok(Self { first, second })
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Rucksack>> {
    let split: LineSep<Rucksack> = s.parse()?;
    Ok(split.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/03.txt");
    const MAIN: &str = include_str!("../inputs/03.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 157);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 7763);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 70);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 2569);
    }
}
