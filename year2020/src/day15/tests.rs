use super::*;

const EXAMPLE: &str = "0,3,6";
const A: &str = "1,3,2";
const B: &str = "2,1,3";
const C: &str = "1,2,3";
const D: &str = "2,3,1";
const E: &str = "3,2,1";
const F: &str = "3,1,2";
const INPUT: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let input = parse(EXAMPLE).unwrap();
    assert_eq!(nth_spoken_number(&input.data, 4), 0);
    assert_eq!(nth_spoken_number(&input.data, 5), 3);
    assert_eq!(nth_spoken_number(&input.data, 6), 3);
    assert_eq!(nth_spoken_number(&input.data, 7), 1);
    assert_eq!(nth_spoken_number(&input.data, 8), 0);
    assert_eq!(nth_spoken_number(&input.data, 9), 4);
    assert_eq!(nth_spoken_number(&input.data, 10), 0);
}

#[test]
fn first_part_test_a() {
    let input = parse(A).unwrap();
    let result = nth_spoken_number(&input.data, FIRST_TURNS);
    assert_eq!(result, 1);
}

#[test]
fn first_part_test_b() {
    let input = parse(B).unwrap();
    let result = nth_spoken_number(&input.data, FIRST_TURNS);
    assert_eq!(result, 10);
}

#[test]
fn first_part_test_c() {
    let input = parse(C).unwrap();
    let result = nth_spoken_number(&input.data, FIRST_TURNS);
    assert_eq!(result, 27);
}

#[test]
fn first_part_test_d() {
    let input = parse(D).unwrap();
    let result = nth_spoken_number(&input.data, FIRST_TURNS);
    assert_eq!(result, 78);
}

#[test]
fn first_part_test_e() {
    let input = parse(E).unwrap();
    let result = nth_spoken_number(&input.data, FIRST_TURNS);
    assert_eq!(result, 438);
}

#[test]
fn first_part_test_f() {
    let input = parse(F).unwrap();
    let result = nth_spoken_number(&input.data, FIRST_TURNS);
    assert_eq!(result, 1836);
}

#[test]
fn first_part_test_input() {
    let input = parse(INPUT).unwrap();
    let result = nth_spoken_number(&input.data, FIRST_TURNS);
    assert_eq!(result, 447);
}

#[test]
fn second_part_test_input() {
    let input = parse(INPUT).unwrap();
    let result = nth_spoken_number(&input.data, SECOND_TURNS);
    assert_eq!(result, 11_721_679);
}
