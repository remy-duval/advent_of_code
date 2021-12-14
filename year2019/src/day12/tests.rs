use super::*;

const TEST_ONE: &str = include_str!("example_1.txt");
const TEST_TWO: &str = include_str!("example_2.txt");
const DATA: &str = include_str!("data.txt");

#[test]
fn parse_test() {
    let first = parse(TEST_ONE).unwrap();
    let second = parse(TEST_TWO).unwrap();

    assert_eq!(
        Moons::new([[-1, 0, 2], [2, -10, -7], [4, -8, 8], [3, 5, -1]]),
        first
    );
    assert_eq!(
        Moons::new([[-8, -10, 0], [5, 5, 10], [2, -7, 3], [9, -8, -3]]),
        second
    );
}

#[test]
fn energy_test() {
    let mut first = parse(TEST_ONE).unwrap();
    let mut second = parse(TEST_TWO).unwrap();

    (0..10).for_each(|_| first.next());
    assert_eq!(179, first.energy());

    (0..100).for_each(|_| second.next());
    assert_eq!(1940, second.energy());
}

#[test]
fn periodicity_test() {
    let first = parse(TEST_ONE).unwrap();
    let second = parse(TEST_TWO).unwrap();

    assert_eq!(2772, find_periodicity(first));
    assert_eq!(4_686_774_924, find_periodicity(second));
}

#[test]
fn solve_test() {
    let mut moons = parse(DATA).unwrap();
    (0..STEPS).for_each(|_| moons.next());
    let total_energy = moons.energy();
    assert_eq!(9493, total_energy);

    let period = find_periodicity(moons);
    assert_eq!(326_365_108_375_488, period);
}
