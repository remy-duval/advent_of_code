use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn display_test() {
    let mut cavern = parse(EXAMPLE).unwrap();
    let _ = cavern.risk_level(); // To compute all tiles in the (0, 0) to the target
    itertools::assert_equal(
        cavern.to_string().lines(),
        include_str!("example_expected.txt").lines(),
    );
}

#[test]
fn first_part_example() {
    let mut cavern = parse(EXAMPLE).unwrap();
    assert_eq!(cavern.risk_level(), 114);
}

#[test]
fn first_part_main() {
    let mut cavern = parse(MAIN).unwrap();
    assert_eq!(cavern.risk_level(), 6_256);
}

#[test]
fn second_part_example() {
    let mut cavern = parse(EXAMPLE).unwrap();
    let shortest = shortest_path(&mut cavern);
    assert_eq!(shortest, Some(45));
}

#[test]
fn second_part_main() {
    let mut cavern = parse(MAIN).unwrap();
    let shortest = shortest_path(&mut cavern);
    assert_eq!(shortest, Some(973));
}
