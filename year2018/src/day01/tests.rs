use super::*;

const ONE: &str = "+1\n-2\n+3\n+1\n";
const TWO: &str = "+1\n-1\n";
const THREE: &str = "+3\n+3\n+4\n-2\n-4\n";
const FOUR: &str = "-6\n+3\n+8\n+5\n-6\n";
const FIVE: &str = "+7\n+7\n-2\n-7\n-4\n";
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let input = parse(ONE).unwrap().data;
    assert_eq!(sum(&input), 3);
}

#[test]
fn first_part_main() {
    let input = parse(MAIN).unwrap().data;
    assert_eq!(sum(&input), 513);
}

#[test]
fn second_part_example() {
    assert_eq!(first_repeated(&parse(ONE).unwrap().data), 2);
    assert_eq!(first_repeated(&parse(TWO).unwrap().data), 0);
    assert_eq!(first_repeated(&parse(THREE).unwrap().data), 10);
    assert_eq!(first_repeated(&parse(FOUR).unwrap().data), 5);
    assert_eq!(first_repeated(&parse(FIVE).unwrap().data), 14);
}

#[test]
fn second_part_main() {
    let input = parse(MAIN).unwrap().data;
    assert_eq!(first_repeated(&input), 287);
}
