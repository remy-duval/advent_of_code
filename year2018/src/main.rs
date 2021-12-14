use commons::arguments::{setup, Arguments};

fn main() -> commons::eyre::Result<()> {
    let Arguments { day, input } = setup("Advent of Code 2018");
    advent_of_code_2018::solve_problem(day.0, input)
}
