//! A wrapper around the [Point](commons::grid::Point) that overrides the ordering
//! The ordering for this point is the reading order used a lot in problems: (Y, X) order
//! This wrapper tries to forward the utilities from the underlying point as much as possible

use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, Deref, DerefMut, Sub};

use commons::grid::Point as CommonPoint;

/// A point, but ordered on the reading order (Y then X) instead of natural order
/// This is useful for BTreeMap/BTreeSet/BinaryHeap ordering
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point(pub CommonPoint);

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self(CommonPoint::new(x, y))
    }
}

impl From<CommonPoint> for Point {
    fn from(point: CommonPoint) -> Self {
        Self(point)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .y
            .cmp(&other.0.y)
            .then_with(|| self.0.x.cmp(&other.0.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Deref for Point {
    type Target = CommonPoint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.0
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.0
    }
}

impl Add<CommonPoint> for Point {
    type Output = Point;

    fn add(self, rhs: CommonPoint) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<CommonPoint> for Point {
    type Output = Point;

    fn sub(self, rhs: CommonPoint) -> Self::Output {
        Self(self.0 - rhs)
    }
}
