use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// The Day of the Advent of Code problem to solve (between 01 and 25)
pub struct Day(pub u8);

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Day {}", self.0)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseDayError {
    #[error("Day must be between 1 and 25")]
    OutsideRange,
    #[error("Day must be an integer")]
    ParseError(#[from] std::num::ParseIntError),
}

impl FromStr for Day {
    type Err = ParseDayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>()? {
            0 => Err(ParseDayError::OutsideRange),
            day if day <= 25 => Ok(Day(day)),
            _ => Err(ParseDayError::OutsideRange),
        }
    }
}
