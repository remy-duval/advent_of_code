use std::path::PathBuf;
use std::time::Instant;

use eyre::{Result, WrapErr};

/// Solve the problem using the given input, displaying the title and time of completion.
///
/// # Arguments
/// * `title` - The title of the day
/// * `path` - The path to the input for the day
/// * `solve` - The solving process from the loaded input
///
/// # Returns
/// Err if any error happened during input loading or solving
pub fn solve_verbose<Solver>(title: &str, path: PathBuf, solve: Solver) -> Result<()>
where
    Solver: FnOnce(String) -> Result<()>,
{
    println!("{}\n{}\n", super::CLEAR_COMMAND, title);
    let input = load(path)?;
    let start = Instant::now();
    solve(input)?;
    let elapsed = start.elapsed();
    println!("\n\nSolve time: {:}μs", elapsed.as_micros());
    Ok(())
}

/// Solve the problem using the given input, displaying only the day number
///
/// # Arguments
/// * `day` - The day number
/// * `path` - The path to the input for the day
/// * `solve` - The solving process from the loaded input
///
/// # Returns
/// Err if any error happened during input loading or solving
pub fn solve_quiet<Solver>(day: u8, path: PathBuf, solve: Solver) -> Result<()>
where
    Solver: FnOnce(String) -> Result<()>,
{
    println!("Day {}:", day);
    solve(load(path)?)
}

/// Load the problem data from the given path
/// ### Arguments
/// * `input_path` - The path to the input file for this problem
pub fn load(path: PathBuf) -> Result<String> {
    std::fs::read_to_string(&path).wrap_err_with(|| format!("Can't load input from '{:?}'", path))
}
