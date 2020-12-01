pub struct Day01;

impl crate::Problem for Day01 {
    type Input = crate::parse::LineSep<u64>;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 1: Report Repair";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        match first_part(&data.data) {
            None => {
                return Err(anyhow::anyhow!(
                    "No 2020 2-elements sum found in {:?}",
                    data.data
                ))
            }
            Some((first, second)) => {
                println!("Found entries {} and {} that sum to 2020", first, second);
                println!("Their product is {}", first * second);
            }
        }
        match second_part(&data.data) {
            None => {
                return Err(anyhow::anyhow!(
                    "No 2020 3-elements sum found in {:?}",
                    data.data
                ))
            }
            Some((first, second, third)) => {
                println!(
                    "Found entries {}, {} and {} that sum to 2020",
                    first, second, third
                );
                println!("Their product is {}", first * second * third);
            }
        }
        Ok(())
    }
}

/// Find two elements at acceptable indexes in the given slice that sum to the wanted value
///
/// # Arguments
/// * `data` - an integer slice that contains the number to sum
/// * `wanted` - the wanted integer
/// * `check_index` - a predicate on an index in the slice, if false the value is ignored
///
/// # Returns
/// Option of the found elements as a tuple (first, second)
fn find_sum(data: &[u64], wanted: u64, check_index: impl Fn(usize) -> bool) -> Option<(u64, u64)> {
    for (i, first) in data.iter().copied().enumerate() {
        if check_index(i) {
            for (j, second) in data.iter().copied().enumerate() {
                if check_index(j) && i != j && first + second == wanted {
                    return Some((first, second));
                }
            }
        }
    }
    return None;
}

fn first_part(expenses: &[u64]) -> Option<(u64, u64)> {
    find_sum(expenses, 2020, |_| true)
}

fn second_part(expenses: &[u64]) -> Option<(u64, u64, u64)> {
    for (i, a) in expenses.iter().copied().enumerate() {
        if let Some((b, c)) = find_sum(expenses, 2020 - a, |idx| i != idx) {
            return Some((a, b, c));
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    use super::*;

    const FIRST_DATA: &str = include_str!("test_resources/01-A.txt");
    const SECOND_DATA: &str = include_str!("test_resources/01-B.txt");

    #[test]
    fn first_part_test_first_data() {
        let data = FIRST_DATA
            .parse::<<Day01 as Problem>::Input>()
            .expect("parse error");
        let (first, second) = first_part(&data.data).expect("result should have been found");
        assert_eq!(first * second, 514579);
    }

    #[test]
    fn first_part_test_second_data() {
        let data = SECOND_DATA
            .parse::<<Day01 as Problem>::Input>()
            .expect("parse error");
        let (first, second) = first_part(&data.data).expect("result should have been found");
        assert_eq!(first * second, 969024);
    }

    #[test]
    fn second_part_test_first_data() {
        let data = FIRST_DATA
            .parse::<<Day01 as Problem>::Input>()
            .expect("parse error");
        let (first, second, third) =
            second_part(&data.data).expect("result should have been found");
        assert_eq!(first * second * third, 241861950);
    }

    #[test]
    fn second_part_test_second_data() {
        let data = SECOND_DATA
            .parse::<<Day01 as Problem>::Input>()
            .expect("parse error");
        let (first, second, third) =
            second_part(&data.data).expect("result should have been found");
        assert_eq!(first * second * third, 230057040);
    }
}
