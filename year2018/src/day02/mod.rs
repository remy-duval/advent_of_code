use std::collections::HashMap;

use commons::{err, error::Result};

pub const TITLE: &str = "Day 2: Inventory Management System";

pub fn run(raw: String) -> Result<()> {
    println!("The checksum is {}", check_sum(&raw));
    println!(
        "The common part of the two found box is {}",
        find_different_by_one(&raw).ok_or_else(|| err!("Could not find the two common boxes"))?
    );

    Ok(())
}

/// Compute the check sum, (boxes with 2 repeated char * boxes with 3 repeated char)
fn check_sum(boxes: &str) -> u32 {
    let mut counts: HashMap<char, u32> = HashMap::with_capacity(26);
    let (two, three) = boxes
        .lines()
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
                exactly_two + u32::from(has_two),
                exactly_three + u32::from(has_three),
            )
        });

    two * three
}

/// Find the two box that differ by one character, and compute the common parts of the two
fn find_different_by_one(boxes: &str) -> Option<String> {
    let (first, second) = boxes.lines().enumerate().skip(1).find_map(|(i, a)| {
        boxes
            .lines()
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
            .map(|b| (a, b))
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
