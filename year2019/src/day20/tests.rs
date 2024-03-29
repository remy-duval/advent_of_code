use super::*;

const TEST_ONE: &str = include_str!("example_1.txt");
const TEST_TWO: &str = include_str!("example_2.txt");
const TEST_THREE: &str = include_str!("example_3.txt");
const DATA: &str = include_str!("data.txt");

#[test]
fn first_part_one() {
    let maze = Maze::parse(TEST_ONE);
    assert_eq!(23, first_part(&maze).unwrap());
}

#[test]
fn first_part_two() {
    let maze = Maze::parse(TEST_TWO);
    assert_eq!(58, first_part(&maze).unwrap());
}

#[test]
fn first_part_three() {
    let maze: Maze = Maze::parse(DATA);
    assert_eq!(552, first_part(&maze).unwrap());
}

#[test]
fn second_part_one() {
    let maze = Maze::parse(TEST_THREE);
    assert_eq!(396, second_part(&maze).unwrap());
}

#[test]
fn second_part_two() {
    let maze = Maze::parse(DATA);
    assert_eq!(6492, second_part(&maze).unwrap());
}
