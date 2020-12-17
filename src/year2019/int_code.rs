//! Implementation of the IntCode computer for most of the problems of AOC
//! # Basic usage
//! - Build and run a Processor
//! ```
//! use advent_of_code::year2019::int_code::{Processor, Status, IntCodeInput};
//!
//! let code: Vec<i64> = "3, 3, 104, -1, 1106, 0, 0, 99".parse::<IntCodeInput>().unwrap().data;
//! let mut cpu: Processor = code[..].into();
//!
//! // Run the Processor until it blocks on IO or halts completely
//! let result: Status = cpu.run().unwrap();
//!
//! // Add an input to the input queue of the Processor
//! cpu.write_int(5);
//!
//! // Run the Processor and collect its next output as the Ok variant of its output
//! // Return Err(status) if no output was produced before it blocked on status.
//! let out = cpu.read_next().expect("This program should output something");
//! println!("{}", out); // 5, as this specific program echoes every input as output
//!
//! cpu.write_int(4);
//! cpu.write_int(3);
//!
//! // Run the Processor and collect many outputs (as many as the limit given) as a VecDeque<i64>
//! let mut out = [0; 2];
//! let (_, status) = cpu.read_next_array(&mut out, 2);
//! println!("{:?}", out); // [4, 3]
//! ```
//!

use std::convert::TryInto;
use std::{
    collections::VecDeque,
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter},
};

/// The type to use when parsing int code inputs
pub type IntCodeInput = crate::parse::CommaSep<i64>;

/// The status of an int_code program after it returns from execution
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    /// The processor is blocked due to last instruction requiring input and having none left.
    RequireInput,
    /// The processor is stopped due to reaching a Halt instruction.
    Halted,
    /// The processor produced an output, and can be restarted immediately.
    WithOutput(i64),
}

/// Represents the state of an int_code processor commonly used in many problems of AOC2019
#[derive(Debug, Clone, PartialEq)]
pub struct Processor {
    /// The underlying memory of the processor
    memory: Vec<i64>,
    /// The instruction pointer
    current: i64,
    /// The offset for the Relative writes/read
    relative_offset: i64,
    /// The queue containing all non-yet-read inputs for the program
    input_queue: VecDeque<i64>,
}

/// An error for the whole IntCode processor.
#[derive(Debug, Clone, PartialEq)]
pub enum IntCodeError {
    InvalidIndexRead,
    InvalidIndexWrite,
    InvalidInstruction,
    Other(String),
}

impl IntCodeError {
    pub fn new(msg: &str) -> Self {
        IntCodeError::Other(msg.into())
    }
}

impl Error for IntCodeError {}

impl Display for IntCodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error: &str = match self {
            IntCodeError::InvalidIndexRead => "Cannot read from an invalid index",
            IntCodeError::InvalidIndexWrite => "Cannot write to an invalid index",
            IntCodeError::InvalidInstruction => "Current instruction is invalid",
            IntCodeError::Other(msg) => &*msg,
        };
        write!(f, "{}", error)
    }
}

impl Processor {
    /// Build a new Processor from an initial state.
    /// # Arguments
    /// * `initial_state` - A slice of ints to be used as the starting state of the program
    pub fn new(initial_state: &[i64]) -> Self {
        Self {
            memory: initial_state.into(),
            current: 0,
            relative_offset: 0,
            input_queue: VecDeque::new(),
        }
    }

    /// Build a new Processor from an initial state and some starting data.
    /// # Arguments
    /// * `initial_state` - A slice of ints to be used as the starting state of the program
    /// * `start_inputs` - A slice of input data for the process
    pub fn with_initial_inputs(initial_state: &[i64], start_inputs: &[i64]) -> Self {
        let mut process = Self::new(initial_state);
        for input in start_inputs {
            process.write_int(*input);
        }
        process
    }

