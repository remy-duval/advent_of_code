use commons::arguments::{setup, Arguments};

fn main() -> commons::eyre::Result<()> {
    let Arguments { day, input } = setup("Advent of Code 2019");
    advent_of_code_2019::solve_problem(day.0, &commons::load(input)?)
}
