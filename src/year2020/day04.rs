use std::str::FromStr;

use crate::Problem;

pub struct Day;

impl Problem for Day {
    type Input = PassportBatch;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 4: Passport Processing";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        println!(
            "{count} passports have all required fields",
            count = first_part(&data.0)
        );
        println!("{count} passports are valid", count = second_part(&data.0));

        Ok(())
    }
}

fn first_part(passports: &[Passport]) -> usize {
    passports.len()
}

fn second_part(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count()
}

/// A batch of passports
pub struct PassportBatch(pub Vec<Passport>);

impl FromStr for PassportBatch {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn first_two<'a>(mut iter: impl Iterator<Item = &'a str>) -> Option<(&'a str, &'a str)> {
            Some((iter.next()?, iter.next()?))
        }

        let mut result = Vec::new();
        let mut builder = PassportBuilder::default();
        for line in s.lines() {
            if line.is_empty() {
                // If the line is empty, the current passport is complete
                if let Some(passport) = std::mem::take(&mut builder).build() {
                    result.push(passport);
                }
            } else {
                // Else the line contains some data for the current passport
                for kv in line.split_whitespace() {
                    if let Some((key, value)) = first_two(kv.splitn(2, ':')) {
                        match key {
                            "pid" => builder.passport_id = Some(value.into()),
                            "byr" => builder.birth = Some(value.into()),
                            "iyr" => builder.issued = Some(value.into()),
                            "eyr" => builder.expiration = Some(value.into()),
                            "hgt" => builder.height = Some(value.into()),
                            "hcl" => builder.hair_color = Some(value.into()),
                            "ecl" => builder.eye_color = Some(value.into()),
                            _ => {}
                        }
                    }
                }
            }
        }

        // Don't forget the last passport
        if let Some(passport) = std::mem::take(&mut builder).build() {
            result.push(passport);
        }

        Ok(PassportBatch(result))
    }
}

/// A passport for the problem
pub struct Passport {
    /// pid (Passport ID)
    pub passport_id: String,
    /// byr (Birth Year)
    pub birth: String,
    /// iyr (Issue Year)
    pub issued: String,
    /// eyr (Expiration Year)
    pub expiration: String,
    /// hgt (Height)
    pub height: String,
    /// hcl (Hair Color)
    pub hair_color: String,
    /// ecl (Eye Color)
    pub eye_color: String,
}

impl Passport {
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
            self.eye_color.as_str(),
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
pub struct PassportBuilder {
    pub passport_id: Option<String>,
    pub birth: Option<String>,
    pub issued: Option<String>,
    pub expiration: Option<String>,
    pub height: Option<String>,
    pub hair_color: Option<String>,
    pub eye_color: Option<String>,
}

impl PassportBuilder {
    /// Build the Passport from this builder if all the required fields are presents
    fn build(self) -> Option<Passport> {
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
        let parsed: Vec<Passport> = Day::parse(A).unwrap().0;
        assert_eq!(2, first_part(&parsed));
    }

    #[test]
    fn first_part_test_b() {
        let parsed: Vec<Passport> = Day::parse(B).unwrap().0;
        assert_eq!(200, first_part(&parsed));
    }

    #[test]
    fn invalid_passports() {
        let parsed: Vec<Passport> = Day::parse(INVALID).unwrap().0;
        assert_eq!(4, parsed.len());
        assert!(parsed.iter().all(|passport| !passport.is_valid()));
    }

    #[test]
    fn valid_passports() {
        let parsed: Vec<Passport> = Day::parse(VALID).unwrap().0;
        assert_eq!(4, parsed.len());
        assert!(parsed.iter().all(|passport| passport.is_valid()));
    }

    #[test]
    fn second_part_a() {
        let parsed: Vec<Passport> = Day::parse(A).unwrap().0;
        assert_eq!(2, second_part(&parsed));
    }

    #[test]
    fn second_part_b() {
        let parsed: Vec<Passport> = Day::parse(B).unwrap().0;
        assert_eq!(116, second_part(&parsed));
    }
}
