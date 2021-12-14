use super::*;

const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    assert_eq!(first_part(5), "0124515891");
    assert_eq!(first_part(9), "5158916779");
    assert_eq!(first_part(18), "9251071085");
    assert_eq!(first_part(2018), "5941429882");
}

#[test]
fn first_part_main() {
    assert_eq!(first_part(parse(MAIN).full_number()), "3811491411");
}

#[test]
fn second_part_example() {
    assert_eq!(second_part(Rules(vec![5, 1, 5, 8, 9])), 9);
    assert_eq!(second_part(Rules(vec![0, 1, 2, 4, 5])), 5);
    assert_eq!(second_part(Rules(vec![9, 2, 5, 1, 0])), 18);
    assert_eq!(second_part(Rules(vec![5, 9, 4, 1, 4])), 2018);
}

#[test]
fn second_part_main() {
    assert_eq!(second_part(parse(MAIN)), 20_408_083);
}
