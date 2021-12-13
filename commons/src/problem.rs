use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

use eyre::{eyre, Result};

pub trait Problem {
    /// The type of the data that is required for the solving the problem
    type Input: FromStr;

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
    fn solve(data: Self::Input) -> Result<()>;
}

/// Load the problem data from the given path
/// ### Arguments
/// * `input_path` - The path to the input file for this problem
pub fn load(input_path: PathBuf) -> Result<String> {
    std::fs::read_to_string(&input_path)
        .map_err(|err| eyre!("Could not read input in '{:?}' due to {}", input_path, err))
}

/// Solve the problem using the given raw input
/// ### Arguments
/// * `input` - The raw input as provided by the advent of code site
///
/// ### Returns
/// Result containing any error that happened during the parsing + solving process
pub fn solve<Day>(input: &str) -> Result<()>
where
    Day: Problem,
    <<Day as Problem>::Input as FromStr>::Err: std::fmt::Display,
{
    println!("{}", super::CLEAR_COMMAND);
    println!("{}\n", Day::TITLE);

    let time = Instant::now();
    let input = Day::parse(input).map_err(|err| eyre!("Parsing failure: {}", err))?;
    let parsing = time.elapsed();

    let solving = Instant::now();
    let _ = Day::solve(input).map_err(|err| eyre!("Solving failure: {}", err))?;
    let solving = solving.elapsed();
    let total = time.elapsed();

    println!("\n");
    println!("Parse: {:>7}μs", parsing.as_micros());
    println!("Solve: {:>7}μs", solving.as_micros());
    println!("Total: {:>7}μs", total.as_micros());

    Ok(())
}
