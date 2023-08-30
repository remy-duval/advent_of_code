use std::str::FromStr;

use commons::error::Result;
use commons::{Report, WrapErr};

pub const TITLE: &str = "Day 4: Camp Cleanup";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The number of full overlap is {first}");
    let second = second_part(&data);
    println!("2. The number of any overlap is {second}");

    Ok(())
}

// Full overlap
fn first_part(input: &[Pair]) -> usize {
    input
        .iter()
        .filter(|Pair(a, b)| (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1))
        .count()
}

// Partial overlap
fn second_part(input: &[Pair]) -> usize {
    input
        .iter()
        .filter(|Pair(a, b)| (a.0 >= b.0 && a.0 <= b.1) || (b.0 >= a.0 && b.0 <= a.1))
        .count()
}

#[derive(Debug)]
struct Pair(Assignment, Assignment);
#[derive(Copy, Clone, Debug)]
struct Assignment(u8, u8);

impl FromStr for Assignment {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self> {
        let (a, b) = s.split_once('-').wrap_err("missing '-'")?;
        let a = a.trim().parse().wrap_err_with(|| format!("for {a}"))?;
        let b = b.trim().parse().wrap_err_with(|| format!("for {b}"))?;
        Ok(Self(a, b))
    }
}

impl FromStr for Pair {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self> {
        let (a, b) = s.split_once(',').wrap_err("missing ','")?;
        let a = a.parse().wrap_err_with(|| format!("for {a}"))?;
        let b = b.parse().wrap_err_with(|| format!("for {b}"))?;
        Ok(Self(a, b))
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Pair>> {
    s.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().wrap_err_with(|| format!("for {l}")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/04.txt");
    const MAIN: &str = include_str!("../inputs/04.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 2);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 477);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 4);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 830);
    }
}
