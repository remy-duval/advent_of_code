use super::*;

const A: &str = include_str!("example_1.txt");
const B: &str = include_str!("data.txt");
const C: &str = include_str!("example_2.txt");

#[test]
fn first_part_test_a() {
    let instructions = Day::parse(A).unwrap().data;
    let first = first_part(&instructions);
    assert_eq!(first, 165);
}

#[test]
fn first_part_test_b() {
    let instructions = Day::parse(B).unwrap().data;
    let first = first_part(&instructions);
    assert_eq!(first, 9_967_721_333_886);
}

#[test]
fn first_part_test_c() {
    let instructions = Day::parse(C).unwrap().data;
    let first = first_part(&instructions);
    assert_eq!(first, 51);
}

#[test]
fn second_part_test_b() {
    let instructions = Day::parse(B).unwrap().data;
    let second = second_part(instructions);
    assert_eq!(second, 4_355_897_790_573);
}

#[test]
fn second_part_test_c() {
    let instructions = Day::parse(C).unwrap().data;
    let second = second_part(instructions);
    assert_eq!(second, 208);
}