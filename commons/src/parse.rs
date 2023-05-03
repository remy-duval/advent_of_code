//! Utilities for parsing the input files easily:
//! - [CommaSep](CommaSep) for parsing a comma separated list of values (whitespace is trimmed)
//! - [LineSep](LineSep) for parsing a value for each line of a text (whitespace is trimmed)

use std::str::FromStr;

/// An intermediate struct to parse commas-separated input.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommaSep<T> {
    /// The parsed data
    pub data: Vec<T>,
}

impl<T: FromStr> FromStr for CommaSep<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|elt| elt.trim().parse::<T>())
            .collect::<Result<Vec<_>, _>>()
            .map(|data| Self { data })
    }
}

/// An intermediate struct to parse lines separated input.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LineSep<T> {
    /// The parsed data
    pub data: Vec<T>,
}

impl<T: FromStr> FromStr for LineSep<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|elt| elt.trim().parse::<T>())
            .collect::<Result<Vec<_>, _>>()
            .map(|data| Self { data })
    }
}

/// An iterator over the parts of the string that are separated by empty new lines
pub fn sep_by_empty_lines(s: &str) -> impl Iterator<Item = &str> {
    s.split_terminator("\r\n\r\n")
        .flat_map(|blk| blk.split_terminator("\n\n"))
}

/// An intermediate struct to parse lines separated input.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SepByEmptyLine<T> {
    pub data: Vec<T>,
}

impl<T: FromStr> FromStr for SepByEmptyLine<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Since Windows exists, splitting on "\n\n" isn't enough
        sep_by_empty_lines(s)
            .map(|block| block.parse())
            .collect::<Result<Vec<_>, _>>()
            .map(|data| Self { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that the parser is successfully parsing the inner values after trimming them
    fn comma_sep_successful() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            [0, 1, 2, 3],
            "0, 1,2 , 3".parse::<CommaSep<i64>>()?.data.as_slice()
        );
        Ok(())
    }

    #[test]
    /// Test that when a sub-element fails parsing the entire parsing fails with that error
    fn comma_sep_on_failure() {
        assert!("0, 1,boom, 3".parse::<CommaSep<i64>>().is_err());
    }

    #[test]
    /// Test that the parser is successfully parsing the inner values after trimming them
    fn line_sep_success() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            [1, 2, 4],
            "1 \n 2 \n4  ".parse::<LineSep<i64>>()?.data.as_slice()
        );
        Ok(())
    }

    #[test]
    /// Test that when a sub-element fails parsing the entire parsing fails with that error
    fn line_sep_on_failure() {
        assert!("1 \n not an Int, definitely not \n4  "
            .parse::<LineSep<i64>>()
            .is_err());
    }

    #[test]
    /// Test that the parser is successfully parsing the inner values
    fn sep_by_empty_line_successful() {
        let first: SepByEmptyLine<LineSep<u8>> = "1\n2\n\n3\n4\n\n".parse().unwrap();
        let second: SepByEmptyLine<LineSep<u8>> = "1\r\n2\r\n\r\n3\r\n4\r\n\r\n".parse().unwrap();

        assert_eq!(
            first.data,
            vec![LineSep { data: vec![1, 2] }, LineSep { data: vec![3, 4] }]
        );
        assert_eq!(
            second.data,
            vec![LineSep { data: vec![1, 2] }, LineSep { data: vec![3, 4] }]
        );
    }

    #[test]
    /// Test that when a sub-element fails parsing the entire parsing fails with that error
    fn sep_by_empty_line_failure() {
        assert!("1\n2\n\nbad\n4\n\n"
            .parse::<SepByEmptyLine<LineSep<u8>>>()
            .is_err());
        assert!("1\r\n2\r\n\r\nbad\r\n4\r\n\r\n"
            .parse::<SepByEmptyLine<LineSep<u8>>>()
            .is_err());
    }
}
