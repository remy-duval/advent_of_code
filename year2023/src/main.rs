fn main() -> commons::error::Result<()> {
    let args = commons::arguments::parse_arguments("Advent of Code 2023");
    advent_of_code_2023::solve_problem(args.day, args.input)
}
