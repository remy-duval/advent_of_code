use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::str::FromStr;

use hashbrown::HashMap;

use commons::grid::{Direction, Point};
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Network;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 13: Mine Cart Madness";

    fn solve(mut network: Self::Input) -> Result<(), Self::Err> {
        let crash = first_part(&mut network);
        println!("The first crash happened at {},{}", crash.x, crash.y);

        let last = second_part(&mut network);
        println!("The last cart is at {},{}", last.x, last.y);

        Ok(())
    }
}

/// Run the network until the first crash, returning its position
fn first_part(network: &mut Network) -> Point {
    loop {
        if let Some(crash) = network.advance() {
            return crash;
        }
    }
}

/// Run the network until the last crash, returning the last cart position
fn second_part(network: &mut Network) -> Point {
    loop {
        if network.advance().is_some() {
            if network.cart_positions.len() == 1 {
                break network.cart_positions.iter().next().unwrap().0 .0;
            } else if network.cart_positions.is_empty() {
                panic!("No cart remains after the last crash");
            }
        }
    }
}

/// A point, but where the order is based on vertical axis first
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct VerticalOrderedPoint(pub Point);

impl Ord for VerticalOrderedPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.y.cmp(&other.0.y).then(self.0.x.cmp(&other.0.x))
    }
}

impl PartialOrd for VerticalOrderedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The rail network and the carts on it
#[derive(Debug, Clone)]
pub struct Network {
    /// The state of the mine carts
    cart_states: Vec<(u8, Direction)>,
    /// The current positions of the mine carts
    cart_positions: BTreeMap<VerticalOrderedPoint, usize>,
    /// The rail way tracks
    tracks: HashMap<Point, Track>,
}

impl Network {
    /// Advance the state of the network, returns an Option of any crash that happened
    pub fn advance(&mut self) -> Option<Point> {
        let mut crash = None;
        self.cart_positions.clone().iter().for_each(|(point, idx)| {
            // Remove the cart from its previous position
            // If nothing has been removed, this cart has been crashed into already
            if self.cart_positions.remove(point).is_none() {
                return;
            }

            let state = &mut self.cart_states[*idx];
            let next = VerticalOrderedPoint(point.0 + state.1.offset());
            if let Some(track) = self.tracks.get(&next.0) {
                *state = track.next(state.1, state.0);
                // If the inserted point is already present, we got a crash !
                if self.cart_positions.insert(next, *idx).is_some() {
                    // Remove the crashed carts
                    self.cart_positions.remove(&next);
                    // Assign the point of the crash
                    crash = Some(next.0);
                }
            } else {
                panic!("A mine-cart went off the tracks !");
            }
        });

        crash
    }
}

// Display implementation for debugging purposes
impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max = self.tracks.keys().fold((0, 0), |(max_x, max_y), point| {
            (max_x.max(point.x), max_y.max(point.y))
        });

        (0..(max.1 + 1)).try_for_each(|y| {
            (0..(max.0 + 1)).try_for_each(|x| {
                let point = Point::new(x, y);
                let v_point = VerticalOrderedPoint(point);
                let c: char = if let Some(cart) = self.cart_positions.get(&v_point) {
                    self.cart_states[*cart].1.char()
                } else if let Some(track) = self.tracks.get(&point) {
                    match track {
                        Track::Horizontal => '-',
                        Track::Vertical => '|',
                        Track::Slash => '/',
                        Track::AntiSlash => '\\',
                        Track::Intersection => '+',
                    }
                } else {
                    ' '
                };

                f.write_char(c)
            })?;

            f.write_char('\n')
        })
    }
}

impl FromStr for Network {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cart_states = Vec::with_capacity(10);
        let mut cart_positions = BTreeMap::new();
        let mut tracks = HashMap::with_capacity(s.len());

        s.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let point = Point::new(x as i64, y as i64);
                let track: Option<Track> = match c {
                    '-' => Some(Track::Horizontal),
                    '|' => Some(Track::Vertical),
                    '\\' => Some(Track::AntiSlash),
                    '/' => Some(Track::Slash),
                    '+' => Some(Track::Intersection),
                    '^' | '>' | '<' | 'v' => {
                        let (track, direction) = match c {
                            '^' => (Track::Vertical, Direction::North),
                            'v' => (Track::Vertical, Direction::South),
                            '>' => (Track::Horizontal, Direction::East),
                            _ => (Track::Horizontal, Direction::West),
                        };
                        cart_states.push((0, direction));
                        cart_positions.insert(VerticalOrderedPoint(point), cart_states.len() - 1);
                        Some(track)
                    }
                    _ => None,
                };

                if let Some(track) = track {
                    tracks.insert(point, track);
                }
            });
        });

        Ok(Self {
            cart_states,
            cart_positions,
            tracks,
        })
    }
}

/// A track in the network
#[derive(Debug, Copy, Clone)]
pub enum Track {
    /// a straight '|'
    Horizontal,
    /// a straight '-'
    Vertical,
    /// a '\' turn
    AntiSlash,
    /// a '/' turn
    Slash,
    /// an '+' intersection
    Intersection,
}

impl Track {
    /// Compute the direction and number of intersection seen after taking this track
    ///
    /// ### Arguments
    /// * `direction` - The direction before taking the track
    /// * `intersections` - The number of intersections seen beforehand
    ///
    /// ### Returns
    /// (new number of intersections seen, new direction)
    pub fn next(self, direction: Direction, intersections: u8) -> (u8, Direction) {
        let mut after = intersections;
        let direction = match self {
            Self::Horizontal | Self::Vertical => direction,
            Self::AntiSlash => match direction {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
            },
            Self::Slash => match direction {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            },
            Self::Intersection => {
                after = (after + 1) % 3;
                match intersections % 3 {
                    0 => direction.left(),
                    1 => direction,
                    _ => direction.right(),
                }
            }
        };

        (after, direction)
    }
}

#[cfg(test)]
mod tests;
