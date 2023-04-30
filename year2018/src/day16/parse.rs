use std::str::FromStr;

use commons::eyre::{eyre, Report, Result, WrapErr};
use itertools::Itertools;
use std::collections::HashMap;

use commons::parse::sep_by_empty_lines;

use super::instructions::{Int, OpCode};

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
    pub fn execute(&self, registers: &mut Register, code: OpCode) -> Result<()> {
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

/// Parse an integer of type [Element](Element)
fn parse_element(str: &str) -> Result<Int> {
    str.trim()
        .parse()
        .wrap_err_with(|| format!("Could not parse an integer {str}"))
}

impl FromStr for Register {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (first, second, third, fourth) = s
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .and_then(|s| {
                s.split(',')
                    .map(parse_element)
                    .collect_tuple::<(_, _, _, _)>()
            })
            .ok_or_else(|| {
                eyre!("Expected [A, B, C, D] for a register, where A, B, C, D integers, got: {s}")
            })?;

        Ok(Self([first?, second?, third?, fourth?]))
    }
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (first, second, third, fourth) = s
            .split_whitespace()
            .map(parse_element)
            .collect_tuple::<(_, _, _, _)>()
            .ok_or_else(|| {
                eyre!("Expected A, B, C, D for an instruction, where A, B, C, D integers, got: {s}")
            })?;

        Ok(Self {
            code: first?,
            a: second?,
            b: third?,
            c: fourth?,
        })
    }
}

impl FromStr for Sample {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (before, instruction, after) =
            s.lines().collect_tuple::<(_, _, _)>().ok_or_else(|| {
                eyre!("Expected register instruction register for a sample, got: {s}")
            })?;

        let before = before.strip_prefix("Before:").ok_or_else(|| {
            eyre!("Expected register instruction register for a sample, got: {s}")
        })?;
        let after = after.strip_prefix("After:").ok_or_else(|| {
            eyre!("Expected register instruction register for a sample, got: {s}")
        })?;

        Ok(Self {
            before: before.trim().parse()?,
            instruction: instruction.trim().parse()?,
            after: after.trim().parse()?,
        })
    }
}

impl FromStr for Program {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
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
