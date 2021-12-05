use std::str::FromStr;

use commons::eyre::{eyre, Report, Result, WrapErr};
use itertools::Itertools;

use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Keys;
    const TITLE: &'static str = "Day 25: Combo Breaker";

    fn solve(keys: Self::Input) -> Result<()> {
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

impl FromStr for Keys {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((card, door)) = itertools::process_results(
            s.lines().map(|line| {
                line.trim()
                    .parse::<Key>()
                    .wrap_err_with(|| format!("Could not parse the key '{}'", line))
            }),
            |result| result.collect_tuple::<(_, _)>(),
        )? {
            Ok(Self { card, door })
        } else {
            Err(eyre!("Wanted exactly two keys, one each line"))
        }
    }
}

#[cfg(test)]
mod tests;
