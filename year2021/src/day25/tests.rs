use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let steps = move_until_deadlock(parse(EXAMPLE).unwrap());
    assert_eq!(steps, 58);
}

#[test]
fn first_part_main() {
    let steps = move_until_deadlock(parse(MAIN).unwrap());
    assert_eq!(steps, 568);
}
