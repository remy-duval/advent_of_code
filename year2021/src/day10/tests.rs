use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    assert_eq!(check_all(EXAMPLE).0, 26_397);
}

#[test]
fn first_part_main() {
    assert_eq!(check_all(MAIN).0, 442_131);
}

#[test]
fn second_part_example() {
    assert_eq!(check_all(EXAMPLE).1, 288_957);
}

#[test]
fn second_part_main() {
    assert_eq!(check_all(MAIN).1, 3_646_451_424);
}
