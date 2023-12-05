use itertools::Itertools;

use commons::error::Result;
use commons::WrapErr;

pub const TITLE: &str = "Day 3: Gear Ratios";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The sum of parts is {first}");
    let second = second_part(&data);
    println!("2. The sum of gear ratios is {second}");

    Ok(())
}

#[derive(Debug)]
struct Engine {
    rows: Vec<Row>,
}

#[derive(Debug, Default)]
struct Row {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct Number {
    index: u32,
    width: u32,
    value: u32,
}

#[derive(Debug)]
struct Symbol {
    index: u32,
    value: char,
}

fn first_part(engine: &Engine) -> u32 {
    fn find_symbol(line: &[Symbol], start: u32, end: u32) -> bool {
        line.iter()
            .find(|s| s.index >= start)
            .is_some_and(|s| s.index <= end)
    }

    std::iter::once(&Row::default())
        .chain(engine.rows.iter())
        .chain(std::iter::once(&Row::default()))
        .tuple_windows::<(_, _, _)>()
        .flat_map(|(prev, row, next)| {
            row.numbers
                .iter()
                .filter(|num| {
                    let start = num.index.saturating_sub(1);
                    let end = num.index + num.width;
                    find_symbol(&prev.symbols, start, end)
                        || find_symbol(&row.symbols, start, end)
                        || find_symbol(&next.symbols, start, end)
                })
                .map(|num| num.value)
        })
        .sum()
}

fn second_part(engine: &Engine) -> u64 {
    std::iter::once(&Row::default())
        .chain(engine.rows.iter())
        .chain(std::iter::once(&Row::default()))
        .tuple_windows::<(_, _, _)>()
        .flat_map(|(prev, row, next)| {
            row.symbols
                .iter()
                .filter(|s| s.value == '*')
                .filter_map(|s| {
                    let mut adjacent = prev
                        .numbers
                        .iter()
                        .chain(row.numbers.iter())
                        .chain(next.numbers.iter())
                        .filter(|num| {
                            let start = num.index.saturating_sub(1);
                            let end = num.index + num.width;
                            (start..=end).contains(&s.index)
                        });
                    let first = adjacent.next()?.value as u64;
                    let second = adjacent.next()?.value as u64;
                    // Exactly two adjacent numbers is required
                    if adjacent.next().is_none() {
                        Some(first * second)
                    } else {
                        None
                    }
                })
        })
        .sum()
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Engine> {
    s.lines()
        .map(|line| {
            let mut numbers = Vec::new();
            let mut symbols = Vec::new();
            line.match_indices(|c: char| !c.is_ascii_digit())
                .chain(std::iter::once((line.len(), "")))
                .try_fold(0, |prev, (index, symbol)| -> Result<usize> {
                    if prev < index {
                        if let Some(value) = line.get(prev..index) {
                            numbers.push(Number {
                                index: prev as u32,
                                width: (index - prev) as u32,
                                value: value.parse().wrap_err_with(|| format!("{value:?}"))?,
                            });
                        }
                    }
                    match symbol.chars().next() {
                        Some('.') | None => (),
                        Some(value) => symbols.push(Symbol {
                            index: index as u32,
                            value,
                        }),
                    }

                    Ok(index + 1)
                })?;
            Ok(Row { numbers, symbols })
        })
        .collect::<Result<Vec<Row>>>()
        .map(|rows| Engine { rows })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/03.txt");
    const MAIN: &str = include_str!("../inputs/03.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 4361);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 544_664);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 467_835);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 84_495_585);
    }
}
