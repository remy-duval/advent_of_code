use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let polymer = Day::parse(EXAMPLE).unwrap();
    let res = polymer.min_max_after(10, &mut HashMap::with_capacity(128));
    assert_eq!(res.1 - res.0, 1_588);
}

#[test]
fn first_part_main() {
    let polymer = Day::parse(MAIN).unwrap();
    let res = polymer.min_max_after(10, &mut HashMap::with_capacity(700));
    assert_eq!(res.1 - res.0, 2_851);
}

#[test]
fn second_part_example() {
    let polymer = Day::parse(EXAMPLE).unwrap();
    let res = polymer.min_max_after(40, &mut HashMap::with_capacity(700));
    assert_eq!(res.1 - res.0, 2_188_189_693_529);
}

#[test]
fn second_part_main() {
    let polymer = Day::parse(MAIN).unwrap();
    let res = polymer.min_max_after(40, &mut HashMap::with_capacity(3400));
    assert_eq!(res.1 - res.0, 10_002_813_279_337);
}
