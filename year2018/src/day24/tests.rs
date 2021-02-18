use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let battle = Day::parse(EXAMPLE).unwrap();
    assert_eq!(first_part(battle), 5_216);
}

#[test]
fn first_part_main() {
    let battle = Day::parse(MAIN).unwrap();
    assert_eq!(first_part(battle), 19_295);
}

#[test]
fn second_part_example() {
    let battle = Day::parse(EXAMPLE).unwrap();
    assert_eq!(second_part(battle), 51);
}

#[test]
fn second_part_main() {
    let battle = Day::parse(MAIN).unwrap();
    assert_eq!(second_part(battle), 12_084);
}
