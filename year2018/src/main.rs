use advent_of_code_2018::{solve_all, solve_problem};
use commons::arguments::{parse_arguments, Arguments, Day};
use commons::eyre::Result;

fn main() -> Result<()> {
    let Arguments { day, input } = parse_arguments("Advent of Code 2018");
    match day {
        Day::All => solve_all(input),
        Day::Number(day) => solve_problem(day, input),
    }
}
