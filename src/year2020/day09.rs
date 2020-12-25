use itertools::{Itertools, MinMaxResult};

use crate::parse::LineSep;
use crate::Problem;

const PREAMBLE: usize = 25;

pub struct Day;

impl Problem for Day {
    type Input = LineSep<u64>;
    type Err = anyhow::Error;
    const TITLE: &'static str = "Day 9: Encoding Error";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        let wanted = first_not_sum(&data.data, PREAMBLE)
            .ok_or_else(|| anyhow::anyhow!("Did not find the first element that is not a sum"))?;

        println!(
            "The first element that is not a sum of the previous ones is {first}",
            first = wanted
        );

        let (min, max) = second_part(&data.data, wanted).ok_or_else(|| {
            anyhow::anyhow!("Did not find the slice that can be summed to {}", wanted)
        })?;

        println!(
            "The slice between {min} and {max} (sum is {sum}) will sum up to it",
            min = min,
            max = max,
            sum = min + max
        );

        Ok(())
    }
}

/// Find the first element in `data` that is not a sum of the `from_previous` previous elements
fn first_not_sum(data: &[u64], from_previous: usize) -> Option<u64> {
    (from_previous..data.len()).find_map(|end| {
        let start = end - from_previous;
        let value = data[end];
        if !is_sum(&data[start..end], value) {
            Some(value)
        } else {
            None
        }
    })
}

/// Find the first contiguous set that sum up to `wanted` in `data` and get its min and max
fn second_part(data: &[u64], wanted: u64) -> Option<(u64, u64)> {
    match contiguous_set(data, wanted)?.iter().minmax() {
        MinMaxResult::MinMax(min, max) => Some((*min, *max)),
        _ => None,
    }
}

/// Find the first contiguous set of at least 2 elements in `data` that sum up to `wanted`
fn contiguous_set(data: &[u64], wanted: u64) -> Option<&[u64]> {
    if data.is_empty() {
        return None;
    }

    let mut start = 0;
    let mut sum = data[0];
    (1..data.len()).find_map(|end| {
        sum += data[end];
        while sum > wanted && start < (end - 1) {
            sum -= data[start];
            start += 1;
        }
        if sum == wanted {
            Some(&data[start..(end + 1)])
        } else {
            None
        }
    })
}

/// Tell if the `wanted` number is a sum of any two numbers in `inside`
fn is_sum(inside: &[u64], wanted: u64) -> bool {
    inside.iter().enumerate().any(|(i, &first)| {
        if first <= wanted {
            let to_find = wanted - first;
            inside
                .iter()
                .enumerate()
                .any(|(j, &second)| i != j && second == to_find)
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = include_str!("test_resources/09-A.txt");
    const B: &str = include_str!("test_resources/09-B.txt");
    const A_EXPECTED: u64 = 127;
    const B_EXPECTED: u64 = 70639851;

    #[test]
    fn first_part_test_a() {
        let data = Day::parse(A).unwrap().data;
        let first = first_not_sum(&data, 5).expect("Should have been found");
        assert_eq!(A_EXPECTED, first);
    }

    #[test]
    fn first_part_test_b() {
        let data = Day::parse(B).unwrap().data;
        let first = first_not_sum(&data, PREAMBLE).expect("Should have been found");
        assert_eq!(B_EXPECTED, first);
    }

    #[test]
    fn contiguous_test_a() {
        let test = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let result = contiguous_set(&test, 13).expect("Should have been found");

        assert_eq!(&[6, 7], result);
    }

    #[test]
    fn contiguous_test_b() {
        let test = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let result = contiguous_set(&test, 15).expect("Should have been found");

        assert_eq!(&[0, 1, 2, 3, 4, 5], result);
    }

    #[test]
    fn contiguous_test_c() {
        let test = vec![0, 1, 2, 3, 4, 5, 6, 7];
        assert!(contiguous_set(&test, 0).is_none());
        assert!(contiguous_set(&test, 2).is_none());
        assert!(contiguous_set(&test, 23).is_none());
        assert!(contiguous_set(&test, 29).is_none());
        assert!(contiguous_set(&[], 1).is_none());
    }

    #[test]
    fn second_part_test_a() {
        let data = Day::parse(A).unwrap().data;
        let (min, max) = second_part(&data, 127).expect("Should have been found");
        assert_eq!(15, min);
        assert_eq!(47, max);
    }

    #[test]
    fn second_part_test_b() {
        let data = Day::parse(B).unwrap().data;
        let (min, max) = second_part(&data, B_EXPECTED).expect("Should have been found");
        assert_eq!(3474524, min);
        assert_eq!(4774716, max);
    }
}
