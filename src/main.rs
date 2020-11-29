use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let arguments = advent_of_code::commons::arguments::AdventOfCode::from_args();
    let input = std::fs::read_to_string(arguments.input)?;
    advent_of_code::solve(arguments.year, arguments.day, input)?;
    Ok(())
}
