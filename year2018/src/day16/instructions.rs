use std::convert::TryFrom;

/// The type of an integer in the system
pub type Int = i32;

/// Error returned when a register index is out of bound
#[derive(Debug, thiserror::Error)]
#[error("{0} is out of bounds for a register")]
pub struct IndexError(Int);

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
    pub fn apply(self, reg: &mut [Int], a: Int, b: Int, c: Int) -> Result<(), IndexError> {
        *get_mut(reg, c)? = match self {
            OpCode::AddR => *get(reg, a)? + *get(reg, b)?,
            OpCode::AddI => *get(reg, a)? + b,
            OpCode::MulR => *get(reg, a)? * *get(reg, b)?,
            OpCode::MulI => *get(reg, a)? * b,
            OpCode::BitAndR => *get(reg, a)? & *get(reg, b)?,
            OpCode::BitAndI => *get(reg, a)? & b,
            OpCode::BitOrR => *get(reg, a)? | *get(reg, b)?,
            OpCode::BitOrI => *get(reg, a)? | b,
            OpCode::SetR => *get(reg, a)?,
            OpCode::SetI => a,
            OpCode::GreaterIR => greater(a, *get(reg, b)?),
            OpCode::GreaterRI => greater(*get(reg, a)?, b),
            OpCode::GreaterRR => greater(*get(reg, a)?, *get(reg, b)?),
            OpCode::EqIR => equal(a, *get(reg, b)?),
            OpCode::EqRI => equal(*get(reg, a)?, b),
            OpCode::EqRR => equal(*get(reg, a)?, *get(reg, b)?),
        };

        Ok(())
    }
}

/// 1 if `a` is strictly greater than `b`
fn greater(a: Int, b: Int) -> Int {
    if a > b {
        1
    } else {
        0
    }
}

/// 1 if `a` and `b` are equal
fn equal(a: Int, b: Int) -> Int {
    if a == b {
        1
    } else {
        0
    }
}

/// Try to convert an [Int](Int) into a [usize](usize) for indexing purpose
fn index(idx: Int) -> Result<usize, IndexError> {
    usize::try_from(idx).map_err(|_| IndexError(idx))
}

/// Get the `idx`th element in the registers
fn get(reg: &[Int], idx: Int) -> Result<&Int, IndexError> {
    reg.get(index(idx)?).ok_or(IndexError(idx))
}

/// Get the `idx`th element in the registers, mutable version
fn get_mut(reg: &mut [Int], idx: Int) -> Result<&mut Int, IndexError> {
    reg.get_mut(index(idx)?).ok_or(IndexError(idx))
}
