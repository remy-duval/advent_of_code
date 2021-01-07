use super::*;

const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example_a() {
    assert_eq!(first_part(&PartialSumGrid::new(18)), (33, 45));
}

#[test]
fn first_part_example_b() {
    assert_eq!(first_part(&PartialSumGrid::new(42)), (21, 61));
}

#[test]
fn first_part_main() {
    let serial = Day::parse(MAIN).unwrap();
    assert_eq!(first_part(&PartialSumGrid::new(serial)), (21, 77));
}

#[test]
fn second_part_example_a() {
    assert_eq!(second_part(&PartialSumGrid::new(18)), (90, 269, 16));
}

#[test]
fn second_part_example_b() {
    assert_eq!(second_part(&PartialSumGrid::new(42)), (232, 251, 12));
}

#[test]
fn second_part_main() {
    let serial = Day::parse(MAIN).unwrap();
    assert_eq!(second_part(&PartialSumGrid::new(serial)), (224, 222, 27));
}
