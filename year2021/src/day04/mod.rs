use commons::eyre::{eyre, Report, Result, WrapErr};
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Bingo;
    const TITLE: &'static str = "Day 4: Giant Squid";

    fn solve(data: Self::Input) -> Result<()> {
        println!("1. First win score is {}", data.first_win_score()?);
        println!("1. Last win score is {}", data.last_win_score()?);

        Ok(())
    }
}

/// All the bingo boards and the draw order for the puzzle
pub struct Bingo {
    draws: Vec<u8>,
    boards: Vec<Board>,
}

impl Bingo {
    /// Find the score of the first board to win in the draw
    fn first_win_score(&self) -> Result<u32> {
        let mut completions = vec![BoardCompletion::default(); self.boards.len()];
        let first = self.draws.iter().find_map(|&draw| {
            self.boards
                .iter()
                .zip(completions.iter_mut())
                .find_map(|(board, completion)| {
                    if board.fill_with(draw, completion) && completion.is_complete() {
                        Some(board.score(draw, completion))
                    } else {
                        None
                    }
                })
        });

        first.ok_or_else(|| eyre!("No board has won after all draws !"))
    }

    /// Find the score of the last board to win in the draw
    fn last_win_score(&self) -> Result<u32> {
        // Here non complete boards are Some(board, completion), and completed are turned to None
        // This makes it so we never complete a board twice
        let mut boards: Vec<_> = self
            .boards
            .iter()
            .map(|b| (b, BoardCompletion::default()))
            .collect();

        let mut last = None;
        self.draws.iter().for_each(|&draw| {
            let mut i = 0;
            while i < boards.len() {
                let (board, completion) = &mut boards[i];
                if board.fill_with(draw, completion) && completion.is_complete() {
                    last = Some(board.score(draw, completion));
                    boards.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        });

        last.ok_or_else(|| eyre!("No board has won after all draws !"))
    }
}

/// A bingo board for the puzzle
struct Board([[u8; Board::COLUMNS]; Board::COLUMNS]);

impl Board {
    /// The number of rows of the board
    const ROWS: usize = 5;

    /// The number of columns of the board
    const COLUMNS: usize = 5;

    /// Fill in the board with the given number, returns true if a number was filled in
    fn fill_with(&self, number: u8, completion: &mut BoardCompletion) -> bool {
        (0..Self::ROWS).any(|y| {
            (0..Self::COLUMNS).any(|x| {
                if self.0[y][x] == number {
                    completion.fill(x, y);
                    true
                } else {
                    false
                }
            })
        })
    }

    /// Return the score of this board (should only be called if it is complete)
    fn score(&self, last: u8, completion: &BoardCompletion) -> u32 {
        let unmarked: u32 = (0..Self::ROWS)
            .flat_map(|y| {
                (0..Self::ROWS)
                    .filter(move |&x| !completion.is_filled(x, y))
                    .map(move |x| self.0[y][x] as u32)
            })
            .sum();

        unmarked * last as u32
    }
}

/// A completion sheet for a bingo board, implemented as a bitset
#[derive(Clone, Debug, Default)]
struct BoardCompletion(u32);

impl BoardCompletion {
    /// Fill the given position on the board
    fn fill(&mut self, x: usize, y: usize) {
        let mask = 1 << (Board::ROWS * y + x);
        self.0 |= mask;
    }

    /// Return true if the given position is filled
    fn is_filled(&self, x: usize, y: usize) -> bool {
        let mask = 1 << (Board::ROWS * y + x);
        (self.0 & mask) != 0
    }

    /// Check if the related board is complete
    /// The board is complete if any row or column is fully complete (diagonals don't count)
    fn is_complete(&self) -> bool {
        (0..Board::ROWS).any(|y| (0..Board::COLUMNS).all(|x| self.is_filled(x, y)))
            || (0..Board::COLUMNS).any(|x| (0..Board::ROWS).all(|y| self.is_filled(x, y)))
    }
}

impl std::str::FromStr for Bingo {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut blocks = commons::parse::sep_by_empty_lines(s);

        if let Some(draws) = blocks.next() {
            let draws: Vec<u8> = draws
                .split(',')
                .map(|n| n.trim().parse())
                .collect::<core::result::Result<_, _>>()
                .wrap_err_with(|| format!("Bad draws: {}", draws))?;
            let boards: Vec<Board> = blocks
                .map(|b| b.parse().wrap_err_with(|| format!("Bad bingo: {}", b)))
                .collect::<Result<_>>()?;
            Ok(Bingo { draws, boards })
        } else {
            Err(eyre!("Missing draws"))
        }
    }
}

impl std::str::FromStr for Board {
    type Err = core::num::ParseIntError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut board = [[0u8; Board::COLUMNS]; Board::ROWS];
        for (y, line) in s.lines().take(Board::ROWS).enumerate() {
            for (x, num) in line.split_whitespace().take(Board::COLUMNS).enumerate() {
                board[y][x] = num.parse()?;
            }
        }
        Ok(Board(board))
    }
}

#[cfg(test)]
mod tests;
