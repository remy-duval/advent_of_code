use hashbrown::HashSet;

use commons::Problem;

use super::instructions::{errors::ExecutionError, Int, Program};

pub struct Day;

impl Problem for Day {
    type Input = Program;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 21: Chronal Conversion";

    fn solve(mut program: Self::Input) -> Result<(), Self::Err> {
        println!(
            "The program will halt after the fewest cycles for input {}",
            first_exit_value(&mut program)?
                .ok_or_else(|| anyhow::anyhow!("No exit values were found"))?
        );
        println!(
            "The program will halt after the most cycles for input {}",
            last_exit_value(&mut program)?
                .ok_or_else(|| anyhow::anyhow!("No exit values were found"))?
        );

        Ok(())
    }
}

/// Find the first possible exit value of a program
fn first_exit_value(program: &mut Program) -> Result<Option<Int>, Box<ExecutionError>> {
    while step_optimized(program)?.is_some() {
        if let Some(exit_value) = possible_exit_value(&program) {
            return Ok(Some(exit_value));
        }
    }

    Ok(None)
}

/// Find the last non duplicate exit value of a program
fn last_exit_value(program: &mut Program) -> Result<Option<Int>, Box<ExecutionError>> {
    let mut last: Option<Int> = None;
    let mut seen: HashSet<Int> = HashSet::new();

    // Note: 0 is NOT an exit value for the program, so this will loop indefinitely
    // We need to exit manually as soon as an exit value is seen a second time
    program.reset();
    while step_optimized(program)?.is_some() {
        if let Some(exit_value) = possible_exit_value(&program) {
            // If the exit value was already present in the map, we have found all of them
            if !seen.insert(exit_value) {
                break;
            } else {
                last = Some(exit_value);
            }
        }
    }

    Ok(last)
}

/// Run the program for one step, optimizing the biggest procedure call
///
/// This might be exclusive to this particular input ?
fn step_optimized(program: &mut Program) -> Result<Option<()>, Box<ExecutionError>> {
    // Optimize the procedure 18-25
    if program.registers[program.ip_index] == 18 {
        program.registers[1] = program.registers[2] / 256;
        program.registers[5] = 1;
        program.registers[4] = 26;
        Ok(Some(()))
    } else {
        program.step()
    }
}

/// Every time the program runs the instruction 28 (eqrr 3 0 1) it can exit
/// As #0 is never touched anywhere, this means that #3 value is an exit value
/// Since if #0 was set to that value, then the program would have exited right here
///
/// This might be exclusive to this particular input ?
fn possible_exit_value(program: &Program) -> Option<Int> {
    if program.line == 28 {
        Some(program.registers[3])
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
