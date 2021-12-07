use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let mut data = Day::parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&mut data.data).unwrap(), 37);
}

#[test]
fn first_part_main() {
    let mut data = Day::parse(MAIN).unwrap();
    assert_eq!(first_part(&mut data.data).unwrap(), 356_958);
}

#[test]
fn second_part_example() {
    let data = Day::parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&data.data).unwrap(), 168);
}

#[test]
fn second_part_main() {
    let data = Day::parse(MAIN).unwrap();
    assert_eq!(second_part(&data.data).unwrap(), 105_461_913);
}
