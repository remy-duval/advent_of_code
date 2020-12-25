use std::str::FromStr;

use itertools::Itertools;
use num_integer::{ExtendedGcd, Integer};

use crate::Problem;

pub type Timestamp = i128;

pub struct Day;

impl Problem for Day {
    type Input = Schedule;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 13: Shuttle Search";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let (bus, time) =
            earliest(&data).ok_or_else(|| anyhow::anyhow!("No bus to find the earliest one"))?;

        println!(
            "The earliest bus to depart with is {bus} by waiting {min}, product is {product}",
            bus = bus,
            min = time,
            product = bus * time
        );

        let timestamp =
            second_part(&data.lines).ok_or_else(|| anyhow::anyhow!("No bus for second part"))??;

        println!(
            "The earliest timestamp at which all lines will start 1 min after the other is {time}",
            time = timestamp
        );

        Ok(())
    }
}

/// Find the earliest departure after the schedule timestamp
fn earliest(schedule: &Schedule) -> Option<(Timestamp, Timestamp)> {
    schedule
        .lines
        .iter()
        .filter_map(|&x| x)
        .map(|bus| (bus, bus - schedule.timestamp.rem_euclid(bus)))
        .min_by_key(|(_, earliest)| *earliest)
}

/// Find the earliest timestamp such as each bus line will depart one minute after the other
fn second_part(bus: &[Option<Timestamp>]) -> Option<Result<Timestamp, NotCoPrime>> {
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
) -> Option<Result<Timestamp, NotCoPrime>> {
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
) -> Result<Timestamp, NotCoPrime> {
    let ExtendedGcd {
        gcd, x: m1, y: m2, ..
    } = n1.extended_gcd(&n2);
    if gcd != 1 {
        Err(NotCoPrime(n1, n2))
    } else {
        let n = n1 * n2;
        let x = a1 * m2 * n2 + a2 * m1 * n1;
        Ok(x.mod_floor(&n))
    }
}

/// The bus schedules
#[derive(Debug, Clone)]
pub struct Schedule {
    /// The earliest timestamp at which we can depart
    timestamp: Timestamp,
    /// The bus lines (None means unavailable, Some(id) is bus that departs every id minutes)
    lines: Vec<Option<Timestamp>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ScheduleParseError {
    #[error("Expected two lines: earliest timestamp then comma separated bus lines, got {0}")]
    BadFormat(Box<str>),
    #[error("Could not parse a bus line or timestamp, got {0} ({1})")]
    NumberParseError(Box<str>, std::num::ParseIntError),
}

#[derive(Debug, thiserror::Error)]
#[error("Can't apply the chinese remainder theorem, as {0} and {1} are not co-prime")]
pub struct NotCoPrime(Timestamp, Timestamp);

impl FromStr for Schedule {
    type Err = ScheduleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .lines()
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| ScheduleParseError::BadFormat(s.into()))?;

        let timestamp: Timestamp = first
            .parse()
            .map_err(|err| ScheduleParseError::NumberParseError(first.into(), err))?;

        let lines: Vec<Option<Timestamp>> = second
            .split(',')
            .map(|elt| match elt.trim() {
                "x" => Ok(None),
                number => match number.parse::<Timestamp>() {
                    Ok(number) => Ok(Some(number)),
                    Err(err) => Err(ScheduleParseError::NumberParseError(first.into(), err)),
                },
            })
            .try_collect()?;

        Ok(Self { timestamp, lines })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = include_str!("test_resources/13-A.txt");
    const B: &str = include_str!("test_resources/13-B.txt");

    #[test]
    fn test_chinese_remainder_theorem() {
        let values = vec![(0, 3), (3, 4), (4, 5)];
        let result = chinese_remainder_theorem(values).unwrap().unwrap();
        assert_eq!(result, 39);
    }

    #[test]
    fn first_part_test_a() {
        let resource = Day::parse(A).unwrap();
        let (bus, time) = earliest(&resource).unwrap();
        assert_eq!(bus, 59);
        assert_eq!(time, 5);
        assert_eq!(bus * time, 295);
    }

    #[test]
    fn first_part_test_b() {
        let resource = Day::parse(B).unwrap();
        let (bus, time) = earliest(&resource).unwrap();
        assert_eq!(bus, 443);
        assert_eq!(time, 5);
        assert_eq!(bus * time, 2_215);
    }

    #[test]
    fn second_part_example_a() {
        let values = [Some(17 as Timestamp), None, Some(13), Some(19)];
        let result = second_part(&values).unwrap().unwrap();
        assert_eq!(result, 3_417);
    }

    #[test]
    fn second_part_example_b() {
        let values = [Some(67 as Timestamp), Some(7), Some(59), Some(61)];
        let result = second_part(&values).unwrap().unwrap();
        assert_eq!(result, 754_018);
    }

    #[test]
    fn second_part_example_c() {
        let values = [Some(67 as Timestamp), None, Some(7), Some(59), Some(61)];
        let result = second_part(&values).unwrap().unwrap();
        assert_eq!(result, 779_210);
    }

    #[test]
    fn second_part_example_d() {
        let values = [Some(67 as Timestamp), Some(7), None, Some(59), Some(61)];
        let result = second_part(&values).unwrap().unwrap();
        assert_eq!(result, 1_261_476);
    }

    #[test]
    fn second_part_example_e() {
        let values = [Some(1789 as Timestamp), Some(37), Some(47), Some(1889)];
        let result = second_part(&values).unwrap().unwrap();
        assert_eq!(result, 1_202_161_486);
    }

    #[test]
    fn second_part_test_a() {
        let resource = Day::parse(A).unwrap();
        let timestamp = second_part(&resource.lines).unwrap().unwrap();
        assert_eq!(timestamp, 1_068_781);
    }

    #[test]
    fn second_part_test_b() {
        let resource = Day::parse(B).unwrap();
        let timestamp = second_part(&resource.lines).unwrap().unwrap();
        assert_eq!(timestamp, 1_058_443_396_696_792);
    }
}
