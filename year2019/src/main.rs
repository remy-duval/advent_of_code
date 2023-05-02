fn main() -> commons::error::Result<()> {
    let args = commons::arguments::parse_arguments("Advent of Code 2019");
    advent_of_code_2019::solve_problem(args.day, args.input)
}
