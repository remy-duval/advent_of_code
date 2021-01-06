use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let input = Day::parse(EXAMPLE).unwrap();
    assert_eq!(first(&input), 10);
}

#[test]
fn first_part_main() {
    let input = Day::parse(MAIN).unwrap();
    assert_eq!(first(&input), 10978);
}

#[test]
fn second_part_example() {
    let input = Day::parse(EXAMPLE).unwrap();
    assert_eq!(second(&input), 4);
}

#[test]
fn second_part_main() {
    let input = Day::parse(MAIN).unwrap();
    assert_eq!(second(&input), 4840);
}
