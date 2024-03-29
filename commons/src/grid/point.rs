//! Utilities for 2D grid representation:
//! - [Point](Point): A 2D Point in a plane, used in a lot of AoC problems
//! - [Direction](Direction): A 2D direction used to move [Point](Point)s around

use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::math::{Integer, IntegerToFloat, SignedInteger};

/// Represents a point in a plane (x is from West to East, y from North to South)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Point<Coordinate = i64> {
    pub x: Coordinate,
    pub y: Coordinate,
}

impl<Coordinate> Point<Coordinate> {
    /// Simple constructor for Point.
    #[inline]
    pub const fn new(x: Coordinate, y: Coordinate) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn tupled(self) -> (Coordinate, Coordinate) {
        (self.x, self.y)
    }
}

impl<Coordinate: Integer> Point<Coordinate> {
    /// Addition for a Point
    #[inline]
    pub fn addition(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Subtraction for a point
    #[inline]
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Multiply all the coordinates of this point with the given value
    #[inline]
    pub fn multiply(&self, mul: Coordinate) -> Self {
        Self {
            x: self.x * mul,
            y: self.y * mul,
        }
    }

    /// Divide all the coordinates of this point with the divisor
    #[inline]
    pub fn divide(&self, divisor: Coordinate) -> Self {
        Self {
            x: self.x / divisor,
            y: self.y / divisor,
        }
    }

    /// Manhattan Distance between this point and the origin.
    #[inline]
    pub fn manhattan_distance(&self) -> Coordinate {
        self.x.absolute_value() + self.y.absolute_value()
    }
}

impl<Coordinate: SignedInteger> Point<Coordinate> {
    /// Return a Point moved in the given direction.
    #[inline]
    pub fn moved(&self, direction: Direction) -> Self {
        self.addition(&direction.offset::<Coordinate>())
    }
}

impl<Coordinate: IntegerToFloat> Point<Coordinate> {
    /// The polar coordinates this point
    #[inline]
    pub fn polar_coordinates(&self) -> (f64, f64) {
        let x = self.x.to_f64();
        let y = self.y.to_f64();
        let r = (x.powi(2) + y.powi(2)).sqrt();
        let theta = if y.is_sign_negative() {
            y.atan2(x) + 2.0 * std::f64::consts::PI
        } else {
            y.atan2(x)
        };

        (r, theta)
    }
}

impl<Coordinate: std::fmt::Display> std::fmt::Display for Point<Coordinate> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<Coordinate: Integer> Add<Point<Coordinate>> for Point<Coordinate> {
    type Output = Point<Coordinate>;

    #[inline]
    fn add(self, rhs: Point<Coordinate>) -> Self::Output {
        self.addition(&rhs)
    }
}

impl<Coordinate: Integer> AddAssign<Point<Coordinate>> for Point<Coordinate> {
    fn add_assign(&mut self, rhs: Point<Coordinate>) {
        *self = self.addition(&rhs);
    }
}

impl<Coordinate: Integer> Add<&Point<Coordinate>> for Point<Coordinate> {
    type Output = Point<Coordinate>;

    #[inline]
    fn add(self, rhs: &Point<Coordinate>) -> Self::Output {
        self.addition(rhs)
    }
}

impl<Coordinate: Integer> AddAssign<&Point<Coordinate>> for Point<Coordinate> {
    fn add_assign(&mut self, rhs: &Point<Coordinate>) {
        *self = self.addition(rhs);
    }
}

impl<Coordinate: Integer> Sub<Point<Coordinate>> for Point<Coordinate> {
    type Output = Point<Coordinate>;

    #[inline]
    fn sub(self, rhs: Point<Coordinate>) -> Self::Output {
        self.subtract(&rhs)
    }
}

impl<Coordinate: Integer> SubAssign<Point<Coordinate>> for Point<Coordinate> {
    fn sub_assign(&mut self, rhs: Point<Coordinate>) {
        *self = self.subtract(&rhs);
    }
}

impl<Coordinate: Integer> Sub<&Point<Coordinate>> for Point<Coordinate> {
    type Output = Point<Coordinate>;

    #[inline]
    fn sub(self, rhs: &Point<Coordinate>) -> Self::Output {
        self.subtract(rhs)
    }
}

impl<Coordinate: Integer> SubAssign<&Point<Coordinate>> for Point<Coordinate> {
    fn sub_assign(&mut self, rhs: &Point<Coordinate>) {
        *self = self.subtract(rhs);
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
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    /// All the directions as a static method
    #[inline]
    pub const fn all() -> [Direction; 4] {
        Self::ALL
    }

    /// Returns the result of starting from the first argument and moving the following directions
    pub fn compute_movement(point: Point, moves: &[Direction]) -> Point {
        moves.iter().fold(point, |acc, next| acc.moved(*next))
    }

    /// The direction to the right of this one.
    #[inline]
    pub const fn right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    /// The direction to the left of this one.
    #[inline]
    pub const fn left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    /// The direction to the back of this one.
    #[inline]
    pub const fn back(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    /// A simple char representation of this direction.
    #[inline]
    pub const fn char(self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }

    /// The offset of this direction on a grid (x is from West to East, y from North to South)
    #[inline]
    pub fn offset<Coordinate: SignedInteger>(self) -> Point<Coordinate> {
        match self {
            Direction::North => Point::new(Coordinate::ZERO, -Coordinate::ONE),
            Direction::South => Point::new(Coordinate::ZERO, Coordinate::ONE),
            Direction::East => Point::new(Coordinate::ONE, Coordinate::ZERO),
            Direction::West => Point::new(-Coordinate::ONE, Coordinate::ZERO),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::{Direction, Point};

    #[test]
    fn operators() {
        let mut a = Point::new(6, -2);
        let mut b = Point::new(43, 7);

        assert_eq!(a + b, Point::new(49, 5));
        assert_eq!(a + &b, Point::new(49, 5));
        assert_eq!(a - b, Point::new(-37, -9));
        assert_eq!(a - &b, Point::new(-37, -9));

        a += b;
        a += Point::new(0, 1);
        a += Point::new(2, 0);
        assert_eq!(a, Point::new(51, 6));
        a -= b;
        a -= Point::new(2, 1);
        assert_eq!(a, Point::new(6, -2));

        b += &a;
        b += &Point::new(42, 3);
        b += &Point::new(-6, 7);
        assert_eq!(b, Point::new(85, 15));
        b -= &a;
        b -= &Point::new(-6, 7);
        b -= &Point::new(42, 3);
        assert_eq!(b, Point::new(43, 7));
    }

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
