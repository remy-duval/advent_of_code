use itertools::Itertools;

use commons::error::Result;

use super::int_code;

pub const TITLE: &str = "Day 2: 1202 Program Alarm";
const WANTED: i64 = 19_690_720;

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let first = run_one(&data.data, 12, 2)
        .ok_or_else(|| int_code::IntCodeError::Other("1202 program error".into()))?;
    let (noun, verb) = find_match(&data.data, WANTED)
        .ok_or_else(|| int_code::IntCodeError::Other("Finding second program error".into()))?;

    println!("1202 program : {first}");
    println!("Found {} program : {} ", noun * 100 + verb, WANTED);
    Ok(())
}

fn parse(s: &str) -> Result<int_code::IntCodeInput> {
    Ok(s.parse()?)
}

fn run_one(start: &[i64], noun: i64, verb: i64) -> Option<i64> {
    let mut memory = start.to_owned();
    *memory.get_mut(1)? = noun;
    *memory.get_mut(2)? = verb;
    let mut program: int_code::Processor = int_code::Processor::new(&memory[..]);
    program.run().ok()?;
    Some(program.into_memory()[0])
}

fn find_match(mem: &[i64], expected: i64) -> Option<(i64, i64)> {
    (0..100).cartesian_product(0..100).find(|(noun, verb)| {
        if let Some(result) = run_one(mem, *noun, *verb) {
            result == expected
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests;
