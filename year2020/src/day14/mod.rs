use std::str::FromStr;

use std::collections::HashMap;

use commons::parse::LineSep;
use commons::{err, Report, Result, WrapErr};

pub const TITLE: &str = "Day 14: Docking Data";

pub fn run(raw: String) -> Result<()> {
    let instructions = parse(&raw)?.data;
    let first = first_part(&instructions);
    println!("Decoder V1: The memory sum after completion is {first}");

    let second = second_part(instructions);
    println!("Decoder V2: The memory sum after completion is {second}");

    Ok(())
}

fn parse(s: &str) -> Result<LineSep<Instruction>> {
    s.parse()
}

type Value = u64;

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
enum Instruction {
    /// Set the current bit mask to this value
    SetMask(Mask),
    /// Set value(s) in memory (according to the mode)
    SetValue { index: Value, value: Value },
}

/// A bit mask that will overwrite some zeros and ones in a 36-bits (at least) integer
#[derive(Copy, Clone, Eq, PartialEq)]
struct Mask {
    /// A bit mask to overwrite some positions to 1 with |, should be 0s except for 1 positions
    ones: Value,
    /// A bit mask to overwrite some positions to 0 with &, should be 1s except for 0 positions
    zeros: Value,
}

impl Mask {
    fn new() -> Self {
        Mask { ones: 0, zeros: 0 }
    }

    /// Apply the bit mask to the current value
    /// This is done with a bitwise or for setting the ones and a bitwise and for setting the zeros
    fn apply(&self, value: Value) -> Value {
        (value | self.ones) & self.zeros
    }
}

struct FloatingMasks {
    possibilities: Vec<Mask>,
    buffer: Vec<Mask>,
}

impl FloatingMasks {
    const BIT_RANGE: u8 = 36;

    fn new() -> Self {
        FloatingMasks {
            possibilities: Vec::new(),
            buffer: Vec::new(),
        }
    }

    /// Apply the floating bit mask to the current value to get all the possible values
    fn apply(&self, value: Value) -> impl Iterator<Item = Value> + '_ {
        self.possibilities.iter().map(move |mask| mask.apply(value))
    }

    /// Compute the new floating masks from the given mask
    /// Re-uses the internal buffer instead of allocating new one
    fn compute(&mut self, mask: Mask) {
        self.possibilities.clear(); // Re-use an already allocated vec
        self.possibilities.push(Mask { ones: 0, zeros: 0 });
        (0..Self::BIT_RANGE).rev().for_each(|bit| {
            // If the bitmask bit is 0, the corresponding memory address bit is unchanged.
            // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
            // If the bitmask bit is X, the corresponding memory address bit is floating.
            let i: Value = 1 << bit;
            let is_one = mask.ones & i != 0;
            let is_float = mask.zeros & i != 0 && !is_one;
            self.possibilities.iter_mut().for_each(|current| {
                current.ones = current.ones * 2 + Value::from(is_one);
                current.zeros = current.zeros * 2 + Value::from(!is_float);
            });
            // If bitmask X, duplicate each bitmask to account for the 2 possibilities
            // The duplicated mask will have its values at the given bit the reverse of the original
            if is_float {
                self.buffer.clear(); // Re-use an already allocated vec
                self.buffer.reserve(self.possibilities.len());
                self.buffer
                    .extend(self.possibilities.iter().map(|mask| Mask {
                        zeros: mask.zeros + 1,
                        ones: mask.ones + 1,
                    }));
                self.possibilities.extend_from_slice(self.buffer.as_slice());
            }
        });
    }
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (instruction, value) = s
            .split_once('=')
            .map(|(a, b)| (a.trim(), b.trim()))
            .wrap_err_with(|| format!("Unknown line {s} (expected 'instruction = value')"))?;

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
                    .wrap_err_with(|| format!("Unknown line {s} (expected 'instruction = value')"))?
                    .parse::<Value>()
                    .wrap_err_with(|| format!("Could not parse a memory index or value {s}"))?;

                let value = value
                    .parse::<Value>()
                    .wrap_err_with(|| format!("Could not parse a memory index or value {s}"))?;

                Ok(Instruction::SetValue { index, value })
            }
            _ => Err(err!("Unknown line {s} (expected 'instruction = value')")),
        }
    }
}

#[cfg(test)]
mod tests;
