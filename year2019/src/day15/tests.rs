use super::*;

const DATA: &str = include_str!("data.txt");

#[test]
fn solve_test() {
    let memory = parse(DATA).unwrap().data;
    let map = explore_map(&memory, false);
    let (oxygen, path_length) = first_part(&map).unwrap();

    assert_eq!(Point { x: 16, y: 16 }, oxygen);
    assert_eq!(424, path_length);

    let longest_path = second_part(&map, oxygen).unwrap();
    assert_eq!(446, longest_path);
}
