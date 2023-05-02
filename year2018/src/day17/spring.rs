use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::ops::RangeInclusive;
use std::str::FromStr;

use commons::{err, Report, Result, WrapErr};
use itertools::Itertools;
use std::collections::HashMap;

use commons::grid::Point;

/// The result of the ground scan (input to the problem)
#[derive(Debug, Clone)]
pub struct Scan {
    /// Location -> Tile. No binding means dry sand
    tiles: HashMap<Point, Tile>,
    /// The top left corner of the scan
    min: Point,
    /// The bottom right corner of the scan
    max: Point,
}

impl Scan {
    /// The location of the water spring in the scan
    const SPRING: Point = Point::new(500, 0);

    /// The count of tiles that are wet of filled with water
    pub fn wet_tiles(&self) -> usize {
        self.tiles
            .values()
            .filter(|t| matches!(**t, Tile::WetSand | Tile::Water))
            .count()
    }

    /// The count of tiles that are filled with water
    pub fn water(&self) -> usize {
        self.tiles
            .values()
            .filter(|t| matches!(**t, Tile::Water))
            .count()
    }

    /// Fill the ground with water falling from the spring
    pub fn fill(&mut self) {
        self.fill_from(Self::SPRING);
    }

    /// Fill the ground with water falling from the given point
    fn fill_from(&mut self, mut from: Point) {
        // Check that we did not already cover a route to avoid useless recomputations
        if from.y > self.max.y || self.tiles.contains_key(&from) {
            return;
        }

        while self.is_empty(&from) {
            if from.y >= self.min.y {
                self.tiles.insert(from, Tile::WetSand);
            }
            from.y += 1;
            if from.y > self.max.y {
                return;
            }
        }

        from.y -= 1;
        if from.y >= self.min.y {
            self.spread_water(from);
        }
    }

    /// Fill the water coming from x
    fn fill_x(&mut self, y: i64, x_range: RangeInclusive<i64>, with: Tile) {
        x_range.for_each(|x| {
            self.tiles.insert(Point::new(x, y), with);
        });
    }

    /// Spread water on x axis until it find either a wall on both side or none
    fn spread_water(&mut self, from: Point) {
        let (right, right_closed) = self.find_descent(from, 1);
        let (left, left_closed) = self.find_descent(from, -1);

        if right_closed && left_closed {
            // The water is enclosed to the left and right by walls
            self.fill_x(from.y, left..=right, Tile::Water);
            self.spread_water(Point::new(from.x, from.y - 1));
        } else if right_closed {
            // The water is blocked to the right, but can descend from the left
            self.fill_x(from.y, left..=right, Tile::WetSand);
            self.fill_from(Point::new(left, from.y + 1));
        } else if left_closed {
            // The water is blocked to the left, but can descend from the right
            self.fill_x(from.y, left..=right, Tile::WetSand);
            self.fill_from(Point::new(right, from.y + 1));
        } else {
            // The water can descend on both side
            self.fill_x(from.y, left..=right, Tile::WetSand);
            self.fill_from(Point::new(left, from.y + 1));
            if left != right {
                self.fill_from(Point::new(right, from.y + 1));
            }
        }
    }

    /// Find the next point to descend with
    ///
    /// ### Arguments
    /// * `point` - The point from which to explore the side directions
    /// * `step_x` - The direction to progress with in x direction
    ///
    /// ### Returns
    /// (the last free x coordinate, true if next is wall)
    fn find_descent(&self, mut point: Point, step_x: i64) -> (i64, bool) {
        loop {
            // Check if point below is empty -> In that case we can descend, this is not a wall
            point.y += 1;
            if self.is_empty(&point) {
                return (point.x, false);
            }

            // Check if next point is full -> In that case we can't descend, this is a wall
            point.y -= 1;
            point.x += step_x;
            if self.is_full(&point) {
                return (point.x - step_x, true);
            }
        }
    }

    /// True if the current tile is a wall or water
    fn is_full(&self, current: &Point) -> bool {
        self.tiles.get(current).map_or(false, |t| t.is_occupied())
    }

    /// True if the current tile is sand or wet sand
    fn is_empty(&self, current: &Point) -> bool {
        self.tiles.get(current).map_or(true, |t| t.is_sand())
    }
}

impl FromStr for Scan {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn split_coordinates<'a>(
            line: &'a str,
            first: &'_ str,
            second: &'_ str,
        ) -> Option<(&'a str, &'a str)> {
            let (a, b) = line
                .strip_prefix(first)?
                .splitn(2, ',')
                .collect_tuple::<(_, _)>()?;

            Some((a, b.trim().strip_prefix(second)?))
        }

        fn parse_range(str: &str) -> Result<RangeInclusive<i64>> {
            let mut numbers = str.splitn(2, "..").map(|num| {
                num.trim()
                    .parse::<i64>()
                    .wrap_err_with(|| format!("Could not parse number {num}"))
            });

            let first = numbers.next().unwrap()?; // The iterator has at least one element
            if let Some(second) = numbers.next() {
                Ok(first..=second?)
            } else {
                Ok(first..=first)
            }
        }

        let mut min = Point::new(i64::MAX, i64::MAX);
        let mut max = Point::new(i64::MIN, i64::MIN);
        let mut tiles: HashMap<Point, Tile> = HashMap::with_capacity(10_000);
        s.lines().try_for_each(|line| -> Result<()> {
            let (x, y) = {
                if line.starts_with('x') {
                    split_coordinates(line, "x=", "y=").ok_or_else(|| {
                        err!("Expected 'AXIS=RANGE, AXIS=RANGE' where AXIS = x/y, but got {line}")
                    })?
                } else {
                    let (y, x) = split_coordinates(line, "y=", "x=").ok_or_else(|| {
                        err!("Expected 'AXIS=RANGE, AXIS=RANGE' where AXIS = x/y, but got {line}")
                    })?;
                    (x, y)
                }
            };

            let x_range = parse_range(x)?;
            let y_range = parse_range(y)?;

            // Update the borders of the scan
            min.x = min.x.min(*x_range.start());
            min.y = min.y.min(*y_range.start());
            max.x = max.x.max(*x_range.end());
            max.y = max.y.max(*y_range.end());

            // Insert all the clay points
            x_range.cartesian_product(y_range).for_each(|(x, y)| {
                tiles.insert(Point::new(x, y), Tile::Clay);
            });

            Ok(())
        })?;

        // Include also the first sand points around the clay
        min.x -= 1;
        max.x += 1;
        Ok(Self { tiles, min, max })
    }
}

impl Display for Scan {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        (0..(self.max.y + 1)).try_for_each(|y| {
            (self.min.x..(self.max.x + 1)).try_for_each(|x| {
                let point = Point::new(x, y);
                let char = if point == Self::SPRING {
                    '+'
                } else if let Some(tile) = self.tiles.get(&point) {
                    tile.char()
                } else {
                    '.'
                };

                f.write_char(char)
            })?;

            f.write_char('\n')
        })
    }
}

/// A tile of the scan of the ground
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    WetSand,
    Water,
    Clay,
}

impl Tile {
    /// The char representing this tile for display
    const fn char(self) -> char {
        match self {
            Tile::WetSand => '|',
            Tile::Water => '~',
            Tile::Clay => '#',
        }
    }

    /// True if the given tile is sand
    const fn is_sand(self) -> bool {
        matches!(self, Self::WetSand)
    }

    /// True if the given tile is occupied (no water can settle there)
    const fn is_occupied(self) -> bool {
        !self.is_sand()
    }
}
