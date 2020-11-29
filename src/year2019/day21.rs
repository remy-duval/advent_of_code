//! Day 21
//!
//! ## PART ONE
//! Jump if any empty tile in 1-3 and 4 is available :
//! This allow to jump over any hole in sight as soon as it is possible.
//!
//! BOOLEAN FORM is : D && (!C || !B || !A)
//! CODE is :
//! NOT A T -> Store !A
//! NOT B J -> Store !B
//! OR J T  -> Compute x = !A || !C
//! NOT C J -> Store !C
//! OR T J  -> Compute y = x || !C
//! AND D J -> Compute D && y
//! WALK
//!
//! ## PART TWO
//! Jump if any empty tile in 1-3, 4 and (5 or 8) are available :
//! This allow to jump over any hole in sight like first part
//! But it also avoid the bad case when the landing point is a lone part with a hole 4 case away.
//!
//! BOOLEAN FORM is : (D && (E || H)) && (!C || !B || !A)
//! CODE is :
//! NOT A T -> Store !A
//! NOT B J -> Store !B
//! OR J T  -> Compute x = !A || !C
//! NOT C J -> Store !C
//! OR T J  -> Compute y = x || !C
//! OR E T
//! OR H T  -> Compute z = E || H
//! AND D T -> Compute a = D && z
//! AND T J -> Compute D && z
//! RUN

use std::io::{stdout, BufWriter, Write};

use aoc::generator::data_from_cli;
use aoc::int_code::{parse_int_code, Processor};

const TITLE: &str = "Day 21: Springdroid Adventure";
const DATA: &str = include_str!("../resources/day21.txt");

fn main() {
    let data = data_from_cli(TITLE, DATA);
    println!("{}", TITLE);
    let memory = parse_int_code(&data).expect("Parse Int code error !");
    let mut stdout = BufWriter::new(stdout());

    // First Part
    let mut robot: Processor = memory[..].into();
    robot.run_with_ascii_callbacks(
        [
            "NOT A T", "NOT B J", "OR J T", "NOT C J", "OR T J", "AND D J", "WALK",
        ]
        .iter(),
        |iterator| Some(format!("{}\n", iterator.next()?)),
        |_, line| write!(stdout, "{}", line).map_err(|_| aoc::int_code::Status::Halted),
    );
    stdout.flush().unwrap();

    // Second Part
    let mut robot: Processor = memory[..].into();
    robot.run_with_ascii_callbacks(
        [
            "NOT A T", "NOT B J", "OR J T", "NOT C J", "OR T J", "OR E T", "OR H T", "AND D T",
            "AND T J", "RUN",
        ]
        .iter(),
        |iterator| Some(format!("{}\n", iterator.next()?)),
        |_, line| write!(stdout, "{}", line).map_err(|_| aoc::int_code::Status::Halted),
    );
    stdout.flush().unwrap();
}
