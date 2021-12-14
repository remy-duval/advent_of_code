use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let result = parse(EXAMPLE).unwrap().normal_play();
    assert_eq!(result, 306);
}

#[test]
fn first_part_main() {
    let result = parse(MAIN).unwrap().normal_play();
    assert_eq!(result, 34_664);
}

#[test]
fn second_part_example() {
    let result = parse(EXAMPLE).unwrap().advanced_play();
    assert_eq!(result, 291);
}

#[test]
fn second_part_main() {
    let result = parse(MAIN).unwrap().advanced_play();
    assert_eq!(result, 32_018);
}
