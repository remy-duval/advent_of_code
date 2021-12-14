use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let mut data = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&mut data.data), 37);
}

#[test]
fn first_part_main() {
    let mut data = parse(MAIN).unwrap();
    assert_eq!(first_part(&mut data.data), 356_958);
}

#[test]
fn second_part_example() {
    let data = parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&data.data), 168);
}

#[test]
fn second_part_main() {
    let data = parse(MAIN).unwrap();
    assert_eq!(second_part(&data.data), 105_461_913);
}
