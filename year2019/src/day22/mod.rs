use std::str::FromStr;

use commons::eyre::{eyre, Report, Result};
use commons::num::integer::{mod_floor, ExtendedGcd, Integer};
use commons::parse::LineSep;

pub const TITLE: &str = "Day 22: Slam Shuffle";
const DECK: i128 = 119_315_717_514_047;
const REPEAT: i128 = 101_741_582_076_661;

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!(
        "The final position of the 2019th card is {}",
        first_part(data.data.clone())
    );

    println!(
        "The initial position of the 2020th card was {}",
        second_part(data.data)
    );

    Ok(())
}

fn parse(s: &str) -> Result<LineSep<Shuffle>> {
    s.parse()
}

fn first_part(shuffles: Vec<Shuffle>) -> i128 {
    LinearFunction::fold(shuffles, 10_007).apply(2019, 10_007)
}

fn second_part(shuffles: Vec<Shuffle>) -> i128 {
    LinearFunction::fold(shuffles, DECK)
        .pow(REPEAT, DECK)
        .inverse(DECK)
        .apply(2020, DECK)
}

/// Invert a number in the modulo space of Z/size using the extended euclidean algorithm
/// See https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
fn modular_inverse(a: i128, n: i128) -> i128 {
    let ExtendedGcd { gcd, x, .. } = a.extended_gcd(&n);
    assert_eq!(gcd, 1, "{a} is not invertible in mod {n}");
    x
}

/// Represents a Shuffle operations for the problem.
#[derive(Debug, Copy, Clone)]
enum Shuffle {
    NewStack,
    Cut(i32),
    Deal(i32),
}

impl FromStr for Shuffle {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with("deal into new stack") {
            Ok(Shuffle::NewStack)
        } else if s.starts_with("cut") {
            if let Ok(int) = s.replace("cut", "").trim().parse() {
                Ok(Shuffle::Cut(int))
            } else {
                Err(eyre!("Could not parse the cut number"))
            }
        } else if s.starts_with("deal with increment") {
            if let Ok(int) = s.replace("deal with increment", "").trim().parse() {
                Ok(Shuffle::Deal(int))
            } else {
                Err(eyre!("Could not parse the deal with increment number"))
            }
        } else {
            Err(eyre!("Could not parse the next shuffle"))
        }
    }
}

/// Represents a Linear Congruential Function for the modulo operations we need for this problem.
/// The reasoning is gleaned from https://codeforces.com/blog/entry/72593
/// Essentially :
/// - Shuffling operations are equivalent to modulo functions f(x) = ax + b mod size
///     - new stack is -x - 1 mod size
///     - cut(n) is x - n mod size
///     - deal(n) is nx + 0 mod size
/// - Those shuffling operations as easily represented as tuples (a, b)
/// - Those shuffling operations can be composed : f o g (x) is the first shuffle then second
///     - composition of f(x) = ax + b mod size and g(x) = cx + d mod size
///     gives a f o g (x) = (ac mod size)x + (bc + d mod size) mod size
///     - composition is forms a multiplicative monoid for the operations
///         - identity is id(x) = 1*x + 0 mod size
///         - the composition is associative, so we can introduce the pow function for repeating
///         n times the same shuffling
/// - Those shuffling operations are invertible if their a scalar is
///     - The inverse of a scalar is found using the extended euclidian algorithm  
///     - The inverse of the function f(x) = ax + b mod size is f-1(x) = (x - b) * a-1 mod size
///
/// First part was done before just chaining all shuffling on a single position,
/// but it was transitioned to the LCF method to test it first.
/// For the second part we can compose all shuffles into a single LCF and invert it.
///
/// As the numbers involved are quite huge we use 128 integers (long long) to represent everything
/// (The operations would possibly overflow 64 integers so this is necessary)
#[derive(Debug, Clone)]
struct LinearFunction(i128, i128);

impl Default for LinearFunction {
    fn default() -> Self {
        Self(1, 0)
    }
}

impl LinearFunction {
    /// Applies this LCF to the given input.
    /// This produces the position of the card after the shuffling.
    fn apply(&self, input: i128, size: i128) -> i128 {
        let &Self(a, b) = self;
        mod_floor(mod_floor(a * input, size) + b, size)
    }

    /// Compose this LCF with another one (with the given modulo)
    /// This is equivalent to chaining shuffling operations corresponding to the operands
    fn compose_with(self, rhs: Self, size: i128) -> Self {
        let Self(a, b) = self;
        let Self(c, d) = rhs;
        let first = mod_floor(a * c, size);
        let second = mod_floor(mod_floor(b * c, size) + d, size);

        Self(first, second)
    }

    /// Exponential for this LCF to the power of the given exponent.
    /// As the LCF composition is associative, repeating exponent times a shuffling
    /// Is equivalent to applying one time shuffling.pow(exponent)
    fn pow(self, exponent: i128, size: i128) -> Self {
        if exponent == 0 {
            return Self::default();
        }
        let mut x = self;
        let mut y = Self::default();
        let mut n = exponent;
        while n > 1 {
            // Odd case
            if n & 1 != 0 {
                y = x.clone().compose_with(y, size);
            }
            x = x.clone().compose_with(x, size);
            n >>= 1; // Divide by 2
        }

        x.compose_with(y, size)
    }

    /// Inverse of this LCF.
    /// This LCF will give the initial position of a card finishing at the input given
    fn inverse(self, size: i128) -> Self {
        let Self(a, b) = self;
        let a_inverse = modular_inverse(a, size);

        Self(a_inverse, mod_floor(-a_inverse * b, size))
    }

    /// Fold the given collection of LCF into one single LCF.
    /// This is used to fuse the whole shuffling into one linear function.
    fn fold<I, A>(shuffles: I, size: i128) -> LinearFunction
    where
        A: Into<Self>,
        I: IntoIterator<Item = A>,
    {
        shuffles.into_iter().fold(Self::default(), |current, next| {
            current.compose_with(next.into(), size)
        })
    }
}

impl From<Shuffle> for LinearFunction {
    fn from(shuffle: Shuffle) -> Self {
        match shuffle {
            Shuffle::NewStack => Self(-1, -1),
            Shuffle::Cut(cut) => Self(1, -cut as i128),
            Shuffle::Deal(inc) => Self(inc as i128, 0),
        }
    }
}

#[cfg(test)]
mod tests;
