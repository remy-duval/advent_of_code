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

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

/// Dispatch to the correct problem and solve it
pub fn solve_problem(day: u8, input: &str) -> anyhow::Result<()> {
    use commons::solve;

    match day {
        1 => solve::<day01::Day>(input),
        2 => solve::<day02::Day>(input),
        3 => solve::<day03::Day>(input),
        4 => solve::<day04::Day>(input),
        5 => solve::<day05::Day>(input),
        6 => solve::<day06::Day>(input),
        7 => solve::<day07::Day>(input),
        8 => solve::<day08::Day>(input),
        9 => solve::<day09::Day>(input),
        10 => solve::<day10::Day>(input),
        11 => solve::<day11::Day>(input),
        12 => solve::<day12::Day>(input),
        13 => solve::<day13::Day>(input),
        _ => Err(anyhow::anyhow!("{} is not implemented for year 2018", day)),
    }
}
