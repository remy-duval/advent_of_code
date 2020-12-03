use std::cmp::Ordering;
use std::collections::HashSet;
use std::f64::consts;
use std::fmt::{Display, Formatter};
use std::io::{BufWriter, stdout, Write};
use std::str::FromStr;
use std::time::Duration;

use anyhow::Context;
use itertools::Itertools;

use crate::commons::{CLEAR_COMMAND, math, TO_TOP};
use crate::commons::grid::Point;
use crate::commons::parse::LineSep;
use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = AsteroidField;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 10: Monitoring Station";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let mut asteroids = data;
        let (station, station_view) = asteroids
            .find_surveillance_point()
            .ok_or(anyhow::anyhow!("Not found any surveillance point"))?;
        asteroids.set_station(station);
        println!("{}", asteroids);
        println!(
            "The best view point is {} which has a view on {} asteroids",
            station,
            station_view.len()
        );
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
        let unit = vector.divide(math::gcd(vector.x, vector.y));

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
mod tests {
    use super::*;

    const TEST_ONE: &str = include_str!("test_resources/day10_1.txt");
    const TEST_TWO: &str = include_str!("test_resources/day10_2.txt");
    const TEST_THREE: &str = include_str!("test_resources/day10_3.txt");
    const TEST_FOUR: &str = include_str!("test_resources/day10_4.txt");
    const DATA: &str = include_str!("test_resources/day10_data.txt");

    #[test]
    fn check_surveillance_point() {
        fn assertion(data: &str, expected_station: Point, expected_count: usize) {
            let (station, view) = data
                .parse::<AsteroidField>()
                .expect("Parse error !")
                .find_surveillance_point()
                .expect("Not found any surveillance point");

            assert_eq!(
                expected_station, station,
                "\n{} != {} for \n{}",
                expected_station, station, data
            );
            assert_eq!(
                expected_count,
                view.len(),
                "\n{} != {} for \n{}",
                expected_count,
                view.len(),
                data
            );
        }

        assertion(TEST_ONE, Point::new(5, 8), 33);
        assertion(TEST_TWO, Point::new(1, 2), 35);
        assertion(TEST_THREE, Point::new(6, 3), 41);
        assertion(TEST_FOUR, Point::new(11, 13), 210);
    }

    #[test]
    fn destroy_order() {
        let asteroids: AsteroidField = TEST_FOUR.parse().expect("Parse error !");
        let (station, view) = asteroids
            .find_surveillance_point()
            .expect("Not found any surveillance point");
        let ordered = field_ordering(&station, view);

        // The 1st asteroid to be vaporized is at 11,12.
        // The 2nd asteroid to be vaporized is at 12,1.
        // The 3rd asteroid to be vaporized is at 12,2.
        // The 10th asteroid to be vaporized is at 12,8.
        // The 20th asteroid to be vaporized is at 16,0.
        // The 50th asteroid to be vaporized is at 16,9.
        // The 100th asteroid to be vaporized is at 10,16.
        // The 199th asteroid to be vaporized is at 9,6.
        // The 200th asteroid to be vaporized is at 8,2.
        // The 201st asteroid to be vaporized is at 10,9.
        assert_eq!(ordered[0], Point::new(11, 12));
        assert_eq!(ordered[1], Point::new(12, 1));
        assert_eq!(ordered[2], Point::new(12, 2));
        assert_eq!(ordered[9], Point::new(12, 8));
        assert_eq!(ordered[19], Point::new(16, 0));
        assert_eq!(ordered[49], Point::new(16, 9));
        assert_eq!(ordered[99], Point::new(10, 16));
        assert_eq!(ordered[198], Point::new(9, 6));
        assert_eq!(ordered[199], Point::new(8, 2));
        assert_eq!(ordered[200], Point::new(10, 9));
    }

    #[test]
    fn solve_test() {
        let mut asteroids: AsteroidField = DATA.parse().unwrap();
        let (station, station_view) = asteroids
            .find_surveillance_point()
            .expect("Not found any surveillance point");
        assert_eq!(340, station_view.len());

        asteroids.set_station(station);
        let ordered = field_ordering(&station, station_view);
        let two_hundredth = ordered[199];

        assert_eq!(2_628, two_hundredth.x * 100 + two_hundredth.y);
    }
}
