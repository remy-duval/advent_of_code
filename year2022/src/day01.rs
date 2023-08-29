use commons::error::Result;
use commons::parse::{LineSep, SepByEmptyLine};

pub const TITLE: &str = "Day 1: Calorie Counting";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(data.as_slice());
    println!("1. The maximum count of calories in a group is {first}");
    let second = second_part(data.as_slice());
    println!("2. The sum of the three maximum calories count is {second}");

    Ok(())
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<u32>> {
    let split: SepByEmptyLine<LineSep<u32>> = s.parse()?;
    Ok(split
        .data
        .into_iter()
        .map(|calories| calories.data.into_iter().sum())
        .collect())
}

fn first_part(input: &[u32]) -> u32 {
    sum_n_max_calories::<1>(input)
}

fn second_part(input: &[u32]) -> u32 {
    sum_n_max_calories::<3>(input)
}

fn sum_n_max_calories<const N: usize>(input: &[u32]) -> u32 {
    let mut max_calories = [0; N];
    let end = N - 1;
    for &calories in input {
        // If the value is higher than any of the max, shift all the max after that and insert it
        if let Some(dest) = max_calories.into_iter().position(|v| v < calories) {
            max_calories.copy_within(dest..end, dest + 1);
            max_calories[dest] = calories;
        }
    }

    max_calories.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/01.txt");
    const MAIN: &str = include_str!("../inputs/01.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 24000);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 69795);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 45000);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 208437);
    }
}
