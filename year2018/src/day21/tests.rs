use super::*;

const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_main() {
    let mut program = Day::parse(MAIN).unwrap();
    let first = first_exit_value(&mut program).unwrap().unwrap();
    assert_eq!(first, 16_311_888);
}

#[test]
fn second_part_main() {
    let mut program = Day::parse(MAIN).unwrap();
    let last = last_exit_value(&mut program).unwrap().unwrap();
    assert_eq!(last, 1_413_889);
}
