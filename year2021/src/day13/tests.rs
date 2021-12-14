use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE_RESULT: &str = include_str!("example_result.txt");
const MAIN: &str = include_str!("data.txt");
const MAIN_RESULT: &str = include_str!("data_result.txt");

#[test]
fn example() {
    let mut origami = parse(EXAMPLE).unwrap();
    origami.fold_once();
    assert_eq!(origami.count(), 17);
    origami.fold_all();
    assert_eq!(origami.count(), 16);
    itertools::assert_equal(format!("{}", origami).lines(), EXAMPLE_RESULT.lines());
}

#[test]
fn main() {
    let mut origami = parse(MAIN).unwrap();
    origami.fold_once();
    assert_eq!(origami.count(), 745);
    origami.fold_all();
    // This spells 'ABKJFBGC'
    itertools::assert_equal(format!("{}", origami).lines(), MAIN_RESULT.lines());
}
