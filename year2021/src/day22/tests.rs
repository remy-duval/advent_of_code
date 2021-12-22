use super::*;

const SIMPLE: &str = include_str!("simple.txt");
const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn simple() {
    let boot = parse(SIMPLE).unwrap();
    assert_eq!(first_part(&boot), 39);
}

#[test]
fn first_part_example() {
    let boot = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&boot), 474_140);
}

#[test]
fn first_part_main() {
    let boot = parse(MAIN).unwrap();
    assert_eq!(first_part(&boot), 537_042);
}

#[test]
fn second_part_example() {
    let boot = parse(EXAMPLE).unwrap();
    assert_eq!(all_points(boot), 2_758_514_936_282_235);
}

#[test]
fn second_part_main() {
    let boot = parse(MAIN).unwrap();
    assert_eq!(all_points(boot), 1_304_385_553_084_863);
}
