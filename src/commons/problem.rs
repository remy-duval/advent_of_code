use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;

pub trait Problem {
    /// The type of the data that is required for the solving the problem
    type Input: FromStr;

    /// The type of the error that could be returned by the problem
    type Err;

    ///  The title of the problem
    const TITLE: &'static str;

    /// Parse the problem initial data
    /// ### Arguments
    /// * `input` The raw input as a string slice
    ///
    /// ### Returns
    /// Result containing the parsed data
    fn parse(input: &str) -> Result<Self::Input, <Self::Input as FromStr>::Err> {
        input.parse::<Self::Input>()
    }

    /// Solve the problem using the given input
    /// ### Arguments
    /// * `data` - The data that was parsed for for the problem
    ///
    /// ### Returns
    /// Result containing any error that happened during the solving process
    fn solve(data: Self::Input) -> Result<(), Self::Err>;
}

/// Parse the data and then solve the problem
/// ### Arguments
/// * `input` - The path to the input file for this problem
///
/// ### Returns
/// Result containing any error that happened during the parsing + solving process
pub fn parse_and_solve<Day>(input: PathBuf) -> Result<(), anyhow::Error>
where
    Day: Problem,
    <<Day as Problem>::Input as FromStr>::Err: std::fmt::Display,
    <Day as Problem>::Err: std::fmt::Display,
{
    println!("{}", super::CLEAR_COMMAND);
    println!("{}\n", Day::TITLE);
    let raw: String = fs_err::read_to_string(input).context("Reading input failure")?;
    match Day::parse(&raw) {
        Err(err) => Err(anyhow::anyhow!("Parsing failure: {}", err)),
        Ok(input) => {
            if let Err(err) = Day::solve(input) {
                Err(anyhow::anyhow!("Solving failure: {}", err))
            } else {
                Ok(())
            }
        }
    }
}
