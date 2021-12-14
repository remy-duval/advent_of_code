use super::*;

const A: &str = include_str!("example.txt");
const B: &str = include_str!("data.txt");

#[test]
fn first_part_test_a() {
    let rules = parse(A).unwrap();
    let containing = first_part(&rules);
    assert_eq!(4, containing);
}

#[test]
fn first_part_test_b() {
    let rules = parse(B).unwrap();
    let containing = first_part(&rules);
    assert_eq!(235, containing);
}

#[test]
fn second_part_test_a() {
    let rules = parse(A).unwrap();
    let contains = second_part(&rules);
    assert_eq!(32, contains);
}

#[test]
fn second_part_test_b() {
    let rules = parse(B).unwrap();
    let contains = second_part(&rules);
    assert_eq!(158493, contains);
}
