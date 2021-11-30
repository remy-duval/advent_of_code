use commons::arguments::{setup, Arguments};

fn main() -> color_eyre::Result<()> {
    let Arguments { day, input } = setup("Advent of Code 2021");
    advent_of_code_2021::solve_problem(day.0, &commons::load(input)?)
}