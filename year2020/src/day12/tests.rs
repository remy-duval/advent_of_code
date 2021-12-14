use super::*;

const A: &str = include_str!("example.txt");
const B: &str = include_str!("data.txt");

#[test]
fn rotations() {
    let mut point = Direction::North.offset();
    point = rotate_right(point);
    assert_eq!(point, Direction::East.offset());
    point = rotate_right(point);
    assert_eq!(point, Direction::South.offset());
    point = rotate_right(point);
    assert_eq!(point, Direction::West.offset());
    point = rotate_right(point);
    assert_eq!(point, Direction::North.offset());
    point = rotate_left(point);
    assert_eq!(point, Direction::West.offset());
    point = rotate_left(point);
    assert_eq!(point, Direction::South.offset());
    point = rotate_left(point);
    assert_eq!(point, Direction::East.offset());
    point = rotate_left(point);
    assert_eq!(point, Direction::North.offset());
}

#[test]
fn first_part_test_a() {
    let instructions = parse(A).unwrap().data;
    let result = first_part(&instructions);
    assert_eq!(Point { x: 17, y: 8 }, result);
    assert_eq!(25, result.manhattan_distance());
}

#[test]
fn first_part_test_b() {
    let instructions = parse(B).unwrap().data;
    let result = first_part(&instructions);
    assert_eq!(Point { x: -112, y: 470 }, result);
    assert_eq!(582, result.manhattan_distance());
}

#[test]
fn second_part_test_a() {
    let instructions = parse(A).unwrap().data;
    let result = second_part(&instructions);
    assert_eq!(Point { x: 214, y: 72 }, result);
    assert_eq!(286, result.manhattan_distance());
}

#[test]
fn second_part_test_b() {
    let instructions = parse(B).unwrap().data;
    let result = second_part(&instructions);
    assert_eq!(Point { x: 15039, y: 37030 }, result);
    assert_eq!(52069, result.manhattan_distance());
}
