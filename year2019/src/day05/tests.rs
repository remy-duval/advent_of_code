use super::*;

const DATA: &str = include_str!("data.txt");

#[test]
fn solve_test() {
    let memory = Day::parse(&DATA).unwrap().data;
    let (first, second) = solve(&memory[..]).unwrap();

    assert_eq!(15_386_262, first);
    assert_eq!(10_376_124, second);
}
