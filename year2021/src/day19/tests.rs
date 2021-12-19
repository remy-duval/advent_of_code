use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn example() {
    let (scanners, beacons) = scan(parse(EXAMPLE).unwrap());
    assert_eq!(beacons.len(), 79);
    assert_eq!(max_distance(&scanners), 3_621);
}

#[test]
fn main() {
    let (scanners, beacons) = scan(parse(MAIN).unwrap());
    assert_eq!(beacons.len(), 385);
    assert_eq!(max_distance(&scanners), 10_707);
}
