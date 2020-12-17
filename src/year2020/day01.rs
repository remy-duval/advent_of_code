pub struct Day;

impl crate::Problem for Day {
    type Input = crate::parse::LineSep<u64>;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 1: Report Repair";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let (first, second) = first_part(&data.data).ok_or(anyhow::anyhow!(
            "No 2020 2-elements sum found in {:?}",
            data.data
        ))?;
        println!(
            "2 expenses that sum to 2020: {a} * {b} = {product}",
            a = first,
            b = second,
            product = first * second
        );

        let (first, second, third) = second_part(&data.data).ok_or(anyhow::anyhow!(
            "No 2020 3-elements sum found in {:?}",
            data.data
        ))?;

        println!(
            "3 expenses that sum to 2020: {a} * {b} * {c} = {product}",
            a = first,
            b = second,
            c = third,
            product = first * second * third
        );
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
        let to_find = if check_index(i) && first <= wanted {
            wanted - first
        } else {
            continue;
        };
        for (j, second) in data.iter().copied().enumerate() {
            if check_index(j) && i != j && second == to_find {
                return Some((first, second));
            }
        }
    }
    None
}

fn first_part(expenses: &[u64]) -> Option<(u64, u64)> {
    find_sum(expenses, 2020, |_| true)
}

fn second_part(expenses: &[u64]) -> Option<(u64, u64, u64)> {
    for (i, a) in expenses.iter().copied().enumerate() {
        let wanted = if a <= 2020 { 2020 - a } else { continue; };
        if let Some((b, c)) = find_sum(expenses, wanted, |idx| i != idx) {
            return Some((a, b, c));
        }
    }
    None
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
            .parse::<<Day as Problem>::Input>()
            .expect("parse error");
        let (first, second) = first_part(&data.data).expect("result should have been found");
        assert_eq!(first * second, 514579);
    }

    #[test]
    fn first_part_test_second_data() {
        let data = SECOND_DATA
            .parse::<<Day as Problem>::Input>()
            .expect("parse error");
        let (first, second) = first_part(&data.data).expect("result should have been found");
        assert_eq!(first * second, 969024);
    }

    #[test]
    fn second_part_test_first_data() {
        let data = FIRST_DATA
            .parse::<<Day as Problem>::Input>()
            .expect("parse error");
        let (first, second, third) =
            second_part(&data.data).expect("result should have been found");
        assert_eq!(first * second * third, 241861950);
    }

    #[test]
    fn second_part_test_second_data() {
        let data = SECOND_DATA
            .parse::<<Day as Problem>::Input>()
            .expect("parse error");
        let (first, second, third) =
            second_part(&data.data).expect("result should have been found");
        assert_eq!(first * second * third, 230057040);
    }
}
