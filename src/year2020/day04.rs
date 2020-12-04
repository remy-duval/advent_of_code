use itertools::Itertools;

use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = String;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 4: Passport Processing";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let batch = PassportBuilder::parse_many(&data);
        println!(
            "{count} passports have all required fields",
            count = first_part(&batch)
        );
        println!("{count} passports are valid", count = second_part(&batch));

        Ok(())
    }
}

fn first_part(passports: &[Passport<'_>]) -> usize {
    passports.len()
}

fn second_part(passports: &[Passport<'_>]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count()
}

/// A passport for the problem
pub struct Passport<'a> {
    /// pid (Passport ID)
    pub passport_id: &'a str,
    /// byr (Birth Year)
    pub birth: &'a str,
    /// iyr (Issue Year)
    pub issued: &'a str,
    /// eyr (Expiration Year)
    pub expiration: &'a str,
    /// hgt (Height)
    pub height: &'a str,
    /// hcl (Hair Color)
    pub hair_color: &'a str,
    /// ecl (Eye Color)
    pub eye_color: &'a str,
}

impl<'a> Passport<'a> {
    /// Check if all the passport data is valid
    pub fn is_valid(&self) -> bool {
        self.passport_id_valid()
            && self.birth_valid()
            && self.issued_valid()
            && self.expiration_valid()
            && self.height_valid()
            && self.hair_valid()
            && self.eye_valid()
    }

    /// pid (Passport ID) - a nine-digit number, including leading zeroes.
    fn passport_id_valid(&self) -> bool {
        self.passport_id.len() == 9 && self.passport_id.parse::<u64>().is_ok()
    }

    /// byr (Birth Year) - four digits; at least 1920 and at most 2002.
    fn birth_valid(&self) -> bool {
        Self::year_valid(&self.birth, 1920, 2002)
    }

    /// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    fn issued_valid(&self) -> bool {
        Self::year_valid(&self.issued, 2010, 2020)
    }

    /// four digits; at least 2020 and at most 2030.
    fn expiration_valid(&self) -> bool {
        Self::year_valid(&self.expiration, 2020, 2030)
    }

    /// hgt (Height) - a number followed by either cm or in:
    /// If cm, the number must be at least 150 and at most 193.
    /// If in, the number must be at least 59 and at most 76.
    fn height_valid(&self) -> bool {
        if let Some(cm) = self.height.strip_suffix("cm") {
            cm.parse::<u8>()
                .map_or(false, |height| height >= 150 && height <= 193)
        } else if let Some(inches) = self.height.strip_suffix("in") {
            inches
                .parse::<u8>()
                .map_or(false, |height| height >= 59 && height <= 76)
        } else {
            false
        }
    }

    /// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    fn hair_valid(&self) -> bool {
        if let Some(color) = self.hair_color.strip_prefix('#') {
            color.len() == 6 && u32::from_str_radix(color, 16).is_ok()
        } else {
            false
        }
    }

    /// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    fn eye_valid(&self) -> bool {
        matches!(
            self.eye_color,
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        )
    }

    fn year_valid(year: &str, min: u16, max: u16) -> bool {
        year.parse::<u16>()
            .map_or(false, |year| year >= min && year <= max)
    }
}

/// A builder for a Passport
#[derive(Default)]
struct PassportBuilder<'a> {
    passport_id: Option<&'a str>,
    birth: Option<&'a str>,
    issued: Option<&'a str>,
    expiration: Option<&'a str>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
}

impl<'a> PassportBuilder<'a> {
    /// Parse many passports from a string slice
    /// ### Arguments
    /// * `string` - A string slice with all the passports, delimited by two consecutives new lines
    ///
    /// ### Returns
    /// A Vector of passports, each containing string slices of the original slice
    pub fn parse_many(string: &'a str) -> Vec<Passport> {
        // Each passport is separated from the others by an empty new line
        // Since Windows exists, splitting on "\n\n" isn't enough
        string
            .split_terminator("\r\n\r\n")
            .flat_map(|s| s.split_terminator("\n\n"))
            .filter_map(Self::parse_one)
            .collect_vec()
    }

    /// Parse a passport from a string slice
    /// ### Argumments
    /// * `string` - A string slice with one passport
    ///
    /// ### Returns
    /// A single passport, containg string slices of the original slice
    pub fn parse_one(string: &'a str) -> Option<Passport<'a>> {
        let mut builder = Self::default();
        string
            .split_whitespace()
            .flat_map(|kv| kv.splitn(2, ':')) // key:value format
            .tuples::<(_, _)>() // Group as (key, value) tuples
            .for_each(|(key, value)| match key {
                "pid" => builder.passport_id = Some(value),
                "byr" => builder.birth = Some(value),
                "iyr" => builder.issued = Some(value),
                "eyr" => builder.expiration = Some(value),
                "hgt" => builder.height = Some(value),
                "hcl" => builder.hair_color = Some(value),
                "ecl" => builder.eye_color = Some(value),
                _ => {}
            });
        builder.build()
    }

    /// Build the Passport from this builder if all the required fields are presents
    fn build(self) -> Option<Passport<'a>> {
        Some(Passport {
            passport_id: self.passport_id?,
            birth: self.birth?,
            issued: self.issued?,
            expiration: self.expiration?,
            height: self.height?,
            hair_color: self.hair_color?,
            eye_color: self.eye_color?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = include_str!("test_resources/04-A.txt");
    const B: &str = include_str!("test_resources/04-B.txt");
    const INVALID: &str = include_str!("test_resources/04-invalid.txt");
    const VALID: &str = include_str!("test_resources/04-valid.txt");

    #[test]
    fn first_part_test_a() {
        let parsed = PassportBuilder::parse_many(A);
        assert_eq!(2, first_part(&parsed));
    }

    #[test]
    fn first_part_test_b() {
        let parsed = PassportBuilder::parse_many(B);
        assert_eq!(200, first_part(&parsed));
    }

    #[test]
    fn invalid_passports() {
        let parsed = PassportBuilder::parse_many(INVALID);
        assert_eq!(4, parsed.len());
        assert!(parsed.iter().all(|passport| !passport.is_valid()));
    }

    #[test]
    fn valid_passports() {
        let parsed = PassportBuilder::parse_many(VALID);
        assert_eq!(4, parsed.len());
        assert!(parsed.iter().all(|passport| passport.is_valid()));
    }

    #[test]
    fn second_part_a() {
        let parsed = PassportBuilder::parse_many(A);
        assert_eq!(2, second_part(&parsed));
    }

    #[test]
    fn second_part_b() {
        let parsed = PassportBuilder::parse_many(B);
        assert_eq!(116, second_part(&parsed));
    }
}
