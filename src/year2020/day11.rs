use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::str::FromStr;

use itertools::Itertools;

use crate::commons::grid2::Grid;
use crate::Problem;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub struct Day;

impl Problem for Day {
    type Input = Ferry;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 11: Seating System";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let first = first_part(data.clone());

        println!(
            "When seeing only adjacent seats the number of occupied seats at the end is {}",
            first.occupied_seats()
        );

        let second = second_part(data);
        println!(
            "When seeing further seats the number of occupied seats at the end is {}",
            second.occupied_seats()
        );

        Ok(())
    }
}

/// Compute the floor next state until the changes count drops to 0
/// Only directly adjacent seats are taken into account
fn first_part(mut floor: Ferry) -> Ferry {
    fn full_seat(grid: &Grid<Tile>, point: (isize, isize), direction: (isize, isize)) -> bool {
        grid.get((point.0 + direction.0, point.1 + direction.1))
            .map_or(false, |t| t.is_occupied())
    }

    let mut swap = floor.0.clone();
    let mut changes = 1;
    while changes != 0 {
        changes = floor.compute_next_state(4, &mut swap, full_seat);
        std::mem::swap(&mut floor.0, &mut swap);
    }

    floor
}

/// Compute the floor next state until the changes count drops to 0
/// The first seat in each direction is taken into account
fn second_part(mut floor: Ferry) -> Ferry {
    fn full_seat(grid: &Grid<Tile>, point: (isize, isize), direction: (isize, isize)) -> bool {
        grid.half_line(point, direction)
            .skip(1)
            .find_map(|(_, t)| {
                if t.is_nothing() {
                    None
                } else {
                    Some(t.is_occupied())
                }
            })
            .unwrap_or_default()
    }

    let mut swap = floor.0.clone();
    let mut changes = 1;
    while changes != 0 {
        changes = floor.compute_next_state(5, &mut swap, full_seat);
        std::mem::swap(&mut floor.0, &mut swap)
    }

    floor
}

/// A tile in the ferry
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    FullSeat,
    EmptySeat,
    Nothing,
}

impl Tile {
    pub fn is_occupied(self) -> bool {
        matches!(self, Tile::FullSeat)
    }

    pub fn is_nothing(self) -> bool {
        matches!(self, Tile::Nothing)
    }
}

/// The floor plan of the ferry
#[derive(Debug, Clone)]
pub struct Ferry(Grid<Tile>);

impl Ferry {
    /// The number of occupied seats in the ferry
    pub fn occupied_seats(&self) -> usize {
        self.0.iter().filter(|t| t.is_occupied()).count()
    }

    /// Compute the next state of the ferry
    ///
    /// ### Arguments
    /// * `max_around` - The maximum number of adjacent points around before emptying a seat
    /// * `destination` - The grid where the new seat states should be written
    /// * `adjacent_full_seat` - (Grid, Current point, Direction) -> True if full seat in sight
    ///
    /// ### Returns
    /// The number of modified tiles in the update process (0 if current == previous)
    /// Also modifies the `destination` in place
    pub fn compute_next_state(
        &self,
        max_around: usize,
        destination: &mut Grid<Tile>,
        adjacent_full_seat: impl Fn(&Grid<Tile>, (isize, isize), (isize, isize)) -> bool,
    ) -> usize {
        let mut changes = 0;
        for (point, tile) in self.0.indexed_values() {
            if !tile.is_nothing() {
                let adjacent = DIRECTIONS
                    .iter()
                    .filter(|dir| adjacent_full_seat(&self.0, point, **dir))
                    .count();

                destination[point] = match tile {
                    Tile::FullSeat if adjacent >= max_around => {
                        changes += 1;
                        Tile::EmptySeat
                    }
                    Tile::EmptySeat if adjacent == 0 => {
                        changes += 1;
                        Tile::FullSeat
                    }
                    other => *other,
                };
            }
        }

        changes
    }
}

impl FromStr for Ferry {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let mut lines = s.lines().map(|line| {
            line.trim()
                .chars()
                .map(|char| match char {
                    'L' => Tile::EmptySeat,
                    '#' => Tile::FullSeat,
                    _ => Tile::Nothing,
                })
                .collect_vec()
        });

        if let Some(first) = lines.next() {
            let width = first.len();
            let mut grid = Grid::new(width, height);
            grid.push_line(first);
            for line in lines {
                grid.push_line(line);
            }

            Ok(Ferry(grid))
        } else {
            Ok(Ferry(Grid::new(0, 0)))
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let char = match self {
            Tile::FullSeat => '#',
            Tile::EmptySeat => 'L',
            Tile::Nothing => '.',
        };

        f.write_char(char)
    }
}

impl Display for Ferry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = include_str!("test_resources/11-A.txt");
    const B: &str = include_str!("test_resources/11-B.txt");

    #[test]
    fn first_part_test_a() {
        let floor = first_part(Day::parse(A).unwrap());
        assert_eq!(37, floor.occupied_seats());
    }

    #[test]
    fn first_part_test_b() {
        let floor = first_part(Day::parse(B).unwrap());
        assert_eq!(2263, floor.occupied_seats());
    }

    #[test]
    fn second_part_test_a() {
        let floor = second_part(Day::parse(A).unwrap());
        assert_eq!(26, floor.occupied_seats());
    }

    #[test]
    fn second_part_test_b() {
        let floor = second_part(Day::parse(B).unwrap());
        assert_eq!(2002, floor.occupied_seats());
    }
}
