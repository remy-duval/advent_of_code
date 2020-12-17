use std::ops::RangeInclusive;
use std::str::FromStr;

use hashbrown::HashMap;
use itertools::Itertools;

use crate::commons::grid2::Grid;
use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = Tickets;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 16: Ticket Translation";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        println!("The ticket scanning error rate is {}", data.error_rate);

        let headers = data
            .find_headers()
            .ok_or_else(|| anyhow::anyhow!("Could not find the headers for the tickets"))?;

        data.print_ticket(&headers)?;

        println!(
            "The product of the 'departure' header values is {}",
            data.departure_product(headers)
        );

        Ok(())
    }
}

/// The scanned tickets and their rules
#[derive(Debug, Clone)]
pub struct Tickets {
    rules: HashMap<String, Rule>,
    ticket: Vec<u16>,
    valid: Grid<u16>,
    error_rate: u16,
}

impl Tickets {
    /// Find the corresponding headers for each column
    pub fn find_headers(&self) -> Option<Vec<&str>> {
        let width = self.ticket.len();

        // First compute the possible header for each column
        let mut possibilities: Vec<Vec<&str>> = vec![Vec::new(); width];
        for (name, rule) in &self.rules {
            let mut valid: Vec<usize> = (0..width).collect();
            // Filter out columns where the rule is invalid at least once
            self.valid.lines().for_each(|line| {
                valid.retain(|idx| line.get(*idx).map_or(false, |num| rule.is_valid_for(num)));
            });

            for idx in valid {
                possibilities[idx].push(name.as_str());
            }
        }

        // Then assign the actual headers one by one
        let mut headers: Vec<Option<&str>> = vec![None; width];
        loop {
            let mut changes = 0;
            for i in 0..width {
                // This relies on the assumption that there will never be 2+ headers that are
                // Equally possible for a column with no tie breaker in other columns
                // This should be faster than computing arrangements until we find one that works
                if possibilities[i].len() == 1 {
                    changes += 1;
                    let ok = possibilities[i][0];
                    headers[i] = Some(ok);
                    for possibility in possibilities.iter_mut() {
                        possibility.retain(|&str| str != ok);
                    }
                }
            }

            // Break the loop if no header has been assigned this pass
            // This should trigger only when everything has been assigned (see assumption above)
            if changes == 0 {
                break;
            }
        }

        // If all the headers have been found this will be Some, else None
        headers.into_iter().collect()
    }

    /// Print the ticket to stdout
    pub fn print_ticket(&self, headers: &[&str]) -> std::io::Result<()> {
        use std::io::prelude::*;
        use std::io::{stdout, BufWriter};
        let mut out = BufWriter::new(stdout());

        writeln!(out, "{}", "-".repeat(30))?;
        for i in 0..headers.len() {
            writeln!(out, "|{:>20} |{:>5} |", headers[i], self.ticket[i])?;
        }
        writeln!(out, "{}", "-".repeat(30))?;

        Ok(())
    }

    /// The product of all values in our ticket that start with "departure"
    pub fn departure_product(&self, headers: Vec<&str>) -> u64 {
        headers
            .into_iter()
            .enumerate()
            .filter_map(|(idx, header)| {
                if header.starts_with("departure") {
                    self.ticket.get(idx)
                } else {
                    None
                }
            })
            .map(|&n| n as u64) // This will overflow otherwise
            .product()
    }
}

/// The rule for a field
#[derive(Debug, Clone)]
pub struct Rule(RangeInclusive<u16>, RangeInclusive<u16>);

impl Rule {
    /// Check if a number is valid for a rule
    pub fn is_valid_for(&self, number: &u16) -> bool {
        self.0.contains(number) || self.1.contains(number)
    }
}

