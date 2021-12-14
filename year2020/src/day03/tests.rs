use super::*;

const A: &str = include_str!("example.txt");
const B: &str = include_str!("data.txt");

#[test]
fn first_part_test_a() {
    let forest = parse(A).unwrap();
    assert_eq!(7, first_part(&forest));
}

#[test]
fn first_part_test_b() {
    let forest = parse(B).unwrap();
    assert_eq!(286, first_part(&forest));
}

#[test]
fn second_part_test_a() {
    let forest = parse(A).unwrap();
    assert_eq!(336, second_part(&forest));
}

#[test]
fn second_part_test_b() {
    let forest = parse(B).unwrap();
    assert_eq!(3_638_606_400, second_part(&forest));
}
