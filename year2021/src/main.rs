use advent_of_code_2021::{solve_all, solve_problem};
use commons::arguments::{parse_arguments, Arguments, Day};
use commons::eyre::Result;

fn main() -> Result<()> {
    let Arguments { day, input } = parse_arguments("Advent of Code 2021");
    match day {
        Day::All => solve_all(input),
        Day::Number(day) => solve_problem(day, input),
    }
}
