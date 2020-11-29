pub use commons::arguments::{Day, Year};
pub use commons::parse;
pub use commons::Problem;

pub mod commons;
pub mod year2019;
pub mod year2020;

/// Solve a problem given its year, day and input
pub fn solve(year: Year, day: Day, input: String) -> anyhow::Result<()> {
    match year.0 {
        2019 => year2019::solve(day, input),
        2020 => year2020::solve(day, input),
        _ => Err(anyhow::anyhow!("{} is not implemented", year)),
    }
}
