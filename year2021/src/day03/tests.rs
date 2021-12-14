use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let data = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&data), 198)
}

#[test]
fn first_part_main() {
    let data = parse(MAIN).unwrap();
    assert_eq!(first_part(&data), 2_583_164)
}

#[test]
fn second_part_example() {
    let data = parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&data).expect("failure for example"), 230)
}

#[test]
fn second_part_main() {
    let data = parse(MAIN).unwrap();
    assert_eq!(second_part(&data).expect("failure for main"), 2_784_375)
}
