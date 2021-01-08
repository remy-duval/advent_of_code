use super::*;

const EXAMPLE_ONE: &str = include_str!("example_1.txt");
const EXAMPLE_TWO: &str = include_str!("example_2.txt");
const EXAMPLE_THREE: &str = include_str!("example_3.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example_a() {
    let mut network = Day::parse(EXAMPLE_ONE).unwrap();
    assert_eq!(first_part(&mut network), Point::new(7, 3));
}

#[test]
fn first_part_example_b() {
    let mut network = Day::parse(EXAMPLE_THREE).unwrap();
    assert_eq!(first_part(&mut network), Point::new(83, 49));
}

#[test]
fn first_part_main() {
    let mut network = Day::parse(MAIN).unwrap();
    assert_eq!(first_part(&mut network), Point::new(53, 133));
}

#[test]
fn second_part_example_a() {
    let mut network = Day::parse(EXAMPLE_TWO).unwrap();
    assert_eq!(second_part(&mut network), Point::new(6, 4));
}

#[test]
fn second_part_example_b() {
    let mut network = Day::parse(EXAMPLE_THREE).unwrap();
    assert_eq!(second_part(&mut network), Point::new(73, 36));
}

#[test]
fn second_part_main() {
    let mut network = Day::parse(MAIN).unwrap();
    assert_eq!(second_part(&mut network), Point::new(111, 68));
}
