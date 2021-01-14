use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn example() {
    let mut scan = Day::parse(EXAMPLE).unwrap();
    scan.fill();
    assert_eq!(scan.wet_tiles(), 57);
    assert_eq!(scan.water(), 29);
}

#[test]
fn main() {
    let mut scan = Day::parse(MAIN).unwrap();
    scan.fill();
    assert_eq!(scan.wet_tiles(), 36_171);
    assert_eq!(scan.water(), 28_204);
}
