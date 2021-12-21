use advent_of_code_2019::solve_problem;
use commons::arguments::{parse_arguments, Arguments, Day};
use commons::eyre::{eyre, Result};

fn main() -> Result<()> {
    let Arguments { day, input } = parse_arguments("Advent of Code 2019");
    match day {
        Day::All => Err(eyre!("--day all is not implemented")),
        Day::Number(day) => solve_problem(day, input),
    }
}
