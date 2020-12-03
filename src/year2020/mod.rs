//! The solutions to the problems of the Advent of Code of 2020

use crate::Day;
use crate::Problem;

mod day01;
mod day02;
mod day03;

/// Solve a problem in the year 2020
pub fn solve(day: Day, input: String) -> anyhow::Result<()> {
    match day.0 {
        1 => day01::Day01::parse_and_solve(input)?,
        2 => day02::Day02::parse_and_solve(input)?,
        3 => day03::Day03::parse_and_solve(input)?,
        _ => return Err(anyhow::anyhow!("{} is not implemented", day)),
    }
    Ok(())
}
