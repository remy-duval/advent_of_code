use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use crate::commons::grid::Point;
use crate::Problem;

const DIRECTIONS: [(i64, i64); 8] = [
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
            "With low visibility:\n{}\nThe number of occupied seats at the end is {}",
            first,
            first.occupied_seats()
        );

        let second = second_part(data);
        println!(
            "With high visibility:\n{}\nThe number of occupied seats at the end is {}",
            second,
            second.occupied_seats()
        );

        Ok(())
    }
}

/// Compute the floor next state until the changes count drops to 0
fn first_part(mut floor: Ferry) -> Ferry {
    let mut swap = floor.seats.clone();
    let mut changes = 1;
    while changes != 0 {
        changes = floor.compute_next_state(false, 4, &mut swap);
        std::mem::swap(&mut floor.seats, &mut swap)
    }

    floor
}

/// Compute the floor next state until the changes count drops to 0
fn second_part(mut floor: Ferry) -> Ferry {
    let mut swap = floor.seats.clone();
    let mut changes = 1;
    while changes != 0 {
        changes = floor.compute_next_state(true, 5, &mut swap);
        std::mem::swap(&mut floor.seats, &mut swap)
    }

    floor
}

/// The floor plan of the ferry
#[derive(Debug, Clone)]
pub struct Ferry {
    pub seats: HashMap<Point, bool>,
    adjacent_low: HashMap<Point, Box<[Point]>>,
    adjacent_high: HashMap<Point, Box<[Point]>>,
    max_x: i64,
    max_y: i64,
}

impl Ferry {
    /// Create a new Ferry based on the given seats
    /// In particular compute the adjacent seats for both low and high visibility
    pub fn new(seats: HashMap<Point, bool>) -> Self {
        let max_y: i64 = seats.keys().map(|p| p.y).max().unwrap_or_default();
        let max_x: i64 = seats.keys().map(|p| p.x).max().unwrap_or_default();
        let mut adjacent_low = HashMap::with_capacity(seats.len());
        let mut adjacent_high = HashMap::with_capacity(seats.len());
        for seat in seats.keys() {
            let mut low = Vec::new();
            let mut high = Vec::new();
            for direction in DIRECTIONS.iter() {
                let mut point = Point::new(seat.x + direction.0, seat.y + direction.1);
                if seats.contains_key(&point) {
                    low.push(point.clone());
                }
                while point.x >= 0 && point.x <= max_x && point.y >= 0 && point.y <= max_y {
                    if seats.contains_key(&point) {
                        high.push(point);
                        break;
                    }
                    point.x += direction.0;
                    point.y += direction.1;
                }
            }
            adjacent_low.insert(*seat, low.into_boxed_slice());
            adjacent_high.insert(*seat, high.into_boxed_slice());
        }

        Ferry {
            seats,
            adjacent_low,
            adjacent_high,
            max_x,
            max_y,
        }
    }

    /// The number of occupied seats in the ferry
    pub fn occupied_seats(&self) -> usize {
        self.seats.iter().filter(|kv| *kv.1).count()
    }

    /// Compute the next state of the ferry
    pub fn compute_next_state(
        &self,
        high_visibility: bool,
        max_around: usize,
        destination: &mut HashMap<Point, bool>,
    ) -> usize {
        let mut changes = 0;
        for (point, current) in &self.seats {
            if let Some(before) = destination.get_mut(point) {
                let adj = if high_visibility {
                    self.adjacent_high.get(point)
                } else {
                    self.adjacent_low.get(point)
                };

                let around = adj.map_or(0, |adjacent| {
                    adjacent
                        .iter()
                        .filter(|point| self.seats.get(*point).copied().unwrap_or_default())
                        .count()
                });

                *before = if *current {
                    around < max_around
                } else {
                    around == 0
                };

                if *before != *current {
                    changes += 1
                }
            }
        }

        changes
    }
}

impl FromStr for Ferry {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: HashMap<Point, bool> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, char)| {
                    let tile = match char {
                        'L' => Some(false),
                        '#' => Some(true),
                        _ => None,
                    }?;

                    Some((Point::new(x as i64, y as i64), tile))
                })
            })
            .collect();

        Ok(Self::new(data))
    }
}

impl Display for Ferry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let length = self.max_x + 1;
        let height = self.max_y + 1;
        write!(f, "{} * {}\n", length, height)?;
        let mut string = String::with_capacity((length + 1) as usize * height as usize);
        for y in 0..height {
            for x in 0..length {
                if let Some(occupied) = self.seats.get(&Point::new(x, y)) {
                    string.push(if *occupied { '#' } else { 'L' });
                } else {
                    string.push('.');
                }
            }
            string.push('\n');
        }

        f.write_str(&string)
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
