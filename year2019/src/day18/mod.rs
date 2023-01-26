use std::ops::Add;

use commons::eyre::Result;
use commons::grid::Point;

mod parsers;
mod shortest_path;

pub const TITLE: &str = "Day 18: Many-Worlds Interpretation";

pub fn run(raw: String) -> Result<()> {
    // First part
    let (start, keys, map) = parsers::parse_and_optimize_map(&raw);
    println!("Map size : {}", map.len());
    let shortest = shortest_path::find_shortest_path(&map, start, keys);
    println!("Shortest path is {shortest} steps long");

    // Second part
    let split = parsers::split_maze_in_four(&raw, start, true);
    let shortest: usize = split
        .iter()
        .map(|data| {
            let (start, keys, map) = parsers::parse_and_optimize_map(data);
            // We need to subtract because for we add the start point on the middle line
            // Which makes the path longer by 1 step for each robot
            shortest_path::find_shortest_path(&map, start, keys) - 1
        })
        .sum();

    println!("Shortest path is {shortest} steps long");

    Ok(())
}

/// A hallway in the maze
#[derive(Debug, Clone, Default)]
pub struct HallWay {
    char: char,
    required: Keys,
    contains: Keys,
    connections: Vec<(Point, usize)>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct Keys(u32);

impl Keys {
    const FULL: u32 = (1 << 26) - 1;

    /// Gets if all keys in the right hand side are present in this
    fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Combine the keys of this and the keys of the argument
    fn combine(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// True if this has no keys
    fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// True if this contains all the keys from 0 to 25
    fn is_full(self) -> bool {
        self.0 == Self::FULL
    }
}

impl Add for Keys {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.combine(rhs)
    }
}

#[cfg(test)]
mod tests;
