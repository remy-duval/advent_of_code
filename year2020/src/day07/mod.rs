use color_eyre::eyre::{eyre, Result, WrapErr};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use commons::Problem;

/// The wanted bag
const WANTED_BAG: &str = "shiny gold";

pub struct Day;

impl Problem for Day {
    type Input = String;
    const TITLE: &'static str = "Day 7: Handy Haversacks";

    fn solve(data: Self::Input) -> Result<()> {
        let rules = parse_rules(&data)?;
        println!(
            "{bags} contains a {wanted} bag",
            bags = first_part(&rules),
            wanted = WANTED_BAG
        );

        println!(
            "A {bag} will contain {count} bags",
            bag = WANTED_BAG,
            count = second_part(&rules)
        );

        Ok(())
    }
}

type Bag<'a> = &'a str;
type BagRule<'a> = HashMap<Bag<'a>, u32>;
type Rules<'a> = HashMap<Bag<'a>, BagRule<'a>>;

/// Find the number of bags that can contain the wanted one (recursively)
fn first_part<'a>(all_rules: &Rules<'a>) -> usize {
    let mut all: HashSet<Bag<'a>> = HashSet::new();
    let mut current: HashSet<Bag<'a>> = HashSet::new();
    current.insert(WANTED_BAG);
    while !current.is_empty() {
        current = bag_containing(all_rules, &current, &all);
        all.extend(current.iter());
    }

    all.len()
}

/// Count the numbers of bag inside the wanted one
fn second_part(all_rules: &Rules<'_>) -> u32 {
    let mut counted = HashMap::new();
    count_bags_inside(all_rules, WANTED_BAG, &mut counted)
}

/// Find all the bags that contains the wanted bag that are not already found
///
/// ### Arguments
/// * `all_rules` - The mapping between bags and the bags they should contain
/// * `wanted` - The bags we want to find containers for
/// * `already_found` - The bags that are already found containing some wanted bags
///
/// ### Returns
/// A set of the bags that are not in `already_found` and contain at least a bag in `wanted`
fn bag_containing<'a>(
    all_rules: &Rules<'a>,
    wanted: &HashSet<Bag<'a>>,
    already_found: &HashSet<Bag<'a>>,
) -> HashSet<Bag<'a>> {
    all_rules
        .iter()
        .filter(|(bag, inner)| {
            !already_found.contains(*bag) && inner.keys().any(|key| wanted.contains(*key))
        })
        .map(|(bag, _)| *bag)
        .collect()
}

/// Count the number of bags inside one
///
/// ### Arguments
/// * `all_rules` - The mapping between bags and the bags they should contain and their numbers
/// * `bag` - The bag we want to count the inner values for
/// * `already_counted` - A memoization already counted bags to improve performance
///
/// ### Returns
/// The number of bags inside `bag`
fn count_bags_inside<'a>(
    all_rules: &Rules<'a>,
    bag: Bag<'a>,
    already_counted: &mut HashMap<Bag<'a>, u32>,
) -> u32 {
    if let Some(rules) = all_rules.get(bag) {
        rules
            .into_iter()
            .map(|(inner, times)| {
                if let Some(count) = already_counted.get(*inner).copied() {
                    count * times
                } else {
                    let count = count_bags_inside(all_rules, *inner, already_counted) + 1;
                    already_counted.insert(inner, count);
                    count * times
                }
            })
            .sum()
    } else {
        0
    }
}

fn parse_rules(raw: &str) -> Result<Rules<'_>> {
    raw.lines()
        .map(|line| {
            let line = line.trim_end_matches('.');
            let (bag, rules) = line
                .split_once("bags contain")
                .map(|(a, b)| (a.trim(), b.trim()))
                .ok_or_else(|| eyre!("Missing element in the string for a rule {}", line))?;

            let rules = if rules == "no other bags" {
                HashMap::new()
            } else {
                rules
                    .split(',')
                    .map(|rule| {
                        let rule = rule.trim().trim_end_matches("bag").trim_end_matches("bags");
                        let (number, bag) = rule
                            .split_once(' ')
                            .map(|(a, b)| (a.trim(), b.trim()))
                            .ok_or_else(|| {
                                eyre!("Missing element in the string for a rule {}", rule)
                            })?;

                        let number = number.parse::<u32>().wrap_err_with(|| {
                            format!("Could not parse a number of bag in the rule {}", rule)
                        })?;

                        Ok((bag, number))
                    })
                    .collect::<Result<_>>()?
            };

            Ok((bag, rules))
        })
        .try_collect()
}

#[cfg(test)]
mod tests;
