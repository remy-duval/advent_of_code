use super::*;

const FIRST_DATA: &str = include_str!("example.txt");
const SECOND_DATA: &str = include_str!("data.txt");

#[test]
fn first_part_test_first_data() {
    let data = parse(FIRST_DATA).unwrap();
    let (first, second) = first_part(&data.data).expect("result should have been found");
    assert_eq!(first * second, 514579);
}

#[test]
fn first_part_test_second_data() {
    let data = parse(SECOND_DATA).unwrap();
    let (first, second) = first_part(&data.data).expect("result should have been found");
    assert_eq!(first * second, 969024);
}

#[test]
fn second_part_test_first_data() {
    let data = parse(FIRST_DATA).unwrap();
    let (first, second, third) = second_part(&data.data).expect("result should have been found");
    assert_eq!(first * second * third, 241861950);
}

#[test]
fn second_part_test_second_data() {
    let data = parse(SECOND_DATA).unwrap();
    let (first, second, third) = second_part(&data.data).expect("result should have been found");
    assert_eq!(first * second * third, 230057040);
}
