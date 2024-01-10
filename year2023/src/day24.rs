use itertools::Itertools;
use num_bigint::BigInt;

use commons::error::Result;
use commons::parse::LineSep;
use commons::{Report, WrapErr};

pub const TITLE: &str = "Day 24: Never Tell Me The Odds";

const FIRST_AREA: (i64, i64) = (200_000_000_000_000, 400_000_000_000_000);
pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data, FIRST_AREA);
    println!("1. {first} hails are crossing paths in the test area");
    let second = second_part(&data)?;
    println!("2. The rock start coordinates will sum to {second}");

    Ok(())
}

fn first_part(hails: &[Hail<i64>], (min, max): (i64, i64)) -> usize {
    fn equation(hail: &Hail<i64>) -> (f64, f64, f64) {
        let a = -hail.speed.y;
        let b = hail.speed.x;
        let c = -hail.start.x * a - hail.start.y * b;
        (a as f64, b as f64, c as f64)
    }

    fn is_in_future(hail: &Hail<i64>, x: f64, y: f64) -> bool {
        (x - hail.start.x as f64).signum() as i64 == hail.speed.x.signum()
            && (y - hail.start.y as f64).signum() as i64 == hail.speed.y.signum()
    }

    let min = min as f64;
    let max = max as f64;
    let is_in_range = |v: f64| min <= v && v <= max;
    hails
        .iter()
        .tuple_combinations::<(_, _)>()
        .filter(|(a, b)| {
            let (a1, b1, c1) = equation(a);
            let (a2, b2, c2) = equation(b);
            let quotient = a1 * b2 - a2 * b1;
            if quotient.abs() < 0.000001 {
                return false; // parallel lines
            }
            let x = (b1 * c2 - b2 * c1) / quotient;
            let y = (c1 * a2 - c2 * a1) / quotient;
            is_in_range(x) && is_in_range(y) && is_in_future(a, x, y) && is_in_future(b, x, y)
        })
        .count()
}

fn second_part(hails: &[Hail<i64>]) -> Result<BigInt> {
    fn find_common_plane(a: &Hail<BigInt>, b: &Hail<BigInt>) -> (Point3D<BigInt>, BigInt) {
        let p = a.start.subtract(&b.start);
        let v = a.speed.subtract(&b.speed);
        let cross_v = a.speed.cross(&b.speed);
        (p.cross(&v), p.dot(&cross_v))
    }

    // Find three hails with linearly independent velocities
    let (h1, h2, h3) = hails
        .iter()
        .tuple_combinations::<(_, _, _)>()
        .find(|(a, b, c)| a.speed.determinant(&b.speed, &c.speed) == 0)
        .map(|(a, b, c)| (a.to_bigint(), b.to_bigint(), c.to_bigint()))
        .wrap_err("could not find independent hails")?;

    // Find the plane of possible rock velocity vectors that can result in hitting both hails
    let a = find_common_plane(&h1, &h2);
    let b = find_common_plane(&h2, &h3);
    let c = find_common_plane(&h3, &h1);

    // Intersecting the planes to get the rock velocity
    let v = {
        let det = a.0.determinant(&b.0, &c.0);
        let first = b.0.cross(&c.0).multiply(&a.1);
        let second = c.0.cross(&a.0).multiply(&b.1);
        let third = a.0.cross(&b.0).multiply(&c.1);
        first.add(&second).add(&third).divide(&det)
    };

    let v1 = h1.speed.subtract(&v);
    let v2 = h2.speed.subtract(&v);
    let cross_v = v1.cross(&v2);
    let e = cross_v.determinant(&h2.start, &v2);
    let f = cross_v.determinant(&h1.start, &v1);
    let g = h1.start.dot(&cross_v);
    let s = cross_v.dot(&cross_v);
    let rock = v1
        .multiply(&e)
        .subtract(&v2.multiply(&f))
        .add(&cross_v.multiply(&g))
        .divide(&s);

    Ok(rock.x + rock.y + rock.z)
}

#[derive(Debug)]
struct Hail<Int> {
    start: Point3D<Int>,
    speed: Point3D<Int>,
}

#[derive(Debug, Clone)]
struct Point3D<Int> {
    x: Int,
    y: Int,
    z: Int,
}

impl Hail<i64> {
    fn to_bigint(&self) -> Hail<BigInt> {
        Hail {
            start: self.start.to_bigint(),
            speed: self.speed.to_bigint(),
        }
    }
}

impl Point3D<i64> {
    fn to_bigint(&self) -> Point3D<BigInt> {
        Point3D {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into(),
        }
    }

    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn dot(&self, other: &Self) -> i64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn determinant(&self, b: &Self, c: &Self) -> i64 {
        self.dot(&b.cross(c))
    }
}

impl Point3D<BigInt> {
    fn add(&self, other: &Self) -> Self {
        Self {
            x: &self.x + &other.x,
            y: &self.y + &other.y,
            z: &self.z + &other.z,
        }
    }
    fn subtract(&self, other: &Self) -> Self {
        Self {
            x: &self.x - &other.x,
            y: &self.y - &other.y,
            z: &self.z - &other.z,
        }
    }

    fn multiply(&self, factor: &BigInt) -> Self {
        Self {
            x: &self.x * factor,
            y: &self.y * factor,
            z: &self.z * factor,
        }
    }

    fn divide(&self, factor: &BigInt) -> Self {
        Self {
            x: &self.x / factor,
            y: &self.y / factor,
            z: &self.z / factor,
        }
    }

    fn cross(&self, other: &Self) -> Self {
        Self {
            x: &self.y * &other.z - &self.z * &other.y,
            y: &self.z * &other.x - &self.x * &other.z,
            z: &self.x * &other.y - &self.y * &other.x,
        }
    }

    fn dot(&self, other: &Self) -> BigInt {
        &self.x * &other.x + &self.y * &other.y + &self.z * &other.z
    }

    fn determinant(&self, b: &Self, c: &Self) -> BigInt {
        self.dot(&b.cross(c))
    }
}

impl std::str::FromStr for Hail<i64> {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (start, speed) = s.split_once('@').wrap_err("expected Point @ Speed")?;
        start
            .parse::<Point3D<i64>>()
            .and_then(|start| {
                Ok(Hail {
                    start,
                    speed: speed.parse()?,
                })
            })
            .wrap_err_with(|| format!("for {s:?}"))
    }
}

impl std::str::FromStr for Point3D<i64> {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(3, ',').map(|p| p.trim().parse::<i64>());
        parts
            .next()
            .and_then(|x| {
                let y = parts.next()?;
                let z = parts.next()?;
                Some(x.and_then(|x| Ok(Point3D { x, y: y?, z: z? })))
            })
            .wrap_err("expected X, Y, Z")
            .and_then(|res| res.wrap_err("coordinates are not valid ints"))
            .wrap_err_with(|| format!("for {s:?}"))
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Hail<i64>>> {
    s.parse::<LineSep<Hail<i64>>>().map(|lines| lines.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/24.txt");
    const MAIN: &str = include_str!("../inputs/24.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data, (7, 27)), 2);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data, FIRST_AREA), 20_434);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), BigInt::from(47));
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(
            second_part(&data).unwrap(),
            BigInt::from(1_025_127_405_449_117i64)
        );
    }
}
