use std::str::FromStr;

use crate::parse::LineSep;
use crate::Problem;

const DECK: i128 = 119_315_717_514_047;
const REPEAT: i128 = 101_741_582_076_661;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<Shuffle>;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 22: Slam Shuffle";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
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

/// The modulo operation which returns only positive numbers
/// The remainder (%) operation of Rust is not entirely like a modulo
/// If a is negative, then a % b is negative, whereas for modulo we need it positive.
/// This functions dirty fixes that issue by re-implementing the modulo for i128.
fn modulo(a: i128, m: i128) -> i128 {
    if a < 0 {
        a % m + m
    } else {
        a % m
    }
}

/// Invert a number in the modulo space of Z/size.
/// See https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
fn modular_inverse(to_inverse: i128, size: i128) -> i128 {
    let (mut inverse, mut temp_inv) = (0, 1);
    let (mut remainder, mut temp_rem) = (size, to_inverse);

    while temp_rem > 0 {
        let quotient = remainder / temp_rem;
        let tmp = inverse - quotient * temp_inv;
        inverse = temp_inv;
        temp_inv = tmp;

        let tmp = remainder % temp_rem;
        remainder = temp_rem;
        temp_rem = tmp;
    }

    assert_eq!(
        remainder, 1,
        "{} is not invertible in mod {}",
        to_inverse, size
    );
    if inverse < 0 {
        inverse + size
    } else {
        inverse
    }
}

/// Represents a Shuffle operations for the problem.
#[derive(Debug, Copy, Clone)]
pub enum Shuffle {
    NewStack,
    Cut(i32),
    Deal(i32),
}

impl FromStr for Shuffle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("deal into new stack") {
            Ok(Shuffle::NewStack)
        } else if s.starts_with("cut") {
            if let Ok(int) = s.replace("cut", "").trim().parse() {
                Ok(Shuffle::Cut(int))
            } else {
                Err("Could not parse the cut number")
            }
        } else if s.starts_with("deal with increment") {
            if let Ok(int) = s.replace("deal with increment", "").trim().parse() {
                Ok(Shuffle::Deal(int))
            } else {
                Err("Could not parse the deal with increment number")
            }
        } else {
            Err("Could not parse the next shuffle")
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
    pub fn apply(&self, input: i128, size: i128) -> i128 {
        let &Self(a, b) = self;
        modulo(modulo(a * input, size) + b, size)
    }

    /// Compose this LCF with another one (with the given modulo)
    /// This is equivalent to chaining shuffling operations corresponding to the operands
    pub fn compose_with(self, rhs: Self, size: i128) -> Self {
        let Self(a, b) = self;
        let Self(c, d) = rhs;
        let first = modulo(a * c, size);
        let second = modulo(modulo(b * c, size) + d, size);

        Self(first, second)
    }

    /// Exponential for this LCF to the power of the given exponent.
    /// As the LCF composition is associative, repeating exponent times a shuffling
    /// Is equivalent to applying one time shuffling.pow(exponent)
    pub fn pow(self, exponent: i128, size: i128) -> Self {
        if exponent == 0 {
            return Self::default();
        }
        let mut x = self;
        let mut y = Self::default();
        let mut n = exponent;
        while n > 1 {
            if n % 2 == 0 {
                x = x.clone().compose_with(x, size);
                n /= 2;
            } else {
                y = x.clone().compose_with(y, size);
                x = x.clone().compose_with(x, size);
                n /= 2;
            }
        }

        x.compose_with(y, size)
    }

    /// Inverse of this LCF.
    /// This LCF will give the initial position of a card finishing at the input given
    pub fn inverse(self, size: i128) -> Self {
        let Self(a, b) = self;
        let a_inverse = modular_inverse(a, size);

        Self(a_inverse, modulo(-a_inverse * b, size))
    }

    /// Fold the given collection of LCF into one single LCF.
    /// This is used to fuse the whole shuffling into one linear function.
    pub fn fold<I, A>(shuffles: I, size: i128) -> LinearFunction
        where
            A: Into<Self>,
            I: IntoIterator<Item=A>,
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
mod tests {
    use super::*;

    const TEST_ONE: &str = include_str!("test_resources/day22_1.txt");
    const TEST_TWO: &str = include_str!("test_resources/day22_2.txt");
    const TEST_THREE: &str = include_str!("test_resources/day22_3.txt");
    const TEST_FOUR: &str = include_str!("test_resources/day22_4.txt");
    const DATA: &str = include_str!("test_resources/day22_data.txt");

    #[test]
    fn small_deck_test() {
        // Test that the shuffling with LCF produces the expected order (as per the examples)
        fn assertion(data: &str, expected: [i128; 10]) {
            let shuffles = Day::parse(data).unwrap().data;
            let lcf = LinearFunction::fold(shuffles, 10);
            let mut result = [0; 10];
            for card in 0..10 {
                let pos = lcf.apply(card, 10);
                result[pos as usize] = card;
            }

            assert_eq!(
                &expected, &result,
                "Last position was expected to be {:?} instead of {:?}",
                expected, result
            );
        }

        assertion(TEST_ONE, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
        assertion(TEST_TWO, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
        assertion(TEST_THREE, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
        assertion(TEST_FOUR, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn inverse_test() {
        // Test that the inverse function produced gives back [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        // When applied to the last positions of the shuffling (as per the examples)
        fn assertion(data: &str, last_position: [i128; 10]) {
            let shuffles = Day::parse(data).unwrap().data;
            let lcf = LinearFunction::fold(shuffles, 10).inverse(10);
            let mut result = [0; 10];
            for (card, &last) in last_position.iter().enumerate() {
                let pos = lcf.apply(card as i128, 10);
                result[pos as usize] = last;
            }

            assert_eq!(
                &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                &result,
                "First position was expected to be {:?} instead of {:?}",
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                result
            );
        }

        assertion(TEST_ONE, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
        assertion(TEST_TWO, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
        assertion(TEST_THREE, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
        assertion(TEST_FOUR, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn first_part_test() {
        let shuffles = Day::parse(DATA).unwrap().data;
        assert_eq!(1_879, first_part(shuffles))
    }

    #[test]
    fn second_part_test() {
        let shuffles = Day::parse(DATA).unwrap().data;
        assert_eq!(73_729_306_030_290, second_part(shuffles))
    }
}
