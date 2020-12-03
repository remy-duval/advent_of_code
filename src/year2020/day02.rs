use std::str::FromStr;

use crate::commons::parse::LineSep;

pub struct Day;

impl crate::Problem for Day {
    type Input = LineSep<Password>;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 2: Password Philosophy";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        println!(
            "{} passwords respect the first policy",
            first_part(&data.data)
        );
        println!(
            "{} passwords respect the second policy",
            second_part(&data.data)
        );
        Ok(())
    }
}

fn first_part(data: &[Password]) -> usize {
    data.iter()
        .filter(|pwd| pwd.check_occurrence_policy())
        .count()
}

fn second_part(data: &[Password]) -> usize {
    data.iter()
        .filter(|pwd| pwd.check_position_policy())
        .count()
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Password {
    parameters: (u8, u8),
    character: char,
    value: String,
}

impl Password {
    /// Check that the occurrence policy is respected for this value
    /// The number of occurrence of the 'character' must be between the parameters
    fn check_occurrence_policy(&self) -> bool {
        let count = self
            .value
            .chars()
            .filter(|char| *char == self.character)
            .count();
        count >= self.parameters.0 as usize && count <= self.parameters.1 as usize
    }

    /// Check that the occurrence policy is respected for this value
    /// The number of occurrence of the 'character' must be between the parameters
    fn check_position_policy(&self) -> bool {
        let mut is_first_set: bool = false;
        for (idx, char) in self.value.chars().enumerate() {
            let idx = idx + 1;
            if idx == self.parameters.0 as usize {
                is_first_set = char == self.character;
                continue;
            }
            if idx == self.parameters.1 as usize {
                return is_first_set ^ (char == self.character);
            }
        }
        return is_first_set;
    }
}

impl FromStr for Password {
    type Err = PasswordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn first_two<'a>(mut iter: impl Iterator<Item = &'a str>) -> Option<(&'a str, &'a str)> {
            Some((iter.next()?, iter.next()?))
        }

        let (pol, pwd) = first_two(s.split(':')).ok_or(PasswordError::MissingPart)?;
        let (occ, char) = first_two(pol.split_whitespace()).ok_or(PasswordError::MissingPart)?;
        let (min, max) = first_two(occ.split('-')).ok_or(PasswordError::MissingPart)?;
        let a: u8 = min.trim().parse::<u8>()?;
        let b: u8 = max.trim().parse::<u8>()?;
        let character = char.chars().next().ok_or(PasswordError::MissingPart)?;

        Ok(Password {
            parameters: (a.min(b), a.max(b)),
            character,
            value: pwd.trim().into(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("Part of the password is missing, the format is 'int-int char: password'")]
    MissingPart,
    #[error("Could not parse one of the integer of the policy parameters {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    use super::*;

    const A: &str = include_str!("test_resources/02-A.txt");
    const B: &str = include_str!("test_resources/02-B.txt");

    #[test]
    fn first_part_test_a() {
        let data = A
            .parse::<<Day as Problem>::Input>()
            .expect("parsing error");

        assert_eq!(2, first_part(&data.data));
    }

    #[test]
    fn first_part_test_b() {
        let data = B
            .parse::<<Day as Problem>::Input>()
            .expect("parsing error");

        assert_eq!(600, first_part(&data.data));
    }

    #[test]
    fn second_part_test_a() {
        let data = A
            .parse::<<Day as Problem>::Input>()
            .expect("parsing error");

        assert_eq!(1, second_part(&data.data));
    }

    #[test]
    fn second_part_test_b() {
        let data = B
            .parse::<<Day as Problem>::Input>()
            .expect("parsing error");

        assert_eq!(245, second_part(&data.data));
    }
}
