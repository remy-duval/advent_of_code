use super::*;

const ONE: &str = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n";
const TWO: &str = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz\n";
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let input = Day::parse(ONE).unwrap().data;
    assert_eq!(check_sum(&input), 12);
}

#[test]
fn first_part_main() {
    let input = Day::parse(MAIN).unwrap().data;
    assert_eq!(check_sum(&input), 6422);
}

#[test]
fn second_part_example() {
    let input = Day::parse(TWO).unwrap().data;
    assert_eq!(find_different_by_one(&input).unwrap().as_str(), "fgij");
}

#[test]
fn second_part_main() {
    let input = Day::parse(MAIN).unwrap().data;
    assert_eq!(
        find_different_by_one(&input).unwrap().as_str(),
        "qcslyvphgkrmdawljuefotxbh"
    );
}
