use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let data = Day::parse(EXAMPLE).unwrap();
    let largest = data.largest_finite_area().unwrap();
    assert_eq!(largest, 17);
}

#[test]
fn first_part_main() {
    let data = Day::parse(MAIN).unwrap();
    let largest = data.largest_finite_area().unwrap();
    assert_eq!(largest, 3358);
}

#[test]
fn second_part_example() {
    let data = Day::parse(EXAMPLE).unwrap();
    let largest = data.near_points(32);
    assert_eq!(largest, 16);
}

#[test]
fn second_part_main() {
    let data = Day::parse(MAIN).unwrap();
    let largest = data.near_points(MAXIMUM_DISTANCE);
    assert_eq!(largest, 45_909);
}
