use super::*;

const EXAMPLE: &str = include_str!("example_1.txt");
const EXAMPLE_TWO: &str = include_str!("example_2.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_test_example() {
    let input = parse(EXAMPLE);
    let result = first_part(&input).unwrap();
    assert_eq!(result, 2);
}

#[test]
fn first_part_test_example_two() {
    let input = parse(EXAMPLE_TWO);
    let result = first_part(&input).unwrap();
    assert_eq!(result, 3);
}

#[test]
fn first_part_test_main() {
    let input = parse(MAIN);
    let result = first_part(&input).unwrap();
    assert_eq!(result, 195);
}

#[test]
fn second_part_test_example_two() {
    let input = parse(EXAMPLE_TWO);
    let result = second_part(input).unwrap();
    assert_eq!(result, 12);
}

#[test]
fn second_part_test_main() {
    let input = parse(MAIN);
    let result = second_part(input).unwrap();
    assert_eq!(result, 309);
}
