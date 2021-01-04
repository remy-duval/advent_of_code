use super::*;

const TEST_ONE: &str = "....#\n#..#.\n#..##\n..#..\n#....";
const TEST_TWO: &str = include_str!("data.txt");

#[test]
fn first_repeat_test_a() {
    let bugs = Day::parse(TEST_ONE).unwrap();
    let result = first_repeat(bugs);

    assert_eq!(2_129_920, result.biodiversity_rating());
}

#[test]
fn first_repeat_test_b() {
    let bugs = Day::parse(TEST_TWO).unwrap();
    let result = first_repeat(bugs);

    assert_eq!(12_129_040, result.biodiversity_rating());
}

#[test]
fn recursion_test_a() {
    let bugs = Day::parse(TEST_ONE).unwrap();
    let result = recursive_expansion(bugs, 10);

    assert_eq!(99, result);
}

#[test]
fn recursion_test_b() {
    let bugs = Day::parse(TEST_TWO).unwrap();
    let result = recursive_expansion(bugs, 200);

    assert_eq!(2109, result);
}
