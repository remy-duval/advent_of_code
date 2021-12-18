use super::*;

const ADD_TEST: &str = include_str!("add_test.txt");
const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

fn number(s: &str) -> Vec<Part> {
    parse_one(s).unwrap()
}

#[test]
fn add_test_1() {
    let a = number("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let b = number("[1,1]");
    assert_eq!(
        add_all(&[a, b]).unwrap(),
        number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );
}

#[test]
fn add_test_2() {
    let result = add_all(&[
        number("[1,1]"),
        number("[2,2]"),
        number("[3,3]"),
        number("[4,4]"),
    ])
    .unwrap();

    assert_eq!(result, number("[[[[1,1],[2,2]],[3,3]],[4,4]]"))
}

#[test]
fn add_test_3() {
    let result = add_all(&[
        number("[1,1]"),
        number("[2,2]"),
        number("[3,3]"),
        number("[4,4]"),
        number("[5,5]"),
    ])
    .unwrap();

    assert_eq!(result, number("[[[[3,0],[5,3]],[4,4]],[5,5]]"))
}

#[test]
fn add_test_4() {
    let result = add_all(&[
        number("[1,1]"),
        number("[2,2]"),
        number("[3,3]"),
        number("[4,4]"),
        number("[5,5]"),
        number("[6,6]"),
    ])
    .unwrap();

    assert_eq!(result, number("[[[[5,0],[7,4]],[5,5]],[6,6]]"))
}

#[test]
fn add_test_5() {
    let numbers = parse(ADD_TEST).unwrap();
    assert_eq!(
        add_all(&numbers).unwrap(),
        number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
    );
}

#[test]
fn magnitude_test_1() {
    assert_eq!(magnitude(&number("[[1,2],[[3,4],5]]")), 143);
}

#[test]
fn magnitude_test_2() {
    assert_eq!(
        magnitude(&number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
        1384
    );
}

#[test]
fn magnitude_test_3() {
    assert_eq!(magnitude(&number("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
}

#[test]
fn magnitude_test_4() {
    assert_eq!(magnitude(&number("[[[[3,0],[5,3]],[4,4]],[5,5]]")), 791);
}

#[test]
fn magnitude_test_5() {
    assert_eq!(magnitude(&number("[[[[5,0],[7,4]],[5,5]],[6,6]]")), 1137);
}

#[test]
fn magnitude_test_6() {
    assert_eq!(
        magnitude(&number(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        )),
        3488
    );
}

#[test]
fn first_part_example() {
    let numbers = parse(EXAMPLE).unwrap();
    assert_eq!(first_part(&numbers), 4_140);
}

#[test]
fn first_part_main() {
    let numbers = parse(MAIN).unwrap();
    assert_eq!(first_part(&numbers), 4_008);
}

#[test]
fn second_part_example() {
    let numbers = parse(EXAMPLE).unwrap();
    assert_eq!(second_part(&numbers), 3_993);
}

#[test]
fn second_part_main() {
    let numbers = parse(MAIN).unwrap();
    assert_eq!(second_part(&numbers), 4_667);
}
