use std::fmt::{Debug, Display};
use std::path::PathBuf;
use std::str::FromStr;

use clap::{value_t, App, Arg};
use eyre::{eyre, Report, Result};

/// Parse the advent of code arguments
pub fn setup(name: &str) -> Arguments {
    let matches = App::new(name)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Solutions for the advent of code problems")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("The specific day of the problem")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .long("input")
                .value_name("FILE")
                .help("The input for that day problem")
                .takes_value(true),
        )
        .get_matches();

    Arguments {
        day: clap::value_t!(matches, "day", Day).unwrap_or_else(|e| e.exit()),
        input: clap::value_t!(matches, "input", PathBuf).unwrap_or_else(|e| e.exit()),
    }
}

/// The Advent of Code arguments
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Arguments {
    /// The specific day of the problem
    pub day: Day,
    /// The input for that day problem
    pub input: PathBuf,
}

/// The Day of the Advent of Code problem to solve (between 01 and 25)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Day(pub u8);

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {}", self.0)
    }
}

impl FromStr for Day {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>()? {
            day if day > 0 && day <= 25 => Ok(Day(day)),
            _ => Err(eyre!("Day must be between 1 and 25")),
        }
    }
}
