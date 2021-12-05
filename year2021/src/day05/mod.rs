use hashbrown::{hash_map::Entry, HashMap};

use commons::eyre::{eyre, Report, Result, WrapErr};
use commons::grid::Point;
use commons::parse::LineSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<Segment>;
    const TITLE: &'static str = "Day 5: Hydrothermal Venture";

    fn solve(data: Self::Input) -> Result<()> {
        println!("1. Overlapping (no diagonals) {}", first_part(&data.data));
        println!("1. Overlapping (diagonals) {}", second_part(&data.data));

        Ok(())
    }
}

fn first_part(segments: &[Segment]) -> usize {
    overlapping_points(segments, false)
}

fn second_part(segments: &[Segment]) -> usize {
    overlapping_points(segments, true)
}

/// Count the overlapping points of those segments
///
/// ### Params
/// * `segments` - The segments to count the points for
/// * `use_diagonals` - False to omit diagonal segments during the counting
///
/// ### Returns
/// The number of points covered by two segments or more
fn overlapping_points(segments: &[Segment], use_diagonals: bool) -> usize {
    // Map points to: false -> one segment, true -> overlapping
    let mut points = HashMap::with_capacity(segments.len());
    for s in segments {
        if use_diagonals || !s.is_diagonal() {
            s.points().for_each(|point| match points.entry(point) {
                Entry::Occupied(mut e) => {
                    e.insert(true);
                }
                Entry::Vacant(e) => {
                    e.insert(false);
                }
            });
        }
    }

    points.into_iter().filter(|(_, overlap)| *overlap).count()
}

/// A segment for the puzzle
pub struct Segment {
    from: Point<i16>,
    to: Point<i16>,
}

impl Segment {
    /// True if this segment is not horizontal or vertical
    fn is_diagonal(&self) -> bool {
        self.from.x != self.to.x && self.from.y != self.to.y
    }

    /// An iterator over all the points of this segment
    fn points(&self) -> impl Iterator<Item = Point<i16>> {
        let (step, count) = match self.to - self.from {
            Point { x: 0, y } => (Point::new(0, y.signum()), y.abs() as usize),
            Point { x, y: 0 } => (Point::new(x.signum(), 0), x.abs() as usize),
            Point { x, y } => {
                // Always positive
                let steps = commons::num::integer::gcd(x, y);
                (Point::new(x / steps, y / steps), steps as usize)
            }
        };

        std::iter::successors(Some(self.from), move |last| Some(step + last)).take(count + 1)
    }
}

impl std::str::FromStr for Segment {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn point(s: &str) -> Result<Point<i16>> {
            let (x, y) = s.split_once(',').ok_or_else(|| eyre!("Bad point {}", s))?;
            let x = x.parse().wrap_err("Bad x coordinate")?;
            let y = y.parse().wrap_err("Bad y coordinate")?;
            Ok(Point::new(x, y))
        }

        let (from, to) = s
            .split_once(" -> ")
            .ok_or_else(|| eyre!("Bad line: {}", s))?;

        Ok(Self {
            from: point(from.trim()).wrap_err("Bad from point")?,
            to: point(to.trim()).wrap_err("Bad to point")?,
        })
    }
}

#[cfg(test)]
mod tests;
