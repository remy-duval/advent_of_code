use super::*;

const DATA: &str = include_str!("data.txt");

#[test]
fn solve_test() {
    let memory: Vec<i64> = Day::parse(DATA).unwrap().data;
    let first = run_one(&memory, 12, 2).expect("1202 program error");
    let (noun, verb) = find_match(&memory, WANTED).expect("Finding second program error");

    assert_eq!(3_409_710, first);
    assert_eq!(7_912, noun * 100 + verb);
}