    /// Run the Processor until it hits a block (halt, no input left, or produced output)
    pub fn run(&mut self) -> Result<Status, IntCodeError> {
        loop {
            let instruction = self.parse_inst()?;
            let (a, b, c) = instruction.modes;
            match instruction.code {
                OpCode::Add | OpCode::Mul | OpCode::Less | OpCode::Equals => {
                    let first = self.get(1, a)?;
                    let second = self.get(2, b)?;
                    let result = match instruction.code {
                        OpCode::Add => first + second,
                        OpCode::Mul => first * second,
                        OpCode::Less => {
                            if first < second {
                                1
                            } else {
                                0
                            }
                        }
                        OpCode::Equals => {
                            if first == second {
                                1
                            } else {
                                0
                            }
                        }
                        _ => unreachable!(),
                    };
                    self.set(3, result, c)?;
                    self.current += 4;
                }
                OpCode::TrueJump | OpCode::FalseJump => {
                    let first = self.get(1, a)?;
                    let second = self.get(2, b)?;
                    let is_jump = match instruction.code {
                        OpCode::TrueJump => first != 0,
                        OpCode::FalseJump => first == 0,
                        _ => unreachable!(),
                    };
                    if is_jump {
                        self.current = second;
                    } else {
                        self.current += 3;
                    }
                }
                OpCode::Offset => {
                    let first = self.get(1, a)?;
                    self.relative_offset += first;
                    self.current += 2;
                }
                OpCode::Input => match self.input_queue.pop_front() {
                    Some(input) => {
                        self.set(1, input, a)?;
                        self.current += 2;
                    }
                    None => return Ok(Status::RequireInput),
                },
                OpCode::Output => {
                    let output = self.get(1, a)?;
                    self.current += 2;
                    return Ok(Status::WithOutput(output));
                }
                OpCode::Halt => return Ok(Status::Halted),
            }
        }
    }

    /// Run the processor, collecting the next output or returning the first block
    pub fn read_next(&mut self) -> Result<i64, Status> {
        match self.run().unwrap_or(Status::Halted) {
            Status::WithOutput(out) => Ok(out),
            blocked => Err(blocked),
        }
    }

    /// Read multiple values from the processor into the destination slice.
    pub fn read_next_array(&mut self, dest: &mut [i64], limit: usize) -> (usize, Option<Status>) {
        let true_limit = limit.min(dest.len());
        let mut current = 0;
        while current < true_limit {
            match self.read_next() {
                Ok(out) => {
                    dest[current] = out;
                    current += 1;
                }
                Err(blocked) => return (current, Some(blocked)),
            }
        }
        (current, None)
    }

    /// Read a line of ascii character from running the processor.
    /// # Behaviour
    /// Read characters until :
    /// - An ascii control (new line in most cases) is seen
    /// - A non ascii i64 is read (in that case the i64 is concatenated to the String)
    /// - The processor blocks (in that case the second return value contains the block)
    pub fn read_next_line(&mut self) -> (String, Option<Status>) {
        let mut acc = String::with_capacity(10); // 10 is a good starting capacity.
        loop {
            match self.read_next() {
                Ok(out) => {
                    let ascii_code: Result<u8, _> = out.try_into();
                    if let Ok(code) = ascii_code {
                        let c: char = code.into();
                        acc.push(c);
                        if c.is_ascii_control() {
                            return (acc, None);
                        }
                    } else {
                        acc.push_str(&format!("\n{}\n", out));
                        return (acc, None);
                    }
                }
                Err(blocked) => return (acc, Some(blocked)),
            }
        }
    }

    /// Adds a new value in the inputs of the processor
    pub fn write_int<T: Into<i64>>(&mut self, input: T) {
        self.input_queue.push_back(input.into());
    }

    /// Adds a slice of text in the inputs of the processor
    pub fn write_text(&mut self, input: &str) {
        for byte in input.bytes() {
            self.write_int(byte);
        }
    }

    /// Run the processor, sending outputs and asking inputs from the callbacks given.
    /// This version uses the raw i64 as the input and outputs.
    /// # Arguments
    /// * `state` - The type that may hold some state for the program
    /// * `when_input` A callback to run on the state whenever some input is required
    /// * `on_output` A callback to run on the state and an output whenever some is produced
    pub fn run_with_callbacks<State, I, O>(
        &mut self,
        mut state: State,
        mut when_input: I,
        mut on_output: O,
    ) -> Status
    where
        I: FnMut(&mut State) -> Option<i64>,
        O: FnMut(&mut State, i64) -> Result<(), Status>,
    {
        loop {
            match self.run() {
                Ok(Status::WithOutput(out)) => {
                    if let Err(status) = on_output(&mut state, out) {
                        return status;
                    }
                }
                Ok(Status::RequireInput) => {
                    let input: i64 = match when_input(&mut state) {
                        Some(int) => int,
                        None => return Status::RequireInput,
                    };
                    self.write_int(input);
                }
                Ok(Status::Halted) => return Status::Halted,
                Err(err) => {
                    println!("Error : {}", err);
                    return Status::Halted;
                }
            }
        }
    }

