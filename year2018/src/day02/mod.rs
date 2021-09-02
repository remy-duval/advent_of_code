use color_eyre::eyre::{eyre, Result};
use hashbrown::HashMap;

use commons::parse::LineSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<String>;
    const TITLE: &'static str = "Day 2: Inventory Management System";

    fn solve(data: Self::Input) -> Result<()> {
        println!("The checksum is {}", check_sum(&data.data));
        println!(
            "The common part of the two found box is {}",
            find_different_by_one(&data.data)
                .ok_or_else(|| eyre!("Could not find the two common boxes"))?
        );

        Ok(())
    }
}

/// Compute the check sum, (boxes with 2 repeated char * boxes with 3 repeated char)
fn check_sum(boxes: &[String]) -> u32 {
    let mut counts: HashMap<char, u32> = HashMap::with_capacity(26);
    let (two, three) = boxes
        .iter()
        .fold((0, 0), |(exactly_two, exactly_three), next| {
            next.chars().for_each(|char| {
                *counts.entry(char).or_insert(0) += 1;
            });

            let (has_two, has_three) = counts
                .drain()
                .fold((false, false), |(two, three), (_, count)| {
                    (two || count == 2, three || count == 3)
                });

            (
                exactly_two + if has_two { 1 } else { 0 },
                exactly_three + if has_three { 1 } else { 0 },
            )
        });

    two * three
}

/// Find the two box that differ by one character, and compute the common parts of the two
fn find_different_by_one(boxes: &[String]) -> Option<String> {
    let (first, second) = boxes.iter().enumerate().skip(1).find_map(|(i, a)| {
        boxes
            .iter()
            .take(i)
            .find(|b| {
                // Check if the number of differing characters is exactly one
                a.chars()
                    .zip(b.chars())
                    .filter(|(a, b)| *a != *b)
                    .take(2)
                    .count()
                    == 1
            })
            .map(|b| (a.as_str(), b.as_str()))
    })?;

    Some(
        first
            .chars()
            .zip(second.chars())
            .filter_map(|(a, b)| if a == b { Some(a) } else { None })
            .collect(),
    )
}

#[cfg(test)]
mod tests;
