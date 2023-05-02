pub use error::{Report, Result, WrapErr};

/// String to print in the console to return to the top
pub const TO_TOP: &str = "\u{001b}[H";
/// String to print in the console to clear the screen
pub const CLEAR_COMMAND: &str = concat!("\u{001b}[2J", "\u{001b}[H");

#[macro_use]
pub mod error;
pub mod arguments;
pub mod grid;
pub mod math;
pub mod parse;
pub mod problem;
