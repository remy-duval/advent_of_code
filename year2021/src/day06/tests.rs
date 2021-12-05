use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn simulate_example() {
    let data = Day::parse(EXAMPLE).unwrap();
    assert_eq!(simulate(&data.data, 18), 26)
}

#[test]
fn first_part_example() {
    let data = Day::parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&data.data), 5_934);
}

#[test]
fn first_part_main() {
    let data = Day::parse(MAIN).unwrap();
    assert_eq!(first_part(&data.data), 360_268);
}

#[test]
fn second_part_example() {
    let data = Day::parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&data.data), 26_984_457_539);
}

#[test]
fn second_part_main() {
    let data = Day::parse(MAIN).unwrap();
    assert_eq!(second_part(&data.data), 1_632_146_183_902);
}
