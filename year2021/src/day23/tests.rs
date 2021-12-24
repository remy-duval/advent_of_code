use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let map = Positions::from(parse(EXAMPLE).unwrap());
    assert_eq!(map.a_star_search(), Some(12_521));
}

#[test]
fn first_part_main() {
    let map = Positions::from(parse(MAIN).unwrap());
    assert_eq!(map.a_star_search(), Some(19_160));
}

#[test]
fn second_part_example() {
    let map = add_rows(&parse(EXAMPLE).unwrap());
    assert_eq!(map.a_star_search(), Some(44_169));
}

#[test]
fn second_part_main() {
    let map = add_rows(&parse(MAIN).unwrap());
    assert_eq!(map.a_star_search(), Some(47_232));
}
