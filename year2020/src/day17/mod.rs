use itertools::iproduct;
use std::collections::{HashMap, HashSet};

use commons::eyre::Result;

pub const TITLE: &str = "Day 17: Conway Cubes";

pub fn run(raw: String) -> Result<()> {
    let input = parse(&raw);
    let first = first_part(input.clone());
    println!("After 6 cycles in 3D the number of active cubes is {first}");

    let second = second_part(input);
    println!("After 6 cycles in 4D the number of active cubes is {second}");

    Ok(())
}

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

fn parse(s: &str) -> ConwayCubes {
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

    ConwayCubes { cubes }
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
#[derive(Clone)]
struct ConwayCubes {
    cubes: HashSet<Point>,
}

impl ConwayCubes {
    /// The number of active cubes
    fn all_active(&self) -> usize {
        self.cubes.len()
    }

    /// Compute the nth cycle of the cubes
    ///
    /// ### Arguments
    /// * `cycles` - The number of cycles to run this
    /// * `directions` - The directions at which to find neighbours of a Point
    fn nth_cycle(&mut self, cycles: usize, directions: &[Point]) {
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
}

#[cfg(test)]
mod tests;
