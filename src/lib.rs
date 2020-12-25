#![forbid(unsafe_code)]

pub use commons::arguments::{Day, Year};
pub use commons::parse;
pub use commons::problem::parse_and_solve;
pub use commons::problem::Problem;

use crate::commons::arguments::AdventOfCode;

pub mod commons;
pub mod year2019;
pub mod year2020;

/// Solve a problem given its year, day and input
pub fn solve(advent: AdventOfCode) -> anyhow::Result<()> {
    match advent.year.0 {
        2019 => year2019::solve(advent.day, advent.input),
        2020 => year2020::solve(advent.day, advent.input),
        _ => Err(anyhow::anyhow!("{} is not implemented", advent.year)),
    }
}
