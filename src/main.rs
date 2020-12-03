use structopt::StructOpt;

use advent_of_code::commons::arguments::AdventOfCode;

fn main() -> anyhow::Result<()> {
    advent_of_code::solve(AdventOfCode::from_args())?;
    Ok(())
}
