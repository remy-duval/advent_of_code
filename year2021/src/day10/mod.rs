use commons::eyre::Result;

pub const TITLE: &str = "Day 10: Syntax Scoring";

pub fn run(data: String) -> Result<()> {
    let (errors, completion) = check_all(&data);
    println!("1. Syntax error score is {}", errors);
    println!("2. Completion score is {}", completion);
    Ok(())
}

/// Check all lines, returning `(completion_score, error_score)`
///
/// Total error score is the sum of all errored lines score
/// Total completion score is the median of all successful lines autocompletion
fn check_all(lines: &str) -> (u32, u64) {
    let mut buf = Vec::with_capacity(64);
    let mut completions = Vec::with_capacity(64);
    let mut errors = 0;
    lines.lines().for_each(|line| match check(line, &mut buf) {
        Ok(completed) => completions.push(completed),
        Err(bad) => errors += bad.error_score(),
    });

    completions.sort_unstable();
    let completion = completions.get(completions.len() / 2).map_or(0, |x| *x);
    (errors, completion)
}

/// Check if this line is syntactically correct, and autocomplete it if possible
///
/// ### Params
/// * `line` - The line to autocomplete
/// * `open` - A common allocation for an open delimiters stack
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
            // Check if this closing delimiter matches the last open delimiter of the stack
            // If not it's a syntax error
            if open.pop().map_or(true, |x| c != x) {
                return Err(c);
            }
        }
    }

    // No errors, compute the completion score
    // This uses the reverse of the remaining open stack as the expected closing elements
    let score = open
        .iter()
        .rfold(0, |total, d| total * 5 + d.completion_score());
    Ok(score)
}

/// A delimiter in the syntax
#[derive(Copy, Clone, Eq, PartialEq)]
enum Delimiter {
    Parens,
    Bracket,
    Brace,
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
            Delimiter::Parens => 3,
            Delimiter::Bracket => 57,
            Delimiter::Brace => 1_197,
            Delimiter::Angled => 25_137,
        }
    }

    /// The score of this delimiter when it is part of an auto-completion
    fn completion_score(self) -> u64 {
        match self {
            Delimiter::Parens => 1,
            Delimiter::Bracket => 2,
            Delimiter::Brace => 3,
            Delimiter::Angled => 4,
        }
    }
}

#[cfg(test)]
mod tests;
