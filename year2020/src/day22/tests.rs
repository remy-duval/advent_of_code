use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let mut game = Day::parse(EXAMPLE).unwrap();
    let result = game.normal_play();
    assert_eq!(result, 306);
}

#[test]
fn first_part_main() {
    let mut game = Day::parse(MAIN).unwrap();
    let result = game.normal_play();
    assert_eq!(result, 34_664);
}

#[test]
fn second_part_example() {
    let mut game = Day::parse(EXAMPLE).unwrap();
    let result = game.advanced_play();
    assert_eq!(result, 291);
}

#[test]
fn second_part_main() {
    let mut game = Day::parse(MAIN).unwrap();
    let result = game.advanced_play();
    assert_eq!(result, 32_018);
}
