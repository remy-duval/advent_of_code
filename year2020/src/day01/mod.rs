use commons::eyre::{eyre, Result};
use hashbrown::HashSet;

use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = commons::parse::LineSep<u64>;
    const TITLE: &'static str = "Day 1: Report Repair";

    fn solve(data: Self::Input) -> Result<()> {
        let (first, second) = first_part(&data.data)
            .ok_or_else(|| eyre!("No 2020 2-elements sum found in {:?}", data.data))?;
        println!(
            "2 expenses that sum to 2020: {a} * {b} = {product}",
            a = first,
            b = second,
            product = first * second
        );

        let (first, second, third) = second_part(&data.data)
            .ok_or_else(|| eyre!("No 2020 3-elements sum found in {:?}", data.data))?;

        println!(
            "3 expenses that sum to 2020: {a} * {b} * {c} = {product}",
            a = first,
            b = second,
            c = third,
            product = first * second * third
        );
        Ok(())
    }
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
