use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let lines = Day::parse(EXAMPLE).unwrap();
    assert_eq!(check_all(&lines).0, 26_397);
}

#[test]
fn first_part_main() {
    let lines = Day::parse(MAIN).unwrap();
    assert_eq!(check_all(&lines).0, 442_131);
}

#[test]
fn second_part_example() {
    let lines = Day::parse(EXAMPLE).unwrap();
    assert_eq!(check_all(&lines).1, 288_957);
}

#[test]
fn second_part_main() {
    let lines = Day::parse(MAIN).unwrap();
    assert_eq!(check_all(&lines).1, 3_646_451_424);
}
