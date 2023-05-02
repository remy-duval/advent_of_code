fn main() -> commons::error::Result<()> {
    let args = commons::arguments::parse_arguments("Advent of Code 2020");
    advent_of_code_2020::solve_problem(args.day, args.input)
}