/// An error that could be thrown when parsing a ticket
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Expected three blocks: rules, ticket and nearby tickets, but got {0}")]
    MissingBlock(Box<str>),
    #[error("Expected rules for the tickets, got {0}")]
    BadRulesSection(Box<str>),
    #[error("Expected 'your ticket:' followed by a comma separated numbers, got {0}")]
    BadTicketSection(Box<str>),
    #[error("Expected 'nearby tickets:' followed by comma separated numbers per line, got {0}")]
    BadNearbySection(Box<str>),
    #[error("Failed to parse a string into an int: {0} ({1})")]
    NumberFormat(Box<str>, std::num::ParseIntError),
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_range(s: &str) -> Result<RangeInclusive<u16>, ParseError> {
            let (from, to) = s
                .splitn(2, '-')
                .collect_tuple::<(_, _)>()
                .ok_or_else(|| ParseError::BadRulesSection(s.into()))?;

            Ok(parse_int(from)?..=parse_int(to)?)
        }

        let (first, second) = s
            .splitn(2, "or")
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| ParseError::BadRulesSection(s.into()))?;

        Ok(Rule(parse_range(first)?, parse_range(second)?))
    }
}

impl FromStr for Tickets {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rule_section, ticket, others) = s
            .split("\n\n")
            .flat_map(|blk| blk.split("\r\n\r\n"))
            .collect_tuple::<(_, _, _)>()
            .ok_or_else(|| ParseError::MissingBlock(s.into()))?;

        let ticket: Vec<_> = parse_line(
            ticket
                .strip_prefix("your ticket:")
                .ok_or_else(|| ParseError::BadTicketSection(ticket.into()))?,
        )?;

        let mut rules = HashMap::with_capacity(ticket.len());
        for line in rule_section.lines() {
            let (name, rule) = line
                .splitn(2, ':')
                .collect_tuple::<(_, _)>()
                .ok_or_else(|| ParseError::BadRulesSection(rule_section.into()))?;

            rules.insert(name.to_owned(), rule.parse::<Rule>()?);
        }

        let nearby = others
            .strip_prefix("nearby tickets:")
            .ok_or_else(|| ParseError::BadNearbySection(others.into()))?;

        let (valid, error_rate) = parse_valid_tickets(nearby, ticket.len(), &rules)?;

        Ok(Self {
            rules,
            ticket,
            valid,
            error_rate,
        })
    }
}

/// Parse an integer with a ParseError
fn parse_int(s: &str) -> Result<u16, ParseError> {
    s.trim()
        .parse()
        .map_err(|e| ParseError::NumberFormat(s.into(), e))
}

/// Parse a comma separated line with a ParseError
fn parse_line(s: &str) -> Result<Vec<u16>, ParseError> {
    s.split(',')
        .filter(|l| !l.is_empty())
        .map(parse_int)
        .try_collect()
}

/// Parse the valid tickets in a string
fn parse_valid_tickets(
    s: &str,
    width: usize,
    rules: &HashMap<String, Rule>,
) -> Result<(Grid<u16>, u16), ParseError> {
    let mut others = Grid::new(width, 0);
    let mut error_rate = 0;
    for line in s.lines().filter(|line| !line.is_empty()) {
        let line = parse_line(line)?;
        let (valid, errors) = line.iter().fold((true, 0), |(valid, err), n| {
            if rules.values().any(|rule| rule.is_valid_for(n)) {
                (valid, err)
            } else {
                (false, err + n)
            }
        });
        if valid {
            others.push_line(line);
        } else {
            error_rate += errors;
        }
    }

    Ok((others, error_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("test_resources/16-A.txt");
    const MAIN: &str = include_str!("test_resources/16-B.txt");

    #[test]
    fn first_part_example() {
        let input = Day::parse(EXAMPLE).unwrap();
        assert_eq!(71, input.error_rate);
    }

    #[test]
    fn first_part_main() {
        let input = Day::parse(MAIN).unwrap();
        assert_eq!(27870, input.error_rate);
    }

    #[test]
    fn second_part_example() {
        let input = Day::parse(EXAMPLE).unwrap();
        let headers = input.find_headers().unwrap();
        assert_eq!(&headers, &["row", "class", "seat"]);
    }

    #[test]
    fn second_part_main() {
        let input = Day::parse(MAIN).unwrap();
        let headers = input.find_headers().unwrap();

        assert_eq!(
            &headers,
            &[
                "price",
                "train",
                "duration",
                "seat",
                "arrival location",
                "departure location",
                "arrival track",
                "zone",
                "arrival station",
                "route",
                "departure date",
                "arrival platform",
                "row",
                "departure track",
                "wagon",
                "type",
                "class",
                "departure platform",
                "departure station",
                "departure time",
            ]
        );

        assert_eq!(input.departure_product(headers), 3_173_135_507_987);
    }
}
