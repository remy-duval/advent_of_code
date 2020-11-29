use super::int_code;

const WANTED: i64 = 19_690_720;

pub struct Day02;

impl crate::Problem for Day02 {
    type Input = int_code::IntCodeInput;
    type Err = int_code::IntCodeError;
    const TITLE: &'static str = "Day 2: 1202 Program Alarm";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let first = run_one(&data.data, 12, 2)
            .ok_or_else(|| int_code::IntCodeError::Other("1202 program error".into()))?;
        let (noun, verb) = find_match(&data.data, WANTED)
            .ok_or_else(|| int_code::IntCodeError::Other("Finding second program error".into()))?;

        println!("1202 program : {}", first);
        println!("Found {} program : {} ", noun * 100 + verb, WANTED);
        Ok(())
    }
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
    use itertools::Itertools;
    (0..100).cartesian_product(0..100).find(|(noun, verb)| {
        if let Some(result) = run_one(mem, *noun, *verb) {
            result == expected
        } else {
            false
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = include_str!("test_resources/day02.txt");

    #[test]
    fn solve_test() {
        let memory: Vec<i64> = DATA.parse::<int_code::IntCodeInput>().unwrap().data;
        let first = run_one(&memory, 12, 2).expect("1202 program error");
        let (noun, verb) = find_match(&memory, WANTED).expect("Finding second program error");

        assert_eq!(3_409_710, first);
        assert_eq!(7_912, noun * 100 + verb);
    }
}
