use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};

use itertools::Itertools;

use commons::grid::Point;
use commons::{err, Report, Result, WrapErr};

pub const TITLE: &str = "Day 10: The Stars Align";

pub fn run(raw: String) -> Result<()> {
    let message = parse(&raw)?;
    let (minimum, time) = message.into_minimum_size();
    println!("The message took {time}s to appear, it is:\n{minimum}");
    Ok(())
}

fn parse(s: &str) -> Result<Message> {
    s.lines()
        .map(str::parse)
        .collect::<Result<Vec<_>>>()
        .map(Message::new)
}

/// The message formed from a gathering of points of light
/// Use the display trait to get the actual message representation
struct Message {
    lights: Vec<Light>,
    max: Point,
    min: Point,
}

impl Message {
    /// Create the message from the individual lights
    fn new(lights: Vec<Light>) -> Self {
        let (max, min) = Self::compute_bounds(&lights);
        Self { lights, max, min }
    }

    /// Advance the message until its points are the most clustered (minimum size)
    /// We assume that the message will be formed at that point (i.e. there are no stray points)
    fn into_minimum_size(mut self) -> (Self, usize) {
        let mut seconds: usize = 0;
        let mut size: i64 = self.size();
        let mut previous: i64 = size;

        while previous >= size {
            self.advance();
            previous = size;
            size = self.size();
            seconds += 1;
        }

        // We went one step too far to exit the while loop, reverse it
        self.reverse();
        (self, seconds.saturating_sub(1))
    }

    /// Advance the state of the message by one step
    fn advance(&mut self) {
        self.lights.iter_mut().for_each(Light::advance);
        let (max, min) = Self::compute_bounds(&self.lights);
        self.max = max;
        self.min = min;
    }

    /// Reverse the state of the message by one step
    fn reverse(&mut self) {
        self.lights.iter_mut().for_each(Light::reverse);
        let (max, min) = Self::compute_bounds(&self.lights);
        self.max = max;
        self.min = min;
    }

    /// The size of the message
    fn size(&self) -> i64 {
        (self.max - self.min).manhattan_distance()
    }

    /// Compute the new bounding box of the message
    fn compute_bounds(lights: &[Light]) -> (Point, Point) {
        let (min, max) = lights.iter().fold(
            ((i64::MAX, i64::MAX), (i64::MIN, i64::MIN)),
            |((min_x, min_y), (max_x, max_y)), l| {
                let min = (min_x.min(l.position.x), min_y.min(l.position.y));
                let max = (max_x.max(l.position.x), max_y.max(l.position.y));
                (min, max)
            },
        );

        (Point::new(max.0 + 1, max.1 + 1), Point::new(min.0, min.1))
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use std::fmt::Write;

        let points: HashSet<Point> = self.lights.iter().map(|light| light.position).collect();
        (self.min.y..self.max.y).try_for_each(|y| {
            (self.min.x..self.max.x).try_for_each(|x| {
                if points.contains(&Point::new(x, y)) {
                    f.write_char('#')
                } else {
                    f.write_char('.')
                }
            })?;
            f.write_char('\n')
        })
    }
}

/// The coordinates of a light in the message
#[derive(Debug, Clone)]
struct Light {
    position: Point,
    velocity: Point,
}

impl Light {
    /// Advance the light by one step
    pub fn advance(&mut self) {
        self.position = self.position + self.velocity;
    }

    /// Reverse the light by one step
    pub fn reverse(&mut self) {
        self.position = self.position - self.velocity;
    }
}

impl std::str::FromStr for Light {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn parse_int(int: &str) -> Result<i64> {
            int.parse()
                .wrap_err_with(|| format!("Can't parse a point coordinate {int}"))
        }

        fn parse_point(point: &str) -> Result<Point> {
            let (x, y) = point
                .trim()
                .strip_prefix('<')
                .and_then(|point| point.strip_suffix('>'))
                .and_then(|point| {
                    point
                        .splitn(2, ',')
                        .map(|coord| parse_int(coord.trim()))
                        .collect_tuple::<(_, _)>()
                })
                .wrap_err_with(|| format!("Bad format for point '<X, Y>>', got {point}"))?;

            Ok(Point::new(x?, y?))
        }

        let (pos, speed) = s
            .trim()
            .strip_prefix("position=")
            .and_then(|s| {
                s.splitn(2, "velocity=")
                    .map(parse_point)
                    .collect_tuple::<(_, _)>()
            })
            .ok_or_else(|| {
                err!(
                    "Bad format for light 'position=POINT velocity=POINT', got {}",
                    s
                )
            })?;

        Ok(Self {
            position: pos?,
            velocity: speed?,
        })
    }
}

#[cfg(test)]
mod tests;
