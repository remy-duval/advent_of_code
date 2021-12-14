use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let area = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(area).trees_and_lumberyards(), (37, 31));
}

#[test]
fn first_part_main() {
    let area = parse(MAIN).unwrap();
    assert_eq!(first_part(area).trees_and_lumberyards(), (825, 553));
}

#[test]
fn second_part_main() {
    let area = parse(MAIN).unwrap();
    assert_eq!(second_part(area).unwrap(), 190_164);
}
