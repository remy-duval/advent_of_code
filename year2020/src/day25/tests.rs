use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn example() {
    let encryption = solve(Day::parse(EXAMPLE).unwrap());
    assert_eq!(encryption, 14_897_079);
}

#[test]
fn main() {
    let encryption = solve(Day::parse(MAIN).unwrap());
    assert_eq!(encryption, 297_257);
}
