use std::fmt::{Debug, Display};
use std::path::PathBuf;
use std::str::FromStr;

use clap::{value_parser, Arg, ArgAction, Command};
use eyre::{eyre, Report, Result, WrapErr};

/// Parse the advent of code arguments
pub fn parse_arguments(name: &'static str) -> Arguments {
    let mut matches = Command::new(name)
        .about("Solutions for the advent of code problems")
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .value_name("DAY")
                .help("The specific day of the problem or 'all'")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(value_parser!(Day)),
        )
        .arg(
            Arg::new("input")
                .long("input")
                .value_name("FILE")
                .help("The problem's input. If day is 'all', a directory from 01.txt to 25.txt")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    Arguments {
        day: matches.remove_one("day").expect("valid day is required"),
        input: matches.remove_one("input").expect("'input' is required"),
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

/// The Day of the Advent of Code problem to solve (between 01 and 25 or all)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Day {
    All,
    Number(u8),
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "All days"),
            Self::Number(n) => write!(f, "Day {n}"),
        }
    }
}

impl FromStr for Day {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "all" {
            Ok(Self::All)
        } else {
            s.parse::<u8>()
                .wrap_err_with(|| format!("For number: {s}"))
                .and_then(|day| {
                    if day > 0 && day <= 25 {
                        Ok(Self::Number(day))
                    } else {
                        Err(eyre!("Day must be between 1 and 25"))
                    }
                })
                .wrap_err("Should be 'all' or a number between 1 and 25")
        }
    }
}
