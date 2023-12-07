use commons::error::Result;
use commons::{bail, WrapErr};

pub const TITLE: &str = "Day 7: Camel Cards";

pub fn run(raw: String) -> Result<()> {
    let mut data = parse(raw.into())?;
    let first = first_part(&mut data);
    println!("1. The total winnings with J = Jack are {first}");
    let second = second_part(&mut data);
    println!("2. The total winnings with J = Joker are {second}");

    Ok(())
}

#[derive(Debug)]
struct Poker {
    hand: Hand,
    bid: u16,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    strength: HandStrength,
    cards: [Card; 5],
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum HandStrength {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPairs,
    OnePair,
    HighCard,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl HandStrength {
    fn new(cards: [Card; 5]) -> HandStrength {
        let mut counts = [0u8; 13];
        let mut jokers = 0;
        for card in cards {
            match card {
                Card::Ace => counts[0] += 1,
                Card::King => counts[1] += 1,
                Card::Queen => counts[2] += 1,
                Card::Jack => counts[3] += 1,
                Card::Ten => counts[4] += 1,
                Card::Nine => counts[5] += 1,
                Card::Eight => counts[6] += 1,
                Card::Seven => counts[7] += 1,
                Card::Six => counts[8] += 1,
                Card::Five => counts[9] += 1,
                Card::Four => counts[10] += 1,
                Card::Three => counts[11] += 1,
                Card::Two => counts[12] += 1,
                Card::Joker => jokers += 1,
            }
        }

        let base = Self::from_counts(counts);
        match jokers {
            0 => base,
            1 => match base {
                Self::FourKind => Self::FiveKind,
                Self::ThreeKind => Self::FourKind,
                Self::TwoPairs => Self::FullHouse,
                Self::OnePair => Self::ThreeKind,
                Self::HighCard => Self::OnePair,
                _ => base,
            },
            2 => match base {
                Self::ThreeKind => Self::FiveKind,
                Self::OnePair => Self::FourKind,
                Self::HighCard => Self::ThreeKind,
                _ => base,
            },
            3 if matches!(base, Self::HighCard) => Self::FourKind,
            _ => Self::FiveKind,
        }
    }

    fn from_counts(counts: [u8; 13]) -> Self {
        let mut one_pair = false;
        let mut three = false;
        for count in counts {
            match count {
                0 | 1 => (),
                2 => {
                    if one_pair {
                        return Self::TwoPairs;
                    }
                    if three {
                        return Self::FullHouse;
                    }
                    one_pair = true;
                }
                3 => {
                    if one_pair {
                        return Self::FullHouse;
                    }
                    three = true;
                }
                4 => return Self::FourKind,
                _ => return Self::FiveKind,
            }
        }

        if one_pair {
            Self::OnePair
        } else if three {
            Self::ThreeKind
        } else {
            Self::HighCard
        }
    }
}

fn first_part(poker: &mut [Poker]) -> u64 {
    poker.sort_unstable_by_key(|p| p.hand);
    poker
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, poker)| (rank + 1) as u64 * poker.bid as u64)
        .sum()
}

fn second_part(poker: &mut [Poker]) -> u64 {
    // Convert Jacks to Jokers, and recompute the hand strength after that
    poker.iter_mut().for_each(|poker| {
        poker.hand.cards.iter_mut().for_each(|c| {
            if *c == Card::Jack {
                *c = Card::Joker;
            }
        });
        poker.hand.strength = HandStrength::new(poker.hand.cards);
    });

    first_part(poker)
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Poker>> {
    s.lines()
        .map(|line| {
            line.split_once(' ')
                .wrap_err("missing space")
                .and_then(|(hand, bid)| {
                    let bid = bid.trim().parse().wrap_err("bid is not a valid number")?;
                    let mut cards = [Card::Two; 5];
                    for (b, card) in hand.trim().bytes().zip(&mut cards) {
                        *card = match b {
                            b'A' => Card::Ace,
                            b'K' => Card::King,
                            b'Q' => Card::Queen,
                            b'J' => Card::Jack,
                            b'T' => Card::Ten,
                            b'9' => Card::Nine,
                            b'8' => Card::Eight,
                            b'7' => Card::Seven,
                            b'6' => Card::Six,
                            b'5' => Card::Five,
                            b'4' => Card::Four,
                            b'3' => Card::Three,
                            b'2' => Card::Two,
                            bad => bail!("unknown card '{bad}' in hand"),
                        };
                    }
                    Ok(Poker {
                        bid,
                        hand: Hand {
                            strength: HandStrength::new(cards),
                            cards,
                        },
                    })
                })
                .wrap_err_with(|| format!("in line {line:?}"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/07.txt");
    const MAIN: &str = include_str!("../inputs/07.txt");

    #[test]
    fn first_part_example() {
        let mut data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&mut data), 6440);
    }

    #[test]
    fn first_part_main() {
        let mut data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&mut data), 253_866_470);
    }

    #[test]
    fn second_part_example() {
        let mut data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&mut data), 5905);
    }

    #[test]
    fn second_part_main() {
        let mut data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&mut data), 254_494_947);
    }
}
