use super::*;

const A: &str = include_str!("example.txt");
const B: &str = include_str!("data.txt");

#[test]
fn first_part_test_a() {
    let data = Day::parse(A).unwrap();
    let sum = first_part(&data.data);
    assert_eq!(11, sum);
}

#[test]
fn first_part_test_b() {
    let data = Day::parse(B).unwrap();
    let sum = first_part(&data.data);
    assert_eq!(6351, sum);
}

#[test]
fn second_part_test_a() {
    let data = Day::parse(A).unwrap();
    let sum = second_part(&data.data);
    assert_eq!(6, sum);
}

#[test]
fn second_part_test_b() {
    let data = Day::parse(B).unwrap();
    let sum = second_part(&data.data);
    assert_eq!(3143, sum);
}
