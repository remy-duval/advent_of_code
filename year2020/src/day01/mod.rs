use std::collections::HashSet;

use commons::{Result, WrapErr};

pub const TITLE: &str = "Day 1: AdventOfCodeError Repair";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let (first, second) = first_part(&data.data)
        .wrap_err_with(|| format!("No 2020 2-elements sum found in {:?}", data.data))?;
    println!(
        "2 expenses that sum to 2020: {a} * {b} = {product}",
        a = first,
        b = second,
        product = first * second
    );

    let (first, second, third) = second_part(&data.data)
        .wrap_err_with(|| format!("No 2020 3-elements sum found in {:?}", data.data))?;

    println!(
        "3 expenses that sum to 2020: {a} * {b} * {c} = {product}",
        a = first,
        b = second,
        c = third,
        product = first * second * third
    );
    Ok(())
}

fn parse(s: &str) -> Result<commons::parse::LineSep<u64>> {
    Ok(s.parse()?)
}

/// Find two elements of `data` that sum to `wanted`.
/// Already seen numbers are stored in `seen` for faster lookup
fn two_sum(data: &[u64], wanted: u64, seen: &mut HashSet<u64>) -> Option<(u64, u64)> {
    data.iter().find_map(|&element| {
        if element < wanted {
            if seen.contains(&(wanted - element)) {
                Some((element, (wanted - element)))
            } else {
                seen.insert(element);
                None
            }
        } else {
            None
        }
    })
}

fn first_part(expenses: &[u64]) -> Option<(u64, u64)> {
    two_sum(expenses, 2020, &mut HashSet::with_capacity(expenses.len()))
}

fn second_part(expenses: &[u64]) -> Option<(u64, u64, u64)> {
    let mut seen = HashSet::with_capacity(expenses.len());
    expenses.iter().find_map(|&element| {
        if element <= 2020 {
            let (a, b) = two_sum(expenses, 2020 - element, &mut seen)?;
            Some((a, b, element))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests;
