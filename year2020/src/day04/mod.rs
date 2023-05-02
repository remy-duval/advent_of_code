use itertools::Itertools;

use commons::parse::sep_by_empty_lines;
use commons::Result;

pub const TITLE: &str = "Day 4: Passport Processing";

pub fn run(data: String) -> Result<()> {
    let batch = PassportBuilder::parse_many(&data);
    println!(
        "{count} passports have all required fields",
        count = first_part(&batch)
    );
    println!("{count} passports are valid", count = second_part(&batch));

    Ok(())
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
struct Passport<'a> {
    /// pid (Passport ID)
    passport_id: &'a str,
    /// byr (Birth Year)
    birth: &'a str,
    /// iyr (Issue Year)
    issued: &'a str,
    /// eyr (Expiration Year)
    expiration: &'a str,
    /// hgt (Height)
    height: &'a str,
    /// hcl (Hair Color)
    hair_color: &'a str,
    /// ecl (Eye Color)
    eye_color: &'a str,
}

impl<'a> Passport<'a> {
    /// Check if all the passport data is valid
    fn is_valid(&self) -> bool {
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
        Self::year_valid(self.birth, 1920, 2002)
    }

    /// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    fn issued_valid(&self) -> bool {
        Self::year_valid(self.issued, 2010, 2020)
    }

    /// four digits; at least 2020 and at most 2030.
    fn expiration_valid(&self) -> bool {
        Self::year_valid(self.expiration, 2020, 2030)
    }

    /// hgt (Height) - a number followed by either cm or in:
    /// If cm, the number must be at least 150 and at most 193.
    /// If in, the number must be at least 59 and at most 76.
    fn height_valid(&self) -> bool {
        if let Some(cm) = self.height.strip_suffix("cm") {
            cm.parse::<u8>()
                .map_or(false, |height| (150..=193).contains(&height))
        } else if let Some(inches) = self.height.strip_suffix("in") {
            inches
                .parse::<u8>()
                .map_or(false, |height| (59..=76).contains(&height))
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
    fn parse_many(string: &'a str) -> Vec<Passport> {
        // Each passport is separated from the others by an empty new line
        // Since Windows exists, splitting on "\n\n" isn't enough
        sep_by_empty_lines(string)
            .filter_map(Self::parse_one)
            .collect_vec()
    }

    /// Parse a passport from a string slice
    /// ### Argumments
    /// * `string` - A string slice with one passport
    ///
    /// ### Returns
    /// A single passport, containg string slices of the original slice
    fn parse_one(string: &'a str) -> Option<Passport<'a>> {
        let mut builder = Self::default();
        string
            .split_whitespace()
            .filter_map(|kv| kv.split_once(':'))
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
mod tests;
