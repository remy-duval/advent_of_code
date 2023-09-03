use std::str::FromStr;

use commons::error::{Result, WrapErr};
use commons::parse::LineSep;
use commons::{err, Report};

pub const TITLE: &str = "Day 10: Cathode-Ray Tube";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The signal sum is {first}");
    let second = second_part(&data);
    println!("2. The display after running the program is:\n{second}");

    Ok(())
}

fn first_part(inst: &[Instruction]) -> i32 {
    let mut sums: i32 = 0;
    execute_cycles(inst, 220, |cycle, register| {
        if cycle % 40 == 20 {
            sums += cycle as i32 * register;
        }
    });

    sums
}

fn second_part(inst: &[Instruction]) -> String {
    const HEIGHT: usize = 6;
    const WIDTH: usize = 40;
    let mut screen = String::with_capacity((WIDTH + 1) * HEIGHT);
    execute_cycles(inst, WIDTH * HEIGHT, |cycle, register| {
        let position = ((cycle - 1) % WIDTH) as i32;
        if position >= register - 1 && position <= register + 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }

        if cycle % 40 == 0 && cycle != WIDTH * HEIGHT {
            screen.push('\n');
        }
    });

    screen
}

fn execute_cycles(
    inst: &[Instruction],
    cycles: usize,
    mut during_each_cycle: impl FnMut(usize, i32),
) {
    let mut register: i32 = 1;
    let mut pending_add: Option<i32> = None;
    let mut instructions = inst.iter().copied();
    for cycle in 0..cycles {
        during_each_cycle(cycle + 1, register);
        if let Some(to_add) = pending_add.take() {
            register += to_add;
        } else if let Some(Instruction::AddX(to_add)) = instructions.next() {
            pending_add = Some(to_add as i32);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    AddX(i8),
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        if s == "noop" {
            Ok(Instruction::Noop)
        } else if let Some(amount) = s.strip_prefix("addx ") {
            let amount: i8 = amount
                .parse()
                .wrap_err_with(|| format!("bad amount: {s}"))?;
            Ok(Instruction::AddX(amount))
        } else {
            Err(err!("bad format): {s}"))
        }
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Instruction>> {
    let split: LineSep<Instruction> = s.parse()?;
    Ok(split.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/10.txt");
    const MAIN: &str = include_str!("../inputs/10.txt");

    const EXAMPLE_SCREEN: &str = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    // PZBGZEJB
    const MAIN_SCREEN: &str = "\
###..####.###...##..####.####...##.###..
#..#....#.#..#.#..#....#.#.......#.#..#.
#..#...#..###..#......#..###.....#.###..
###...#...#..#.#.##..#...#.......#.#..#.
#....#....#..#.#..#.#....#....#..#.#..#.
#....####.###...###.####.####..##..###..";

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 13_140);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 14_520);
    }

    #[test]
    fn second_part_example() {
        assert_eq!(
            second_part(&parse(EXAMPLE.into()).unwrap()).as_str(),
            EXAMPLE_SCREEN,
            "expected:\n{}",
            EXAMPLE_SCREEN
        );
    }

    #[test]
    fn second_part_main() {
        assert_eq!(
            second_part(&parse(MAIN.into()).unwrap()).as_str(),
            MAIN_SCREEN,
            "expected:\n{}",
            MAIN_SCREEN
        );
    }
}