    /// Run the processor, sending outputs and asking inputs from the callbacks given.
    /// This version uses the ascii String lines as the input and outputs.
    /// # Arguments
    /// * `state` - The type that may hold some state for the program
    /// * `when_input` A callback to run on the state whenever some input is required
    /// * `on_output` A callback to run on the state and an output whenever some is produced
    pub fn run_with_ascii_callbacks<State, I, O>(
        &mut self,
        mut state: State,
        mut when_input: I,
        mut on_output: O,
    ) -> (Status, State)
    where
        I: FnMut(&mut State) -> Option<String>,
        O: FnMut(&mut State, &str) -> Result<(), Status>,
    {
        loop {
            let (out, status_opt) = self.read_next_line();
            // Read the status of the ascii line read
            match status_opt {
                Some(Status::RequireInput) => {
                    let input: String = match when_input(&mut state) {
                        Some(str) => str,
                        None => return (Status::RequireInput, state),
                    };
                    self.write_text(&input);
                }
                Some(status) => return (status, state),
                None => {}
            }
            // Feed the output to the function
            if let Err(status) = on_output(&mut state, &out) {
                return (status, state);
            }
        }
    }

    /// Returns the ownership of the current memory
    pub fn into_memory(self) -> Vec<i64> {
        self.memory
    }

    /// Read an instruction from the current position in memory.
    fn parse_inst(&self) -> Result<Instruction, IntCodeError> {
        let inst = self.read_memory(self.current)?;
        let code = OpCode::try_from(inst)?;
        let mode1 = Mode::try_from(inst / 100)?;
        let mode2 = Mode::try_from(inst / 1000)?;
        let mode3 = Mode::try_from(inst / 10000)?;
        Ok(Instruction {
            code,
            modes: (mode1, mode2, mode3),
        })
    }

    /// Get an input from memory according to its mode.
    fn get(&self, offset: i64, mode: Mode) -> Result<i64, IntCodeError> {
        let pos = self.read_memory(self.current + offset)?;
        match mode {
            Mode::Immediate => Ok(pos),
            Mode::Absolute => self.read_memory(pos),
            Mode::Relative => self.read_memory(pos + self.relative_offset),
        }
    }

    /// Sets an output in memory according to its mode.
    fn set(&mut self, offset: i64, value: i64, mode: Mode) -> Result<(), IntCodeError> {
        let pos = self.read_memory(self.current + offset)?;
        match mode {
            Mode::Relative => self.write_memory(pos + self.relative_offset, value)?,
            _ => self.write_memory(pos, value)?,
        };
        Ok(())
    }

    /// Direct read from the program memory
    /// # Panics
    /// If pos is negative
    fn read_memory(&self, pos: i64) -> Result<i64, IntCodeError> {
        if pos < 0 {
            return Err(IntCodeError::InvalidIndexRead);
        }
        Ok(*self.memory.get(pos as usize).unwrap_or(&0))
    }

    /// Direct write to the program memory
    /// # Panics
    /// If pos is negative
    fn write_memory(&mut self, pos: i64, value: i64) -> Result<(), IntCodeError> {
        if pos < 0 {
            return Err(IntCodeError::InvalidIndexWrite);
        }

        let pos = pos as usize;
        // extends the memory if it is too small.
        if pos >= self.memory.len() {
            self.memory.resize(pos + 1, 0);
        }
        *self.memory.get_mut(pos).unwrap() = value;
        Ok(())
    }
}

/// A single instruction for an int_code processor
#[derive(Debug, Clone)]
struct Instruction {
    // The OpCode defining the instruction
    code: OpCode,
    // The mode for getting data for the instruction
    modes: (Mode, Mode, Mode),
}

/// All the different instructions for an int_code processor
#[derive(Debug, Clone)]
enum OpCode {
    Add,
    Mul,
    Less,
    Equals,
    TrueJump,
    FalseJump,
    Offset,
    Input,
    Output,
    Halt,
}

impl TryFrom<i64> for OpCode {
    type Error = IntCodeError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value % 100 {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Mul),
            3 => Ok(OpCode::Input),
            4 => Ok(OpCode::Output),
            5 => Ok(OpCode::TrueJump),
            6 => Ok(OpCode::FalseJump),
            7 => Ok(OpCode::Less),
            8 => Ok(OpCode::Equals),
            9 => Ok(OpCode::Offset),
            99 => Ok(OpCode::Halt),
            _ => Err(IntCodeError::InvalidInstruction),
        }
    }
}

/// The different modes of accessing the memory in an instruction
#[derive(Debug, Clone)]
enum Mode {
    Absolute,
    Immediate,
    Relative,
}

