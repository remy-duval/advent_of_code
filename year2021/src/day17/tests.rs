use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let target = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&target), 45);
}

#[test]
fn first_part_main() {
    let target = parse(MAIN).unwrap();
    assert_eq!(first_part(&target), 2_701);
}

#[test]
fn second_part_example() {
    let target = parse(EXAMPLE).unwrap();
    assert_eq!(second_part(target), 112);
}

#[test]
fn second_part_main() {
    let target = parse(MAIN).unwrap();
    assert_eq!(second_part(target), 1_070);
}
