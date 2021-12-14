use super::*;

const A: &str = include_str!("example.txt");
const B: &str = include_str!("data.txt");

#[test]
fn first_part_test_a() {
    let floor = first_part(parse(A));
    assert_eq!(37, floor.occupied_seats());
}

#[test]
fn first_part_test_b() {
    let floor = first_part(parse(B));
    assert_eq!(2263, floor.occupied_seats());
}

#[test]
fn second_part_test_a() {
    let floor = second_part(parse(A));
    assert_eq!(26, floor.occupied_seats());
}

#[test]
fn second_part_test_b() {
    let floor = second_part(parse(B));
    assert_eq!(2002, floor.occupied_seats());
}
