use itertools::Itertools;

use commons::{Result, WrapErr};

pub const TITLE: &str = "Day 21: Dirac Dice";

pub fn run(raw: String) -> Result<()> {
    let state = parse(&raw)?;
    println!("1. Deterministic score: {}", first_part(state));
    println!("2. Dirac wins: {}", second_part(state));

    Ok(())
}

/// Parse the initial position of the two players
fn parse(s: &str) -> Result<[u8; 2]> {
    let (a, b) = s
        .lines()
        .filter_map(|line| line.trim().split_once("position:"))
        .map(|(_, pos)| pos.trim().parse().map(|i: u8| i - 1))
        .collect_tuple::<(_, _)>()
        .wrap_err_with(|| format!("Expected 2 players in '{s}'"))?;

    Ok([a?, b?])
}

/// Play the game using the dice that cycles from 1 to 100;
fn first_part([a, b]: [u8; 2]) -> usize {
    let mut state = [(a as u16, 0u16), (b as u16, 0u16)];
    let mut turn = 0;
    let mut rolls = 0;
    let mut roll = {
        let mut current = 1;
        let rolls = &mut rolls;
        let mut roll = move || {
            *rolls += 1;
            if current == 100 {
                current = 1;
                100
            } else {
                current += 1;
                current - 1
            }
        };

        move || roll() + roll() + roll()
    };

    loop {
        let (pos, score) = &mut state[turn];
        *pos = (*pos + roll()) % 10;
        *score += *pos + 1;
        turn = usize::from(turn == 0);
        if *score >= 1000 {
            return rolls * state[turn].1 as usize;
        }
    }
}

/// The universes created by summing three dice rolls (from 3 to 9)
/// The index is the dice sum - 3
const ROLLS: [i64; 7] = {
    let mut result = [0; 7];
    let mut i = 0;
    while i < 3 {
        let mut j = 0;
        while j < 3 {
            let mut k = 0;
            while k < 3 {
                result[i + j + k] += 1;
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    result
};

/// Find the maximum count of possible universes in which one of the player wins
fn second_part([a, b]: [u8; 2]) -> i64 {
    // Iterates the possible next states (and their occurrence count) from the given one
    fn next_states((pos, score): (u8, u8)) -> impl Iterator<Item = (i64, (u8, u8))> {
        ROLLS.into_iter().enumerate().map(move |(roll, n)| {
            let pos = (pos + (roll + 3) as u8) % 10;
            let score = score + pos + 1;
            (n, (pos, score))
        })
    }

    // Count the number of possible wins from the given state, cached
    fn wins_from(state: [(u8, u8); 2], cache: &mut [[i64; 2]]) -> [i64; 2] {
        // Convert the state to an index in the cache
        let key = {
            let first = state[0].1 as usize * 10 + state[0].0 as usize;
            let second = state[1].1 as usize * 10 + state[1].0 as usize;
            first * 210 + second
        };

        let mut wins = cache[key];
        if wins[0] >= 0 && wins[1] >= 0 {
            return wins;
        }

        wins[0] = 0;
        wins[1] = 0;
        next_states(state[0]).for_each(|(count, one)| {
            if one.1 >= 21 {
                wins[0] += count;
                return;
            }

            next_states(state[1]).for_each(|(n, two)| {
                let count = count * n;
                if two.1 >= 21 {
                    wins[1] += count;
                    return;
                }

                wins_from([one, two], cache)
                    .into_iter()
                    .zip(&mut wins)
                    .for_each(|(wins, total)| *total += count * wins);
            });
        });

        cache[key] = wins;
        wins
    }

    // The cache is a huge pre-allocated Vec, which is weirdly enough a lot faster than a HashMap
    let mut cache = vec![[-1, -1]; 10 * 10 * 21 * 21];
    let result = wins_from([(a, 0), (b, 0)], &mut cache);
    result[0].max(result[1])
}

#[cfg(test)]
mod tests;
