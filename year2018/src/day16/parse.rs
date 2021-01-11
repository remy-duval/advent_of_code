use std::str::FromStr;

use hashbrown::HashMap;
use itertools::Itertools;

use commons::parse::sep_by_empty_lines;

use super::instructions::{IndexError, Int, OpCode};

/// The value of the registers at a certain point in time
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Register(pub [Int; 4]);

/// An instruction to execute on the registers
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Instruction {
    /// The opcode (to resolve)
    pub code: Int,
    /// First input
    pub a: Int,
    /// Second input
    pub b: Int,
    /// Address of the output register
    pub c: Int,
}

impl Instruction {
    /// Execute this instruction with the given starting registers
    pub fn execute(&self, registers: &mut Register, code: OpCode) -> Result<(), IndexError> {
        code.apply(&mut registers.0, self.a, self.b, self.c)
    }
}

/// A sample execution from the program to deduce which instruction is which
#[derive(Debug, Clone)]
pub struct Sample {
    pub before: Register,
    pub instruction: Instruction,
    pub after: Register,
}

/// The input to the program
#[derive(Debug, Clone)]
pub struct Program {
    pub samples: Vec<Sample>,
    pub program: Vec<Instruction>,
}

impl Program {
    /// Execute the full program with the given codes mappings
    pub fn execute(&self, codes: &HashMap<Int, OpCode>) -> Register {
        let mut register = Register([0; 4]);
        self.program.iter().for_each(|instruction| {
            if let Some(code) = codes.get(&instruction.code) {
                instruction
                    .execute(&mut register, *code)
                    .expect("Failed program after finding the op codes");
            }
        });

        register
    }
}

/// An error while parsing an integer
#[derive(Debug, thiserror::Error)]
#[error("Could not parse an integer {0} ({1})")]
pub struct ParseIntError(Box<str>, std::num::ParseIntError);

/// An error that happens while parsing a register description
#[derive(Debug, thiserror::Error)]
pub enum RegisterParseError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Expected [A, B, C, D] for a register, where A, B, C, D integers, got: {0}")]
    BadFormat(Box<str>),
}

/// An error that happens while parsing an instruction description
#[derive(Debug, thiserror::Error)]
pub enum InstructionParseError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Expected A, B, C, D for an instruction, where A, B, C, D integers, got: {0}")]
    BadFormat(Box<str>),
}

/// An error that happens while parsing a sample
#[derive(Debug, thiserror::Error)]
pub enum SampleParseError {
    #[error(transparent)]
    RegisterParseError(#[from] RegisterParseError),
    #[error(transparent)]
    InstructionParseError(#[from] InstructionParseError),
    #[error("Expected register instruction register for a sample, got: {0}")]
    BadFormat(Box<str>),
}

/// Parse an integer of type [Element](Element)
fn parse_element(str: &str) -> Result<Int, ParseIntError> {
    str.trim()
        .parse()
        .map_err(|err| ParseIntError(str.into(), err))
}

impl FromStr for Register {
    type Err = RegisterParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second, third, fourth) = s
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .and_then(|s| {
                s.split(',')
                    .map(parse_element)
                    .collect_tuple::<(_, _, _, _)>()
            })
            .ok_or_else(|| RegisterParseError::BadFormat(s.into()))?;

        Ok(Self([first?, second?, third?, fourth?]))
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second, third, fourth) = s
            .trim()
            .split_whitespace()
            .map(parse_element)
            .collect_tuple::<(_, _, _, _)>()
            .ok_or_else(|| InstructionParseError::BadFormat(s.into()))?;

        Ok(Self {
            code: first?,
            a: second?,
            b: third?,
            c: fourth?,
        })
    }
}

impl FromStr for Sample {
    type Err = SampleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (before, instruction, after) = s
            .lines()
            .collect_tuple::<(_, _, _)>()
            .ok_or_else(|| SampleParseError::BadFormat(s.into()))?;

        let before = before
            .strip_prefix("Before:")
            .ok_or_else(|| SampleParseError::BadFormat(s.into()))?;
        let after = after
            .strip_prefix("After:")
            .ok_or_else(|| SampleParseError::BadFormat(s.into()))?;

        Ok(Self {
            before: before.trim().parse()?,
            instruction: instruction.trim().parse()?,
            after: after.trim().parse()?,
        })
    }
}

impl FromStr for Program {
    type Err = SampleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = sep_by_empty_lines(s).peekable();
        let samples: Vec<Sample> = blocks
            .peeking_take_while(|s| s.starts_with("Before:"))
            .map(str::parse)
            .try_collect()?;

        // Discard empty lines until the test program
        while blocks.peek().map_or(false, |s| s.is_empty()) {
            blocks.next();
        }

        let program: Vec<Instruction> = blocks
            .next()
            .unwrap_or("")
            .lines()
            .map(str::parse)
            .try_collect()?;

        Ok(Self { samples, program })
    }
}
