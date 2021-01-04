use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example_a() {
    let cups = Day::parse(EXAMPLE).unwrap().0;
    let mut ring = CupRing::new(&cups, 9);
    ring.nth(10);
    assert_eq!(ring.to_string(), "92658374");
}

#[test]
fn first_part_example_b() {
    let cups = Day::parse(EXAMPLE).unwrap().0;
    let result = first_part(&cups);
    assert_eq!(result, "67384529");
}

#[test]
fn first_part_main() {
    let cups = Day::parse(MAIN).unwrap().0;
    let result = first_part(&cups);
    assert_eq!(result, "54327968");
}

#[test]
fn second_part_example() {
    let cups = Day::parse(EXAMPLE).unwrap().0;
    let (first, second) = second_part(&cups);
    assert_eq!(first * second, 149_245_887_792);
}

#[test]
fn second_part_main() {
    let cups = Day::parse(MAIN).unwrap().0;
    let (first, second) = second_part(&cups);
    assert_eq!(first * second, 157_410_423_276);
}
