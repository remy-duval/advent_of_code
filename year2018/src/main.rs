fn main() -> commons::error::Result<()> {
    let args = commons::arguments::parse_arguments("Advent of Code 2018");
    advent_of_code_2018::solve_problem(args.day, args.input)
}
