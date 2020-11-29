use std::collections::HashMap;
use std::iter::Iterator;
use std::str::FromStr;

use aoc::generator::{data_from_cli, LineSep};

const TITLE: &str = "Day 14: Space Stoichiometry";
const DATA: &str = include_str!("../resources/day14.txt");
const ORE: &str = "ORE";
const FUEL: &str = "FUEL";
const TRILLION: u64 = 1_000_000_000_000;

fn main() {
    let data = data_from_cli(TITLE, DATA);
    println!("{}", TITLE);
    let reactions = parse_reactions(&data).expect("Could not parse the reactions");
    // Part one
    let cost_for_one_fuel = produce_fuel_from_ore(1, &reactions);
    println!(
        "To produce 1 {} we need {} {}",
        FUEL, cost_for_one_fuel, ORE
    );

    let maximum_amount = maximum_fuel_produced_from(TRILLION, &reactions);
    println!(
        "With {} {} we can produce {} {}",
        TRILLION, ORE, maximum_amount, FUEL
    );
}

/// Parse the reactions string into a map of the produced component and its reaction
fn parse_reactions(data: &str) -> Result<HashMap<String, Reaction>, &'static str> {
    Ok(data
        .parse::<LineSep<Reaction>>()?
        .data
        .into_iter()
        .map(|reaction| (reaction.result.clone(), reaction))
        .collect::<HashMap<_, _>>())
}

/// Produces a certain amount of fuel from ore and a reaction list and returns ORE cost
fn produce_fuel_from_ore(number: u64, reactions: &HashMap<String, Reaction>) -> u64 {
    let mut remaining: HashMap<&str, u64> = HashMap::new();
    let mut requested: HashMap<&str, u64> = HashMap::new();
    requested.insert(FUEL, number);

    let mut ore_required: u64 = 0;
    loop {
        let production = requested
            .into_iter()
            .filter_map(|(ingredient, times)| {
                // If ingredient is ORE we increase the count and remove it from the request.
                if ingredient == ORE {
                    ore_required += times;
                    return None;
                }

                let already_created = remaining.entry(ingredient).or_default();
                // Check if we already have all the needed ingredients in reserve
                if times <= *already_created {
                    *already_created -= times;
                    return None;
                }

                // If we don't, we produce what we are missing (and update remaining)
                let rest = times - *already_created;
                let reaction = reactions.get(ingredient).unwrap();
                let times = rest / reaction.times + if rest % reaction.times == 0 { 0 } else { 1 };
                *already_created = reaction.times * times - rest; // The produced surplus

                Some(
                    reaction
                        .ingredients
                        .iter()
                        .map(move |(name, num)| (name.as_str(), num * times)),
                )
            })
            .flatten();

        // Store the required products in requested again
        requested = HashMap::new();
        for (ingredient, number) in production {
            *requested.entry(ingredient).or_default() += number;
        }

        // If the requested products are empty, then we are done
        if requested.is_empty() {
            return ore_required;
        }
    }
}

/// Find the maximum amount of fuel that can be produced with the given amount of ore.
fn maximum_fuel_produced_from(ore: u64, reactions: &HashMap<String, Reaction>) -> u64 {
    binary_search(0, ore, ore, |number| {
        produce_fuel_from_ore(number, &reactions)
    })
}

/// Simple binary search for an expected result
fn binary_search<F>(bottom: u64, top: u64, expected: u64, function: F) -> u64
where
    F: Fn(u64) -> u64,
{
    let middle = (bottom + top) / 2;
    let result = function(middle);
    if result == expected || bottom == middle {
        middle
    } else if result < expected {
        binary_search(middle, top, expected, function)
    } else {
        binary_search(bottom, middle, expected, function)
    }
}

#[derive(Debug)]
struct Reaction {
    result: String,
    times: u64,
    ingredients: HashMap<String, u64>,
}

impl FromStr for Reaction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_ingredient(data: &str) -> Option<(String, u64)> {
            let mut parsed = data.trim().split(' ').map(|part| part.trim());
            let number: u64 = parsed.next()?.parse().ok()?;
            let name: String = parsed.next()?.into();

            Some((name, number))
        }

        let mut split = s.split("=>");
        let ingredients = split.next().ok_or("Could not find the ingredients")?;
        let product = split.next().ok_or("Could not find the product")?;

        let ingredients = ingredients
            .split(',')
            .map(|data| parse_ingredient(data))
            .collect::<Option<HashMap<_, _>>>()
            .ok_or("Could not parse the ingredients")?;
        let (result, times) = parse_ingredient(product).ok_or("Could not parse the product")?;

        Ok(Self {
            result,
            times,
            ingredients,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ONE: &str = include_str!("../test_resources/day14_1.txt");
    const TEST_TWO: &str = include_str!("../test_resources/day14_2.txt");
    const TEST_THREE: &str = include_str!("../test_resources/day14_3.txt");
    const TEST_FOUR: &str = include_str!("../test_resources/day14_4.txt");
    const TEST_FIVE: &str = include_str!("../test_resources/day14_5.txt");

    #[test]
    fn single_fuel_production() {
        fn assertion(data: &str, requested_fuel: u64, expected_cost: u64) {
            let reactions = parse_reactions(data).expect("Could not parse the reactions");
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
    }

    #[test]
    fn maximum_fuel_production() {
        fn assertion(data: &str, available_ore: u64, expected_fuel: u64) {
            let reactions = parse_reactions(data).expect("Could not parse the reactions");
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
    }
}
