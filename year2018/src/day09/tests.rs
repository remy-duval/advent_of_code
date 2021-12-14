use super::*;

const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example_a() {
    let rules = Rules {
        players: 9,
        points: 25,
    };
    assert_eq!(winning_score(&rules), 32);
}

#[test]
fn first_part_example_b() {
    let rules = Rules {
        players: 10,
        points: 1_618,
    };
    assert_eq!(winning_score(&rules), 8_317);
}

#[test]
fn first_part_example_c() {
    let rules = Rules {
        players: 17,
        points: 1_104,
    };
    assert_eq!(winning_score(&rules), 2_764);
}

#[test]
fn first_part_example_d() {
    let rules = Rules {
        players: 13,
        points: 7_999,
    };
    assert_eq!(winning_score(&rules), 146_373);
}

#[test]
fn first_part_example_e() {
    let rules = Rules {
        players: 21,
        points: 6_111,
    };
    assert_eq!(winning_score(&rules), 54_718);
}

#[test]
fn first_part_example_f() {
    let rules = Rules {
        players: 30,
        points: 5_807,
    };
    assert_eq!(winning_score(&rules), 37_305);
}

#[test]
fn first_part_main() {
    let rules = parse(MAIN).unwrap();
    assert_eq!(winning_score(&rules), 383_475);
}

#[test]
fn second_part_main() {
    let mut rules = parse(MAIN).unwrap();
    rules.points *= 100;
    assert_eq!(winning_score(&rules), 3_148_209_772);
}
