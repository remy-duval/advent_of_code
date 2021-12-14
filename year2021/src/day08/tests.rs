use super::*;

const SMALL: &str = include_str!("small.txt");
const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let data = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&data.data), 26);
}

#[test]
fn first_part_main() {
    let data = parse(MAIN).unwrap();
    assert_eq!(first_part(&data.data), 284);
}

#[test]
fn second_part_small() {
    let data = parse(SMALL).unwrap();
    assert_eq!(second_part(&data.data).unwrap(), 5_353);
}

#[test]
fn second_part_example() {
    let data = parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&data.data).unwrap(), 61_229);
}

#[test]
fn second_part_main() {
    let data = parse(MAIN).unwrap();
    assert_eq!(second_part(&data.data).unwrap(), 973_499);
}
