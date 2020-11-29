//! Common utilities for the Advent of Code solutions

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::FromStr;

pub mod arguments;
pub mod grid;
pub mod math;
pub mod parse;

pub trait Problem {
    /// The type of the data that is required for the solving the problem
    type Input: FromStr;

    /// The type of the error that could be returned by the problem
    type Err;

    ///  The title of the problem
    const TITLE: &'static str;

    /// Solve the problem using the given input
    fn solve(data: Self::Input) -> Result<(), Self::Err>;

    /// Parse the data and then solve the problem
    fn parse_and_solve(
        raw: impl AsRef<str>,
    ) -> Result<(), ProblemError<<Self::Input as FromStr>::Err, Self::Err>> {
        println!("{}", Self::TITLE);
        let input = match raw.as_ref().parse::<Self::Input>() {
            Err(err) => return Err(ProblemError::ParseError(err)),
            Ok(ok) => ok,
        };
        if let Err(err) = Self::solve(input) {
            Err(ProblemError::SolveError(err))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
/// An error that can happen in
pub enum ProblemError<FromParsing, FromSolving> {
    ParseError(FromParsing),
    SolveError(FromSolving),
}

impl<FromParsing: Display, FromSolving: Display> Display
    for ProblemError<FromParsing, FromSolving>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ParseError(parse) => write!(f, "Can't parse the input:\n{}", parse),
            Self::SolveError(solve) => write!(f, "Can't solve the problem:\n{}", solve),
        }
    }
}

impl<FromParsing: Display + Debug, FromSolving: Display + Debug> Error
    for ProblemError<FromParsing, FromSolving>
{
}
