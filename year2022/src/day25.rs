use commons::err;
use commons::error::Result;

pub const TITLE: &str = "Day 25: Full of Hot Air";

pub fn run(raw: String) -> Result<()> {
    let result = first_part(raw.as_str())?;
    println!("1. The sum is {result}");

    Ok(())
}

fn first_part(input: &str) -> Result<String> {
    input
        .lines()
        .try_fold(0, |acc, n| parse(n).map(|n| acc + n))
        .map(to_string)
}

fn parse(number: &str) -> Result<i64> {
    number.chars().try_fold(0, |acc, c| {
        let digit = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            bad => return Err(err!("bad digit: {bad:?} in number {number:?}")),
        };

        Ok(acc * 5 + digit)
    })
}

fn to_string(mut number: i64) -> String {
    let mut reverse_digits: Vec<char> = Vec::new();
    while number != 0 {
        let mut next = number.div_euclid(5);
        let digit = match number - next * 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            o => {
                next += 1;
                if o == 3 {
                    '='
                } else {
                    '-'
                }
            }
        };
        reverse_digits.push(digit);
        number = next;
    }

    reverse_digits.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/25.txt");
    const MAIN: &str = include_str!("../inputs/25.txt");

    #[test]
    fn first_part_example() {
        let result = first_part(EXAMPLE).unwrap();
        assert_eq!(result, "2=-1=0");
    }

    #[test]
    fn first_part_main() {
        let result = first_part(MAIN).unwrap();
        assert_eq!(result, "2011-=2=-1020-1===-1");
    }
}
