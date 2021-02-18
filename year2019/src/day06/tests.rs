use super::*;

const A: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
const B: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
const DATA: &str = include_str!("data.txt");

#[test]
fn checked_sum() {
    let data = Day::parse(A).unwrap();
    let orbits = parse_map(&data.data);
    let from_origin = depth_first_search(COM, orbits).unwrap();
    let result = check_sum(&from_origin);

    assert_eq!(42, result);
}

#[test]
fn shortest_paths() {
    let data = Day::parse(B).unwrap();
    let orbits = parse_map(&data.data);
    let from_origin = depth_first_search(COM, orbits).unwrap();
    let result = shortest_path(&from_origin, "YOU", "SAN").unwrap();

    assert_eq!(4, result);
}

#[test]
fn solve_test() {
    let data = Day::parse(DATA).unwrap();
    let orbits = parse_map(&data.data);
    let from_origin = depth_first_search(COM, orbits).unwrap();
    let first = check_sum(&from_origin);
    let second = shortest_path(&from_origin, "YOU", "SAN").unwrap();

    assert_eq!(158_090, first);
    assert_eq!(241, second);
}