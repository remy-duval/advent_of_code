use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE_EXPECTED: &str = include_str!("example_expected.txt");
const MAIN: &str = include_str!("data.txt");
const MAIN_EXPECTED: &str = include_str!("data_expected.txt");

#[test]
fn example() {
    let message = parse(EXAMPLE).unwrap();
    let (message, time) = message.into_minimum_size();
    assert_eq!(time, 3);
    itertools::assert_equal(message.to_string().lines(), EXAMPLE_EXPECTED.lines());
}

#[test]
fn main() {
    let message = parse(MAIN).unwrap();
    let (message, time) = message.into_minimum_size();
    assert_eq!(time, 10_511);
    itertools::assert_equal(message.to_string().lines(), MAIN_EXPECTED.lines());
}
