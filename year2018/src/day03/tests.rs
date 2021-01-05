use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let input = Day::parse(EXAMPLE).unwrap().data;
    let tissue = Tissue::new(input);
    assert_eq!(tissue.multiple_claims(), 4);
}

#[test]
fn first_part_main() {
    let input = Day::parse(MAIN).unwrap().data;
    let tissue = Tissue::new(input);
    assert_eq!(tissue.multiple_claims(), 116140);
}

#[test]
fn second_part_example() {
    let input = Day::parse(EXAMPLE).unwrap().data;
    let tissue = Tissue::new(input);
    assert_eq!(tissue.find_intact_claim().unwrap().id, 3);
}

#[test]
fn second_part_main() {
    let input = Day::parse(MAIN).unwrap().data;
    let tissue = Tissue::new(input);
    assert_eq!(tissue.find_intact_claim().unwrap().id, 574);
}
