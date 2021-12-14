use super::*;

const ONE: &str = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n";
const TWO: &str = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz\n";
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    assert_eq!(check_sum(ONE), 12);
}

#[test]
fn first_part_main() {
    assert_eq!(check_sum(MAIN), 6422);
}

#[test]
fn second_part_example() {
    assert_eq!(find_different_by_one(TWO).unwrap().as_str(), "fgij");
}

#[test]
fn second_part_main() {
    assert_eq!(
        find_different_by_one(MAIN).unwrap().as_str(),
        "qcslyvphgkrmdawljuefotxbh"
    );
}
