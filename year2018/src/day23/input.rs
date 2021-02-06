use std::str::FromStr;

use itertools::{Itertools, process_results};

/// The type of the dimensions of the point
pub type Dimension = i32;

/// A bot in the problem, with a position and a signal radius
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bot {
    pub pos: Point3,
    pub r: Dimension,
}

impl Bot {
    /// Check if a bot is in the radius of this bot
    pub fn can_reach(&self, other: &Self) -> bool {
        self.pos.distance(&other.pos) <= self.r
    }
}

/// A three dimensional point
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Point3 {
    pub x: Dimension,
    pub y: Dimension,
    pub z: Dimension,
}

impl Point3 {
    /// Basic constructor
    pub fn new(x: Dimension, y: Dimension, z: Dimension) -> Self {
        Self { x, y, z }
    }

    /// The manhattan distance of this point to another
    pub fn distance(&self, other: &Self) -> Dimension {
        let x = (self.x - other.x).abs();
        let y = (self.y - other.y).abs();
        let z = (self.z - other.z).abs();
        x + y + z
    }

    /// The manhattan distance of this point to the center
    pub fn origin_distance(&self) -> Dimension {
       self.x.abs() + self.y.abs() + self.z.abs()
    }
}

/// An error when parsing a Bot
#[derive(Debug, thiserror::Error)]
pub enum ParseBotError {
    #[error("Expected pos=POINT, r=RADIUS, got: {0}")]
    BadFormat(Box<str>),
    #[error("Couldn't parse the position: {0}")]
    ParsePoint(#[from] ParsePointError),
    #[error("Couldn't parse the radius of the bot: {0} ({1})")]
    ParseInt(Box<str>, #[source] std::num::ParseIntError),
}

/// An error when parsing a Point3
#[derive(Debug, thiserror::Error)]
pub enum ParsePointError {
    #[error("Expected <X, Y, Z> for a point, got: {0}")]
    BadFormat(Box<str>),
    #[error("Couldn't parse a dimension of a point: {0} ({1})")]
    ParseInt(Box<str>, #[source] std::num::ParseIntError),
}

impl FromStr for Bot {
    type Err = ParseBotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn split(s: &str) -> Option<(&str, &str)> {
            let (radius, point) = s.rsplitn(2, ',').collect_tuple::<(_, _)>()?;
            let pos = point.trim().strip_prefix("pos=")?;
            let radius = radius.trim().strip_prefix("r=")?;
            Some((pos, radius))
        }

        let (pos, radius) = split(s).ok_or_else(|| ParseBotError::BadFormat(s.into()))?;
        let r = radius
            .parse::<Dimension>()
            .map_err(|e| ParseBotError::ParseInt(radius.into(), e))?;
        let pos = pos.parse()?;

        Ok(Self { pos, r })
    }
}

impl FromStr for Point3 {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn inner(s: &str) -> Option<Result<(Dimension, Dimension, Dimension), ParsePointError>> {
            let elements = s
                .strip_prefix('<')?
                .strip_suffix('>')?
                .trim()
                .split(',')
                .map(|n| {
                    n.parse::<Dimension>()
                        .map_err(|e| ParsePointError::ParseInt(n.into(), e))
                });

            process_results(elements, |results| results.collect_tuple::<(_, _, _)>()).transpose()
        }

        let (x, y, z) = inner(s).unwrap_or_else(|| Err(ParsePointError::BadFormat(s.into())))?;
        Ok(Self { x, y, z })
    }
}
