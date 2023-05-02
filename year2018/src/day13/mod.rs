use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};

use std::collections::HashMap;

use commons::grid::Direction;
use commons::Result;

use crate::points::Point;

pub const TITLE: &str = "Day 13: Mine Cart Madness";

pub fn run(raw: String) -> Result<()> {
    let mut network = parse(&raw);
    let crash = first_part(&mut network);
    println!("The first crash happened at {},{}", crash.x, crash.y);

    let last = second_part(&mut network);
    println!("The last cart is at {},{}", last.x, last.y);

    Ok(())
}

fn parse(s: &str) -> Network {
    let mut next_id = 0;
    let mut carts = BTreeMap::new();
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
                    let cart = Cart {
                        id: next_id,
                        turn: Default::default(),
                        direction,
                    };
                    carts.insert(point, cart);
                    next_id += 1;
                    Some(track)
                }
                _ => None,
            };

            if let Some(track) = track {
                tracks.insert(point, track);
            }
        });
    });

    Network { carts, tracks }
}

/// Run the network until the first crash, returning its position
fn first_part(network: &mut Network) -> Point {
    loop {
        if let Some(crash) = network.next_tick() {
            return crash;
        }
    }
}

/// Run the network until the last crash, returning the last cart position
fn second_part(network: &mut Network) -> Point {
    loop {
        if let Some(last) = network.next_tick().and_then(|_| network.last_cart()) {
            return last;
        }
    }
}
/// The rail network and the carts on it
struct Network {
    /// The mine carts indexed by their position (ordered correctly since this is a BTreeMap)
    carts: BTreeMap<Point, Cart>,
    /// The rail way tracks
    tracks: HashMap<Point, Track>,
}

impl Network {
    /// Get the position of the last cart if there is only one left, None otherwise
    fn last_cart(&self) -> Option<Point> {
        match self.carts.len() {
            1 => Some(*self.carts.iter().next()?.0),
            0 => panic!("No cart remains after the last crash"),
            _ => None,
        }
    }

    /// Compute the next tick of the network, returns an Option of any crash that happened
    fn next_tick(&mut self) -> Option<Point> {
        let mut crash = None;
        self.carts
            .clone()
            .into_iter()
            .for_each(|(point, mut cart)| {
                // First of all, remove the cart from its previous position
                if let Some(other) = self.carts.remove(&point) {
                    let next = point + cart.direction.offset();

                    // If the removed cart is not this one, reverse the change
                    if other.id != cart.id {
                        self.carts.insert(point, other);
                    } else if let Some(track) = self.tracks.get(&next) {
                        // Update the cart with the arrival track, then insert it
                        cart.update(*track);
                        // If the inserted point is already present, we got a crash !
                        if self.carts.insert(next, cart).is_some() {
                            // Remove the crashed carts
                            self.carts.remove(&next);
                            // Assign the point of the crash
                            crash = Some(next);
                        }
                    } else {
                        panic!("A mine-cart went off the tracks !");
                    }
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
                let c: char = if let Some(cart) = self.carts.get(&point) {
                    cart.direction.char()
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

/// The state of a mine cart
#[derive(Copy, Clone)]
struct Cart {
    /// A unique ID for the cart
    /// Checked when taking the cart turn to see if the cart was not already replaced
    id: u8,
    /// The inner memory of the cart for it to know which turn to take next
    turn: Turn,
    /// The current direction of the cart
    direction: Direction,
}

impl Cart {
    /// Update the cart turn and direction from the given track
    fn update(&mut self, track: Track) {
        self.direction = match track {
            Track::Horizontal | Track::Vertical => self.direction,
            Track::AntiSlash => match self.direction {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
            },
            Track::Slash => match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            },
            Track::Intersection => {
                let direction = self.turn.direction(self.direction);
                self.turn = self.turn.next();
                direction
            }
        };
    }
}

/// The inner memory of a mine cart about the next turn to make
#[derive(Copy, Clone)]
enum Turn {
    /// next turn is to the left
    Left,
    /// next turn is straight ahead
    Straight,
    /// next turn is to the right
    Right,
}

impl Default for Turn {
    fn default() -> Self {
        Self::Left
    }
}

impl Turn {
    /// The next turn the cart will make
    fn next(self) -> Self {
        match self {
            Self::Left => Self::Straight,
            Self::Straight => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// Compute the direction of this turn
    fn direction(self, from: Direction) -> Direction {
        match self {
            Turn::Left => from.left(),
            Turn::Straight => from,
            Turn::Right => from.right(),
        }
    }
}

/// A track in the network
#[derive(Copy, Clone)]
enum Track {
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

#[cfg(test)]
mod tests;
