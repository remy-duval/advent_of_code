use std::cmp::Ordering;
use std::f64::consts;
use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;
use std::time::Duration;

use anyhow::Context;
use hashbrown::HashSet;
use itertools::Itertools;
use num_integer::gcd;

use commons::grid::Point;
use commons::parse::LineSep;
use commons::Problem;
use commons::{CLEAR_COMMAND, TO_TOP};

pub struct Day;

impl Problem for Day {
    type Input = AsteroidField;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 10: Monitoring Station";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let mut asteroids = data;
        let (station, station_view) = asteroids
            .find_surveillance_point()
            .ok_or_else(|| anyhow::anyhow!("Not found any surveillance point"))?;
        asteroids.set_station(station);
        println!("{}", asteroids);
        println!(
            "The best view point is {} which has a view on {} asteroids\n",
            station,
            station_view.len()
        );

        // Wait for user input before destroying asteroids
        println!("Press enter to continue");
        stdin().read_line(&mut String::new())?;

        let ordered = field_ordering(&station, station_view);
        let two_hundredth = ordered[199];
        asteroids.set_marked(two_hundredth);
        visualize(&mut asteroids, ordered).context("IO error during visualization")?;
        println!("200th destroyed asteroid is {}", two_hundredth);

        Ok(())
    }
}

/// Orders the field around the given center point (the station)
fn field_ordering<T: IntoIterator<Item = Point>>(center: &Point, field: T) -> Vec<Point> {
    let (_, reference_angle) = Point::new(0, -center.y).polar_coordinates();
    let order = move |point: &Point| {
        let (_, theta) = (*point - *center).polar_coordinates();
        if theta < reference_angle {
            theta - reference_angle + consts::PI * 2.0
        } else {
            theta - reference_angle
        }
    };
    let compare = |point1: &Point, point2: &Point| {
        let angle1 = order(point1);
        let angle2 = order(point2);

        angle1.partial_cmp(&angle2).unwrap_or(Ordering::Less)
    };

    field.into_iter().sorted_by(compare).collect()
}

/// Visualize the destruction of the asteroid field by printing each state to the console.
fn visualize<T: IntoIterator<Item = Point>>(
    asteroids: &mut AsteroidField,
    destroyed: T,
) -> std::io::Result<()> {
    let mut f = BufWriter::new(stdout());
    write!(f, "{}", CLEAR_COMMAND)?;
    for point in destroyed {
        asteroids.field.remove(&point);
        write!(f, "{}{}", TO_TOP, asteroids)?;
        f.flush()?;
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

pub struct AsteroidField {
    max: (usize, usize),
    field: HashSet<Point>,
    station: Option<Point>,
    marked: Option<Point>,
}

impl FromStr for AsteroidField {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field: HashSet<Point> = s
            .parse::<LineSep<String>>()?
            .data
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, asteroid)| match asteroid {
                        '#' => Some(Point::new(x as i64, y as i64)),
                        _ => None,
                    })
            })
            .collect();

        Ok(AsteroidField::new(field))
    }
}

impl Display for AsteroidField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let station = self.station.unwrap_or_else(|| Point::new(-1, -1));
        let marked = self.marked.unwrap_or_else(|| Point::new(-1, -1));
        let repr = (0..self.max.1)
            .map(|y| {
                (0..self.max.0)
                    .map(|x| {
                        let point = Point::new(x as i64, y as i64);
                        if station == point {
                            '@'
                        } else if marked == point {
                            if self.field.contains(&point) {
                                'O'
                            } else {
                                'X'
                            }
                        } else if self.field.contains(&point) {
                            '#'
                        } else {
                            ' '
                        }
                    })
                    .join("")
            })
            .join("\n");
        write!(
            f,
            "Dimensions:{x}x{y}\n{field}",
            x = self.max.0 + 1,
            y = self.max.1 + 1,
            field = repr
        )
    }
}

impl AsteroidField {
    /// Build a new AsteroidField from the set of asteroids in it.
    pub fn new(field: HashSet<Point>) -> Self {
        let (max_x, max_y) = field.iter().fold((0, 0), |acc, point| {
            (acc.0.max(point.x), acc.1.max(point.y))
        });

        Self {
            max: (max_x as usize, max_y as usize),
            field,
            station: None,
            marked: None,
        }
    }

    /// Sets the asteroid being the observation station in the asteroid field.
    pub fn set_station(&mut self, station: Point) {
        self.station.replace(station);
    }

    /// Sets the asteroid being the marked one (usually the 200th destroyed).
    pub fn set_marked(&mut self, station: Point) {
        self.marked.replace(station);
    }

    /// Finds the best surveillance point in the asteroid field
    /// (the asteroid which views the most asteroids).
    pub fn find_surveillance_point(&self) -> Option<(Point, HashSet<Point>)> {
        let start = std::time::Instant::now();
        let mut max = 0;
        let mut best: Option<(Point, HashSet<Point>)> = None;
        for pov in self.field.iter() {
            if let Some(visible) = self.visible_asteroids(*pov, max) {
                let length = visible.len();
                if length > max {
                    max = length;
                    best = Some((*pov, visible));
                }
            }
        }

        println!(
            "Finding the surveillance point took {} ms",
            start.elapsed().as_millis()
        );
        best
    }

    /// Return the visible asteroids from this point of view.
    /// # Arguments
    /// * `point_of_view` The point from which the field is viewed.
    /// * `minimum` A cut-off point for pruning useless computations (0 to not use it).
    /// If the vision field size becomes smaller than this then we stop the computation early.
    /// # Returns
    /// * Some(set of visible asteroids) if successfully computed
    /// * None if the cut-off point was reached
    pub fn visible_asteroids(
        &self,
        point_of_view: Point,
        minimum: usize,
    ) -> Option<HashSet<Point>> {
        let mut rest: HashSet<Point> = self.field.clone();
        rest.remove(&point_of_view);
        for blockade in self.field.iter() {
            if rest.contains(blockade) {
                for blocked in self.masked_points(&point_of_view, blockade) {
                    rest.remove(&blocked);
                }

                // Early cut-off point
                if minimum > 0 && rest.len() < minimum {
                    return None;
                }
            }
        }

        Some(rest)
    }

    /// Computes all the points that are masked by the blockade from the point of view
    fn masked_points(&self, point_of_view: &Point, blockade: &Point) -> HashSet<Point> {
        fn check_bounds(point: &Point, max: (i64, i64)) -> bool {
            point.x >= 0 && point.x <= max.0 && point.y >= 0 && point.y <= max.1
        }

        let max = (self.max.0 as i64, self.max.1 as i64);
        let vector = *blockade - *point_of_view;
        let unit = vector.divide(gcd(vector.x, vector.y));

        let mut current = *blockade;
        std::iter::from_fn(move || {
            current = current + unit;
            if check_bounds(&current, max) {
                Some(current)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
    }
}

#[cfg(test)]
mod tests;
