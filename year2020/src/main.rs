use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Advent of Code 2020",
    author = "Remy Duval",
    about = "Solutions for the advent of code problems of year 2020"
)]
struct Arguments {
    #[structopt(short, long)]
    /// The specific day of the problem
    day: commons::Day,
    /// The input for that day problem
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let Arguments { day, input } = Arguments::from_args();
    advent_of_code_2020::solve_problem(day.0, &commons::load(input)?)
}
