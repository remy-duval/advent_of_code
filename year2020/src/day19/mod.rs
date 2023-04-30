use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use itertools::Itertools;
use std::collections::HashMap;

use commons::eyre::{eyre, Result};

pub const TITLE: &str = "Day 19: Monster Messages";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw);
    println!(
        "There are {} valid words in the input",
        first_part(&data).ok_or_else(|| eyre!("Failed to create the parser for P1"))?
    );

    println!(
        "There are {} valid words in the input after modifying the rules",
        second_part(data).ok_or_else(|| eyre!("Failed to create the parser for P2"))?
    );

    Ok(())
}

type BoxedParser = Rc<dyn Parser<()>>;

fn parse(s: &str) -> RulesAndWords {
    let mut blocks = commons::parse::sep_by_empty_lines(s);

    let rules = blocks.next().map_or_else(HashMap::new, |block| {
        block
            .lines()
            .filter_map(|line| line.split_once(':'))
            .map(|(key, value)| (key.trim().to_owned(), value.trim().to_owned()))
            .collect::<HashMap<_, _>>()
    });

    let words = blocks.next().map_or_else(Vec::new, |block| {
        block.lines().map(|line| line.to_owned()).collect()
    });

    RulesAndWords { rules, words }
}

fn first_part(data: &RulesAndWords) -> Option<usize> {
    let parser = And(
        build_parser("0", &data.rules, &mut HashMap::with_capacity(150))?,
        End,
    );

    Some(
        data.words
            .iter()
            .filter(|word| parser.parse(word.as_str()).is_some())
            .count(),
    )
}

fn second_part(data: RulesAndWords) -> Option<usize> {
    let mut cache = HashMap::with_capacity(150);
    let a = Rep(build_parser("42", &data.rules, &mut cache)?);
    let b = Rep(build_parser("31", &data.rules, &mut cache)?);

    // Rule 0 is 8 11
    // 8 is: 1+ of 42
    // 11 is matching number of 42 and 31
    // This means by just checking the input is only 42 + 31 and that the number of 42 is greater
    // We can check that the grammar is respected
    let check = |str: &str| {
        if let Some((a, rest)) = a.parse(str) {
            if let Some((b, rest)) = b.parse(rest) {
                return rest.is_empty() && a.len() > b.len();
            }
        }

        false
    };

    Some(
        data.words
            .iter()
            .filter(|word| check(word.as_str()))
            .count(),
    )
}

/// Build a parser of the given rule, caching results
fn build_parser(
    current: &str,
    raw_rules: &HashMap<String, String>,
    cached: &mut HashMap<String, BoxedParser>,
) -> Option<BoxedParser> {
    if let Some(cached) = cached.get(current) {
        Some(Rc::clone(cached))
    } else {
        let full_rule = raw_rules.get(current)?;

        let created = full_rule
            .split('|')
            .map(|rules| {
                rules
                    .split_whitespace()
                    .filter(|rule| !rule.is_empty())
                    .map(|rule| {
                        if rule.starts_with('"') {
                            // Handle terminal symbol in quotes
                            rule.chars()
                                .nth(1)
                                .map(|c| Rc::new(Void::new(Char(c))) as BoxedParser)
                        } else {
                            // Else recursively build the parser
                            build_parser(rule, raw_rules, cached)
                        }
                    })
                    .while_some()
                    .reduce(|a, b| Rc::new(Void::new(And(a, b))))
            })
            .while_some()
            .reduce(|a, b| Rc::new(Void::new(Or(a, b))))?;
        cached.insert(current.to_owned(), Rc::clone(&created));
        Some(created)
    }
}

/// The input of the days problem
struct RulesAndWords {
    /// Rules by name
    rules: HashMap<String, String>,
    /// The words to check
    words: Vec<String>,
}

/// A trait for formalizing parsing
trait Parser<T> {
    /// Parse part of the given slice, returning `Some((result, rest))` if it succeeds
    fn parse<'a>(&self, slice: &'a str) -> Option<(T, &'a str)>;
}

impl<T, P, Boxed> Parser<T> for Boxed
where
    P: Parser<T> + ?Sized,
    Boxed: Deref<Target = P>,
{
    fn parse<'a>(&self, slice: &'a str) -> Option<(T, &'a str)> {
        self.deref().parse(slice)
    }
}

/// A parser that succeeds only if there is nothing left to parse (the end of the string)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct End;

impl Parser<()> for End {
    fn parse<'a>(&self, slice: &'a str) -> Option<((), &'a str)> {
        if slice.is_empty() {
            Some(((), slice))
        } else {
            None
        }
    }
}

/// A parser that matches a specific character
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Char(char);

impl Parser<char> for Char {
    fn parse<'a>(&self, slice: &'a str) -> Option<(char, &'a str)> {
        let mut chars = slice.chars();
        if let Some(first) = chars.next() {
            if first == self.0 {
                Some((first, chars.as_str()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// A parser that applies an inner parser and discards its result
#[derive(Copy, Clone, Eq, PartialEq)]
struct Void<A, P: Parser<A>>(pub P, PhantomData<A>);

impl<A, P: Parser<A>> Void<A, P> {
    fn new(parser: P) -> Self {
        Self(parser, PhantomData)
    }
}

impl<A, P: Parser<A>> Parser<()> for Void<A, P> {
    fn parse<'a>(&self, slice: &'a str) -> Option<((), &'a str)> {
        let (_, slice) = self.0.parse(slice)?;
        Some(((), slice))
    }
}

/// A parser that succeeds if both inner parser succeeds, returning a tuple of the results
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct And<A, B>(A, B);

impl<A, B, ParseA, ParseB> Parser<(A, B)> for And<ParseA, ParseB>
where
    ParseA: Parser<A>,
    ParseB: Parser<B>,
{
    fn parse<'a>(&self, slice: &'a str) -> Option<((A, B), &'a str)> {
        let (a, slice) = self.0.parse(slice)?;
        let (b, slice) = self.1.parse(slice)?;
        Some(((a, b), slice))
    }
}

/// A parser that succeeds if either inner parser succeed
///
/// Returns a Result where Ok contains the result of the first, Err contains the second
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Or<A, B>(A, B);

impl<A, B, ParseA, ParseB> Parser<Result<A, B>> for Or<ParseA, ParseB>
where
    ParseA: Parser<A>,
    ParseB: Parser<B>,
{
    fn parse<'a>(&self, slice: &'a str) -> Option<(Result<A, B>, &'a str)> {
        if let Some((a, slice)) = self.0.parse(slice) {
            Some((Ok(a), slice))
        } else if let Some((b, slice)) = self.1.parse(slice) {
            Some((Err(b), slice))
        } else {
            None
        }
    }
}

/// A parser that parse 1+ number of the inner parser
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Rep<A>(A);

impl<A, P: Parser<A>> Parser<Vec<A>> for Rep<P> {
    fn parse<'a>(&self, slice: &'a str) -> Option<(Vec<A>, &'a str)> {
        let mut result = Vec::with_capacity(10);
        let mut rest = slice;
        while let Some((a, r)) = self.0.parse(rest) {
            rest = r;
            result.push(a);
        }

        if result.is_empty() {
            None
        } else {
            Some((result, rest))
        }
    }
}

#[cfg(test)]
mod tests;
