use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let mut program = parse(EXAMPLE).unwrap();
    let last = program.run().unwrap();
    assert_eq!(last, 7);
}

#[test]
fn first_part_main() {
    let program = parse(MAIN).unwrap();
    let last = run_optimized(program, 0).unwrap();
    assert_eq!(last, 2_520);
}

#[test]
fn second_part_main() {
    let program = parse(MAIN).unwrap();
    let last = run_optimized(program, 1).unwrap();
    assert_eq!(last, 27_941_760);
}
