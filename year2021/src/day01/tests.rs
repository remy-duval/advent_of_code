use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let data = Day::parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&data.data), 7);
}

#[test]
fn first_part_main() {
    let data = Day::parse(MAIN).unwrap();
    assert_eq!(first_part(&data.data), 1624);
}

#[test]
fn second_part_example() {
    let data = Day::parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&data.data), 5);
}

#[test]
fn second_part_main() {
    let data = Day::parse(MAIN).unwrap();
    assert_eq!(second_part(&data.data), 1653);
}
