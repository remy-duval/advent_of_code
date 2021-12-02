use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let data = Day::parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&data.data), 150);
}

#[test]
fn first_part_main() {
    let data = Day::parse(MAIN).unwrap();
    assert_eq!(first_part(&data.data), 1_561_344);
}

#[test]
fn second_part_example() {
    let data = Day::parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&data.data), 900);
}

#[test]
fn second_part_main() {
    let data = Day::parse(MAIN).unwrap();
    assert_eq!(second_part(&data.data), 1_848_454_425);
}
