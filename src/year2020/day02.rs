use std::str::FromStr;

use crate::commons::parse::LineSep;

pub struct Day02;

impl crate::Problem for Day02 {
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

pub struct Password {
    policy: PasswordPolicy,
    value: String,
}

impl Password {
    /// Check that the policy is respected for this value
    fn check_occurrence_policy(&self) -> bool {
        self.policy.check_occurrence_policy(&self.value)
    }

    /// Check that the policy is respected for this value
    fn check_position_policy(&self) -> bool {
        self.policy.check_position_policy(&self.value)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PasswordPolicy {
    parameters: (u8, u8),
    character: char,
}

impl PasswordPolicy {
    /// Check that the occurrence policy is respected for this value
    /// The number of occurrence of the 'character' must be between the parameters
    fn check_occurrence_policy(self, value: &str) -> bool {
        let count = value.chars().filter(|char| *char == self.character).count();
        count >= self.parameters.0 as usize && count <= self.parameters.1 as usize
    }

    /// Check that the occurrence policy is respected for this value
    /// The number of occurrence of the 'character' must be between the parameters
    fn check_position_policy(self, value: &str) -> bool {
        let mut is_first_set: bool = false;
        for (idx, char) in value.chars().enumerate() {
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

/// Collect two elements of a string iterator into a tuple
fn collect_two<'a>(mut iter: impl Iterator<Item = &'a str>) -> Option<(&'a str, &'a str)> {
    Some((iter.next()?, iter.next()?))
}

impl FromStr for PasswordPolicy {
    type Err = PolicyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (occ, char) = collect_two(s.split_whitespace()).ok_or(PolicyParseError::MissingPart)?;
        let (min, max) = collect_two(occ.split('-')).ok_or(PolicyParseError::MissingPart)?;
        let a: u8 = min.trim().parse::<u8>()?;
        let b: u8 = max.trim().parse::<u8>()?;
        let character = char.chars().next().ok_or(PolicyParseError::MissingPart)?;

        Ok(PasswordPolicy {
            parameters: (a.min(b), a.max(b)),
            character,
        })
    }
}

impl FromStr for Password {
    type Err = PasswordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pol, pwd) = collect_two(s.split(':')).ok_or(PasswordParseError::MissingPart)?;

        Ok(Password {
            policy: pol.parse::<PasswordPolicy>()?,
            value: pwd.trim().into(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PolicyParseError {
    #[error("At least one part of the password policy is missing 'int-int char'")]
    MissingPart,
    #[error("Could not parse one of the integer of the policy {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordParseError {
    #[error("Missing either the policy or the password")]
    MissingPart,
    #[error("Could not parse the password policy {0}")]
    BadPolicy(#[from] PolicyParseError),
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
            .parse::<<Day02 as Problem>::Input>()
            .expect("parsing error");

        assert_eq!(2, first_part(&data.data));
    }

    #[test]
    fn first_part_test_b() {
        let data = B
            .parse::<<Day02 as Problem>::Input>()
            .expect("parsing error");

        assert_eq!(600, first_part(&data.data));
    }

    #[test]
    fn second_part_test_a() {
        let data = A
            .parse::<<Day02 as Problem>::Input>()
            .expect("parsing error");

        assert_eq!(1, second_part(&data.data));
    }

    #[test]
    fn second_part_test_b() {
        let data = B
            .parse::<<Day02 as Problem>::Input>()
            .expect("parsing error");

        assert_eq!(245, second_part(&data.data));
    }
}
