use itertools::Itertools;

use commons::grid::Grid;
use commons::Result;

pub const TITLE: &str = "Day 11: Seating System";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw);
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

fn parse(s: &str) -> Ferry {
    let mut lines = s.lines().map(|line| {
        line.trim().chars().map(|char| match char {
            'L' => Tile::EmptySeat,
            '#' => Tile::FullSeat,
            _ => Tile::Nothing,
        })
    });

    if let Some(first) = lines.next() {
        let first = first.collect_vec();
        let width = first.len();
        let mut grid = Grid::from_vec(width, first);
        lines.for_each(|mut line| {
            grid.insert_filled_line(|_| line.next().unwrap_or(Tile::Nothing));
        });

        Ferry(grid)
    } else {
        Ferry(Grid::new(0, 0))
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

/// The floor plan of the ferry
#[derive(Clone)]
struct Ferry(Grid<Tile>);

/// A tile in the ferry
#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    FullSeat,
    EmptySeat,
    Nothing,
}

impl Tile {
    fn is_occupied(self) -> bool {
        matches!(self, Tile::FullSeat)
    }

    fn is_nothing(self) -> bool {
        matches!(self, Tile::Nothing)
    }
}

impl Ferry {
    /// The number of occupied seats in the ferry
    fn occupied_seats(&self) -> usize {
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
    fn compute_next_state(
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

#[cfg(test)]
mod tests;
