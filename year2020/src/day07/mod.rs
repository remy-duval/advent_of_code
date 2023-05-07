use std::collections::{HashMap, HashSet};

use commons::{Result, WrapErr};

pub const TITLE: &str = "Day 7: Handy Haversacks";
pub fn run(raw: String) -> Result<()> {
    let rules = parse(&raw)?;
    println!("{} bags contains a {WANTED_BAG} bag", first_part(&rules));
    println!("A {WANTED_BAG} will contain {} bags", second_part(&rules));
    Ok(())
}

/// The wanted bag
const WANTED_BAG: &str = "shiny gold";
type Bag<'a> = &'a str;
type BagRule<'a> = HashMap<Bag<'a>, u32>;
type Rules<'a> = HashMap<Bag<'a>, BagRule<'a>>;

fn parse(raw: &str) -> Result<Rules> {
    raw.lines()
        .map(|line| line.trim_end_matches('.'))
        .map(|line| {
            line.split_once("bags contain")
                .wrap_err_with(|| format!("Missing element in the string for a rule {line}"))
                .and_then(|(bag, rules)| match rules.trim() {
                    "no other bags" => Ok((bag.trim(), HashMap::default())),
                    rules => rules
                        .split(',')
                        .map(|rule| {
                            let rule = rule.trim().trim_end_matches("bag").trim_end_matches("bags");
                            let (number, bag) = rule.split_once(' ').wrap_err_with(|| {
                                format!("Missing element in the string for a rule {rule}")
                            })?;

                            let number: u32 = number.trim().parse().wrap_err_with(|| {
                                format!("Could not parse a number of bag in the rule {rule}")
                            })?;

                            Ok((bag.trim(), number))
                        })
                        .collect::<Result<_>>()
                        .map(|rules| (bag.trim(), rules)),
                })
        })
        .collect()
}

/// Find the number of bags that can contain the wanted one (recursively)
fn first_part(all_rules: &Rules) -> usize {
    let mut remaining = all_rules.iter().map(|(&k, v)| (k, v)).collect::<Vec<_>>();
    let mut wanted = HashSet::from([WANTED_BAG]);
    let mut next_wanted = Vec::new();
    while !wanted.is_empty() {
        remaining.retain(|(bag, inner)| {
            if inner.keys().any(|k| wanted.contains(*k)) {
                next_wanted.push(*bag);
                false
            } else {
                true
            }
        });
        wanted.clear();
        wanted.extend(next_wanted.drain(..));
    }

    all_rules.len() - remaining.len()
}

/// Count the numbers of bag inside the wanted one
fn second_part(all_rules: &Rules) -> u32 {
    count_bags_inside(all_rules, WANTED_BAG, &mut HashMap::default())
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
    let mut sum: u32 = 0;
    for (inner, times) in all_rules.get(bag).into_iter().flatten() {
        if let Some(count) = already_counted.get(*inner) {
            sum += *count * times;
        } else {
            let count = count_bags_inside(all_rules, inner, already_counted) + 1;
            already_counted.insert(inner, count);
            sum += count * times;
        }
    }
    sum
}

#[cfg(test)]
mod tests;
