use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

use crate::parse::LineSep;
use crate::Problem;

pub type Value = u64;
pub struct Day;

impl Problem for Day {
    type Input = LineSep<Instruction>;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 14: Docking Data";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let instructions = data.data;
        let first = first_part(&instructions);
        println!("Decoder V1: The memory sum after completion is {}", first);

        let second = second_part(instructions);
        println!("Decoder V2: The memory sum after completion is {}", second);

        Ok(())
    }
}

/// Use the first mode of the decoder to compute the sum of values in memory at the end
fn first_part(instructions: &[Instruction]) -> Value {
    let mut mask = Mask::new();
    let mut memory = HashMap::with_capacity(instructions.len());
    instructions.iter().for_each(|inst| match inst {
        Instruction::SetMask(new) => mask = *new,
        Instruction::SetValue { index, value } => {
            let value = mask.apply(*value);
            memory.insert(*index, value);
        }
    });

    memory.values().sum()
}

/// Use the second mode of the decoder to compute the sum of values in memory at the end
fn second_part(instructions: Vec<Instruction>) -> Value {
    let mut masks = FloatingMasks::new();
    let mut memory = HashMap::with_capacity(instructions.len());
    instructions.into_iter().for_each(|inst| match inst {
        Instruction::SetMask(new) => masks.compute(new),
        Instruction::SetValue { index, value } => {
            for address in masks.apply(index) {
                memory.insert(address, value);
            }
        }
    });

    memory.values().sum()
}

/// An instruction for the decoder
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Set the current bit mask to this value
    SetMask(Mask),
    /// Set value(s) in memory (according to the mode)
    SetValue { index: Value, value: Value },
}

/// A bit mask that will overwrite some zeros and ones in a 36-bits (at least) integer
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Mask {
    /// A bit mask to overwrite some positions to 1 with |, should be 0s except for 1 positions
    ones: Value,
    /// A bit mask to overwrite some positions to 0 with &, should be 1s except for 0 positions
    zeros: Value,
}

impl Mask {
    pub fn new() -> Self {
        Mask { ones: 0, zeros: 0 }
    }

    /// Apply the bit mask to the current value
    /// This is done with a bitwise or for setting the ones and a bitwise and for setting the zeros
    pub fn apply(&self, value: Value) -> Value {
        (value | self.ones) & self.zeros
    }
}

#[derive(Debug, Clone)]
pub struct FloatingMasks {
    possibilities: Vec<Mask>,
    buffer: Vec<Mask>,
}

impl FloatingMasks {
    const BIT_RANGE: u8 = 36;

    pub fn new() -> Self {
        FloatingMasks {
            possibilities: Vec::new(),
            buffer: Vec::new(),
        }
    }

    /// Apply the floating bit mask to the current value to get all the possible values
    pub fn apply(&self, value: Value) -> impl Iterator<Item = Value> + '_ {
        self.possibilities.iter().map(move |mask| mask.apply(value))
    }

    /// Compute the new floating masks from the given mask
    /// Re-uses the internal buffer instead of allocating new one
    pub fn compute(&mut self, mask: Mask) {
        fn bit(at: u8) -> Value {
            1 << at
        }

        self.possibilities.clear(); // Re-use an already allocated vec
        self.possibilities.push(Mask { ones: 0, zeros: 0 });
        for i in (0..Self::BIT_RANGE).rev().map(bit) {
            // If the bitmask bit is 0, the corresponding memory address bit is unchanged.
            // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
            // If the bitmask bit is X, the corresponding memory address bit is floating.
            let is_one = mask.ones & i != 0;
            let is_float = mask.zeros & i != 0 && !is_one;
            for current in &mut self.possibilities {
                current.ones = current.ones * 2 + if is_one { 1 } else { 0 };
                current.zeros = current.zeros * 2 + if !is_float { 1 } else { 0 };
            }
            // If bitmask X, duplicate each bitmask to account for the 2 possibilities
            // The duplicated mask will have its values at the given bit the reverse of the original
            if is_float {
                self.buffer.clear(); // Re-use an already allocated vec
                self.buffer.reserve(self.possibilities.len());
                for mask in &self.possibilities {
                    self.buffer.push(Mask {
                        zeros: mask.zeros + 1,
                        ones: mask.ones + 1,
                    });
                }
                self.possibilities.extend_from_slice(self.buffer.as_slice());
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InstructionParseError {
    #[error("Unknown line {0} (expected 'instruction = value')")]
    Unknown(Box<str>),
    #[error("Could not parse a memory index or value {0} ({1})")]
    ParseInt(Box<str>, std::num::ParseIntError),
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, value) = s
            .splitn(2, '=')
            .map(|part| part.trim())
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| InstructionParseError::Unknown(s.into()))?;

        // This is really dirty, at some point I should consider adding regex
        match instruction.get(0..4) {
            Some("mask") => {
                // ones should contain 0 for all bit except the position to overwrite as 1
                // zeros should contain 1 for all bit except the position to overwrite as 0
                let (ones, zeros) = value
                    .chars()
                    .fold((0, 0), |(ones, zeros), next| match next {
                        '1' => (ones * 2 + 1, zeros * 2 + 1),
                        '0' => (ones * 2, zeros * 2),
                        _ => (ones * 2, zeros * 2 + 1),
                    });

                Ok(Instruction::SetMask(Mask { ones, zeros }))
            }
            Some("mem[") => {
                let index = instruction
                    .get(4..(instruction.len() - 1)) // Take everything until the ']'
                    .ok_or_else(|| InstructionParseError::Unknown(s.into()))?
                    .parse::<Value>()
                    .map_err(|err| InstructionParseError::ParseInt(s.into(), err))?;

                let value = value
                    .parse::<Value>()
                    .map_err(|err| InstructionParseError::ParseInt(s.into(), err))?;

                Ok(Instruction::SetValue { index, value })
            }
            _ => Err(InstructionParseError::Unknown(s.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = include_str!("test_resources/14-A.txt");
    const B: &str = include_str!("test_resources/14-B.txt");
    const C: &str = include_str!("test_resources/14-C.txt");

    #[test]
    fn first_part_test_a() {
        let instructions = Day::parse(A).unwrap().data;
        let first = first_part(&instructions);
        assert_eq!(first, 165);
    }

    #[test]
    fn first_part_test_b() {
        let instructions = Day::parse(B).unwrap().data;
        let first = first_part(&instructions);
        assert_eq!(first, 9_967_721_333_886);
    }

    #[test]
    fn first_part_test_c() {
        let instructions = Day::parse(C).unwrap().data;
        let first = first_part(&instructions);
        assert_eq!(first, 51);
    }

    #[test]
    fn second_part_test_b() {
        let instructions = Day::parse(B).unwrap().data;
        let second = second_part(instructions);
        assert_eq!(second, 4_355_897_790_573);
    }

    #[test]
    fn second_part_test_c() {
        let instructions = Day::parse(C).unwrap().data;
        let second = second_part(instructions);
        assert_eq!(second, 208);
    }
}