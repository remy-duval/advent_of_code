use std::str::FromStr;

use hashbrown::HashSet;

use commons::parse::LineSep;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<Operation>;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 8: Handheld Halting";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let mut state = ProgramState::new(data.data);
        let (cause, accumulator) = run_until_duplicate_execution(&mut state);

        println!(
            "A loop was detected (caused by {pos}), the accumulator was at {acc}",
            pos = cause,
            acc = accumulator
        );

        // Part 2
        let (last, accumulator) = replace_and_run(&mut state);
        println!(
            "Ran program until {pos}/{max}, the accumulator was at {acc}",
            pos = last,
            max = state.operations.len(),
            acc = accumulator
        );

        Ok(())
    }
}

fn run_until_duplicate_execution(state: &mut ProgramState) -> (usize, i32) {
    let mut current_pointer = state.instruction_pointer;
    let mut visited = HashSet::new();
    visited.insert(current_pointer);
    while state.execute_next() && !visited.contains(&state.instruction_pointer) {
        visited.insert(current_pointer);
        current_pointer = state.instruction_pointer;
    }

    (current_pointer, state.accumulator)
}

fn replace_and_run(state: &mut ProgramState) -> (usize, i32) {
    let mut acc = i32::MIN;
    for idx in 0..state.operations.len() {
        state.clear();
        let previous = state.operations[idx];
        match previous {
            Operation::Noop(value) => state.operations[idx] = Operation::Jmp(value),
            Operation::Jmp(value) => state.operations[idx] = Operation::Noop(value),
            _ => continue,
        }

        acc = run_until_duplicate_execution(state).1;
        state.operations[idx] = previous;
        if state.instruction_pointer == state.operations.len() {
            break;
        }
    }

    (state.instruction_pointer, acc)
}

/// The state of the program
#[derive(Debug, Clone, Default)]
pub struct ProgramState {
    operations: Vec<Operation>,
    /// The current instruction pointer
    instruction_pointer: usize,
    /// The current accumulator
    accumulator: i32,
}

impl ProgramState {
    /// Create a new program state from the given instructions
    fn new(operations: Vec<Operation>) -> Self {
        Self {
            operations,
            ..Self::default()
        }
    }

    /// Reset the program state
    fn clear(&mut self) {
        self.instruction_pointer = 0;
        self.accumulator = 0;
    }

    /// Execute the next operation in this program
    ///
    /// ### Returns
    /// false if the instruction pointer is past the end of the operations to execute
    fn execute_next(&mut self) -> bool {
        if let Some(current) = self.operations.get(self.instruction_pointer) {
            current.execute(self);
            true
        } else {
            false
        }
    }
}

/// An operation to execute in the system
#[derive(Debug, Copy, Clone)]
pub enum Operation {
    /// Increase the global accumulator by this value, then advance the instruction pointer
    Acc(i32),
    /// Jump to the instruction located relative to itself by its argument
    /// +n means next nth instruction
    /// -n means previous nth instruction
    Jmp(i32),
    /// Do nothing, then advance the instruction pointer
    Noop(i32),
}

impl Operation {
    /// Execute an operation on the given state
    ///
    /// ### Arguments
    /// * `state` - The current state of the program that this instruction should modify
    fn execute(self, state: &mut ProgramState) {
        match self {
            Self::Acc(inc) => {
                state.accumulator += inc;
                state.instruction_pointer += 1;
            }
            Self::Jmp(jump) => {
                if jump < 0 {
                    state.instruction_pointer -= (-jump) as usize
                } else {
                    state.instruction_pointer += jump as usize;
                }
            }
            Self::Noop(_) => {
                state.instruction_pointer += 1;
            }
        }
    }
}

/// An error that could happen during an operation parsing
#[derive(Debug, thiserror::Error)]
pub enum ParseOpError {
    #[error("The operation is not formatted correctly '{0}'")]
    BadFormat(String),
    #[error("Unknown operation name '{0}'")]
    UnknownOperation(String),
    #[error("Could not parse the argument in '{0}' (error is {1})")]
    ArgumentParseError(String, std::num::ParseIntError),
}

impl FromStr for Operation {
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_arg(arg: &str) -> Result<i32, ParseOpError> {
            arg.parse()
                .map_err(|e| ParseOpError::ArgumentParseError(arg.to_owned(), e))
        }

        if let Some((op, arg)) = s.split_once(' ') {
            match op {
                "acc" => Ok(Operation::Acc(parse_arg(arg)?)),
                "jmp" => Ok(Operation::Jmp(parse_arg(arg)?)),
                "nop" => Ok(Operation::Noop(parse_arg(arg)?)),
                _ => Err(ParseOpError::UnknownOperation(op.to_owned())),
            }
        } else {
            Err(ParseOpError::BadFormat(s.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests;
