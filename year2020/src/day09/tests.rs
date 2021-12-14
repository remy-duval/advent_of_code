use super::*;

const A: &str = include_str!("example.txt");
const B: &str = include_str!("data.txt");
const A_EXPECTED: u64 = 127;
const B_EXPECTED: u64 = 70639851;

#[test]
fn first_part_test_a() {
    let data = parse(A).unwrap().data;
    let first = first_not_sum(&data, 5).expect("Should have been found");
    assert_eq!(A_EXPECTED, first);
}

#[test]
fn first_part_test_b() {
    let data = parse(B).unwrap().data;
    let first = first_not_sum(&data, PREAMBLE).expect("Should have been found");
    assert_eq!(B_EXPECTED, first);
}

#[test]
fn contiguous_test_a() {
    let test = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let result = contiguous_set(&test, 13).expect("Should have been found");

    assert_eq!(&[6, 7], result);
}

#[test]
fn contiguous_test_b() {
    let test = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let result = contiguous_set(&test, 15).expect("Should have been found");

    assert_eq!(&[0, 1, 2, 3, 4, 5], result);
}

#[test]
fn contiguous_test_c() {
    let test = vec![0, 1, 2, 3, 4, 5, 6, 7];
    assert!(contiguous_set(&test, 0).is_none());
    assert!(contiguous_set(&test, 2).is_none());
    assert!(contiguous_set(&test, 23).is_none());
    assert!(contiguous_set(&test, 29).is_none());
    assert!(contiguous_set(&[], 1).is_none());
}

#[test]
fn second_part_test_a() {
    let data = parse(A).unwrap().data;
    let (min, max) = second_part(&data, 127).expect("Should have been found");
    assert_eq!(15, min);
    assert_eq!(47, max);
}

#[test]
fn second_part_test_b() {
    let data = parse(B).unwrap().data;
    let (min, max) = second_part(&data, B_EXPECTED).expect("Should have been found");
    assert_eq!(3474524, min);
    assert_eq!(4774716, max);
}
