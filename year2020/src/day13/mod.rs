use itertools::Itertools;

use commons::math::{extended_gcd, ExtendedGcd, Integer};
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

    let lines: Vec<Option<Timestamp>> = second
        .split(',')
        .map(|elt| match elt.trim() {
            "x" => Ok(None),
            number => number
                .parse::<Timestamp>()
                .map(Some)
                .wrap_err_with(|| format!("Could not parse a timestamp, got {number}")),
        })
        .try_collect()?;

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
fn second_part(bus: &[Option<Timestamp>]) -> Option<Result<Timestamp>> {
    chinese_remainder_theorem(bus.iter().enumerate().filter_map(|(index, bus)| {
        let bus = *bus.as_ref()?;
        let time_diff = -(index as Timestamp);
        Some((time_diff, bus))
    }))
}

/// Apply the [`Chinese remainder theorem`] on more than two values:
/// * `x mod n1 == a1`
/// * `x mod n2 == a2`
/// * `x mod n3 == a3`
/// * etc...
///
/// ### Arguments
/// * `a_n` - An iterator over (ai, ni)
///
/// ### Returns
/// None if `a_n` is empty
/// Some(Ok(x)) if all the n are co-primes where x is positive
/// Some(Err) if at least on the n are not co-primes
///
/// [`Chinese remainder theorem`]: https://en.wikipedia.org/wiki/Chinese_remainder_theorem
fn chinese_remainder_theorem(
    a_n: impl IntoIterator<Item = (Timestamp, Timestamp)>,
) -> Option<Result<Timestamp>> {
    let mut a_n = a_n.into_iter();
    let first = a_n.next()?;
    let result = a_n.try_fold(first, |(a1, n1), (a2, n2)| {
        let x = chinese_remainder_theorem_2((a1, a2), (n1, n2))?;
        Ok((x, n1 * n2))
    });

    Some(result.map(|(x, _)| x))
}

/// Apply the [`Chinese remainder theorem`] for two values, finding the smallest `x` such that:
/// * `x mod n1 == a1`
/// * `x mod n2 == a2`
///
/// ### Returns
/// An x (positive or negative) that satisfies the constraints
///
/// [`Chinese remainder theorem`]: https://en.wikipedia.org/wiki/Chinese_remainder_theorem
fn chinese_remainder_theorem_2(
    (a1, a2): (Timestamp, Timestamp),
    (n1, n2): (Timestamp, Timestamp),
) -> Result<Timestamp> {
    let ExtendedGcd { a, b, gcd } = extended_gcd(n1, n2);
    if gcd != 1 {
        return Err(err!("{n1} and {n2} are not co-prime"));
    }
    let n = n1 * n2;
    let x = a1 * b * n2 + a2 * a * n1;
    Ok(if x < 0 { x % n + n } else { x % n })
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
