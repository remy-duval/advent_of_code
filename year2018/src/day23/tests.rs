use super::*;

const ONE: &str = include_str!("example_1.txt");
const TWO: &str = include_str!("example_2.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let bots = parse(ONE).unwrap().data;
    let first = first_part(&bots);
    assert_eq!(first, 7);
}

#[test]
fn first_part_main() {
    let bots = parse(MAIN).unwrap().data;
    let first = first_part(&bots);
    assert_eq!(first, 510);
}

#[test]
fn second_part_example() {
    let bots = parse(TWO).unwrap().data;
    let second = second_part(&bots);
    assert_eq!(second, 36);
}

#[test]
fn second_part_main() {
    let bots = parse(MAIN).unwrap().data;
    let second = second_part(&bots);
    assert_eq!(second, 108_889_300);
}
