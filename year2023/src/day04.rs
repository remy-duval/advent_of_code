use commons::error::{Result, WrapErr};

pub const TITLE: &str = "Day 4: Scratchcards";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The cards are worth {first} points");
    let second = second_part(&data);
    println!("2. You will end up with {second} cards");

    Ok(())
}

#[derive(Debug)]
struct Card {
    winning: Vec<u8>,
    current: Vec<u8>,
}

fn first_part(cards: &[Card]) -> u32 {
    cards
        .iter()
        .filter_map(|c| count_matches(c).checked_sub(1).map(|pow| 1 << pow as u32))
        .sum()
}

fn second_part(cards: &[Card]) -> u64 {
    let mut card_worth = vec![1; cards.len()];
    for i in (0..cards.len()).rev() {
        let start = i + 1;
        let end = (count_matches(&cards[i]) + start).min(cards.len());
        let total = card_worth.get(start..end).map_or(0, |s| s.iter().sum());

        card_worth[i] += total;
    }

    card_worth.iter().sum()
}

fn count_matches(card: &Card) -> usize {
    card.winning
        .iter()
        .filter(|i| card.current.contains(*i))
        .count()
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Card>> {
    s.lines()
        .filter_map(|s| s.split_once(':').and_then(|(_, s)| s.split_once('|')))
        .map(|(winning_str, current_str)| {
            let winning = winning_str
                .trim()
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<u8>, _>>()
                .wrap_err_with(|| format!("for winning in '{winning_str} | {current_str}'"))?;
            let current = current_str
                .trim()
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<u8>, _>>()
                .wrap_err_with(|| format!("for current in '{winning_str} | {current_str}'"))?;
            Ok(Card { winning, current })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/04.txt");
    const MAIN: &str = include_str!("../inputs/04.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 13);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 22_674);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 30);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 5_747_443);
    }
}
