use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let rules = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&rules), 325);
}

#[test]
fn first_part_main() {
    let rules = parse(MAIN).unwrap();
    assert_eq!(first_part(&rules), 1_184);
}

#[test]
fn second_part_example() {
    let rules = parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&rules), 999_999_999_374);
}

#[test]
fn second_part_main() {
    let rules = parse(MAIN).unwrap();
    assert_eq!(second_part(&rules), 250_000_000_219);
}
