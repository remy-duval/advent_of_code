use super::*;

const A: &str = "1 + 2 * 3 + 4 * 5 + 6";
const B: &str = "1 + (2 * 3) + (4 * (5 + 6))";
const C: &str = "2 * 3 + (4 * 5)";
const D: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
const E: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
const F: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
const MAIN: &str = include_str!("data.txt");

#[test]
fn no_precedence_test_a() {
    let operation = A.parse::<Operation>().unwrap();
    let result = operation.evaluate_no_precedence().unwrap();
    assert_eq!(result, 71);
}

#[test]
fn no_precedence_test_b() {
    let operation = B.parse::<Operation>().unwrap();
    let result = operation.evaluate_no_precedence().unwrap();
    assert_eq!(result, 51);
}

#[test]
fn no_precedence_test_c() {
    let operation = C.parse::<Operation>().unwrap();
    let result = operation.evaluate_no_precedence().unwrap();
    assert_eq!(result, 26);
}

#[test]
fn no_precedence_test_d() {
    let operation = D.parse::<Operation>().unwrap();
    let result = operation.evaluate_no_precedence().unwrap();
    assert_eq!(result, 437);
}

#[test]
fn no_precedence_test_e() {
    let operation = E.parse::<Operation>().unwrap();
    let result = operation.evaluate_no_precedence().unwrap();
    assert_eq!(result, 12_240);
}

#[test]
fn no_precedence_test_f() {
    let operation = F.parse::<Operation>().unwrap();
    let result = operation.evaluate_no_precedence().unwrap();
    assert_eq!(result, 13_632);
}

#[test]
fn addition_precedence_test_a() {
    let operation = A.parse::<Operation>().unwrap();
    let result = operation.evaluate_addition_has_precedence().unwrap();
    assert_eq!(result, 231);
}

#[test]
fn addition_precedence_test_b() {
    let operation = B.parse::<Operation>().unwrap();
    let result = operation.evaluate_addition_has_precedence().unwrap();
    assert_eq!(result, 51);
}

#[test]
fn addition_precedence_test_c() {
    let operation = C.parse::<Operation>().unwrap();
    let result = operation.evaluate_addition_has_precedence().unwrap();
    assert_eq!(result, 46);
}

#[test]
fn addition_precedence_test_d() {
    let operation = D.parse::<Operation>().unwrap();
    let result = operation.evaluate_addition_has_precedence().unwrap();
    assert_eq!(result, 1445);
}

#[test]
fn addition_precedence_test_e() {
    let operation = E.parse::<Operation>().unwrap();
    let result = operation.evaluate_addition_has_precedence().unwrap();
    assert_eq!(result, 669_060);
}

#[test]
fn addition_precedence_test_f() {
    let operation = F.parse::<Operation>().unwrap();
    let result = operation.evaluate_addition_has_precedence().unwrap();
    assert_eq!(result, 23_340);
}

#[test]
fn first_part_test() {
    let input = Day::parse(MAIN).unwrap().data;
    let result = first_part(&input).unwrap();
    assert_eq!(result, 202_553_439_706);
}

#[test]
fn second_part_test() {
    let input = Day::parse(MAIN).unwrap().data;
    let result = second_part(&input).unwrap();
    assert_eq!(result, 88_534_268_715_686);
}
