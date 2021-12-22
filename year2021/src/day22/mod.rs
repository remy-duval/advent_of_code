#![allow(unused)]

use std::ops::Range;
use std::str::FromStr;

use itertools::Itertools;

use commons::eyre::{eyre, Report, Result, WrapErr};
use commons::parse::LineSep;

pub const TITLE: &str = "Day 22: Reactor Reboot";

pub fn run(raw: String) -> Result<()> {
    let boot = parse(&raw)?;
    println!("1. Initialization: {}", first_part(&boot));
    println!("2. Full: {}", all_points(boot));

    Ok(())
}

/// Parse the reboot instructions from the input
fn parse(s: &str) -> Result<Vec<Boot>> {
    Ok(s.parse::<LineSep<Boot>>()?.data)
}

/// Count the points active in the initialization region after the instructions
fn first_part(instructions: &[Boot]) -> i64 {
    let center = instructions.iter().filter_map(|boot| {
        let mut result = boot.clone();
        for (range, dest) in boot.zone.0.iter().zip(result.zone.0.iter_mut()) {
            let range = range.start.max(-50)..range.end.min(51);
            if range.is_empty() {
                return None;
            }
            *dest = range;
        }

        Some(result)
    });

    all_points(center)
}

/// Count the points active after the instructions
fn all_points(instructions: impl IntoIterator<Item = Boot>) -> i64 {
    let mut on: Vec<Cuboid> = Vec::new();
    let mut off: Vec<Cuboid> = Vec::new();
    instructions.into_iter().for_each(|Boot { active, zone }| {
        let before = off.len();
        off.extend(on.iter().filter_map(|c| zone.overlap(c)));
        on.extend(off[..before].iter().filter_map(|c| zone.overlap(c)));
        if active {
            on.push(zone);
        }
    });

    let mut sum = 0;
    while !on.is_empty() || !off.is_empty() {
        if let Some(c) = on.pop() {
            sum += c.size();
        }
        if let Some(c) = off.pop() {
            sum -= c.size();
        }
    }
    sum
}

/// The boot instructions
#[derive(Debug, Clone)]
struct Boot {
    active: bool,
    zone: Cuboid,
}

/// A zone in space
#[derive(Debug, Clone)]
struct Cuboid([Range<i32>; 3]);

impl Cuboid {
    /// The size of this cuboid
    fn size(&self) -> i64 {
        self.0
            .iter()
            .map(|x| x.end as i64 - x.start as i64)
            .product()
    }

    /// Compute the overlap of two cuboids
    fn overlap(&self, other: &Self) -> Option<Self> {
        fn range(a: &Range<i32>, b: &Range<i32>) -> Option<Range<i32>> {
            let r = a.start.max(b.start)..(a.end.min(b.end));
            if r.is_empty() {
                None
            } else {
                Some(r)
            }
        }

        let x = range(&self.0[0], &other.0[0])?;
        let y = range(&self.0[1], &other.0[1])?;
        let z = range(&self.0[2], &other.0[2])?;
        Some(Self([x, y, z]))
    }
}

impl FromStr for Boot {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (active, zone) = s
            .split_once(' ')
            .ok_or_else(|| eyre!("Bad format for '{}'", s))?;

        Ok(Self {
            active: active == "on",
            zone: zone.parse()?,
        })
    }
}

impl FromStr for Cuboid {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y, z) = s
            .split(',')
            .map(|coord| -> Result<Range<i32>> {
                let c = coord
                    .strip_prefix("x=")
                    .or_else(|| coord.strip_prefix("y="))
                    .or_else(|| coord.strip_prefix("z="))
                    .ok_or_else(|| eyre!("Missing x=, y= or z= prefix: '{}'", coord))?;

                let (from, to) = c
                    .splitn(2, "..")
                    .map(|n| n.parse().wrap_err_with(|| format!("In '{}'", c)))
                    .collect_tuple::<(Result<i32>, Result<i32>)>()
                    .ok_or_else(|| eyre!("Missing '..' range delimiter in '{}'", c))?;

                let (from, to) = (from?, to?);
                Ok(from.min(to)..(from.max(to) + 1))
            })
            .collect_tuple::<(_, _, _)>()
            .ok_or_else(|| eyre!("Missing X, Y or Z in '{}'", s))?;

        Ok(Self([x?, y?, z?]))
    }
}

#[cfg(test)]
mod tests;
