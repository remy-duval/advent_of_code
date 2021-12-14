#![deny(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

/// Re-export of eyre error library
pub use eyre;
/// Re-export of num library
pub use num;

/// String to print in the console to return to the top
pub const TO_TOP: &str = "\u{001b}[H";
/// String to print in the console to clear the screen
pub const CLEAR_COMMAND: &str = concat!("\u{001b}[2J", "\u{001b}[H");

pub mod arguments;
pub mod grid;
pub mod parse;
pub mod problem;
