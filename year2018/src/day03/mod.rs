use std::str::FromStr;

use itertools::Itertools;

use commons::parse::LineSep;
use commons::{err, Report, Result, WrapErr};

pub const TITLE: &str = "Day 3: No Matter How You Slice It";
const DIMENSION: usize = 1000;

pub fn run(raw: String) -> Result<()> {
    let tissue = Tissue::new(parse(&raw)?.data);

    println!(
        "{} squares of the tissue are claimed multiple times",
        tissue.multiple_claims()
    );

    println!(
        "The claim #{} is intact",
        tissue
            .find_intact_claim()
            .wrap_err("Could not find the intact claim on the tissue")?
            .id
    );

    Ok(())
}

fn parse(s: &str) -> Result<LineSep<Claim>> {
    s.parse()
}

struct Tissue {
    /// * 0 - Means that a square is not claimed
    /// * Negative number - Means that a square is claimed multiple times
    /// * Positive number - square is claimed by only this claim
    squares: Vec<[i16; DIMENSION]>,
    claims: Vec<Claim>,
}

impl Tissue {
    /// Build the tissue definition
    fn new(claims: Vec<Claim>) -> Self {
        let mut squares = vec![[0; DIMENSION]; DIMENSION];
        claims.iter().for_each(|claim| {
            let (left, top) = claim.top_left();
            let (right, bottom) = claim.bottom_right();
            assert!(right < DIMENSION);
            assert!(left < right);

            squares[top..bottom].iter_mut().for_each(|line| {
                line[left..right].iter_mut().for_each(|point| {
                    *point = if *point == 0 { claim.id } else { -1 };
                })
            });
        });

        Self { squares, claims }
    }

    /// Count the number of squares that are claimed multiple times (ie negative values)
    fn multiple_claims(&self) -> usize {
        self.squares
            .iter()
            .flatten()
            .filter(|&&from| from < 0)
            .count()
    }

    /// Find a claim that has all its squares belonging only to it
    fn find_intact_claim(&self) -> Option<&Claim> {
        self.claims.iter().find(|&claim| {
            let (left, top) = claim.top_left();
            let (right, bottom) = claim.bottom_right();
            assert!(right < DIMENSION);
            assert!(left < right);

            self.squares[top..bottom]
                .iter()
                .all(|line| line[left..right].iter().all(|&point| point == claim.id))
        })
    }
}

/// A claim from an elf on the tissue
struct Claim {
    id: i16,
    position: (i16, i16),
    size: (i16, i16),
}

impl Claim {
    /// The top left point of the claim
    pub fn top_left(&self) -> (usize, usize) {
        (self.position.0 as usize, self.position.1 as usize)
    }

    /// The bottom right point of the claim
    pub fn bottom_right(&self) -> (usize, usize) {
        (
            (self.position.0 + self.size.0) as usize,
            (self.position.1 + self.size.1) as usize,
        )
    }
}

impl FromStr for Claim {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        // Generate a bad format error
        fn bad_format(s: &str) -> Report {
            err!("Expected #<ID> @ <LEFT>,<TOP>: <WIDTH>x<HEIGHT> for claim, got {s}")
        }

        fn parse_int(s: &str) -> Result<i16> {
            s.parse()
                .wrap_err_with(|| format!("Could not parse a number in the claim {s}"))
        }

        fn parse_coordinates(s: &str, sep: char) -> Result<(i16, i16)> {
            itertools::process_results(s.splitn(2, sep).map(|s| parse_int(s.trim())), |iter| {
                iter.collect_tuple::<(_, _)>()
            })?
            .wrap_err_with(|| format!("Expected <FIRST>{s}<SECOND> for coordinates, got {sep}"))
        }

        let (id, claim) = s
            .strip_prefix('#')
            .ok_or_else(|| bad_format(s))?
            .splitn(2, '@')
            .map(|s| s.trim())
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| bad_format(s))?;

        let (position, size) = claim
            .splitn(2, ':')
            .map(str::trim)
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| bad_format(s))?;

        Ok(Self {
            id: parse_int(id)?,
            position: parse_coordinates(position, ',')?,
            size: parse_coordinates(size, 'x')?,
        })
    }
}

#[cfg(test)]
mod tests;
