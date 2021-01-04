use super::*;

const DATA: &str = include_str!("data.txt");

#[test]
fn digits() {
    assert_eq!(&[1, 1, 1, 1, 1, 1], &split_digits(111_111));
    assert_eq!(&[2, 2, 3, 4, 5, 0], &split_digits(223_450));
    assert_eq!(&[1, 2, 3, 7, 8, 9], &split_digits(123_789));
}

#[test]
fn ordered() {
    assert!(!check_ordered([2, 2, 3, 4, 5, 0]));
    assert!(check_ordered([1, 1, 1, 1, 1, 1]));
    assert!(check_ordered([1, 2, 3, 7, 8, 9]));
}

#[test]
fn pairs() {
    assert_eq!((false, false), check_pair(&[1, 2, 3, 5, 6, 7]));
    assert_eq!((true, false), check_pair(&[1, 3, 3, 3, 6, 7]));
    assert_eq!((true, true), check_pair(&[1, 3, 3, 3, 6, 6]));
}

#[test]
fn solve_test() {
    let Between { from, to } = DATA.parse().unwrap();
    let (first, second) = solve(from, to);

    assert_eq!(1_653, first);
    assert_eq!(1_133, second);
}
