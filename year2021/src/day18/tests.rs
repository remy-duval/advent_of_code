use super::*;

const ADD_TEST: &str = include_str!("add_test.txt");
const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

fn number(s: &str) -> Number {
    Number::parse(s).unwrap().0
}

#[test]
fn add_test_1() {
    let a = number("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let b = number("[1,1]");
    assert_eq!(a.add(b), number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
}

#[test]
fn add_test_2() {
    let result = number("[1,1]")
        .add(number("[2,2]"))
        .add(number("[3,3]"))
        .add(number("[4,4]"));

    assert_eq!(result, number("[[[[1,1],[2,2]],[3,3]],[4,4]]"))
}

#[test]
fn add_test_3() {
    let result = number("[1,1]")
        .add(number("[2,2]"))
        .add(number("[3,3]"))
        .add(number("[4,4]"))
        .add(number("[5,5]"));

    assert_eq!(result, number("[[[[3,0],[5,3]],[4,4]],[5,5]]"))
}

#[test]
fn add_test_4() {
    let result = number("[1,1]")
        .add(number("[2,2]"))
        .add(number("[3,3]"))
        .add(number("[4,4]"))
        .add(number("[5,5]"))
        .add(number("[6,6]"));

    assert_eq!(result, number("[[[[5,0],[7,4]],[5,5]],[6,6]]"))
}

#[test]
fn add_test_big() {
    let numbers = parse(ADD_TEST).unwrap();
    let result = numbers.into_iter().reduce(|a, b| a.add(b)).unwrap();
    assert_eq!(
        result,
        number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
    );
}

#[test]
fn first_part_example() {
    let numbers = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(numbers), 4_140);
}

#[test]
fn first_part_main() {
    let numbers = parse(MAIN).unwrap();
    assert_eq!(first_part(numbers), 4_008);
}

#[test]
fn second_part_example() {
    let numbers = parse(EXAMPLE).unwrap();
    assert_eq!(second_part(numbers), 3_993);
}

#[test]
fn second_part_main() {
    let numbers = parse(MAIN).unwrap();
    assert_eq!(second_part(numbers), 4_667);
}
