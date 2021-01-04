use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn paths_test() {
    let offset = Path::from_str("esew").unwrap().offset();
    assert_eq!(offset, Point::new(-1, -1));

    let offset = Path::from_str("nwwswee").unwrap().offset();
    assert_eq!(offset, Point::new(0, 0));
}

#[test]
fn first_part_example() {
    let paths = Day::parse(EXAMPLE).unwrap().data;
    let result = initial_state(paths).len();
    assert_eq!(result, 10);
}

#[test]
fn first_part_main() {
    let paths = Day::parse(MAIN).unwrap().data;
    let result = initial_state(paths).len();
    assert_eq!(result, 332);
}

#[test]
fn second_part_example() {
    let state = initial_state(Day::parse(EXAMPLE).unwrap().data);
    assert_eq!(compute_next_state(state.clone(), 1).len(), 15);
    assert_eq!(compute_next_state(state.clone(), 2).len(), 12);
    assert_eq!(compute_next_state(state.clone(), 3).len(), 25);
    assert_eq!(compute_next_state(state.clone(), 4).len(), 14);
    assert_eq!(compute_next_state(state.clone(), 5).len(), 23);
    assert_eq!(compute_next_state(state.clone(), 6).len(), 28);
    assert_eq!(compute_next_state(state.clone(), 7).len(), 41);
    assert_eq!(compute_next_state(state.clone(), 8).len(), 37);
    assert_eq!(compute_next_state(state.clone(), 9).len(), 49);
    assert_eq!(compute_next_state(state.clone(), 10).len(), 37);
    assert_eq!(compute_next_state(state.clone(), 100).len(), 2208);
}

#[test]
fn second_part_main() {
    let state = initial_state(Day::parse(MAIN).unwrap().data);
    let state = compute_next_state(state, 100);
    assert_eq!(state.len(), 3900);
}