impl TryFrom<i64> for Mode {
    type Error = IntCodeError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value % 10 {
            0 => Ok(Mode::Absolute),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(IntCodeError::InvalidInstruction),
        }
    }
}

impl From<&[i64]> for Processor {
    fn from(int_array: &[i64]) -> Self {
        Self::new(int_array)
    }
}

/// Tests for the int_code module
#[cfg(test)]
mod tests {
    use super::{Processor, Status};

    /// An IntCode binary Program that will ask for input and output it back. Perfect to test IO.
    const ECHO_PROGRAM: [i64; 8] = [3, 3, 104, -1, 1106, 0, 0, 99];

    #[test]
    fn simple_operations() {
        fn execute_and_compare(start: &[i64], expected: &[i64]) {
            let mut program: Processor = start.into();
            let status = program.run();
            assert!(status.is_ok(), "Program should not have failed !");
            assert_eq!(&program.into_memory()[..], expected);
        }

        execute_and_compare(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
        execute_and_compare(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
        execute_and_compare(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
        execute_and_compare(
            &[1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn io_operations() {
        let initial = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut program: Processor = initial.as_ref().into();
        assert_eq!(
            program.run(),
            Ok(Status::RequireInput),
            "The program should ask for input"
        );

        // When input < 8 => output == 999
        program = initial.as_ref().into();
        program.write_int(7);
        assert_eq!(
            program.run(),
            Ok(Status::WithOutput(999)),
            "Output should be 999"
        );

        // When input == 8 => output == 1000
        program = initial.as_ref().into();
        program.write_int(8);
        assert_eq!(
            program.run(),
            Ok(Status::WithOutput(1000)),
            "Output should be 1000"
        );

        // When input > 8 => output == 1001
        program = initial.as_ref().into();
        program.write_int(9);
        assert_eq!(
            program.run(),
            Ok(Status::WithOutput(1001)),
            "Output should be 1001"
        );
    }

    #[test]
    fn relative_offset() {
        fn execute_and_check_outputs(start: &[i64], expected: &[i64]) {
            let mut program: Processor = start.into();
            let mut outputs = [0; 32];
            let (read, _) = program.read_next_array(&mut outputs[..], expected.len());
            assert_eq!(
                read,
                expected.len(),
                "Length of element written into {:?} should be {}",
                outputs,
                expected.len()
            );
            assert_eq!(&outputs[..expected.len()], expected);
        }

        const LARGE_RESULT: i64 = 1_219_070_632_396_864;
        const MIDDLE: i64 = 1_125_899_906_842_624;
        let quine: [i64; 16] = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let large: [i64; 8] = [1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
        let middle_output: [i64; 3] = [104, MIDDLE, 99];

        execute_and_check_outputs(&quine, &quine);
        execute_and_check_outputs(&large, &[LARGE_RESULT]);
        execute_and_check_outputs(&middle_output, &[MIDDLE]);
    }

    #[test]
    fn ascii() {
        let hello: &str = "Hello world\n";
        let mut program: Processor = ECHO_PROGRAM.as_ref().into();

        assert_eq!(
            program.run(),
            Ok(Status::RequireInput),
            "The program should ask for input"
        );

        program.write_text(hello);
        let (out, _) = program.read_next_line();
        assert_eq!(&out, hello);

        program.write_text("Hello there :");
        program.write_int(154_324);
        let (out, _) = program.read_next_line();
        assert_eq!(&out, "Hello there :\n154324\n");
    }

    #[test]
    fn int_callback() {
        let input: [i64; 5] = [0, 1, 2, 3, 4];
        let mut program: Processor = ECHO_PROGRAM.as_ref().into();

        let mut data = input.iter().copied();
        let mut acc: Vec<i64> = Vec::with_capacity(input.len());
        program.run_with_callbacks(
            0,
            |_| data.next(),
            |_, int| {
                acc.push(int);
                Ok(())
            },
        );

        assert_eq!(input.as_ref(), acc.as_slice());
    }

    #[test]
    fn ascii_callback() {
        let hello: &str = "Hello world\n";
        let mut program: Processor = ECHO_PROGRAM.as_ref().into();

        let mut data = std::iter::once(hello.to_string());
        let mut acc: String = String::with_capacity(hello.len());
        program.run_with_ascii_callbacks(
            0,
            |_| data.next(),
            |_, line| {
                acc.push_str(line);
                Ok(())
            },
        );

        assert_eq!(&acc, hello);
    }
}
