use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use commons::{err, Report, Result, WrapErr};
use itertools::Itertools;

/// The type of an integer in the system
pub type Int = i64;

/// A program to execute
#[derive(Debug, Clone)]
pub struct Program {
    pub ip_index: usize,
    pub registers: [Int; 6],
    pub instructions: Vec<Instruction>,
    pub line: usize,
}

impl Program {
    /// Execute the program until it halts, returning the value of the first register (0)
    pub fn run(&mut self) -> Result<Int> {
        loop {
            if self.step()?.is_none() {
                break Ok(self.registers[0]);
            }
        }
    }

    /// Execute the next step of the program
    pub fn step(&mut self) -> Result<Option<()>> {
        self.line = self.next_instruction()?;
        if let Some(instruction) = self.instructions.get(self.line) {
            instruction
                .apply(&mut self.registers)
                .wrap_err_with(|| self.error())?;
            self.registers[self.ip_index] += 1;
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    /// The index of the next instruction to execute for the program
    pub fn next_instruction(&self) -> Result<usize> {
        index(self.registers[self.ip_index]).wrap_err_with(|| self.error())
    }

    /// Reset this program to its starting state
    pub fn reset(&mut self) {
        self.line = 0;
        self.registers.iter_mut().for_each(|reg| *reg = 0);
    }

    /// AdventOfCodeError an error during the program execution
    fn error(&self) -> String {
        format!(
            "The program failed on {line}.\n- registers: {reg:?}\n- inst: {inst}",
            line = self.line,
            reg = self.registers,
            inst = self.instructions[self.line].clone(),
        )
    }
}

/// An instruction with an OpCode and its inputs
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Instruction {
    pub code: OpCode,
    pub a: Int,
    pub b: Int,
    pub c: Int,
}

impl Instruction {
    /// Apply this Instruction to the given registers
    pub fn apply(&self, reg: &mut [Int]) -> Result<()> {
        self.code.apply(reg, self.a, self.b, self.c)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?} {} {} {}", self.code, self.a, self.b, self.c)
    }
}

/// An OpCode for the temporal device
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OpCode {
    AddR,
    AddI,
    MulR,
    MulI,
    BitAndR,
    BitAndI,
    BitOrR,
    BitOrI,
    SetR,
    SetI,
    GreaterIR,
    GreaterRI,
    GreaterRR,
    EqIR,
    EqRI,
    EqRR,
}

impl OpCode {
    /// All available op codes
    pub const ALL: [OpCode; 16] = [
        Self::AddR,
        Self::AddI,
        Self::MulR,
        Self::MulI,
        Self::BitAndR,
        Self::BitAndI,
        Self::BitOrR,
        Self::BitOrI,
        Self::SetR,
        Self::SetI,
        Self::GreaterIR,
        Self::GreaterRI,
        Self::GreaterRR,
        Self::EqIR,
        Self::EqRI,
        Self::EqRR,
    ];

    /// Apply this OpCode to the given registers
    pub fn apply(self, reg: &mut [Int], a: Int, b: Int, c: Int) -> Result<()> {
        *get_mut(reg, c)? = match self {
            Self::AddR => *get(reg, a)? + *get(reg, b)?,
            Self::AddI => *get(reg, a)? + b,
            Self::MulR => *get(reg, a)? * *get(reg, b)?,
            Self::MulI => *get(reg, a)? * b,
            Self::BitAndR => *get(reg, a)? & *get(reg, b)?,
            Self::BitAndI => *get(reg, a)? & b,
            Self::BitOrR => *get(reg, a)? | *get(reg, b)?,
            Self::BitOrI => *get(reg, a)? | b,
            Self::SetR => *get(reg, a)?,
            Self::SetI => a,
            Self::GreaterIR => greater(a, *get(reg, b)?),
            Self::GreaterRI => greater(*get(reg, a)?, b),
            Self::GreaterRR => greater(*get(reg, a)?, *get(reg, b)?),
            Self::EqIR => equal(a, *get(reg, b)?),
            Self::EqRI => equal(*get(reg, a)?, b),
            Self::EqRR => equal(*get(reg, a)?, *get(reg, b)?),
        };

        Ok(())
    }
}

/// 1 if `a` is strictly greater than `b`
fn greater(a: Int, b: Int) -> Int {
    Int::from(a > b)
}

/// 1 if `a` and `b` are equal
fn equal(a: Int, b: Int) -> Int {
    Int::from(a == b)
}

/// Try to convert an [Int](Int) into a [usize](usize) for indexing purpose
pub fn index(idx: Int) -> Result<usize> {
    usize::try_from(idx).map_err(|_| err!("{} is out of bounds for a register", idx))
}

/// Get the `idx`th element in the registers
fn get(reg: &[Int], idx: Int) -> Result<&Int> {
    reg.get(index(idx)?)
        .wrap_err_with(|| format!("{} is out of bounds for a register", idx))
}

/// Get the `idx`th element in the registers, mutable version
fn get_mut(reg: &mut [Int], idx: Int) -> Result<&mut Int> {
    reg.get_mut(index(idx)?)
        .wrap_err_with(|| format!("{} is out of bounds for a register", idx))
}

impl FromStr for Program {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut ip_index = 0;
        let instructions: Vec<Instruction> = s
            .lines()
            .filter_map(|line| {
                if let Some(ip) = line.strip_prefix("#ip") {
                    match ip.trim().parse() {
                        Ok(ip) => {
                            ip_index = ip;
                            None
                        }
                        Err(_) => Some(Err(err!("Could not parse Instruction pointer {ip}"))),
                    }
                } else {
                    Some(line.parse())
                }
            })
            .try_collect()?;

        assert!(ip_index < 6, "instruction pointer out of bounds");
        Ok(Self {
            ip_index,
            registers: [0; 6],
            instructions,
            line: 0,
        })
    }
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_int(s: &str) -> Result<Int> {
            s.parse()
                .wrap_err_with(|| format!("Could not parse an input Int {s}"))
        }

        let (code, a, b, c) = s
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect_tuple::<(_, _, _, _)>()
            .wrap_err_with(|| {
                format!("Bad format for a instruction: {s} (expected 'CODE A B C')")
            })?;

        Ok(Self {
            code: code.parse()?,
            a: parse_int(a)?,
            b: parse_int(b)?,
            c: parse_int(c)?,
        })
    }
}

impl FromStr for OpCode {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "addr" => Ok(Self::AddR),
            "addi" => Ok(Self::AddI),
            "mulr" => Ok(Self::MulR),
            "muli" => Ok(Self::MulI),
            "banr" => Ok(Self::BitAndR),
            "bani" => Ok(Self::BitAndI),
            "borr" => Ok(Self::BitOrR),
            "bori" => Ok(Self::BitOrI),
            "setr" => Ok(Self::SetR),
            "seti" => Ok(Self::SetI),
            "gtir" => Ok(Self::GreaterIR),
            "gtri" => Ok(Self::GreaterRI),
            "gtrr" => Ok(Self::GreaterRR),
            "eqir" => Ok(Self::EqIR),
            "eqri" => Ok(Self::EqRI),
            "eqrr" => Ok(Self::EqRR),
            other => Err(err!("Unknown op code {other}")),
        }
    }
}
