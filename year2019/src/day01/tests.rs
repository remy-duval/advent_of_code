use super::*;

const DATA: &str = include_str!("data.txt");

#[test]
fn all_parts_test() {
    let masses = parse(DATA).unwrap();
    let (first, second) = solve(&masses.data);
    assert_eq!(3_256_794, first);
    assert_eq!(4_882_337, second);
}
