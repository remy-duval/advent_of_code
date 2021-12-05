use std::str::FromStr;

use commons::eyre::{eyre, Report, Result, WrapErr};
use itertools::Itertools;

use commons::parse::LineSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<Point4>;
    const TITLE: &'static str = "Day 25: Four-Dimensional Adventure";

    fn solve(data: Self::Input) -> Result<()> {
        let count = count_constellations(&data.data);
        println!("The number of constellations is {}", count);
        Ok(())
    }
}

/// Count the number of constellations formed by the given points
/// A point belongs to a constellation if there is any point in it that is <= 3 distance away
fn count_constellations(points: &[Point4]) -> usize {
    let mut groups: Vec<Vec<&Point4>> = vec![];
    points.iter().for_each(|point| {
        // Find all groups that can connect to this point and fuse them in one
        let mut main: Option<&mut Vec<&Point4>> = None; // The main group connected to the point
        let mut connected: usize = 0; // Number of groups connected to the point
        groups.iter_mut().for_each(|group| {
            if group.iter().any(|p| p.distance(point) <= 3) {
                connected += 1;
                match main {
                    Some(ref mut cons) => {
                        cons.append(group);
                    }
                    None => {
                        group.push(point);
                        main = Some(group);
                    }
                };
            }
        });

        if connected == 0 {
            // No groups connected to the point, create a new group for it
            groups.push(vec![point]);
        } else if connected > 1 {
            // Since there were more than 1 group connected to the point, remove the empty remains
            groups.retain(|cons| !cons.is_empty());
        }
    });

    groups.len()
}

/// A four dimensional point with small dimensions
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point4([i16; 4]);

impl Point4 {
    /// The manhattan distance between `self` and `to`
    pub fn distance(&self, to: &Self) -> i16 {
        let x = (self.0[0] - to.0[0]).abs();
        let y = (self.0[1] - to.0[1]).abs();
        let z = (self.0[2] - to.0[2]).abs();
        let t = (self.0[3] - to.0[3]).abs();
        x + y + z + t
    }
}

impl FromStr for Point4 {
    type Err = Report;

    fn from_str(string: &str) -> Result<Self> {
        itertools::process_results(
            string.split(',').map(|coord| {
                coord
                    .trim()
                    .parse()
                    .wrap_err_with(|| format!("Can't parse an integer '{}'", coord))
            }),
            |iter| match iter.collect_tuple::<(_, _, _, _)>() {
                Some((a, b, c, d)) => Ok(Point4([a, b, c, d])),
                None => Err(eyre!("Bad format for a four dimensional point: {}", string)),
            },
        )?
    }
}

#[cfg(test)]
mod tests;
