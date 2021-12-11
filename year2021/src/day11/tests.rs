use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let grid = Day::parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&grid), 1_656);
}

#[test]
fn first_part_main() {
    let grid = Day::parse(MAIN).unwrap();
    assert_eq!(first_part(&grid), 1_686);
}

#[test]
fn second_part_example() {
    let mut grid = Day::parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&mut grid), 195);
}

#[test]
fn second_part_main() {
    let mut grid = Day::parse(MAIN).unwrap();
    assert_eq!(second_part(&mut grid), 360);
}
