use super::*;

const A: &str = include_str!("example.txt");
const B: &str = include_str!("data.txt");

#[test]
fn first_part_test_a() {
    let data = parse(A).unwrap();
    assert_eq!(2, first_part(&data.data));
}

#[test]
fn first_part_test_b() {
    let data = parse(B).unwrap();
    assert_eq!(600, first_part(&data.data));
}

#[test]
fn second_part_test_a() {
    let data = parse(A).unwrap();
    assert_eq!(1, second_part(&data.data));
}

#[test]
fn second_part_test_b() {
    let data = parse(B).unwrap();
    assert_eq!(245, second_part(&data.data));
}
