use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::str::FromStr;

use hashbrown::HashMap;
use itertools::{process_results, Itertools};

/// The type of a point in the cavern
pub type Point = (u32, u32);

/// Compute the distance between two points
pub fn distance((a_x, a_y): Point, (b_x, b_y): Point) -> u32 {
    let x = if a_x > b_x { a_x - b_x } else { b_x - a_x };
    let y = if a_y > b_y { a_y - b_y } else { b_y - a_y };
    x + y
}

/// A tool that is used in the shortest path
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Tool {
    ClimbingGear,
    Torch,
    Neither,
}

impl Tool {
    /// Check if a tool can be used in a given tile
    pub fn can_be_used(self, at: Tile) -> bool {
        match at {
            Tile::Rocky => !matches!(self, Self::Neither),
            Tile::Wet => !matches!(self, Self::Torch),
            Tile::Narrow => !matches!(self, Self::ClimbingGear),
        }
    }

    /// Switch to the other tool that is usable in this current tile
    pub fn switch_tool(self, current: Tile) -> Self {
        match current {
            Tile::Rocky => match self {
                Self::ClimbingGear => Self::Torch,
                _ => Self::ClimbingGear,
            },
            Tile::Wet => match self {
                Self::Neither => Self::ClimbingGear,
                _ => Self::Neither,
            },
            Tile::Narrow => match self {
                Self::Neither => Self::Torch,
                _ => Self::Neither,
            },
        }
    }
}

/// The type of a tile in the cavern
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Tile {
    Rocky,
    Wet,
    Narrow,
}

impl Tile {
    /// The tile based on erosion
    fn new(erosion: u64) -> Self {
        match erosion % 3 {
            0 => Self::Rocky,
            1 => Self::Wet,
            _ => Self::Narrow,
        }
    }

    /// The danger/risk level of this tile
    fn danger_level(self) -> u64 {
        match self {
            Tile::Rocky => 0,
            Tile::Wet => 1,
            Tile::Narrow => 2,
        }
    }

    /// The char representation of this tile
    fn to_char(self) -> char {
        match self {
            Tile::Rocky => '.',
            Tile::Wet => '=',
            Tile::Narrow => '|',
        }
    }
}

/// The cavern to traverse
#[derive(Debug, Clone)]
pub struct Cavern {
    pub target: Point,
    depth: u64,
    grid: HashMap<Point, u64>,
}

impl Cavern {
    /// Compute the risk level of the area between (0, 0) and the target
    pub fn risk_level(&mut self) -> u64 {
        (0..(self.target.1 + 1))
            .map(|y| {
                (0..(self.target.0 + 1))
                    .map(|x| self.get_or_insert((x, y)).danger_level())
                    .sum::<u64>()
            })
            .sum()
    }

    /// Get the tile at the given point or insert it
    pub fn get_or_insert(&mut self, point: Point) -> Tile {
        Tile::new(self.compute_erosion(point))
    }

    /// Compute the erosion level of a point (cache the result in the grid)
    fn compute_erosion(&mut self, point: Point) -> u64 {
        if let Some(&erosion) = self.grid.get(&point) {
            erosion
        } else {
            let geologic_index = match point {
                (x, 0) => x as u64 * 16807,
                (0, y) => y as u64 * 48271,
                (x, y) => self.compute_erosion((x - 1, y)) * self.compute_erosion((x, y - 1)),
            };
            let erosion = (geologic_index + self.depth as u64) % 20_183;
            self.grid.insert(point, erosion);
            erosion
        }
    }
}

/// An error while parsing the rules
#[derive(Debug, thiserror::Error)]
pub enum RulesParseError {
    #[error("Could not parse integer {0} ({1})")]
    ParseIntError(Box<str>, std::num::ParseIntError),
    #[error("Expected 'depth: INT', got {0}")]
    BadDepth(Box<str>),
    #[error("Expected 'target: INT,INT', got {0}")]
    BadTarget(Box<str>),
    #[error("Expected two lines, depth and target, got {0}")]
    BadFormat(Box<str>),
}

impl Display for Cavern {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = self.grid.keys().fold((0, 0), |(max_x, max_y), point| {
            (max_x.max(point.0), max_y.max(point.1))
        });

        (0..(max.0 + 1)).try_for_each(|y| {
            (0..(max.1 + 1)).try_for_each(|x| {
                f.write_char(
                    self.grid
                        .get(&(x, y))
                        .map_or('?', |&erosion| Tile::new(erosion).to_char()),
                )
            })?;
            f.write_char('\n')
        })
    }
}

impl FromStr for Cavern {
    type Err = RulesParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (depth, target) = s
            .lines()
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| RulesParseError::BadFormat(s.into()))?;

        let depth = depth
            .strip_prefix("depth:")
            .ok_or_else(|| RulesParseError::BadDepth(depth.into()))
            .and_then(|str| {
                str.trim()
                    .parse()
                    .map_err(|err| RulesParseError::ParseIntError(str.into(), err))
            })?;

        let target = target
            .strip_prefix("target:")
            .and_then(|str| {
                process_results(
                    str.split(',').map(|str| {
                        str.trim()
                            .parse()
                            .map_err(|err| RulesParseError::ParseIntError(str.into(), err))
                    }),
                    |iter| iter.collect_tuple::<(_, _)>(),
                )
                .transpose()
            })
            .unwrap_or_else(|| Err(RulesParseError::BadTarget(target.into())))?;

        let dimensions = (target.0 as usize + 1) * (target.1 as usize + 1);
        let mut grid = HashMap::with_capacity(dimensions);
        grid.insert((0, 0), depth);
        grid.insert(target, depth);
        Ok(Self {
            target,
            depth,
            grid,
        })
    }
}
