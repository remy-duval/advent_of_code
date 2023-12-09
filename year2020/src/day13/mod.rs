use itertools::Itertools;

use commons::math::{chinese_remainder_theorem, Integer, NotCoPrimeError};
use commons::{err, Result, WrapErr};

pub const TITLE: &str = "Day 13: Shuttle Search";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let (bus, time) = earliest(&data).wrap_err("No bus to find the earliest one")?;

    println!(
        "The earliest bus to depart with is {bus} by waiting {min}, product is {product}",
        bus = bus,
        min = time,
        product = bus * time
    );

    let timestamp = second_part(&data.lines).wrap_err("No bus for second part")??;

    println!("The earliest time at which all lines will start 1min after the other is {timestamp}");

    Ok(())
}

type Timestamp = i128;

fn parse(s: &str) -> Result<Schedule> {
    let (first, second) = s.lines().collect_tuple::<(_, _)>().ok_or_else(|| {
        err!(
            "Expected two lines: earliest timestamp then comma separated bus lines, got {}",
            s
        )
    })?;

    let timestamp: Timestamp = first
        .parse()
        .wrap_err_with(|| format!("Could not parse a bus line, got {first}"))?;

    let lines = second
        .split(',')
        .map(|elt| match elt.trim() {
            "x" => Ok(None),
            number => number
                .parse()
                .map(Some)
                .wrap_err_with(|| format!("Could not parse a timestamp, got {number}")),
        })
        .collect::<Result<Vec<Option<Timestamp>>>>()?;

    Ok(Schedule { timestamp, lines })
}

/// Find the earliest departure after the schedule timestamp
fn earliest(schedule: &Schedule) -> Option<(Timestamp, Timestamp)> {
    schedule
        .lines
        .iter()
        .filter_map(|&x| x)
        .map(|bus| (bus, bus - schedule.timestamp.remainder_euclid(bus)))
        .min_by_key(|(_, earliest)| *earliest)
}

/// Find the earliest timestamp such as each bus line will depart one minute after the other
fn second_part(bus: &[Option<Timestamp>]) -> Option<Result<Timestamp, NotCoPrimeError<Timestamp>>> {
    chinese_remainder_theorem(bus.iter().enumerate().filter_map(|(index, bus)| {
        let bus = *bus.as_ref()?;
        let time_diff = -(index as Timestamp);
        Some((time_diff, bus))
    }))
}

/// The bus schedules
struct Schedule {
    /// The earliest timestamp at which we can depart
    timestamp: Timestamp,
    /// The bus lines (None means unavailable, Some(id) is bus that departs every id minutes)
    lines: Vec<Option<Timestamp>>,
}

#[cfg(test)]
mod tests;
