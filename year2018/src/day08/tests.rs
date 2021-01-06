use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let tree = Day::parse(EXAMPLE).unwrap();
    assert_eq!(tree.metadata_sum(), 138);
}

#[test]
fn first_part_main() {
    let tree = Day::parse(MAIN).unwrap();
    assert_eq!(tree.metadata_sum(), 37439);
}

#[test]
fn second_part_example() {
    let tree = Day::parse(EXAMPLE).unwrap();
    assert_eq!(tree.root_node_value(), 66);
}

#[test]
fn second_part_main() {
    let tree = Day::parse(MAIN).unwrap();
    assert_eq!(tree.root_node_value(), 20_815);
}
