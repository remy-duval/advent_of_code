use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;

use crate::Problem;

/// Number of cycles to run the conway cube for
const CYCLES: usize = 6;

/// The type of a value in a Point (one byte is enough, the values will remain between -10 and 10)
type Dimension = i8;
/// (x, y, z, w) point in 4D space
type Point = (Dimension, Dimension, Dimension, Dimension);

pub struct Day;

impl Problem for Day {
    type Input = ConwayCubes;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 17: Conway Cubes";

    fn solve(input: Self::Input) -> Result<(), Self::Err> {
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
    let neighbours: Vec<Point> = (-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .filter_map(|((x, y), z)| {
            if x == 0 && y == 0 && z == 0 {
                None
            } else {
                Some((x, y, z, 0))
            }
        })
        .collect();

    let mut secondary = main.clone();
    for _ in 0..CYCLES {
        main.compute_next_cycle(&mut secondary, &neighbours);
        std::mem::swap(&mut main, &mut secondary);
    }

    main.all_active()
}

fn second_part(mut main: ConwayCubes) -> usize {
    let neighbours: Vec<Point> = (-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .filter_map(|(((x, y), z), w)| {
            if x == 0 && y == 0 && z == 0 && w == 0 {
                None
            } else {
                Some((x, y, z, w))
            }
        })
        .collect();

    let mut secondary = main.clone();
    for _ in 0..CYCLES {
        main.compute_next_cycle(&mut secondary, &neighbours);
        std::mem::swap(&mut main, &mut secondary);
    }

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

    /// Fill the next active state of the conway cubes
    ///
    /// ### Arguments
    /// * `into` - The cube into which the next cycle will be written
    /// * `directions` - The directions at which to find neighbours of a Point
    pub fn compute_next_cycle(&self, into: &mut ConwayCubes, directions: &[Point]) {
        let get_neighbours = |point: Point| {
            directions
                .iter()
                .map(move |&(x, y, z, w)| (x + point.0, y + point.1, z + point.2, w + point.3))
        };

        into.cubes.clear();
        self.cubes
            .iter()
            .flat_map(|&point| std::iter::once(point).chain(get_neighbours(point)))
            .for_each(|point| {
                if !into.cubes.contains(&point) {
                    let count = get_neighbours(point)
                        .filter(|other| self.cubes.contains(other))
                        .take(4) // We need to known only if count is 2 or 3
                        .count();

                    if count == 3 || (count == 2 && self.cubes.contains(&point)) {
                        into.cubes.insert(point);
                    }
                }
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

impl std::fmt::Display for ConwayCubes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.occupied_space();
        let mut line = String::with_capacity((max.0 - min.0) as usize);
        for w in min.3..(max.3 + 1) {
            for z in min.2..(max.2 + 1) {
                writeln!(f, "z = {}, w = {}", z, w)?;
                for y in min.1..(max.1 + 1) {
                    for x in min.0..(max.0 + 1) {
                        line.push(if self.cubes.contains(&(x, y, z, w)) {
                            '#'
                        } else {
                            '.'
                        });
                    }
                    writeln!(f, "{}", line)?;
                    line.clear();
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl FromStr for ConwayCubes {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cubes: s
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '#')
                        .map(move |(x, _)| (x as Dimension, y as Dimension, 0, 0))
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("test_resources/17-A.txt");
    const MAIN: &str = include_str!("test_resources/17-B.txt");

    #[test]
    fn first_part_example() {
        let input = Day::parse(EXAMPLE).unwrap();
        let result = first_part(input);
        assert_eq!(result, 112);
    }

    #[test]
    fn first_part_main() {
        let input = Day::parse(MAIN).unwrap();
        let result = first_part(input);
        assert_eq!(result, 301);
    }

    #[test]
    fn second_part_example() {
        let input = Day::parse(EXAMPLE).unwrap();
        let result = second_part(input);
        assert_eq!(result, 848);
    }

    #[test]
    fn second_part_main() {
        let input = Day::parse(MAIN).unwrap();
        let result = second_part(input);
        assert_eq!(result, 2424);
    }
}
