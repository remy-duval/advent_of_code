use super::*;

const ONE: &str = include_str!("example_1.txt");
const TWO: &str = include_str!("example_2.txt");
const THREE: &str = include_str!("example_3.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example_one() {
    let map = build_map(ONE);
    assert_eq!(first_part(&map), 18);
}

#[test]
fn first_part_example_two() {
    let map = build_map(TWO);
    assert_eq!(first_part(&map), 23);
}

#[test]
fn first_part_example_three() {
    let map = build_map(THREE);
    assert_eq!(first_part(&map), 31);
}

#[test]
fn first_part_main() {
    let map = build_map(MAIN);
    assert_eq!(first_part(&map), 3699);
}

#[test]
fn second_part_main() {
    let map = build_map(MAIN);
    assert_eq!(second_part(&map), 8517);
}
