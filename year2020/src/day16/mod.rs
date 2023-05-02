use std::ops::RangeInclusive;
use std::str::FromStr;

use itertools::Itertools;
use std::collections::HashMap;

use commons::grid::Grid;
use commons::parse::sep_by_empty_lines;
use commons::{err, Report, Result, WrapErr};

pub const TITLE: &str = "Day 16: Ticket Translation";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!("The ticket scanning error rate is {}", data.error_rate);

    let headers = data
        .find_headers()
        .ok_or_else(|| err!("Could not find the headers for the tickets"))?;

    data.print_ticket(&headers)?;

    println!(
        "The product of the 'departure' header values is {}",
        data.departure_product(headers)
    );

    Ok(())
}

fn parse(s: &str) -> Result<Tickets> {
    s.parse()
}

/// The scanned tickets and their rules
struct Tickets {
    rules: HashMap<String, Rule>,
    ticket: Vec<u16>,
    valid: Grid<u16>,
    error_rate: u16,
}

impl Tickets {
    /// Find the corresponding headers for each column
    fn find_headers(&self) -> Option<Vec<&str>> {
        let width = self.ticket.len();

        // First compute the possible header for each column
        let mut possibilities: Vec<Vec<&str>> = vec![Vec::new(); width];
        self.rules.iter().for_each(|(name, rule)| {
            let mut valid: Vec<usize> = (0..width).collect();
            // Filter out columns where the rule is invalid at least once
            self.valid.lines().for_each(|line| {
                valid.retain(|idx| line.get(*idx).map_or(false, |num| rule.is_valid_for(num)));
            });

            valid
                .into_iter()
                .for_each(|idx| possibilities[idx].push(name.as_str()));
        });

        // Then assign the actual headers one by one
        let mut headers: Vec<Option<&str>> = vec![None; width];
        loop {
            let mut changes = 0;
            (0..width).for_each(|i| {
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
            });

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
    fn print_ticket(&self, headers: &[&str]) -> std::io::Result<()> {
        use std::io::prelude::*;
        use std::io::{stdout, BufWriter};
        let mut out = BufWriter::new(stdout());

        writeln!(out, "{}", "-".repeat(30))?;
        self.ticket
            .iter()
            .zip(headers)
            .try_for_each(|(&value, &header)| writeln!(out, "|{header:>20} |{value:>5} |"))?;
        writeln!(out, "{}", "-".repeat(30))?;

        Ok(())
    }

    /// The product of all values in our ticket that start with "departure"
    fn departure_product(&self, headers: Vec<&str>) -> u64 {
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
struct Rule(RangeInclusive<u16>, RangeInclusive<u16>);

impl Rule {
    /// Check if a number is valid for a rule
    fn is_valid_for(&self, number: &u16) -> bool {
        self.0.contains(number) || self.1.contains(number)
    }
}

impl FromStr for Rule {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        fn parse_range(s: &str) -> Result<RangeInclusive<u16>> {
            let (from, to) = s
                .split_once('-')
                .ok_or_else(|| err!("Expected rules for the tickets, got {}", s))?;

            Ok(parse_int(from)?..=parse_int(to)?)
        }

        let (first, second) = s
            .split_once("or")
            .ok_or_else(|| err!("Expected rules for the tickets, got {}", s))?;

        Ok(Rule(parse_range(first)?, parse_range(second)?))
    }
}

impl FromStr for Tickets {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (rule_section, ticket, others) = sep_by_empty_lines(s)
            .collect_tuple::<(_, _, _)>()
            .ok_or_else(|| {
                err!(
                    "Expected three blocks: rules, ticket and nearby tickets, but got {}",
                    s
                )
            })?;

        let ticket: Vec<_> = parse_line(ticket.strip_prefix("your ticket:").ok_or_else(|| {
            err!(
                "Expected 'your ticket:' followed by a comma separated numbers, got {}",
                ticket
            )
        })?)?;

        let mut rules = HashMap::with_capacity(ticket.len());
        for line in rule_section.lines() {
            let (name, rule) = line
                .split_once(':')
                .ok_or_else(|| err!("Expected rules for the tickets, got {}", s))?;

            rules.insert(name.to_owned(), rule.parse::<Rule>()?);
        }

        let nearby = others.strip_prefix("nearby tickets:").ok_or_else(|| {
            err!(
                "Expected 'nearby tickets:' followed by comma separated numbers per line, got {}",
                others
            )
        })?;

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
fn parse_int(s: &str) -> Result<u16> {
    s.trim()
        .parse()
        .wrap_err_with(|| format!("Failed to parse a string into an int: {s}"))
}

/// Parse a comma separated line with a ParseError
fn parse_line(s: &str) -> Result<Vec<u16>> {
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
) -> Result<(Grid<u16>, u16)> {
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
mod tests;
