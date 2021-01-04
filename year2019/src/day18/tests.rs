use super::*;

const TEST_ONE: &str = include_str!("example_1.txt");
const TEST_TWO: &str = include_str!("example_2.txt");
const TEST_THREE: &str = include_str!("example_3.txt");
const TEST_FOUR: &str = include_str!("example_4.txt");
const DATA: &str = include_str!("data.txt");

#[test]
fn shortest_test_one_player() {
    fn assertion(data: &str, expected_steps: usize) {
        let (start, keys, map) = parsers::parse_and_optimize_map(&data);
        let shortest = shortest_path::find_shortest_path(&map, start, keys);

        assert_eq!(expected_steps, shortest)
    }

    assertion(TEST_ONE, 132);
    assertion(TEST_TWO, 136);
    assertion(TEST_THREE, 81);
    assertion(DATA, 5392);
}

#[test]
fn shortest_test_split_a() {
    let split = parsers::split_maze_in_four(&TEST_FOUR, Point::new(7, 3), false);
    let shortest: usize = split
        .iter()
        .map(|data| {
            let (start, keys, map) = parsers::parse_and_optimize_map(data);
            shortest_path::find_shortest_path(&map, start, keys)
        })
        .sum();

    assert_eq!(24, shortest);
}

#[test]
fn shortest_test_split_b() {
    let (start, _, _) = parsers::parse_and_optimize_map(DATA);
    let split = parsers::split_maze_in_four(DATA, start, true);
    let shortest: usize = split
        .iter()
        .map(|data| {
            let (start, keys, map) = parsers::parse_and_optimize_map(data);
            shortest_path::find_shortest_path(&map, start, keys) - 1
        })
        .sum();

    assert_eq!(1684, shortest);
}
