use commons::error::Result;
use commons::{bail, WrapErr};

pub const TITLE: &str = "Day 15: Lens Library";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let first = first_part(data);
    println!("1. The hash of the steps sum to {first}");
    let second = second_part(data)?;
    println!("2. The focusing power of the resulting lens is {second}");

    Ok(())
}

fn first_part(data: &str) -> u32 {
    data.split(',')
        .fold(0u32, |acc, next| acc + hash(next) as u32)
}

fn second_part(data: &str) -> Result<usize> {
    let mut map: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    data.split(',').try_for_each(|step| -> Result<()> {
        if let Some(key) = step.strip_suffix('-') {
            match map.get_mut(hash(key) as usize) {
                Some(slots) => {
                    if let Some(i) = slots.iter().position(|(k, _)| key == *k) {
                        slots.remove(i);
                    }
                }
                None => bail!("missing slot for step {step}"),
            };
        } else if let Some((key, lens)) = step.split_once('=') {
            match (map.get_mut(hash(key) as usize), lens.parse()) {
                (Some(slots), Ok(lens)) => match slots.iter_mut().find(|(k, _)| key == *k) {
                    Some((_, slot)) => *slot = lens,
                    None => slots.push((key, lens)),
                },
                (None, _) => bail!("missing slot for step {step}"),
                _ => bail!("bad lens {lens} for step {step}"),
            };
        } else {
            bail!("unknown step {step}");
        }
        Ok(())
    })?;

    let sum = map
        .into_iter()
        .enumerate()
        .flat_map(|(box_idx, content)| {
            content
                .into_iter()
                .enumerate()
                .map(move |(slot, (_, lens))| (box_idx + 1) * (slot + 1) * lens)
        })
        .sum();

    Ok(sum)
}

fn hash(label: &str) -> u8 {
    label
        .bytes()
        .fold(0, |acc, c| ((acc as u32 + c as u32) * 17 % 256) as u8)
}

fn parse(s: &str) -> Result<&str> {
    s.lines().next().wrap_err("empty input")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/15.txt");
    const MAIN: &str = include_str!("../inputs/15.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE).unwrap();
        assert_eq!(first_part(&data), 1320);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN).unwrap();
        assert_eq!(first_part(&data), 505_379);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE).unwrap();
        assert_eq!(second_part(data).unwrap(), 145);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN).unwrap();
        assert_eq!(second_part(data).unwrap(), 263_211);
    }
}
