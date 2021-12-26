use super::*;

const MAIN: &str = include_str!("data.txt");

#[test]
fn main() {
    let (min, max) = search(MAIN).unwrap();
    assert_eq!(min, 18_116_121_134_117);
    assert_eq!(max, 39_999_698_799_429);
}
