use std::collections::HashMap;
use std::time::Instant;

use crate::parse::CommaSep;
use crate::Problem;

const FIRST_TURNS: u32 = 2020;
const SECOND_TURNS: u32 = 30000000;

/// This day is based on the [Van Eck sequence](https://www.numberphile.com/videos/van-eck-sequence)
///
/// The definition of the Van Eck sequence is:
/// * `VanEck(0)` = `0`
/// * `VanEck(n+1)` =
///   * if `VanEck(n)` exists in `VanEck(0..n)` `n - prev_occ`
///   * else `0`
///
/// In this our the base case is overridden with the given `prefix`:
/// Such that `VanEck(0..prefix.len())` = `prefix`
pub struct Day;

impl Problem for Day {
    type Input = CommaSep<u32>;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 15: Rambunctious Recitation";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        println!("Part 1:");
        let using_vec = Instant::now();
        let first = nth_via_vector(&data.data, FIRST_TURNS);
        let using_vec = using_vec.elapsed();
        println!("The {}th spoken is {}", FIRST_TURNS, first);

        let using_map = Instant::now();
        assert_eq!(first, nth_via_hash_map(&data.data, FIRST_TURNS));
        let using_map = using_map.elapsed();
        println!("\nUsing Vec:     {}s", using_vec.as_secs_f64());
        println!("Using HashMap: {}s", using_map.as_secs_f64());

        println!("\nPart 2:");
        let using_vec = Instant::now();
        let second = nth_via_vector(&data.data, SECOND_TURNS);
        let using_vec = using_vec.elapsed();
        println!("The {}th spoken is {}", SECOND_TURNS, second);

        let using_map = Instant::now();
        assert_eq!(second, nth_via_hash_map(&data.data, SECOND_TURNS));
        let using_map = using_map.elapsed();
        println!("\nUsing Vec:     {}s", using_vec.as_secs_f64());
        println!("Using HashMap: {}s", using_map.as_secs_f64());

        Ok(())
    }
}

/// Compute the nth term using a Vec as the backing memoization:
///
/// Use a Vec of fixed length max of (prefix values, turns) + 1
/// For any `number`, the value at the index `number` means:
/// 0 => number was never spoken,
/// t => number was spoken on turn n
fn nth_via_vector(prefix: &[u32], turns: u32) -> u32 {
    // Compute the appropriate size for the spoken array
    let size = prefix.iter().copied().max().unwrap_or_default().max(turns);
    let mut spoken: Vec<u32> = vec![0; (size as usize) + 1];

    let mut current = 0;
    let mut index = 1;
    for number in prefix.iter().copied() {
        spoken[number as usize] = index;
        current = number;
        index += 1;
    }

    for turn in (prefix.len() as u32)..turns {
        let spoken_at = spoken[current as usize];
        spoken[current as usize] = turn;
        if spoken_at == 0 {
            current = 0;
        } else {
            current = turn - spoken_at;
        }
    }

    current
}

/// Compute the nth term using an HashMap as the backing memoization
fn nth_via_hash_map(prefix: &[u32], turns: u32) -> u32 {
    let mut spoken = HashMap::new();
    spoken.reserve((turns as usize) / 10000); // Allocate some memory up front

    let mut current = 0;
    let mut index = 1;
    for number in prefix.iter().copied() {
        spoken.insert(number, index);
        current = number;
        index += 1;
    }

    for turn in (prefix.len() as u32)..turns {
        // Insert will return the previous element if there was one
        current = match spoken.insert(current, turn) {
            Some(previous) => turn - previous,
            None => 0,
        };
    }

    current
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0,3,6";
    const A: &str = "1,3,2";
    const B: &str = "2,1,3";
    const C: &str = "1,2,3";
    const D: &str = "2,3,1";
    const E: &str = "3,2,1";
    const F: &str = "3,1,2";
    const INPUT: &str = "8,11,0,19,1,2";

    #[test]
    fn first_part_example() {
        let input = Day::parse(EXAMPLE).unwrap();
        assert_eq!(nth_via_vector(&input.data, 4), 0);
        assert_eq!(nth_via_vector(&input.data, 5), 3);
        assert_eq!(nth_via_vector(&input.data, 6), 3);
        assert_eq!(nth_via_vector(&input.data, 7), 1);
        assert_eq!(nth_via_vector(&input.data, 8), 0);
        assert_eq!(nth_via_vector(&input.data, 9), 4);
        assert_eq!(nth_via_vector(&input.data, 10), 0);

        assert_eq!(nth_via_hash_map(&input.data, 4), 0);
        assert_eq!(nth_via_hash_map(&input.data, 5), 3);
        assert_eq!(nth_via_hash_map(&input.data, 6), 3);
        assert_eq!(nth_via_hash_map(&input.data, 7), 1);
        assert_eq!(nth_via_hash_map(&input.data, 8), 0);
        assert_eq!(nth_via_hash_map(&input.data, 9), 4);
        assert_eq!(nth_via_hash_map(&input.data, 10), 0);
    }

    #[test]
    fn first_part_test_a() {
        let input = Day::parse(A).unwrap();
        let result = nth_via_vector(&input.data, FIRST_TURNS);
        assert_eq!(result, nth_via_hash_map(&input.data, FIRST_TURNS));
        assert_eq!(result, 1);
    }

    #[test]
    fn first_part_test_b() {
        let input = Day::parse(B).unwrap();
        let result = nth_via_vector(&input.data, FIRST_TURNS);
        assert_eq!(result, nth_via_hash_map(&input.data, FIRST_TURNS));
        assert_eq!(result, 10);
    }

    #[test]
    fn first_part_test_c() {
        let input = Day::parse(C).unwrap();
        let result = nth_via_vector(&input.data, FIRST_TURNS);
        assert_eq!(result, nth_via_hash_map(&input.data, FIRST_TURNS));
        assert_eq!(result, 27);
    }

    #[test]
    fn first_part_test_d() {
        let input = Day::parse(D).unwrap();
        let result = nth_via_vector(&input.data, FIRST_TURNS);
        assert_eq!(result, nth_via_hash_map(&input.data, FIRST_TURNS));
        assert_eq!(result, 78);
    }

    #[test]
    fn first_part_test_e() {
        let input = Day::parse(E).unwrap();
        let result = nth_via_vector(&input.data, FIRST_TURNS);
        assert_eq!(result, nth_via_hash_map(&input.data, FIRST_TURNS));
        assert_eq!(result, 438);
    }

    #[test]
    fn first_part_test_f() {
        let input = Day::parse(F).unwrap();
        let result = nth_via_vector(&input.data, FIRST_TURNS);
        assert_eq!(result, nth_via_hash_map(&input.data, FIRST_TURNS));
        assert_eq!(result, 1836);
    }

    #[test]
    fn first_part_test_input() {
        let input = Day::parse(INPUT).unwrap();
        let result = nth_via_vector(&input.data, FIRST_TURNS);
        assert_eq!(result, nth_via_hash_map(&input.data, FIRST_TURNS));
        assert_eq!(result, 447);
    }

    #[test]
    fn second_part_test_input() {
        let input = Day::parse(INPUT).unwrap();
        let result = nth_via_vector(&input.data, SECOND_TURNS);
        assert_eq!(result, 11_721_679);
    }
}
