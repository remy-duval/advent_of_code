use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::str::FromStr;

use commons::eyre::{bail, eyre, Report, Result};

use commons::grid::Grid;
use commons::num::integer::Integer;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Area;
    const TITLE: &'static str = "Day 18: Settlers of The North Pole";

    fn solve(area: Self::Input) -> Result<()> {
        let (trees, lumberyard) = first_part(area.clone()).trees_and_lumberyards();
        println!(
            "After 10 minutes: {} trees and {} lumberyards: {} resources",
            trees,
            lumberyard,
            trees * lumberyard
        );

        let second =
            second_part(area).ok_or_else(|| eyre!("Could not find the period of the system"))?;
        println!("After one billion minutes: {} resources", second);

        Ok(())
    }
}

/// Compute the first ten minutes
fn first_part(mut area: Area) -> Area {
    let mut swap = area.clone();
    (0..10).for_each(|_| {
        area.compute_next(&mut swap);
        std::mem::swap(&mut area, &mut swap);
    });

    area
}

/// Compute a billion minutes by finding a period in the system
fn second_part(mut area: Area) -> Option<isize> {
    const BILLION: usize = 1_000_000_000;
    const STABILIZE: usize = 500;
    const MAX_PERIOD: usize = 100;

    let mut swap = area.clone();
    // Skip the first values (not stabilized)
    (0..STABILIZE).for_each(|_| {
        area.compute_next(&mut swap);
        std::mem::swap(&mut area, &mut swap);
    });

    let initial_resources = area.resources() as isize;

    // Remember the first differences to compute the period
    let resources = (0..MAX_PERIOD)
        .scan(initial_resources, |state, _| {
            area.compute_next(&mut swap);
            std::mem::swap(&mut area, &mut swap);
            let resources = area.resources() as isize;
            let diff = resources - *state;
            *state = resources;
            Some(diff)
        })
        .collect::<Vec<isize>>();

    let (period, from) = find_period(&resources)?;
    let rest = BILLION - from - STABILIZE;
    let result: isize = {
        let (periods, remainder) = rest.div_mod_floor(&period.len());
        initial_resources
            + resources.iter().take(from).sum::<isize>() // The part before the period starts
            + period.iter().sum::<isize>() * periods as isize // The total of the following periods
            + period.iter().take(remainder).sum::<isize>() // The remainder of the last period
    };

    Some(result)
}

/// Find the first period and its starting index
fn find_period(diffs: &[isize]) -> Option<(Vec<isize>, usize)> {
    (0..diffs.len()).find_map(|start| {
        let slice = &diffs[start..];
        (2..(slice.len() / 2)).find_map(|length| {
            let first = &slice[..length];
            if first == &slice[length..(length * 2)] {
                Some((first.to_vec(), start))
            } else {
                None
            }
        })
    })
}

/// The representation of an area
#[derive(Debug, Clone)]
pub struct Area(Grid<Tile>);

impl Area {
    /// Compute the total of the resources in the area
    fn resources(&self) -> usize {
        let (tree, lumberyard) = self.trees_and_lumberyards();
        tree * lumberyard
    }

    /// Count the amount of trees and lumberyards in the area
    fn trees_and_lumberyards(&self) -> (usize, usize) {
        self.0
            .iter()
            .fold((0, 0), |(trees, lumberyard), next| match next {
                Tile::Open => (trees, lumberyard),
                Tile::Trees => (trees + 1, lumberyard),
                Tile::Lumberyard => (trees, lumberyard + 1),
            })
    }

    /// Compute the next state of an area into the given buffer
    fn compute_next(&self, into: &mut Self) {
        assert_eq!(into.0.width(), self.0.width());
        assert_eq!(into.0.height(), self.0.height());
        self.0.indexed_values().for_each(|(point, tile)| {
            let (trees, lumberyards) = adjacent_points(point).iter().fold(
                (0, 0),
                |(trees, lumberyards), point| match self.0.get(*point) {
                    Some(Tile::Trees) => (trees + 1, lumberyards),
                    Some(Tile::Lumberyard) => (trees, lumberyards + 1),
                    _ => (trees, lumberyards),
                },
            );

            into.0[point] = match tile {
                Tile::Open => {
                    if trees >= 3 {
                        Tile::Trees
                    } else {
                        Tile::Open
                    }
                }
                Tile::Trees => {
                    if lumberyards >= 3 {
                        Tile::Lumberyard
                    } else {
                        Tile::Trees
                    }
                }
                Tile::Lumberyard => {
                    if trees >= 1 && lumberyards >= 1 {
                        Tile::Lumberyard
                    } else {
                        Tile::Open
                    }
                }
            };
        });
    }
}

impl FromStr for Area {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (width, height) = s.lines().try_fold((0, 0), |(width, height), next| {
            let next_width = next.chars().count();
            if width == 0 {
                Ok((next_width, height + 1))
            } else if next_width == width {
                Ok((width, height + 1))
            } else {
                bail!(
                    "Line n°{line} is of width {actual} instead of expected {expected}",
                    line = height + 1,
                    expected = width,
                    actual = next_width,
                );
            }
        })?;

        let mut grid: Grid<Tile> = Grid::with_default(width, height);
        s.lines().enumerate().try_for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .try_for_each(|(x, c)| match Tile::from_char(c) {
                    None => Err(eyre!("Tile {} in line n°{} is not '.', '|' or '#'", c, y)),
                    Some(tile) => {
                        grid[(x as isize, y as isize)] = tile;
                        Ok(())
                    }
                })
        })?;

        Ok(Self(grid))
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.lines().try_for_each(|line| {
            line.iter().try_for_each(|tile| f.write_char(tile.char()))?;
            f.write_char('\n')
        })
    }
}

/// An array of the points that are adjacent to one
fn adjacent_points((x, y): (isize, isize)) -> [(isize, isize); 8] {
    [
        (x + -1, y + -1),
        (x + -1, y),
        (x + -1, y + 1),
        (x, y + -1),
        (x, y + 1),
        (x + 1, y + -1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

/// A tile in the lumber collection area
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open,
    Trees,
    Lumberyard,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Open
    }
}

impl Tile {
    /// Parse a character into a Tile
    const fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Open),
            '|' => Some(Self::Trees),
            '#' => Some(Self::Lumberyard),
            _ => None,
        }
    }

    /// The char representation of this Tile
    const fn char(self) -> char {
        match self {
            Tile::Open => '.',
            Tile::Trees => '|',
            Tile::Lumberyard => '#',
        }
    }
}

#[cfg(test)]
mod tests;
