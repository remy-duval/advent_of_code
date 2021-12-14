use super::*;

const EXAMPLE: &str = include_str!("example.txt");
const MAIN: &str = include_str!("data.txt");

#[test]
fn first_part_example() {
    let events = parse(EXAMPLE).unwrap().events;
    let (guard, sleepiest) = Schedule::new(&events).first_strategy().unwrap();
    assert_eq!(guard, 10);
    assert_eq!(sleepiest, 24);
}

#[test]
fn first_part_main() {
    let events = parse(MAIN).unwrap().events;
    let (guard, sleepiest) = Schedule::new(&events).first_strategy().unwrap();
    assert_eq!(guard, 641);
    assert_eq!(sleepiest, 41);
}

#[test]
fn second_part_example() {
    let events = parse(EXAMPLE).unwrap().events;
    let (guard, sleepiest) = Schedule::new(&events).second_strategy().unwrap();
    assert_eq!(guard, 99);
    assert_eq!(sleepiest, 45);
}

#[test]
fn second_part_main() {
    let events = parse(MAIN).unwrap().events;
    let (guard, sleepiest) = Schedule::new(&events).second_strategy().unwrap();
    assert_eq!(guard, 1973);
    assert_eq!(sleepiest, 37);
}
