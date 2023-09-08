use std::ops::Range;
use std::str::FromStr;

use itertools::Itertools;

use commons::error::Result;
use commons::grid::Point;
use commons::parse::LineSep;
use commons::{Report, WrapErr};

pub const TITLE: &str = "Day 15: Beacon Exclusion Zone";
const ROW: i64 = 2_000_000;
const MIN_COORDINATE: i64 = 0;
const MAX_COORDINATE: i64 = 4_000_000;

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data, ROW);
    println!("1. There are {first} points at Y={ROW} that are definitely not beacons");
    let second = second_part(&data, MIN_COORDINATE..(MAX_COORDINATE + 1));
    let second = second.wrap_err("distress beacon not found")?;
    println!("2. The distress beacon frequency is {second}");

    Ok(())
}

fn first_part(sensors: &[Sensor], y: i64) -> u64 {
    excluded_ranges(&mut Vec::new(), sensors, y, i64::MIN..i64::MAX)
        .map(|range| range.end.abs_diff(range.start).saturating_sub(1))
        .sum()
}

fn second_part(sensors: &[Sensor], coordinates: Range<i64>) -> Option<i64> {
    let mut buf = Vec::new();
    coordinates
        .clone()
        .rev()
        .find_map(
            // Brute-force solution: check for each Y if there is a candidate X
            |y| match excluded_ranges(&mut buf, sensors, y, coordinates.clone()).next() {
                Some(x) if x.end < coordinates.end => Some((x.end, y)),
                Some(x) if x.start > coordinates.start => Some((x.start - 1, y)),
                _ => None,
            },
        )
        .map(|(x, y)| {
            println!("Distress beacon at position x={x}, y={y}");
            x * MAX_COORDINATE + y
        })
}

fn excluded_ranges<'a>(
    buffer: &'a mut Vec<Range<i64>>, // Avoid doing too many allocations in a loop
    sensors: &[Sensor],
    y: i64,
    x: Range<i64>,
) -> impl Iterator<Item = Range<i64>> + 'a {
    buffer.extend(sensors.iter().filter_map(|sensor| {
        let distance = (sensor.sensor - sensor.beacon).manhattan_distance() as u64;
        let x_diff = distance.checked_sub(sensor.sensor.y.abs_diff(y))? as i64;
        let start = (sensor.sensor.x - x_diff).max(x.start);
        let end = (sensor.sensor.x + x_diff + 1).min(x.end);
        if start <= end {
            Some(start..end)
        } else {
            None
        }
    }));
    buffer.sort_unstable_by_key(|r| r.start);
    buffer.drain(..).coalesce(|first, second| {
        // Merge overlapping ranges
        if first.end >= second.start {
            Ok(first.start..second.end.max(first.end))
        } else {
            Err((first, second))
        }
    })
}

#[derive(Debug)]
struct Sensor {
    sensor: Point<i64>,
    beacon: Point<i64>,
}

impl FromStr for Sensor {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn point(s: &str) -> Result<Point<i64>> {
            s.split_once(',')
                .and_then(|(x, y)| {
                    let x = x.trim().strip_prefix("x=")?.parse::<i64>();
                    let y = y.trim().strip_prefix("y=")?.parse::<i64>();
                    match (x, y) {
                        (Ok(x), Ok(y)) => Some(Ok(Point::new(x, y))),
                        (Err(e), _) | (_, Err(e)) => Some(Err(e)),
                    }
                })
                .wrap_err("expected 'x={int}, y={int}'")
                .and_then(|res| res.wrap_err("unexpected coordinate"))
        }

        s.trim()
            .strip_prefix("Sensor at")
            .and_then(|s| s.split_once(": closest beacon is at"))
            .wrap_err("line format is unexpected")
            .and_then(|(sensor, beacon)| {
                let sensor = point(sensor)?;
                let beacon = point(beacon)?;
                Ok(Self { sensor, beacon })
            })
            .wrap_err_with(|| format!("for {s:?}"))
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Sensor>> {
    let split: LineSep<Sensor> = s.parse()?;
    Ok(split.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/15.txt");
    const MAIN: &str = include_str!("../inputs/15.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data, 10), 26);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data, ROW), 5_040_643);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        let result = second_part(&data, 0..21).unwrap();
        assert_eq!(result, 56_000_011);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        let result = second_part(&data, MIN_COORDINATE..(MAX_COORDINATE + 1)).unwrap();
        assert_eq!(result, 11_016_575_214_126);
    }
}
