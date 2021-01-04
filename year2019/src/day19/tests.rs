use super::*;

const DATA: &str = include_str!("data.txt");

#[test]
fn first_part() {
    let memory = Day::parse(DATA).unwrap().data;
    let affected = count_pulled(&memory, Point::new(0, 0), Point::new(50, 50));

    assert_eq!(189, affected);
}

#[test]
fn second_part() {
    let memory = Day::parse(DATA).unwrap().data;
    let first = find_first_square(&memory, 100);

    assert_eq!(Point { x: 762, y: 1042 }, first);
}
