use std::collections::HashMap;

use commons::{ensure, Result, WrapErr};

pub const TITLE: &str = "Day 7: Handy Haversacks";
pub fn run(raw: String) -> Result<()> {
    let rules = parse(&raw)?;
    let containing_bags = first_part(&rules);
    println!("{containing_bags} bags contains a {WANTED_BAG} bag");
    let contained_bags = second_part(&rules);
    println!("A {WANTED_BAG} will contain {contained_bags} bags");
    Ok(())
}

/// The wanted bag
const WANTED_BAG: &str = "shiny gold";

struct Rules {
    /// Each element is a bag content
    by_index: Vec<Vec<BagContent>>,
    /// The index of the wanted bag in the Vec
    wanted: usize,
}

struct BagContent {
    /// The index of the contained (u32 to use less memory)
    contained: u32,
    /// Number of times this bag is contained
    count: u32,
}

fn parse(raw: &str) -> Result<Rules> {
    // Parse the descriptions of each bag content by name
    let by_name: HashMap<&str, (usize, &str)> = raw
        .lines()
        .map(|line| line.trim_end_matches('.'))
        .enumerate()
        .map(|(index, line)| {
            line.split_once("bags contain")
                .wrap_err_with(|| format!("missing elements in {line}"))
                .map(|(bag, content)| (bag.trim(), (index, content)))
        })
        .collect::<Result<_>>()?;

    ensure!(
        by_name.len() < u32::MAX as usize,
        "too many bags to fit in a 32-bits int"
    );

    // Convert all the names into indexes in a Vec for lookup efficiency
    let mut by_index: Vec<Vec<BagContent>> = Vec::with_capacity(by_name.len());
    (0..by_index.capacity()).for_each(|_| by_index.push(Vec::new()));
    for (bag, (index, line)) in by_name.iter() {
        by_index[*index] = match line.trim() {
            "no other bags" => continue,
            description => description
                .split(',')
                .map(|content| {
                    let (num, bag) = content
                        .trim()
                        .trim_end_matches("bag")
                        .trim_end_matches("bags")
                        .split_once(' ')
                        .wrap_err("missing contained bags syntax")?;

                    let count = num
                        .trim()
                        .parse()
                        .wrap_err_with(|| format!("number {num}"))?;

                    let contained = by_name
                        .get(bag.trim())
                        .wrap_err_with(|| format!("missing the line describing {bag}"))
                        .map(|t| t.0 as u32)?;

                    Ok(BagContent { contained, count })
                })
                .collect::<Result<_>>()
                .wrap_err_with(|| format!("in '{bag} bags contain {description}'"))?,
        };
    }

    // Find the position of the wanted bag
    let wanted = by_name
        .get(WANTED_BAG)
        .wrap_err_with(|| format!("missing target bag {WANTED_BAG} line"))
        .map(|t| t.0)?;

    Ok(Rules { by_index, wanted })
}

/// Find the number of bags that can contain the wanted one (recursively)
fn first_part(rules: &Rules) -> usize {
    let length = rules.by_index.len();
    let mut bags: Vec<u32> = (0..(length as u32)).collect();
    bags.swap(rules.wanted, length - 1);

    let (mut remaining, mut wanted) = bags.split_at_mut(length - 1);
    while !wanted.is_empty() {
        wanted.sort_unstable(); // Required for binary search
        let mut start = 0;
        let mut end = remaining.len();
        while start < end {
            let start_contains_wanted = rules.by_index[remaining[start] as usize]
                .iter()
                .any(|c| wanted.binary_search(&c.contained).is_ok());

            if start_contains_wanted {
                remaining.swap(start, end - 1);
                end -= 1;
            } else {
                start += 1;
            }
        }

        (remaining, wanted) = remaining.split_at_mut(end);
    }

    length - remaining.len() - 1
}

/// Count the numbers of bag inside the wanted one
fn second_part(all_rules: &Rules) -> u32 {
    let mut cache = vec![0; all_rules.by_index.len()];
    count_bags_inside(all_rules, all_rules.wanted, &mut cache)
}

/// Count the number of bags inside the bag with the given index
fn count_bags_inside(all_rules: &Rules, bag: usize, cache: &mut [u32]) -> u32 {
    let mut sum: u32 = 0;
    for content in all_rules.by_index.get(bag).into_iter().flatten() {
        let index = content.contained as usize;
        if let Some(mut count) = cache.get(index).copied() {
            // The cached value is always at least 1, 0 is not yet computed
            if count == 0 {
                count = count_bags_inside(all_rules, index, cache) + 1;
                cache[index] = count;
            }

            sum += count * content.count;
        }
    }
    sum
}

#[cfg(test)]
mod tests;
