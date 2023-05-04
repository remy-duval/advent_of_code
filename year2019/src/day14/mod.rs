use std::collections::HashMap;
use std::iter::Iterator;
use std::str::FromStr;

use commons::parse::LineSep;
use commons::{Report, Result, WrapErr};

pub const TITLE: &str = "Day 14: Space Stoichiometry";
const ORE: &str = "ORE";
const FUEL: &str = "FUEL";
const TRILLION: u64 = 1_000_000_000_000;

pub fn run(raw: String) -> Result<()> {
    let reactions = as_reaction_map(parse(&raw)?.data);
    // Part one
    let cost_for_one_fuel = produce_fuel_from_ore(1, &reactions);
    println!("To produce 1 {FUEL} we need {cost_for_one_fuel} {ORE}");

    let maximum_amount = maximum_fuel_produced_from(TRILLION, &reactions);
    println!("With {TRILLION} {ORE} we can produce {maximum_amount} {FUEL}");

    Ok(())
}

fn parse(s: &str) -> Result<LineSep<Reaction>> {
    s.parse()
}

/// Group the reactions by name
fn as_reaction_map(data: Vec<Reaction>) -> HashMap<String, Reaction> {
    data.into_iter()
        .map(|reaction| (reaction.result.clone(), reaction))
        .collect()
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
                let times = rest / reaction.times + u64::from(rest % reaction.times != 0);
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
        produce_fuel_from_ore(number, reactions)
    })
}

/// Simple binary search for an expected result
fn binary_search<F>(bottom: u64, top: u64, expected: u64, function: F) -> u64
where
    F: Fn(u64) -> u64,
{
    let mut bottom = bottom;
    let mut top = top;
    loop {
        let middle = (bottom + top) / 2;
        let result = function(middle);
        if result == expected || bottom == middle {
            return middle;
        } else if result < expected {
            bottom = middle;
        } else {
            top = middle;
        }
    }
}

struct Reaction {
    result: String,
    times: u64,
    ingredients: HashMap<String, u64>,
}

impl FromStr for Reaction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn parse_ingredient(data: &str) -> Option<(String, u64)> {
            let mut parsed = data.trim().split(' ').map(|part| part.trim());
            let number: u64 = parsed.next()?.parse().ok()?;
            let name: String = parsed.next()?.into();

            Some((name, number))
        }

        let mut split = s.split("=>");
        let ingredients = split.next().wrap_err("Could not find the ingredients")?;
        let product = split.next().wrap_err("Could not find the product")?;

        let ingredients = ingredients
            .split(',')
            .map(parse_ingredient)
            .collect::<Option<HashMap<_, _>>>()
            .wrap_err("Could not parse the ingredients")?;
        let (result, times) = parse_ingredient(product).wrap_err("Could not parse the product")?;

        Ok(Self {
            result,
            times,
            ingredients,
        })
    }
}

#[cfg(test)]
mod tests;
