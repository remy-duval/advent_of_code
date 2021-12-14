use super::*;

const A: &str = include_str!("example.txt");
const B: &str = include_str!("data.txt");

#[test]
fn test_chinese_remainder_theorem() {
    let values = vec![(0, 3), (3, 4), (4, 5)];
    let result = chinese_remainder_theorem(values).unwrap().unwrap();
    assert_eq!(result, 39);
}

#[test]
fn first_part_test_a() {
    let resource = parse(A).unwrap();
    let (bus, time) = earliest(&resource).unwrap();
    assert_eq!(bus, 59);
    assert_eq!(time, 5);
    assert_eq!(bus * time, 295);
}

#[test]
fn first_part_test_b() {
    let resource = parse(B).unwrap();
    let (bus, time) = earliest(&resource).unwrap();
    assert_eq!(bus, 443);
    assert_eq!(time, 5);
    assert_eq!(bus * time, 2_215);
}

#[test]
fn second_part_example_a() {
    let values = [Some(17 as Timestamp), None, Some(13), Some(19)];
    let result = second_part(&values).unwrap().unwrap();
    assert_eq!(result, 3_417);
}

#[test]
fn second_part_example_b() {
    let values = [Some(67 as Timestamp), Some(7), Some(59), Some(61)];
    let result = second_part(&values).unwrap().unwrap();
    assert_eq!(result, 754_018);
}

#[test]
fn second_part_example_c() {
    let values = [Some(67 as Timestamp), None, Some(7), Some(59), Some(61)];
    let result = second_part(&values).unwrap().unwrap();
    assert_eq!(result, 779_210);
}

#[test]
fn second_part_example_d() {
    let values = [Some(67 as Timestamp), Some(7), None, Some(59), Some(61)];
    let result = second_part(&values).unwrap().unwrap();
    assert_eq!(result, 1_261_476);
}

#[test]
fn second_part_example_e() {
    let values = [Some(1789 as Timestamp), Some(37), Some(47), Some(1889)];
    let result = second_part(&values).unwrap().unwrap();
    assert_eq!(result, 1_202_161_486);
}

#[test]
fn second_part_test_a() {
    let resource = parse(A).unwrap();
    let timestamp = second_part(&resource.lines).unwrap().unwrap();
    assert_eq!(timestamp, 1_068_781);
}

#[test]
fn second_part_test_b() {
    let resource = parse(B).unwrap();
    let timestamp = second_part(&resource.lines).unwrap().unwrap();
    assert_eq!(timestamp, 1_058_443_396_696_792);
}
