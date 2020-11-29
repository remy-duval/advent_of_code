//! Utilities for 2D grid representation:
//! - [Point](Point): A 2D Point in a plane, used in a lot of AoC problems
//! - [Direction](Direction): A 2D direction used to move [Point](Point)s around

use std::{
    f64::consts,
    fmt::{Display, Formatter},
    ops::{Add, Sub},
};

/// Represents a point in a plane (x is from West to East, y from North to South)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    /// Simple constructor for Point.
    pub fn new(x: i64, y: i64) -> Point {
        Self { x, y }
    }

    /// Return a Point moved in the given direction.
    pub fn moved(&self, direction: Direction) -> Point {
        *self + direction.offset()
    }

    /// Manhattan Distance between this point and the origin.
    pub fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    /// Divide all the coordinates of this point with the divisor
    pub fn divide(&self, divisor: i64) -> Point {
        Point::new(self.x / divisor, self.y / divisor)
    }

    /// The polar coordinates this point
    pub fn polar_coordinates(&self) -> (f64, f64) {
        let x = self.x as f64;
        let y = self.y as f64;
        let r = (x.powi(2) + y.powi(2)).sqrt();
        let theta = if y.is_sign_negative() {
            y.atan2(x) + 2.0 * consts::PI
        } else {
            y.atan2(x)
        };

        (r, theta)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Point> for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// Represents a direction in a plane
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// All the directions as a constant
    const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    /// All the directions as a static method
    pub fn all() -> [Direction; 4] {
        Self::ALL
    }

    /// Returns the result of starting from the first argument and moving the following directions
    pub fn compute_movement(point: Point, moves: &[Direction]) -> Point {
        moves.iter().fold(point, |acc, next| acc.moved(*next))
    }

    /// The direction to the right of this one.
    pub fn right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    /// The direction to the left of this one.
    pub fn left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    /// The direction to the back of this one.
    pub fn back(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    /// A simple char representation of this direction.
    pub fn char(self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }

    /// The offset of this direction on a grid (x is from West to East, y from North to South)
    pub fn offset(self) -> Point {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::South => Point::new(0, 1),
            Direction::East => Point::new(1, 0),
            Direction::West => Point::new(-1, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::{Direction, Point};

    #[test]
    fn point_algebra() {
        let origin = Point::new(0, 0);
        let first = Point::new(1, 1);
        let second = Point::new(-1, -1);
        assert_eq!(
            origin + first,
            first,
            "Point(0,0) should the neutral element of Monoid Point"
        );
        assert_eq!(
            origin + second,
            second,
            "Point(0,0) should the neutral element of Monoid Point"
        );
        assert_eq!(second + first, origin, "(1, 1) + (-1, -1) should be (0,0)");
        assert_eq!(
            first + (origin - first),
            origin,
            "a point - itself should be (0, 0)"
        );
        assert_eq!(
            Point::new(5, 10).divide(5),
            Point::new(1, 2),
            "(5, 10) / 5 should  be (1, 2)"
        )
    }

    #[test]
    fn moving() {
        let north = Direction::North;
        let origin = Point::new(0, 0);
        let south_east = Point::new(1, 1);
        let north_west = Point::new(-1, -1);
        assert_eq!(
            origin.moved(north.back()).moved(north.right()),
            south_east,
            "south east point should be reached"
        );
        assert_eq!(
            origin.moved(north).moved(north.left()),
            north_west,
            "north west point should be reached"
        );
    }

    #[test]
    fn distances() {
        let origin = Point::new(0, 0);
        let first = Point::new(1, 1);
        let second = Point::new(-1, -1);
        assert_eq!(
            0,
            origin.manhattan_distance(),
            "Origin is at 0 distance from origin"
        );
        assert_eq!(
            2,
            first.manhattan_distance(),
            "(1, 1) is at 2 distance from origin"
        );
        assert_eq!(
            2,
            second.manhattan_distance(),
            "(-1, -1) is at 2 distance from origin"
        );
        assert_eq!(
            4,
            (first - second).manhattan_distance(),
            "(1, 1) is at 4 distance from (-1, -1)"
        )
    }

    #[test]
    fn polar() {
        fn assertion(point: Point, expected_r: f64, expected_theta: f64) {
            let (r, theta) = point.polar_coordinates();
            assert!((r - expected_r).abs() < 1e-10, "{} != {}", r, expected_r);
            assert!(
                (theta - expected_theta).abs() < 1e-10,
                "{} != {}",
                theta,
                expected_theta
            );
        }

        assertion(Point::new(1, 0), 1.0, 0.0);
        assertion(Point::new(0, 1), 1.0, consts::FRAC_PI_2);
        assertion(Point::new(0, -1), 1.0, consts::FRAC_PI_2 * 3.0);
        assertion(Point::new(-1, 0), 1.0, consts::PI);
    }
}
