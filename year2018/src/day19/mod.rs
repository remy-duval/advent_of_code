use commons::eyre::Result;

use super::instructions::{Int, Program};

pub const TITLE: &str = "Day 19: Go With The Flow";

pub fn run(raw: String) -> Result<()> {
    let program = parse(&raw)?;
    let first = run_optimized(program.clone(), 0)?;
    println!("Run 1: The register 0 contains {} on exit", first);

    let second = run_optimized(program, 1)?;
    println!("Run 2: The register 0 contains {} on exit", second);

    Ok(())
}

fn parse(s: &str) -> Result<Program> {
    s.parse()
}

/// Run the given input program, optimizing the critical section to make it not run for hours
/// This will only work for programs that have the same critical section as mine though
fn run_optimized(mut program: Program, initial: Int) -> Result<Int> {
    program.reset();
    program.registers[0] = initial;

    loop {
        if program.registers[program.ip_index] == 3 {
            // This range of the code contains inefficient code that makes it run far too long
            // Optimize this two nested loops:
            //
            // Inner loop:
            // - takes in parameter #1, #3 and #5
            // - Output result in #0
            // - #2 is used to store intermediate results
            // - Can be written as:
            // loop {
            //     program.registers[2] = program.registers[1] * program.registers[3];
            //     if program.registers[2] == program.registers[5] {
            //         program.registers[0] += program.registers[1];
            //     }
            //
            //     program.registers[3] += 1;
            //     if program.registers[3] <= program.registers[5] {
            //         break;
            //     }
            // }
            // - Optimized as adding #1 to #0 if it divides #5, doing nothing otherwise
            //
            // Outer loop:
            // - takes in parameter #5
            // - #2 is used to store the 0 of 1 resulting from comparisons
            // - #1 starts at 0
            // - Call the inner loop, then increment #1 and loop
            // - Optimized as adding to #0 the sum of factors of #5

            program.registers[0] += sum_of_factors(program.registers[1], program.registers[5]);
            program.registers[1] = program.registers[5] + 1;
            program.registers[program.ip_index] = 16;
        }

        if program.step()?.is_none() {
            break Ok(program.registers[0]);
        }
    }
}

/// A very dumb way of summing the factors of `number`, starting from `from`
fn sum_of_factors(from: Int, number: Int) -> Int {
    (from..=(number / 2))
        .filter(|&i| number % i == 0)
        .sum::<Int>()
        + number
}

#[cfg(test)]
mod tests;
