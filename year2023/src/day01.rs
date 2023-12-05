use commons::error::Result;

pub const TITLE: &str = "Day 1: Trebuchet?!";

pub fn run(raw: String) -> Result<()> {
    let first = first_part(raw.as_str());
    println!("1. The sum of calibration values is {first}");
    let second = second_part(raw.as_str());
    println!("2. The second sum of calibration is {second}");

    Ok(())
}

fn first_part(data: &str) -> u32 {
    data.lines()
        .filter_map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next()?;
            let last = digits.next_back().unwrap_or(first);
            Some(first * 10 + last)
        })
        .sum()
}

fn second_part(data: &str) -> u32 {
    data.lines()
        .filter_map(|line| {
            let prefixes = [
                '1', '2', '3', '4', '5', '6', '7', '8', '9', 'o', 't', 'f', 's', 'e', 'n',
            ];
            let search = |(idx, char): (usize, &str)| -> Option<u32> {
                let char = char.chars().next()?;
                let rest = line.get(idx..)?;
                match char {
                    '1'..='9' => Some(char as u32 - '0' as u32),
                    'o' if rest.starts_with("one") => Some(1),
                    't' if rest.starts_with("two") => Some(2),
                    't' if rest.starts_with("three") => Some(3),
                    'f' if rest.starts_with("four") => Some(4),
                    'f' if rest.starts_with("five") => Some(5),
                    's' if rest.starts_with("six") => Some(6),
                    's' if rest.starts_with("seven") => Some(7),
                    'e' if rest.starts_with("eight") => Some(8),
                    'n' if rest.starts_with("nine") => Some(9),
                    _ => None,
                }
            };
            let first = line.match_indices(&prefixes).find_map(search)?;
            let last = line.rmatch_indices(&prefixes).find_map(search)?;
            Some(first * 10 + last)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = include_str!("../examples/01_a.txt");
    const EXAMPLE_B: &str = include_str!("../examples/01_b.txt");
    const MAIN: &str = include_str!("../inputs/01.txt");

    #[test]
    fn first_part_example() {
        assert_eq!(first_part(EXAMPLE_A), 142);
    }

    #[test]
    fn first_part_main() {
        assert_eq!(first_part(MAIN), 55_538);
    }

    #[test]
    fn second_part_example() {
        assert_eq!(second_part(EXAMPLE_B), 281);
    }

    #[test]
    fn second_part_main() {
        assert_eq!(second_part(MAIN), 54_875);
    }
}
