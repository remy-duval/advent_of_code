use super::*;

const A: &str = include_str!("example.txt");
const B: &str = include_str!("data.txt");
const INVALID: &str = include_str!("invalid.txt");
const VALID: &str = include_str!("valid.txt");

#[test]
fn first_part_test_a() {
    let parsed = PassportBuilder::parse_many(A);
    assert_eq!(2, first_part(&parsed));
}

#[test]
fn first_part_test_b() {
    let parsed = PassportBuilder::parse_many(B);
    assert_eq!(200, first_part(&parsed));
}

#[test]
fn invalid_passports() {
    let parsed = PassportBuilder::parse_many(INVALID);
    assert_eq!(4, parsed.len());
    assert!(parsed.iter().all(|passport| !passport.is_valid()));
}

#[test]
fn valid_passports() {
    let parsed = PassportBuilder::parse_many(VALID);
    assert_eq!(4, parsed.len());
    assert!(parsed.iter().all(|passport| passport.is_valid()));
}

#[test]
fn second_part_a() {
    let parsed = PassportBuilder::parse_many(A);
    assert_eq!(2, second_part(&parsed));
}

#[test]
fn second_part_b() {
    let parsed = PassportBuilder::parse_many(B);
    assert_eq!(116, second_part(&parsed));
}
