use super::*;

const ONE: &str = include_str!("example_1.txt");
const TWO: &str = include_str!("example_2.txt");
const THREE: &str = include_str!("example_3.txt");
const FOUR: &str = include_str!("example_4.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example_one() {
    let points = parse(ONE).unwrap().data;
    assert_eq!(count_constellations(&points), 2);
}

#[test]
fn first_part_example_two() {
    let points = parse(TWO).unwrap().data;
    assert_eq!(count_constellations(&points), 4);
}

#[test]
fn first_part_example_three() {
    let points = parse(THREE).unwrap().data;
    assert_eq!(count_constellations(&points), 3);
}

#[test]
fn first_part_example_four() {
    let points = parse(FOUR).unwrap().data;
    assert_eq!(count_constellations(&points), 8);
}

#[test]
fn first_part_main() {
    let points = parse(MAIN).unwrap().data;
    assert_eq!(count_constellations(&points), 310);
}
