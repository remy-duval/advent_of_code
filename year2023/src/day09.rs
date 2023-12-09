use commons::error::Result;
use commons::WrapErr;

pub const TITLE: &str = "Day 9: Mirage Maintenance";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The sum of forward extrapolated values is {first}");
    let second = second_part(&data);
    println!("2. The sum of backward extrapolated values is {second}");

    Ok(())
}

#[derive(Debug)]
struct History(Vec<i32>);

fn first_part(values: &[History]) -> i32 {
    values
        .iter()
        .filter_map(|v| derive(v, |v| v.last().copied(), |d_x, x| x + d_x))
        .sum()
}

fn second_part(values: &[History]) -> i32 {
    values
        .iter()
        .filter_map(|v| derive(v, |v| v.first().copied(), |d_x, x| x - d_x))
        .sum()
}

fn derive(
    values: &History,
    derived_value: impl Fn(&[i32]) -> Option<i32>,
    extrapolate: impl Fn(i32, i32) -> i32,
) -> Option<i32> {
    let mut derived = vec![derived_value(&values.0)?];
    let mut current = values.0.clone();
    while current.iter().any(|i| *i != 0) {
        (1..current.len()).for_each(|i| current[i - 1] = current[i] - current[i - 1]);
        current.pop();
        derived.push(derived_value(&current)?);
    }

    derived.into_iter().rev().reduce(extrapolate)
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<History>> {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .map(History)
                .wrap_err_with(|| format!("invalid numbers in line {line:?}"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/09.txt");
    const MAIN: &str = include_str!("../inputs/09.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 114);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 1_993_300_041);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 2);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 1_038);
    }
}
