use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let _ = parse(EXAMPLE).unwrap();
}

#[test]
fn first_part_main() {
    let _ = parse(MAIN).unwrap();
}