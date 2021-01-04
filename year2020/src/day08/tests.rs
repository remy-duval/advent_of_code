use super::*;

const A: &str = include_str!("example.txt");
const B: &str = include_str!("data.txt");

#[test]
fn first_part_a() {
    let data = Day::parse(A).unwrap();
    let mut state = ProgramState::new(data.data);

    let (pos, acc) = run_until_duplicate_execution(&mut state);
    assert_eq!(5, acc);
    assert_eq!(4, pos);
}

#[test]
fn first_part_b() {
    let data = Day::parse(B).unwrap();
    let mut state = ProgramState::new(data.data);

    let (pos, acc) = run_until_duplicate_execution(&mut state);
    assert_eq!(1586, acc);
    assert_eq!(463, pos);
}

#[test]
fn second_part_a() {
    let data = Day::parse(A).unwrap();
    let mut state = ProgramState::new(data.data);

    let (pos, acc) = replace_and_run(&mut state);
    assert_eq!(9, pos, "did not reach the end of the instructions");
    assert_eq!(8, acc, "bad accumulator value");
}

#[test]
fn second_part_b() {
    let data = Day::parse(B).unwrap();
    let mut state = ProgramState::new(data.data);

    let (pos, acc) = replace_and_run(&mut state);
    assert_eq!(641, pos, "did not reach the end of the instructions");
    assert_eq!(703, acc, "bad accumulator value");
}