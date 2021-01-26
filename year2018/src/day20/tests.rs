use super::*;

const ONE: &str = include_str!("example_1.txt");
const TWO: &str = include_str!("example_2.txt");
const THREE: &str = include_str!("example_3.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example_one() {
    let regex = Day::parse(ONE).unwrap();
    let map = build_map(&regex);
    assert_eq!(first_part(&map), 18);
}

#[test]
fn first_part_example_two() {
    let regex = Day::parse(TWO).unwrap();
    let map = build_map(&regex);
    assert_eq!(first_part(&map), 23);
}

#[test]
fn first_part_example_three() {
    let regex = Day::parse(THREE).unwrap();
    let map = build_map(&regex);
    assert_eq!(first_part(&map), 31);
}

#[test]
fn first_part_main() {
    let regex = Day::parse(MAIN).unwrap();
    let map = build_map(&regex);
    assert_eq!(first_part(&map), 3699);
}

#[test]
fn second_part_main() {
    let regex = Day::parse(MAIN).unwrap();
    let map = build_map(&regex);
    assert_eq!(second_part(&map), 8517);
}
