//! Common utilities for the Advent of Code solutions

/// String to print in the console to return to the top
pub const TO_TOP: &str = "\u{001b}[H";
/// String to print in the console to clear the screen
pub const CLEAR_COMMAND: &str = concat!("\u{001b}[2J", "\u{001b}[H");

pub mod arguments;
pub mod grid;
pub mod grid2;
pub mod math;
pub mod parse;
pub mod problem;
