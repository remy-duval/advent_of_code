use std::collections::BTreeMap;

use itertools::Itertools;

use commons::error::Result;

pub const TITLE: &str = "Day 11: Cosmic Expansion";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(data.clone());
    println!("1. The sum of distances between galaxies is {first}");
    let second = second_part(data);
    println!("2. The sum of distances between very old galaxies is {second}");

    Ok(())
}

type Galaxies = Vec<(u64, Vec<u64>)>;

fn first_part(galaxies: Galaxies) -> u64 {
    sum_distances(expand(galaxies, 2))
}

fn second_part(galaxies: Galaxies) -> u64 {
    sum_distances(expand(galaxies, 1_000_000))
}

fn sum_distances(galaxies: Galaxies) -> u64 {
    galaxies
        .into_iter()
        .flat_map(|(y, line)| line.into_iter().map(move |x| (x, y)))
        .tuple_combinations::<(_, _)>()
        .map(|((x_a, y_a), (x_b, y_b))| x_a.abs_diff(x_b) + y_a.abs_diff(y_b))
        .sum()
}

fn expand(mut galaxies: Galaxies, times: u64) -> Galaxies {
    let max_x = galaxies.iter().flat_map(|(_, line)| line.iter()).max();
    let expand_x: BTreeMap<u64, u64> = (0..max_x.map_or(0, |x| *x + 1))
        .scan(0, |increase, x| {
            if galaxies.iter().any(|(_, line)| line.contains(&x)) {
                Some(Some((x, x + *increase)))
            } else {
                *increase += times - 1;
                Some(None)
            }
        })
        .flatten()
        .collect();

    let mut prev = 0;
    let mut prev_expanded = 0;
    galaxies.iter_mut().for_each(|(y, ine)| {
        ine.iter_mut()
            .for_each(|x| *x = expand_x.get(x).map_or(0, |&x| x));
        let diff = y.abs_diff(prev);
        let empty_rows = diff.saturating_sub(1);
        let expanded = prev_expanded + empty_rows * (times - 1) + diff;
        prev = *y;
        prev_expanded = expanded;
        *y = expanded;
    });

    galaxies
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Galaxies> {
    let galaxies: Galaxies = s
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            let line: Vec<u64> = line
                .trim()
                .char_indices()
                .filter_map(|(x, c)| (c == '#').then_some(x as u64))
                .collect();

            if line.is_empty() {
                None
            } else {
                Some((y as u64, line))
            }
        })
        .collect();
    Ok(galaxies)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/11.txt");
    const MAIN: &str = include_str!("../inputs/11.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(data), 374);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(data), 9_627_977);
    }

    #[test]
    fn second_part_example_a() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(sum_distances(expand(data, 10)), 1030);
    }

    #[test]
    fn second_part_example_b() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(sum_distances(expand(data, 100)), 8410);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(data), 644_248_339_497);
    }
}
