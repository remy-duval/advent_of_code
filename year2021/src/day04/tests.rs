use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let bingo = Day::parse(EXAMPLE).unwrap();
    assert_eq!(bingo.first_win_score().unwrap(), 4_512)
}

#[test]
fn first_part_main() {
    let bingo = Day::parse(MAIN).unwrap();
    assert_eq!(bingo.first_win_score().unwrap(), 14_093)
}

#[test]
fn last_part_example() {
    let bingo = Day::parse(EXAMPLE).unwrap();
    assert_eq!(bingo.last_win_score().unwrap(), 1_924)
}

#[test]
fn last_part_main() {
    let bingo = Day::parse(MAIN).unwrap();
    assert_eq!(bingo.last_win_score().unwrap(), 17_388)
}
