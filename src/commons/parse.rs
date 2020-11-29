//! Utilities for parsing the input files easily:
//! - [CommaSep](CommaSep) for parsing a comma separated list of values (whitespace is trimmed)
//! - [LineSep](LineSep) for parsing a value for each line of a text (whitespace is trimmed)

use std::str::FromStr;

/// An intermediate struct to parse commas-separated input.
#[derive(Debug, Clone)]
pub struct CommaSep<T> {
    /// The parsed data
    pub data: Vec<T>,
}

impl<T: FromStr> FromStr for CommaSep<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: s
                .split(',')
                .map(|elt| elt.trim().parse::<T>())
                .collect::<Result<_, _>>()?,
        })
    }
}

/// An intermediate struct to parse lines separated input.
#[derive(Debug, Clone)]
pub struct LineSep<T> {
    /// The parsed data
    pub data: Vec<T>,
}

impl<T: FromStr> FromStr for LineSep<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: s
                .lines()
                .map(|elt| elt.trim().parse::<T>())
                .collect::<Result<_, _>>()?,
        })
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
}
