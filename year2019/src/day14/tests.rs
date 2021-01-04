use super::*;

const TEST_ONE: &str = include_str!("1.txt");
const TEST_TWO: &str = include_str!("2.txt");
const TEST_THREE: &str = include_str!("3.txt");
const TEST_FOUR: &str = include_str!("4.txt");
const TEST_FIVE: &str = include_str!("5.txt");
const TEST_SIX: &str = include_str!("6.txt");

#[test]
fn single_fuel_production() {
    fn assertion(data: &str, requested_fuel: u64, expected_cost: u64) {
        let parsed = Day::parse(data).expect("Could not parse");
        let reactions = as_reaction_map(parsed.data);
        let times = produce_fuel_from_ore(requested_fuel, &reactions);

        assert_eq!(
            expected_cost,
            times,
            "Did not produce {fuel_number} {fuel} with {expected} {ore} but instead {real} {ore}",
            fuel = FUEL,
            ore = ORE,
            fuel_number = requested_fuel,
            expected = expected_cost,
            real = times
        )
    }

    assertion(TEST_ONE, 1, 31);
    assertion(TEST_TWO, 1, 165);
    assertion(TEST_THREE, 1, 13312);
    assertion(TEST_FOUR, 1, 180_697);
    assertion(TEST_FIVE, 1, 2_210_736);
    assertion(TEST_SIX, 1, 1_037_742);
}

#[test]
fn maximum_fuel_production() {
    fn assertion(data: &str, available_ore: u64, expected_fuel: u64) {
        let parsed = Day::parse(data).expect("Could not parse");
        let reactions = as_reaction_map(parsed.data);
        let fuel = maximum_fuel_produced_from(available_ore, &reactions);

        assert_eq!(
            expected_fuel,
            fuel,
            "We produced {fuel_number} {fuel} instead of {expected} {fuel} with {available} {ore}",
            fuel = FUEL,
            ore = ORE,
            available = available_ore,
            fuel_number = fuel,
            expected = expected_fuel
        )
    }

    assertion(TEST_THREE, TRILLION, 82_892_753);
    assertion(TEST_FOUR, TRILLION, 5_586_022);
    assertion(TEST_FIVE, TRILLION, 460_664);
    assertion(TEST_SIX, TRILLION, 1572358);
}