use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn example() {
    let (enhancer, mut image) = parse(EXAMPLE).unwrap();
    let (first, second) = enhance(&mut image, &enhancer);
    assert_eq!(first, 35);
    assert_eq!(second, 3_351);
}

#[test]
fn main() {
    let (enhancer, mut image) = parse(MAIN).unwrap();
    let (first, second) = enhance(&mut image, &enhancer);
    assert_eq!(first, 5_489);
    assert_eq!(second, 19_066);
}
