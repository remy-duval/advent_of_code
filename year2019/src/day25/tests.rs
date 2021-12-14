use super::*;

const A: &str = include_str!("data_A.txt");
const B: &str = include_str!("data_B.txt");
const C: &str = include_str!("data_C.txt");

#[test]
fn solve_test_a() {
    let input = parse(A).unwrap().data;
    let result = auto_play(&input).unwrap();
    assert_eq!(result, 33_624_080);
}

#[test]
fn solve_test_b() {
    let input = parse(B).unwrap().data;
    let result = auto_play(&input).unwrap();
    assert_eq!(result, 328_960);
}

#[test]
fn solve_test_c() {
    let input = parse(C).unwrap().data;
    let result = auto_play(&input).unwrap();
    assert_eq!(result, 262_848);
}
