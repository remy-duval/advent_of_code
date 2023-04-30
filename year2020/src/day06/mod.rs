use std::str::FromStr;

use std::collections::HashSet;

use commons::eyre::Result;

pub const TITLE: &str = "Day 6: Custom Customs";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw);
    println!(
        "The total of YES answers for any participant of each group is {}",
        first_part(&data)
    );

    println!(
        "The total of YES answers for all participant of each group is {}",
        second_part(&data)
    );

    Ok(())
}

fn parse(s: &str) -> Vec<&str> {
    commons::parse::sep_by_empty_lines(s).collect()
}

/// Compute the sum of yes answers for any participant of a group
fn first_part(groups: &[&str]) -> usize {
    groups
        .iter()
        .map(|group| group.parse::<AnyYesAnswers>().map_or(0, |ans| ans.0))
        .sum()
}

/// Compute the sum of yes answers for all participants of a group
fn second_part(groups: &[&str]) -> usize {
    groups
        .iter()
        .map(|group| group.parse::<AllYesAnswers>().map_or(0, |ans| ans.0))
        .sum()
}

/// The answers that any member of a group answered YES for
struct AnyYesAnswers(usize);

impl FromStr for AnyYesAnswers {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut any_yes = HashSet::new();
        for line in s.lines() {
            any_yes.extend(line.chars());
        }
        Ok(Self(any_yes.len()))
    }
}

/// The answers that all members of a group answered YES for
struct AllYesAnswers(usize);

impl FromStr for AllYesAnswers {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        if let Some(first) = lines.next() {
            let mut all_yes: HashSet<char> = first.chars().collect();
            for line in lines {
                all_yes.retain(|current| line.contains(*current));
            }
            Ok(Self(all_yes.len()))
        } else {
            Ok(Self(0))
        }
    }
}

#[cfg(test)]
mod tests;
