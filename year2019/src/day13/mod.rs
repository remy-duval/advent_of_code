use std::fmt::{Display, Formatter};

use itertools::Itertools;

use commons::Problem;
use commons::{CLEAR_COMMAND, TO_TOP};

use super::int_code::{IntCodeInput, Processor, Status};

/// The delay in milliseconds between printing each frame
const FRAME_DELAY: u64 = 0;

pub struct Day;

impl Problem for Day {
    type Input = IntCodeInput;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 13: Care Package";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        println!("{}", CLEAR_COMMAND);
        let mut memory = data.data;
        memory[0] = 2;
        let mut engine = Processor::new(&memory);
        let mut state = GameState::default();
        let (score, (remaining, total_blocks)) =
            state.run_with_decider(&mut engine, true, simple_decider);

        println!(
            "Final score: {} with {}/{} blocks remaining.",
            score, remaining, total_blocks
        );

        Ok(())
    }
}

fn simple_decider(state: &GameState) -> i64 {
    (state.ball.unwrap().0 as i64 - state.player.unwrap().0 as i64).signum()
}

#[derive(Debug, Default, Clone)]
struct GameState {
    score: i64,
    width: usize,
    height: usize,
    screen: Vec<Vec<Tile>>,
    player: Option<(usize, usize)>,
    ball: Option<(usize, usize)>,
    blocks: u64,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display = self
            .screen
            .iter()
            .map(|row| row.iter().map(|tile| tile.char()).join(""))
            .join("\n");
        write!(
            f,
            "Blocks: {blocks:04}\nScore: {score:05}\n{display}",
            score = self.score,
            blocks = self.blocks,
            display = display
        )
    }
}

impl GameState {
    /// Run the game until it halts.
    /// Inputs are provided by the decider closure which has access to the state.
    /// # Returns
    /// A tuple of (score, (remaining blocks, total blocks))
    pub fn run_with_decider<F>(
        &mut self,
        engine: &mut Processor,
        show: bool,
        decider: F,
    ) -> (i64, (u64, u64))
    where
        F: Fn(&Self) -> i64,
    {
        let mut status = self.run(engine, show);
        let blocks = self.blocks;
        loop {
            match status {
                Status::RequireInput => engine.write_int(decider(&self)),
                _ => return (self.score, (self.blocks, blocks)),
            }
            status = self.run(engine, false);
            if show {
                std::thread::sleep(std::time::Duration::from_millis(FRAME_DELAY));
                println!("{}{}", TO_TOP, &self);
            }
        }
    }

    /// Run the game until it blocks for any reason.
    pub fn run(&mut self, engine: &mut Processor, show: bool) -> Status {
        let mut outputs = [0; 3];
        loop {
            match engine.read_next_array(&mut outputs, 3) {
                (_, Some(status)) => return status,
                (read, _) if read == 3 => self.update((outputs[0], outputs[1], outputs[2])),
                _ => return Status::Halted,
            }
            if show {
                std::thread::sleep(std::time::Duration::from_millis(FRAME_DELAY));
                println!("{}{}", TO_TOP, &self);
            }
        }
    }

    /// Dispatch the output from the IntCode engine to update the game state.
    fn update(&mut self, values: (i64, i64, i64)) {
        match values {
            (-1, 0, new_score) => self.update_score(new_score),
            // Here we cast the x and y as usize, this is safe for this specific IntCode program
            (x, y, tile) => self.update_tile((x as usize, y as usize), tile),
        }
    }

    /// Update operation for the score of the game.
    fn update_score(&mut self, new_score: i64) {
        self.score = new_score;
    }

    /// Update operation for a single tile of the game screen.
    fn update_tile(&mut self, position: (usize, usize), tile: i64) {
        let tile: Tile = tile.into();

        // Fetch the data in screen, possibly resizing it to fit.
        if self.height <= position.1 {
            self.height = position.1 + 1;
            self.screen
                .resize(self.height, Vec::with_capacity(self.width));
        }

        // Now resized, this should work
        let row = &mut self.screen[position.1];

        // Then resize the row if necessary
        if row.len() <= position.0 {
            self.width = self.width.max(position.0 + 1);
            row.resize(self.width, Tile::Empty);
        }

        let original = row[position.0];
        row[position.0] = tile;

        // Updates the block count if the original or new one where blocks
        if original == Tile::Block && tile != Tile::Block {
            self.blocks -= 1;
        } else if original != Tile::Block && tile == Tile::Block {
            self.blocks += 1;
        }

        // Updates the player and ball position
        if tile == Tile::Player {
            self.player = Some(position);
        } else if tile == Tile::Ball {
            self.ball = Some(position);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Player,
    Ball,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Player,
            4 => Tile::Ball,
            _ => Tile::Empty,
        }
    }
}

impl Tile {
    /// The char representation of this tile
    pub fn char(self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => 'X',
            Tile::Player => '@',
            Tile::Ball => 'O',
        }
    }
}

#[cfg(test)]
mod tests;
