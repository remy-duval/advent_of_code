use itertools::Itertools;

use commons::grid::Point;
use commons::{Result, WrapErr};

pub const TITLE: &str = "Day 6: Chronal Coordinates";
const MAXIMUM_DISTANCE: i64 = 10_000;

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!(
        "The largest finite area is of size {}",
        data.largest_finite_area()
            .wrap_err("Could not find a finite area")?
    );

    println!(
        "{} points are less than {} from all the coordinates",
        data.near_points(MAXIMUM_DISTANCE),
        MAXIMUM_DISTANCE
    );

    Ok(())
}

fn parse(s: &str) -> Result<Coordinates> {
    let points = s
        .lines()
        .map(|line| {
            let (x, y) = line
                .splitn(2, ',')
                .map(|part| {
                    part.trim()
                        .parse::<i64>()
                        .wrap_err_with(|| format!("Could not parse a number {part}"))
                })
                .collect_tuple::<(_, _)>()
                .wrap_err_with(|| format!("Expected '<X>, <Y>' but got: {line}"))?;

            Ok(Point::new(x?, y?))
        })
        .collect::<Result<Vec<Point>>>()?;

    Ok(Coordinates::new(points))
}

/// All the coordinates that were returned
struct Coordinates {
    /// The coordinates to consider
    points: Vec<Point>,
    /// The maximum coordinates of the plane containing the points (minimum are (0, 0))
    max: Point,
}

impl Coordinates {
    /// Create the coordinates from a set of points
    fn new(points: Vec<Point>) -> Self {
        let (max_x, max_y) = points.iter().fold((0, 0), |(max_x, max_y), point| {
            (max_x.max(point.x), max_y.max(point.y))
        });

        Self {
            points,
            max: Point::new(max_x + 1, max_y + 1),
        }
    }

    /// Compute the area of each coordinate in the grid, then find the largest finite one
    fn largest_finite_area(&self) -> Option<i32> {
        let mut areas = vec![0; self.points.len()];
        let mut finite = vec![true; self.points.len()];
        (0..self.max.x)
            .cartesian_product(0..self.max.y)
            .for_each(|(x, y)| {
                if let Some(nearest) = self.nearest_coordinate(Point::new(x, y)) {
                    areas[nearest] += 1;
                    if x == 0 || y == 0 || x == (self.max.x - 1) || y == (self.max.y - 1) {
                        finite[nearest] = false;
                    }
                }
            });

        finite
            .into_iter()
            .zip(areas)
            .filter_map(|(ok, area)| if ok { Some(area) } else { None })
            .max()
    }

    /// Count the total of points for which the total distance is less than `maximum_distance`
    fn near_points(&self, maximum_distance: i64) -> usize {
        // This depends on the hypothesis than the near points will be contained
        // In the (0, 0) x (max_x, max_y) Grid
        // Since the maximum_distance is not that huge it should work fine
        (0..self.max.x)
            .cartesian_product(0..self.max.y)
            .filter(|(x, y)| {
                let point: Point = Point::new(*x, *y);
                let distances: i64 = self
                    .points
                    .iter()
                    .map(|&other| (point - other).manhattan_distance())
                    .sum();

                distances < maximum_distance
            })
            .count()
    }

    /// Find the position in the vector of the nearest coordinate to this point
    /// If multiple coordinates are tied, this returns None
    fn nearest_coordinate(&self, point: Point) -> Option<usize> {
        let mut previous_min: i64 = i64::MAX;
        let mut min: i64 = i64::MAX;
        let mut pos: Option<usize> = None;
        self.points.iter().enumerate().for_each(|(i, &other)| {
            let distance = (point - other).manhattan_distance();
            if distance <= min {
                previous_min = min;
                min = distance;
                pos = Some(i);
            }
        });

        if previous_min == min {
            // We found at least two elements tied to the minimum, so no unique minimum
            None
        } else {
            pos
        }
    }
}

#[cfg(test)]
mod tests;
