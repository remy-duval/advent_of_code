use std::collections::VecDeque;
use std::hash::{BuildHasher, Hash, Hasher};
use std::str::FromStr;

use color_eyre::eyre::{eyre, Report, Result, WrapErr};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use commons::parse::sep_by_empty_lines;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Game;
    const TITLE: &'static str = "Day 22: Crab Combat";

    fn solve(mut game: Self::Input) -> Result<()> {
        println!("Normal winner score is {}", game.clone().normal_play());
        println!("Recursive winner score: {}", game.advanced_play());

        Ok(())
    }
}

/// The number type used to represents the game cards (u8 should be enough, cards are < 50
type Card = u8;

/// The state of the decks of a game
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Game {
    first_player: VecDeque<Card>,
    second_player: VecDeque<Card>,
}

impl Game {
    /// Play a normal game (no recursion)
    pub fn normal_play(&mut self) -> usize {
        while !self.is_done() {
            if let Some((first, second)) = self.draw() {
                if first > second {
                    self.on_first_win(first, second);
                } else {
                    self.on_second_win(first, second);
                }
            }
        }

        self.score()
    }

    /// Play an advanced game (recursion enabled)
    pub fn advanced_play(&mut self) -> usize {
        self.play_recursively(&mut HashMap::with_capacity(100));
        self.score()
    }

    /// Play a recursive round of the game
    ///
    /// ### Returns
    /// true if the first player won, false if the second player won
    fn play_recursively(&mut self, known_games: &mut HashMap<u64, bool>) -> bool {
        // Create a hash to store instead of the full game (this will spare some memory)
        // At the cost of some hash collisions, but they should be rare enough
        // And since a loop has many rounds, the potential collisions should not break anything
        fn hashed(game: &Game, builder: &impl BuildHasher) -> u64 {
            let mut hasher = builder.build_hasher();
            game.hash(&mut hasher);
            hasher.finish()
        }

        let memoized = hashed(self, known_games.hasher());
        if let Some(known) = known_games.get(&memoized).copied() {
            known
        } else {
            // Some game will last 100s of rounds, but a lot end before round 5
            let mut played_turns = HashSet::with_capacity(5);
            while !self.is_done() {
                // Check if a round already happened. If it did make the P1 win
                if !played_turns.insert(hashed(self, played_turns.hasher())) {
                    self.second_player.clear(); // Should be enough to force P1 win
                } else if let Some((first, second)) = self.draw() {
                    let win = self.recursive_deck(first, second).map_or_else(
                        || first > second,
                        |mut inner| inner.play_recursively(known_games),
                    );

                    if win {
                        self.on_first_win(first, second);
                    } else {
                        self.on_second_win(first, second);
                    }
                }
            }

            let first_player_won = self.second_player.is_empty();
            known_games.insert(memoized, first_player_won);
            first_player_won
        }
    }

    /// Build a recursive deck by copying the given amount of card from the current deck
    fn recursive_deck(&self, first: Card, second: Card) -> Option<Self> {
        let first = first as usize;
        let second = second as usize;
        if self.first_player.len() >= first && self.second_player.len() >= second {
            Some(Self {
                first_player: self.first_player.iter().copied().take(first).collect(),
                second_player: self.second_player.iter().copied().take(second).collect(),
            })
        } else {
            None
        }
    }

    /// Draw a card from each player deck
    fn draw(&mut self) -> Option<(Card, Card)> {
        Some((
            self.first_player.pop_front()?,
            self.second_player.pop_front()?,
        ))
    }

    /// True if the current game is done
    fn is_done(&self) -> bool {
        self.first_player.is_empty() || self.second_player.is_empty()
    }

    /// Push the cards in the correct order to the bottom of first player's deck
    fn on_first_win(&mut self, first: Card, second: Card) {
        self.first_player.push_back(first);
        self.first_player.push_back(second);
    }

    /// Push the cards in the correct order to the bottom of second player's deck
    fn on_second_win(&mut self, first: Card, second: Card) {
        self.second_player.push_back(second);
        self.second_player.push_back(first);
    }

    /// Compute the score of a deck of card
    fn score(&self) -> usize {
        let cards = if self.first_player.is_empty() {
            &self.second_player
        } else {
            &self.first_player
        };

        cards
            .iter()
            .map(|&card| card as usize)
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, card)| acc + (idx + 1) * card)
    }
}

impl FromStr for Game {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_player, second_player) = sep_by_empty_lines(s)
            .flat_map(|blk| blk.split_terminator("\n\n"))
            .map(|block| {
                block
                    .lines()
                    .skip(1)
                    .map(|line| {
                        line.trim().parse::<Card>().wrap_err_with(|| {
                            format!("Could not parse a card as an integer {}", line)
                        })
                    })
                    .collect::<Result<VecDeque<Card>, _>>()
            })
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| eyre!("Missing a player section in {}", s))?;

        let first_player = first_player?;
        let second_player: VecDeque<Card> = second_player?;

        Ok(Self {
            first_player,
            second_player,
        })
    }
}

#[cfg(test)]
mod tests;
