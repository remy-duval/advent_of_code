use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use color_eyre::eyre::Result;
use hashbrown::{HashMap, HashSet};
use itertools::iproduct;

use commons::Problem;

/// Number of cycles to run the conway cube for
const CYCLES: usize = 6;

/// The number of points that are expected to be active to pre-size the set
const ACTIVE_POINTS: usize = 4 * 1024;

/// The number of points that are expected to appear in the cache to pre-size it
const INACTIVE_POINTS: usize = 8 * 1024;

/// The type of a value in a Point (one byte is enough, the values will remain between -10 and 10)
type Dimension = i8;
/// (x, y, z, w) point in 4D space
type Point = (Dimension, Dimension, Dimension, Dimension);

pub struct Day;

impl Problem for Day {
    type Input = ConwayCubes;
    const TITLE: &'static str = "Day 17: Conway Cubes";

    fn solve(input: Self::Input) -> Result<()> {
        let first = first_part(input.clone());
        println!(
            "After 6 cycles in 3D the number of active cubes is {}",
            first
        );

        let second = second_part(input);
        println!(
            "After 6 cycles in 4D the number of active cubes is {}",
            second
        );

        Ok(())
    }
}

fn first_part(mut main: ConwayCubes) -> usize {
    let neighbours: Vec<Point> = iproduct!(-1..2, -1..2, -1..2)
        .filter_map(|(x, y, z)| {
            if x == 0 && y == 0 && z == 0 {
                None
            } else {
                Some((x, y, z, 0))
            }
        })
        .collect();

    main.nth_cycle(CYCLES, &neighbours);
    main.all_active()
}

fn second_part(mut main: ConwayCubes) -> usize {
    let neighbours: Vec<Point> = iproduct!(-1..2, -1..2, -1..2, -1..2)
        .filter_map(|(x, y, z, w)| {
            if x == 0 && y == 0 && z == 0 && w == 0 {
                None
            } else {
                Some((x, y, z, w))
            }
        })
        .collect();

    main.nth_cycle(CYCLES, &neighbours);
    main.all_active()
}

/// The state of the conway cubes
#[derive(Debug, Clone)]
pub struct ConwayCubes {
    cubes: HashSet<Point>,
}

impl ConwayCubes {
    /// The number of active cubes
    pub fn all_active(&self) -> usize {
        self.cubes.len()
    }

    /// Compute the nth cycle of the cubes
    ///
    /// ### Arguments
    /// * `cycles` - The number of cycles to run this
    /// * `directions` - The directions at which to find neighbours of a Point
    pub fn nth_cycle(&mut self, cycles: usize, directions: &[Point]) {
        let get_neighbours = |(p_x, p_y, p_z, p_w): Point| {
            directions
                .iter()
                .map(move |&(x, y, z, w)| (x + p_x, y + p_y, z + p_z, w + p_w))
        };

        let mut cache = HashMap::with_capacity(INACTIVE_POINTS);
        let mut swap = HashSet::with_capacity(ACTIVE_POINTS);
        (0..cycles).for_each(|_| {
            swap.clear(); // The n - 1 state can be discarded now that we are computing n + 1
            self.cubes.iter().for_each(|&point| {
                let mut count = 0;
                // For each neighbour, if it is active add + 1 to count
                // Else memoize the neighbour own neighbour's count for later
                get_neighbours(point).for_each(|neighbour| {
                    if self.cubes.contains(&neighbour) {
                        count += 1;
                    } else {
                        cache.entry(neighbour).or_insert_with(|| {
                            get_neighbours(neighbour)
                                .filter(|other| self.cubes.contains(other))
                                .take(4) // We need to know only if count is 3
                                .count()
                        });
                    }
                });

                // Keep the point at n + 1 if it satisfies the active neighbour count
                if count == 2 || count == 3 {
                    swap.insert(point);
                }
            });

            // For each inactive neighbour that was memoized,
            // If it has exactly 3 neighbours we can add it to the active set
            cache.drain().for_each(|(point, count)| {
                if count == 3 {
                    swap.insert(point);
                }
            });

            // Update the original cubes now that we have fully computed their next state
            std::mem::swap(&mut self.cubes, &mut swap);
        });
    }

    /// The space occupied by the active cubes
    pub fn occupied_space(&self) -> (Point, Point) {
        self.cubes
            .iter()
            .copied()
            .fold(((0, 0, 0, 0), (0, 0, 0, 0)), |(min, max), (x, y, z, w)| {
                let min = (min.0.min(x), min.1.min(y), min.2.min(z), min.3.min(w));
                let max = (max.0.max(x), max.1.max(y), max.2.max(z), max.3.max(w));
                (min, max)
            })
    }
}

impl Display for ConwayCubes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let (min, max) = self.occupied_space();
        let mut line = String::with_capacity((max.0 - min.0) as usize);
        (min.3..(max.3 + 1)).try_for_each(|w| {
            (min.2..(max.2 + 1)).try_for_each(|z| {
                writeln!(f, "z = {}, w = {}", z, w)?;
                (min.1..(max.1 + 1)).try_for_each(|y| {
                    line.clear();
                    (min.0..(max.0 + 1)).for_each(|x| {
                        line.push(if self.cubes.contains(&(x, y, z, w)) {
                            '#'
                        } else {
                            '.'
                        });
                    });

                    writeln!(f, "{}", line)
                })?;
                writeln!(f)
            })
        })
    }
}

impl FromStr for ConwayCubes {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = HashSet::with_capacity(ACTIVE_POINTS);
        s.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| (x as Dimension, y as Dimension, 0, 0))
            })
            .for_each(|point| {
                cubes.insert(point);
            });

        Ok(Self { cubes })
    }
}

#[cfg(test)]
mod tests;
