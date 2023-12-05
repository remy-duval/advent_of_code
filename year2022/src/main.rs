fn main() -> commons::error::Result<()> {
    let args = commons::arguments::parse_arguments("Advent of Code 2022");
    advent_of_code_2022::solve_problem(args.day, args.input)
}
