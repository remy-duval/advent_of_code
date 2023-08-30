use std::str::FromStr;

use itertools::Itertools;

use commons::error::{Result, WrapErr};
use commons::parse::{sep_by_empty_lines, LineSep};
use commons::{err, Report};

pub const TITLE: &str = "";

pub fn run(raw: String) -> Result<()> {
    let (stacks, moves) = parse(raw.into())?;
    let first = first_part(stacks.clone(), &moves);
    println!("1. Top crates after the moves one-by-one are '{first}'");
    let second = second_part(stacks, &moves);
    println!("1. Top crates after the moves in bulks are '{second}'");

    Ok(())
}

fn top_crates(stacks: Stacks) -> String {
    stacks
        .0
        .into_iter()
        .map(|mut s| s.pop().unwrap_or(' '))
        .collect()
}

fn first_part(mut stacks: Stacks, moves: &[Move]) -> String {
    for mv in moves {
        let from = (mv.from - 1) as usize;
        let to = (mv.to - 1) as usize;
        for _ in 0..(mv.count) {
            if let Some(c) = stacks.0[from].pop() {
                stacks.0[to].push(c);
            }
        }
    }

    top_crates(stacks)
}

fn second_part(mut stacks: Stacks, moves: &[Move]) -> String {
    let mut temp = Vec::new();
    for mv in moves {
        let from = (mv.from - 1) as usize;
        let to = (mv.to - 1) as usize;

        let src = &mut stacks.0[from];
        let remains = src.len() - mv.count as usize;
        temp.extend(src.drain(remains..));
        stacks.0[to].append(&mut temp);
    }

    top_crates(stacks)
}

#[derive(Clone, Debug)]
struct Stacks(Vec<Vec<char>>);

impl FromStr for Stacks {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut stacks = Vec::new();
        for line in s.lines().rev().filter(|l| l.contains('[')) {
            let mut e = line.chars().dropping(1);
            for (i, c) in e
                .next()
                .into_iter()
                .chain(e.batching(|e| e.nth(3)))
                .enumerate()
            {
                while i >= stacks.len() {
                    stacks.push(Vec::new());
                }

                let stack = &mut stacks[i];
                match c {
                    c @ 'A'..='Z' => stack.push(c),
                    ' ' => (),
                    _ => return Err(err!("unknown stack letter {c}")),
                }
            }
        }

        Ok(Self(stacks))
    }
}

#[derive(Clone, Debug)]
struct Move {
    count: u8,
    from: u8,
    to: u8,
}

impl FromStr for Move {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let elements = s.trim().split_ascii_whitespace();
        if let Some(("move", count, "from", from, "to", to)) = elements.collect_tuple() {
            let count = count.parse().wrap_err_with(|| format!("For {count}"))?;
            let from = from.parse().wrap_err_with(|| format!("For {from}"))?;
            let to = to.parse().wrap_err_with(|| format!("For {to}"))?;
            Ok(Self { count, from, to })
        } else {
            Err(err!("Wanted 'move COUNT from FROM to TO'"))
        }
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<(Stacks, Vec<Move>)> {
    if let Some((s, m)) = sep_by_empty_lines(s.as_ref()).collect_tuple() {
        let stacks: Stacks = s.parse().wrap_err_with(|| format!("For stacks: '{s}'"))?;
        let moves: LineSep<Move> = m.parse().wrap_err_with(|| format!("For moves '{m}'"))?;
        Ok((stacks, moves.data))
    } else {
        Err(err!("Bad format: '{s}'"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/05.txt");
    const MAIN: &str = include_str!("../inputs/05.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(data.0, &data.1).as_str(), "CMZ");
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(data.0, &data.1).as_str(), "HNSNMTLHQ");
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(data.0, &data.1).as_str(), "MCD");
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(data.0, &data.1).as_str(), "RNLFDJMCT");
    }
}
