use std::str::FromStr;

use itertools::Itertools;

use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Keys;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 25: Combo Breaker";

    fn solve(keys: Self::Input) -> Result<(), Self::Err> {
        println!("The encryption key is {}", solve(keys));
        Ok(())
    }
}

/// The type of integer we use for this problem
type Key = u64;

/// The n we are using
const N: Key = 20201227;

/// The subject for calculating the public key
const SUBJECT: Key = 7;

/// The two public keys of the card and door
#[derive(Debug, Copy, Clone)]
pub struct Keys {
    card: Key,
    door: Key,
}

/// Brute-forcing either card or door secret loop size (whichever comes first) to compute key
fn solve(keys: Keys) -> Key {
    let mut acc = 1;
    let mut secret = 0;
    loop {
        acc = (acc * SUBJECT) % N;
        secret += 1;

        if acc == keys.card {
            break modular_exponentiation(keys.door, secret, N);
        } else if acc == keys.door {
            break modular_exponentiation(keys.card, secret, N);
        }
    }
}

/// Fast modular exponentiation `a ^ n mod modulo`
fn modular_exponentiation(mut a: Key, mut n: Key, modulo: Key) -> Key {
    if n == 0 {
        1
    } else {
        let mut rest = 1;
        while n > 1 {
            if n & 1 != 0 {
                rest = (a * rest) % modulo;
            }
            a = (a * a) % modulo;
            n >>= 1; // divide by 2 in place
        }

        (a * rest) % modulo
    }
}

/// An error that can happen when parsing the keys
#[derive(Debug, thiserror::Error)]
pub enum ParseKeysError {
    #[error("Could not parse the key '{0}' ({1})")]
    ParseIntError(Box<str>, #[source] std::num::ParseIntError),
    #[error("Wanted exactly two keys, one each line")]
    NotExactlyTwoKeys,
}

impl FromStr for Keys {
    type Err = ParseKeysError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((card, door)) = itertools::process_results(
            s.lines().map(|line| {
                line.trim()
                    .parse::<Key>()
                    .map_err(|e| ParseKeysError::ParseIntError(line.into(), e))
            }),
            |result| result.collect_tuple::<(_, _)>(),
        )? {
            Ok(Self { card, door })
        } else {
            Err(ParseKeysError::NotExactlyTwoKeys)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("test_resources/25-example.txt");
    const MAIN: &str = include_str!("test_resources/25-main.txt");

    #[test]
    fn example() {
        let encryption = solve(Day::parse(EXAMPLE).unwrap());
        assert_eq!(encryption, 14_897_079);
    }

    #[test]
    fn main() {
        let encryption = solve(Day::parse(MAIN).unwrap());
        assert_eq!(encryption, 297_257);
    }
}
