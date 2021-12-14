use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let input = parse(EXAMPLE);
    let result = first_part(input);
    assert_eq!(result, 112);
}

#[test]
fn first_part_main() {
    let input = parse(MAIN);
    let result = first_part(input);
    assert_eq!(result, 301);
}

#[test]
fn second_part_example() {
    let input = parse(EXAMPLE);
    let result = second_part(input);
    assert_eq!(result, 848);
}

#[test]
fn second_part_main() {
    let input = parse(MAIN);
    let result = second_part(input);
    assert_eq!(result, 2424);
}
