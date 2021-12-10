use commons::eyre::Result;
use commons::Problem;

pub struct Day;

impl Problem for Day {
    type Input = String;
    const TITLE: &'static str = "Day 10: Syntax Scoring";

    fn solve(data: Self::Input) -> Result<()> {
        let (completion, errors) = check_all(&data);
        println!("1. Errors score {}", errors);
        println!("1. Completion score {}", completion);

        Ok(())
    }
}

/// Check all lines, returning `(completion_score, error_score)`
///
/// Total error score is the sum of all errored lines score
/// Total completion score is the median of all successful lines autocompletion
fn check_all(lines: &str) -> (u32, u64) {
    let mut buffer = Vec::with_capacity(100);
    let mut completions = Vec::with_capacity(100);
    let errors = lines
        .lines()
        .fold(0, |errors, line| match check(line, &mut buffer) {
            Ok(completed) => {
                completions.push(completed);
                errors
            }
            Err(bad) => errors + bad.error_score(),
        });

    completions.sort_unstable();
    if completions.is_empty() {
        (errors, 0)
    } else {
        (errors, completions[completions.len() / 2])
    }
}

/// Check if this line is syntactically correct, and autocomplete it if possible
///
/// ### Params
/// * `line` - The line to autocomplete
/// * `open` - A common buffer to avoid allocating too much
///
/// ### Returns
/// - `Ok(completion_score)` - if the line is correct and was autocompleted
/// - `Err(delimiter)` - if the line was incorrect, with the first bad delimiter
fn check(line: &str, open: &mut Vec<Delimiter>) -> Result<u64, Delimiter> {
    open.clear();
    for b in line.bytes() {
        if let Some(o) = Delimiter::open(b) {
            open.push(o);
        } else if let Some(c) = Delimiter::closing(b) {
            if open.pop().map_or(true, |x| c != x) {
                open.push(c);
                return Err(c);
            }
        }
    }

    let score = open
        .iter()
        .rev()
        .fold(0, |total, d| total * 5 + d.completion_score() as u64);
    Ok(score)
}

/// A delimiter in the syntax
#[derive(Copy, Clone, Eq, PartialEq)]
enum Delimiter {
    Brace,
    Bracket,
    Parens,
    Angled,
}

impl Delimiter {
    /// Parse this byte as an opening delimiter if possible
    fn open(c: u8) -> Option<Self> {
        match c {
            b'{' => Some(Self::Brace),
            b'[' => Some(Self::Bracket),
            b'(' => Some(Self::Parens),
            b'<' => Some(Self::Angled),
            _ => None,
        }
    }

    /// Parse this byte as a closing delimiter if possible
    fn closing(c: u8) -> Option<Self> {
        match c {
            b'}' => Some(Self::Brace),
            b']' => Some(Self::Bracket),
            b')' => Some(Self::Parens),
            b'>' => Some(Self::Angled),
            _ => None,
        }
    }

    /// The score of this delimiter when it is a syntax error
    fn error_score(self) -> u32 {
        match self {
            Delimiter::Brace => 1_197,
            Delimiter::Bracket => 57,
            Delimiter::Parens => 3,
            Delimiter::Angled => 25_137,
        }
    }

    /// The score of this delimiter when it is part of an auto-completion
    fn completion_score(self) -> u32 {
        match self {
            Delimiter::Brace => 3,
            Delimiter::Bracket => 2,
            Delimiter::Parens => 1,
            Delimiter::Angled => 4,
        }
    }
}

#[cfg(test)]
mod tests;
