use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let grid = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&grid), 40);
}

#[test]
fn first_part_main() {
    let grid = parse(MAIN).unwrap();
    assert_eq!(first_part(&grid), 707);
}

#[test]
fn second_part_example() {
    let grid = parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&grid), 315);
}

#[test]
fn second_part_main() {
    let grid = parse(MAIN).unwrap();
    assert_eq!(second_part(&grid), 2_942);
}
