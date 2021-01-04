use super::*;

const A: &str = include_str!("example_1.txt");
const B: &str = include_str!("example_2.txt");
const C: &str = include_str!("data.txt");

#[test]
fn first_part_test_a() {
    let adapters = Day::parse(A).unwrap();
    let (ones, threes) = first_part(&adapters);
    assert_eq!(7, ones);
    assert_eq!(5, threes);
}

#[test]
fn first_part_test_b() {
    let adapters = Day::parse(B).unwrap();
    let (ones, threes) = first_part(&adapters);
    assert_eq!(22, ones);
    assert_eq!(10, threes);
}

#[test]
fn first_part_test_c() {
    let adapters = Day::parse(C).unwrap();
    let (ones, threes) = first_part(&adapters);
    assert_eq!(66, ones);
    assert_eq!(39, threes);
}

#[test]
fn second_part_test_a() {
    let adapters = Day::parse(A).unwrap();
    assert_eq!(8, second_part(adapters));
}

#[test]
fn second_part_test_b() {
    let adapters = Day::parse(B).unwrap();
    assert_eq!(19_208, second_part(adapters));
}

#[test]
fn second_part_test_c() {
    let adapters = Day::parse(C).unwrap();
    assert_eq!(2_644_613_988_352, second_part(adapters))
}
