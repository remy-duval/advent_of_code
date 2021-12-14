use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    assert_eq!(first(EXAMPLE), 10);
}

#[test]
fn first_part_main() {
    assert_eq!(first(MAIN), 10978);
}

#[test]
fn second_part_example() {
    assert_eq!(second(EXAMPLE), 4);
}

#[test]
fn second_part_main() {
    assert_eq!(second(MAIN), 4840);
}
