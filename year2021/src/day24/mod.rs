use commons::eyre::{eyre, Report, Result, WrapErr};

pub const TITLE: &str = "Day 24: Arithmetic Logic Unit";

pub fn run(raw: String) -> Result<()> {
    let (min, max) = search(&raw)?;
    println!("1. Highest valid input: {max}");
    println!("2. Minimum valid input: {min}");

    Ok(())
}

fn search(s: &str) -> Result<(i64, i64)> {
    let mut min_max: Vec<(i32, i32)> = Vec::with_capacity(14);
    let mut z_stack: Vec<(i32, usize)> = Vec::with_capacity(7);
    s.split("inp w")
        .skip(1)
        .try_for_each(|block| -> Result<()> {
            let block: Block = block.parse()?;
            let (offset, prev_idx) = match z_stack.last().copied() {
                Some((off, idx)) => (block.comparison_offset + off, idx),
                None => (block.comparison_offset, 14),
            };

            if block.pop {
                z_stack.pop();
            }

            // To not push a new value onto the Z-Stack, we need to input a digit for which:
            // CURRENT = PREVIOUS + OFFSET
            //
            // That is:
            // 1 <= PREVIOUS + OFFSET <= 9
            // PREVIOUS >= 1 - OFFSET
            // PREVIOUS <= 9 - OFFSET
            //
            // If this is possible:
            // CURRENT is bounded by PREVIOUS_MIN + OFFSET and PREVIOUS_MAX + OFFSET
            //
            // Otherwise we are forced to push something on the Z-Stack
            let max = (9 - offset).min(9);
            let min = (1 - offset).max(1);
            if max < 1 || min > 9 || min > max {
                z_stack.push((block.push_offset, min_max.len()));
                min_max.push((1, 9));
            } else {
                min_max.push(((min + offset).max(1), (max + offset).min(9)));
                if let Some((prev_min, prev_max)) = min_max.get_mut(prev_idx) {
                    *prev_min = min.max(*prev_min);
                    *prev_max = max.min(*prev_max);
                }
            }

            Ok(())
        })?;

    if z_stack.is_empty() {
        let mut min: i64 = 0;
        let mut max: i64 = 0;
        for (min_digit, max_digit) in min_max {
            min = min * 10 + min_digit as i64;
            max = max * 10 + max_digit as i64;
        }
        Ok((min, max))
    } else {
        Err(eyre!("Can't find a solution to have z equals to 0"))
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Block {
    comparison_offset: i32,
    pop: bool,
    push_offset: i32,
}

impl std::str::FromStr for Block {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn check(line: &str, expected: &str) -> Result<()> {
            if line.trim() == expected {
                Ok(())
            } else {
                Err(eyre!("Expected '{}', got '{}'", expected, line))
            }
        }

        fn extract(line: &str, prefix: &str, into: &mut i32) -> Result<()> {
            *into = line
                .trim()
                .strip_prefix(prefix)
                .ok_or_else(|| eyre!("Missing '{prefix}' prefix"))
                .and_then(|s| Ok(s.trim().parse::<i32>()?))
                .wrap_err_with(|| format!("Expected '{prefix} <NUMBER>', got '{line}'"))?;

            Ok(())
        }

        let mut comparison_offset = 0;
        let mut z_div = 0;
        let mut push_offset = 0;
        s.lines()
            .filter(|l| !l.trim().is_empty())
            .enumerate()
            .try_for_each(|(i, line)| match i {
                0 => check(line, "mul x 0"),
                1 => check(line, "add x z"),
                2 => check(line, "mod x 26"),
                3 => extract(line, "div z", &mut z_div),
                4 => extract(line, "add x", &mut comparison_offset),
                5 => check(line, "eql x w"),
                6 => check(line, "eql x 0"),
                7 => check(line, "mul y 0"),
                8 => check(line, "add y 25"),
                9 => check(line, "mul y x"),
                10 => check(line, "add y 1"),
                11 => check(line, "mul z y"),
                12 => check(line, "mul y 0"),
                13 => check(line, "add y w"),
                14 => extract(line, "add y", &mut push_offset),
                15 => check(line, "mul y x"),
                16 => check(line, "add z y"),
                _ => Ok(()),
            })
            .wrap_err_with(|| format!("For block '{s}'"))?;

        Ok(Self {
            comparison_offset,
            pop: z_div == 26,
            push_offset,
        })
    }
}

#[cfg(test)]
mod tests;
