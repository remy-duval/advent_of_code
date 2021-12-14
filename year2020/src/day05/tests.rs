use super::*;

/// row 70, column 7, seat ID 567
const FIRST: &str = "BFFFBBFRRR";

/// row 14, column 7, seat ID 119
const SECOND: &str = "FFFBBBFRRR";

/// row 102, column 4, seat ID 820
const THIRD: &str = "BBFFBBFRLL";

const DATA: &str = include_str!("data.txt");

#[test]
fn parsing_first_example() {
    let pass: BoardingPass = FIRST.parse().unwrap();
    assert_eq!(567, pass.seat);
}

#[test]
fn parsing_second_example() {
    let pass: BoardingPass = SECOND.parse().unwrap();
    assert_eq!(119, pass.seat);
}

#[test]
fn parsing_third_example() {
    let pass: BoardingPass = THIRD.parse().unwrap();
    assert_eq!(820, pass.seat);
}

#[test]
fn ordering_test() {
    let first: BoardingPass = FIRST.parse().unwrap();
    let second: BoardingPass = SECOND.parse().unwrap();
    let third: BoardingPass = THIRD.parse().unwrap();

    assert!(first > second);
    assert!(third > first);
}

#[test]
fn first_part_test() {
    let passes = parse(DATA).unwrap();
    let max = first_part(&passes.data).unwrap();

    assert_eq!(866, max);
}

#[test]
fn second_part_test() {
    let passes = parse(DATA).unwrap();
    let missing = second_part(passes.data).unwrap();

    assert_eq!(583, missing);
}
