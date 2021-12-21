use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    assert_eq!(first_part(parse(EXAMPLE).unwrap()), 739_785);
}

#[test]
fn first_part_main() {
    assert_eq!(first_part(parse(MAIN).unwrap()), 913_560);
}

#[test]
fn second_part_example() {
    assert_eq!(second_part(parse(EXAMPLE).unwrap()), 444_356_092_776_315);
}

#[test]
fn second_part_main() {
    assert_eq!(second_part(parse(MAIN).unwrap()), 110_271_560_863_819);
}
