use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let segments = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&segments.data), 5);
}

#[test]
fn first_part_main() {
    let segments = parse(MAIN).unwrap();
    assert_eq!(first_part(&segments.data), 6_397);
}

#[test]
fn second_part_example() {
    let segments = parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&segments.data), 12);
}

#[test]
fn second_part_main() {
    let segments = parse(MAIN).unwrap();
    assert_eq!(second_part(&segments.data), 22_335);
}
