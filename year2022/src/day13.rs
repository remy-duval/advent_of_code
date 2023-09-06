use std::cmp::Ordering;
use std::str::FromStr;

use itertools::{EitherOrBoth, Itertools, Position};

use commons::error::Result;
use commons::parse::sep_by_empty_lines;
use commons::{err, Report, WrapErr};

pub const TITLE: &str = "Day 13: Distress Signal";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. Indices of packets in order sum up to {first}");
    let second = second_part(&data);
    println!("2. Indices of marker packets in sorted inputs times up to {second}");

    Ok(())
}

fn first_part(data: &[(Packet, Packet)]) -> usize {
    data.iter()
        .enumerate()
        .filter(|(_, (a, b))| a <= b)
        .map(|(i, _)| i + 1)
        .sum()
}

fn second_part(data: &[(Packet, Packet)]) -> usize {
    let first = Packet::List(vec![Packet::List(vec![Packet::One(2)])]);
    let second = Packet::List(vec![Packet::List(vec![Packet::One(6)])]);

    // Instead of sorting the packets, just shift the position of the two markers
    let mut first_position = 1;
    let mut second_position = 2;
    for (a, b) in data.iter() {
        if a < &second {
            second_position += 1;
            if a < &first {
                first_position += 1;
            }
        }

        if b < &second {
            second_position += 1;
            if b < &first {
                first_position += 1;
            }
        }
    }

    first_position * second_position
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    One(u8),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::One(a) => write!(f, "{a}"),
            Packet::List(b) if b.is_empty() => write!(f, "[]"),
            Packet::List(b) => b.iter().with_position().try_for_each(|(pos, p)| match pos {
                Position::First => write!(f, "[{p},"),
                Position::Middle => write!(f, "{p},"),
                Position::Last => write!(f, "{p}]"),
                Position::Only => write!(f, "[{p}]"),
            }),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        fn inner<'a>(
            a: impl Iterator<Item = &'a Packet>,
            b: impl Iterator<Item = &'a Packet>,
        ) -> Ordering {
            for zipped in a.zip_longest(b) {
                match zipped {
                    EitherOrBoth::Both(a, b) => match a.cmp(b) {
                        Ordering::Equal => (),
                        done => return done,
                    },
                    EitherOrBoth::Left(_) => return Ordering::Greater,
                    EitherOrBoth::Right(_) => return Ordering::Less,
                }
            }

            Ordering::Equal
        }

        match (self, other) {
            (Self::One(a), Self::One(b)) => a.cmp(b),
            (a @ Self::One(_), Self::List(b)) => inner(std::iter::once(a), b.iter()),
            (Self::List(a), b @ Self::One(_)) => inner(a.iter(), std::iter::once(b)),
            (Self::List(a), Self::List(b)) => inner(a.iter(), b.iter()),
        }
    }
}

impl FromStr for Packet {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        enum Token {
            Open,
            Close,
            Value(u8),
        }

        fn go(tokens: &mut impl Iterator<Item = Token>) -> Result<Option<Packet>> {
            match tokens.next() {
                Some(Token::Open) => {
                    let mut list = vec![];
                    while let Some(packet) = go(tokens)? {
                        list.push(packet);
                    }
                    Ok(Some(Packet::List(list)))
                }
                Some(Token::Value(v)) => Ok(Some(Packet::One(v))),
                Some(Token::Close) | None => Ok(None),
            }
        }

        let tokens = s.trim().split_inclusive([',', '[', ']']).flat_map(|s| {
            let (token, delim) = {
                let mut chars = s.chars();
                let delim = chars.next_back();
                (chars.as_str(), delim)
            };

            let first = match token.parse() {
                Ok(value) => Some(Ok(Token::Value(value))),
                Err(_) if token.is_empty() => None,
                Err(_) => Some(Err(err!("not a number: {s:?}"))),
            };
            let second = match delim {
                Some('[') => Some(Ok(Token::Open)),
                Some(']') => Some(Ok(Token::Close)),
                _ => None,
            };
            [first, second].into_iter().flatten()
        });

        itertools::process_results(tokens, |mut tokens| {
            go(&mut tokens).and_then(|opt| opt.wrap_err("empty input"))
        })
        .and_then(std::convert::identity)
        .wrap_err_with(|| format!("for {s:?}"))
    }
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<(Packet, Packet)>> {
    sep_by_empty_lines(s.as_ref())
        .filter_map(|block| block.lines().collect_tuple::<(_, _)>())
        .map(|(a, b)| Ok((a.parse()?, b.parse()?)))
        .collect::<Result<Vec<(Packet, Packet)>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/13.txt");
    const MAIN: &str = include_str!("../inputs/13.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 13);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 6_395);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 140);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 24_921);
    }
}
