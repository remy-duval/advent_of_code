use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Advent of Code",
    author = "Remy Duval",
    about = "Solutions for the advent of code problems"
)]
pub struct AdventOfCode {
    #[structopt(short, long)]
    /// The year of the problem
    pub year: Year,
    #[structopt(short, long)]
    /// The specific day of the problem
    pub day: Day,

    /// The input for that day problem
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// The Year of the Advent of Code problem to solve
pub struct Year(pub u16);

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Year {}", self.0)
    }
}

impl FromStr for Year {
    type Err = ParseDayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: u16 = s.parse()?;
        // In case we only get the last digits, assume it is the 2XXX
        if parsed < 1000 {
            Ok(Year(2000 + parsed))
        } else {
            Ok(Year(parsed))
        }
    }
}

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
