use hashbrown::HashMap;
use itertools::{iproduct, Itertools};

use commons::eyre::{eyre, Result};

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
        .ok_or_else(|| eyre!("Expected 2 players in '{}'", s))?;

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
        turn = if turn == 0 { 1 } else { 0 };
        if *score >= 1000 {
            return rolls * state[turn].1 as usize;
        }
    }
}

/// Play all the possible games with a dice of 1 to 3
fn second_part([a, b]: [u8; 2]) -> usize {
    // The universes created by summing three dice rolls (from 3 to 9)
    // The index is the dice sum - 3
    let rolls: [usize; 7] = {
        let mut result = [0; 7];
        for (i, j, k) in iproduct!(0..3, 0..3, 0..3) {
            result[i + j + k] += 1;
        }
        result
    };

    let mut universes = HashMap::with_capacity(4096);
    let mut swap = universes.clone();
    let mut wins = [0; 2];
    let mut turn = 0;
    universes.insert([(a, 0u8), (b, 0u8)], 1usize);

    while !universes.is_empty() {
        for (state, count) in universes.drain() {
            for (roll, &created) in rolls.iter().enumerate() {
                let mut state = state;
                let (pos, score) = &mut state[turn];
                *pos = (*pos + (roll + 3) as u8) % 10;
                *score += *pos + 1;
                if *score >= 21 {
                    wins[turn] += count * created;
                } else {
                    *swap.entry(state).or_default() += count * created;
                }
            }
        }
        std::mem::swap(&mut universes, &mut swap);
        turn = if turn == 1 { 0 } else { 1 };
    }

    wins[0].max(wins[1])
}

#[cfg(test)]
mod tests;
