use super::*;

const EXAMPLE_ONE: &str = include_str!("example_1.txt");
const EXAMPLE_TWO: &str = include_str!("example_2.txt");
const EXAMPLE_THREE: &str = include_str!("example_3.txt");
const EXAMPLE_FOUR: &str = include_str!("example_4.txt");
const EXAMPLE_FIVE: &str = include_str!("example_5.txt");
const EXAMPLE_SIX: &str = include_str!("example_6.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn next_move_order_test() {
    let mut before = Day::parse(include_str!("move.txt")).unwrap();
    let expected = Day::parse(include_str!("move_expected.txt")).unwrap();
    before.next_round(0, 0);

    assert_eq!(before.to_string(), expected.to_string())
}

#[test]
fn first_part_example_a() {
    let mut fight = Day::parse(EXAMPLE_ONE).unwrap();
    assert_eq!(fight.first_part(), (47, 590));
}

#[test]
fn first_part_example_b() {
    let mut fight = Day::parse(EXAMPLE_TWO).unwrap();
    assert_eq!(fight.first_part(), (37, 982));
}

#[test]
fn first_part_example_c() {
    let mut fight = Day::parse(EXAMPLE_THREE).unwrap();
    assert_eq!(fight.first_part(), (46, 859));
}

#[test]
fn first_part_example_d() {
    let mut fight = Day::parse(EXAMPLE_FOUR).unwrap();
    assert_eq!(fight.first_part(), (35, 793));
}

#[test]
fn first_part_example_e() {
    let mut fight = Day::parse(EXAMPLE_FIVE).unwrap();
    assert_eq!(fight.first_part(), (54, 536));
}

#[test]
fn first_part_example_f() {
    let mut fight = Day::parse(EXAMPLE_SIX).unwrap();
    assert_eq!(fight.first_part(), (20, 937));
}

#[test]
fn first_part_main() {
    let mut fight = Day::parse(MAIN).unwrap();
    assert_eq!(fight.first_part(), (101, 2554));
}

#[test]
fn second_part_example_a() {
    let fight = Day::parse(EXAMPLE_ONE).unwrap();
    assert_eq!(fight.second_part().unwrap(), (29, 172));
}

#[test]
fn second_part_example_b() {
    let fight = Day::parse(EXAMPLE_THREE).unwrap();
    assert_eq!(fight.second_part().unwrap(), (33, 948));
}

#[test]
fn second_part_example_c() {
    let fight = Day::parse(EXAMPLE_FOUR).unwrap();
    assert_eq!(fight.second_part().unwrap(), (37, 94));
}

#[test]
fn second_part_example_d() {
    let fight = Day::parse(EXAMPLE_FIVE).unwrap();
    assert_eq!(fight.second_part().unwrap(), (39, 166));
}

#[test]
fn second_part_example_e() {
    let fight = Day::parse(EXAMPLE_SIX).unwrap();
    assert_eq!(fight.second_part().unwrap(), (30, 38));
}

#[test]
fn second_part_main() {
    let fight = Day::parse(MAIN).unwrap();
    assert_eq!(fight.second_part().unwrap(), (43, 1187));
}
