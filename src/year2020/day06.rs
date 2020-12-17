use std::str::FromStr;

use hashbrown::HashSet;

use crate::parse::SepByEmptyLine;
use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = SepByEmptyLine<String>;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 6: Custom Customs";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        println!(
            "The total of YES answers for any participant of each group is {}",
            first_part(&data.data)
        );

        println!(
            "The total of YES answers for all participant of each group is {}",
            second_part(&data.data)
        );

        Ok(())
    }
}

/// Compute the sum of yes answers for any participant of a group
fn first_part(groups: &[String]) -> usize {
    groups
        .iter()
        .map(|group| group.parse::<AnyYesAnswers>().map_or(0, |ans| ans.0))
        .sum()
}

/// Compute the sum of yes answers for all participants of a group
fn second_part(groups: &[String]) -> usize {
    groups
        .iter()
        .map(|group| group.parse::<AllYesAnswers>().map_or(0, |ans| ans.0))
        .sum()
}

/// The answers that any member of a group answered YES for
struct AnyYesAnswers(usize);

impl FromStr for AnyYesAnswers {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut any_yes = HashSet::new();
        for line in s.lines() {
            any_yes.extend(line.chars());
        }
        Ok(Self(any_yes.len()))
    }
}

/// The answers that all members of a group answered YES for
struct AllYesAnswers(usize);

impl FromStr for AllYesAnswers {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        if let Some(first) = lines.next() {
            let mut all_yes: HashSet<char> = first.chars().collect();
            for line in lines {
                all_yes.retain(|current| line.contains(*current));
            }
            Ok(Self(all_yes.len()))
        } else {
            Ok(Self(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = include_str!("test_resources/06-A.txt");
    const B: &str = include_str!("test_resources/06-B.txt");

    #[test]
    fn first_part_test_a() {
        let data = Day::parse(A).unwrap();
        let sum = first_part(&data.data);
        assert_eq!(11, sum);
    }

    #[test]
    fn first_part_test_b() {
        let data = Day::parse(B).unwrap();
        let sum = first_part(&data.data);
        assert_eq!(6351, sum);
    }

    #[test]
    fn second_part_test_a() {
        let data = Day::parse(A).unwrap();
        let sum = second_part(&data.data);
        assert_eq!(6, sum);
    }

    #[test]
    fn second_part_test_b() {
        let data = Day::parse(B).unwrap();
        let sum = second_part(&data.data);
        assert_eq!(3143, sum);
    }
}
