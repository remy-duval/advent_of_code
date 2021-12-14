use super::*;

const DATA: &str = include_str!("data.txt");

#[test]
fn solve_test() {
    let mut image = parse(DATA).unwrap();
    let (_, w, t) = image.check_sum();
    image.build();
    assert_eq!(2_375, w * t);
}
