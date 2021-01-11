use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn example() {
    let program = Day::parse(EXAMPLE).unwrap();
    let (first, _) = find_possible(&program.samples);
    assert_eq!(first, 1);
}

#[test]
fn main() {
    let program = Day::parse(MAIN).unwrap();
    let (first, possible) = find_possible(&program.samples);
    assert_eq!(first, 677);

    let found = sieve(possible);
    assert_eq!(program.execute(&found).0[0], 540);
}
