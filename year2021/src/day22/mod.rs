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
    // Split a main range to not overlap with the second range
    // Update the main range to only keep the part that remains to be split on other axis
    fn split_axis(main: &mut Range<i32>, by: &Range<i32>, mut produce: impl FnMut(Range<i32>)) {
        if main.start < by.start {
            produce(main.start..by.start);
            main.start = by.start;
        }
        if main.end > by.end {
            produce(by.end..main.end);
            main.end = by.end;
        }
    }

    // Store the non-overlapping active cuboids
    let mut current: Vec<Cuboid> = Vec::with_capacity(4800);
    let mut to_add: Vec<Cuboid> = Vec::with_capacity(100);
    instructions.into_iter().for_each(|Boot { active, zone }| {
        // For each new cuboid, split the existing cubes to not overlap with it
        current.retain(|cuboid| {
            if cuboid.is_disjoint(&zone) {
                true
            } else {
                // Split each axis, each split occurring on the remaining space
                let mut x = cuboid.0[0].clone();
                let mut y = cuboid.0[1].clone();
                let mut z = cuboid.0[2].clone();
                split_axis(&mut x, &zone.0[0], |x| {
                    to_add.push(Cuboid([x, y.clone(), z.clone()]));
                });
                split_axis(&mut y, &zone.0[1], |y| {
                    to_add.push(Cuboid([x.clone(), y, z.clone()]));
                });
                split_axis(&mut z, &zone.0[2], |z| {
                    to_add.push(Cuboid([x.clone(), y.clone(), z]));
                });
                false
            }
        });

        // Then if the new cuboid is active, add it to the queue
        if active {
            current.push(zone);
        }

        current.append(&mut to_add);
    });

    // As no cuboid is overlapping, computing the number of active point is just a sum
    current.into_iter().map(|c| c.size()).sum()
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

    /// True if the two cuboids don't overlap
    fn is_disjoint(&self, other: &Self) -> bool {
        fn axis(first: &Range<i32>, second: &Range<i32>) -> bool {
            first.end <= second.start || second.end <= first.start
        }

        axis(&self.0[0], &other.0[0])
            || axis(&self.0[1], &other.0[1])
            || axis(&self.0[2], &other.0[2])
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
