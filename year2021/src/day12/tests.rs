use super::*;

const EXAMPLE_ONE: &str = include_str!("example_1.txt");
const EXAMPLE_TWO: &str = include_str!("example_2.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example_1() {
    let paths = parse(EXAMPLE_ONE).unwrap();
    assert_eq!(first_part(&paths).unwrap(), 10);
}

#[test]
fn first_part_example_2() {
    let paths = parse(EXAMPLE_TWO).unwrap();
    assert_eq!(first_part(&paths).unwrap(), 226);
}

#[test]
fn first_part_main() {
    let paths = parse(MAIN).unwrap();
    assert_eq!(first_part(&paths).unwrap(), 5_252);
}

#[test]
fn second_part_example_1() {
    let paths = parse(EXAMPLE_ONE).unwrap();
    assert_eq!(second_part(&paths).unwrap(), 36);
}

#[test]
fn second_part_example_2() {
    let paths = parse(EXAMPLE_TWO).unwrap();
    assert_eq!(second_part(&paths).unwrap(), 3_509);
}

#[test]
fn second_part_main() {
    let paths = parse(MAIN).unwrap();
    assert_eq!(second_part(&paths).unwrap(), 147_784);
}
