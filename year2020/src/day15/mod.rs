use commons::parse::CommaSep;
use commons::Result;

pub const TITLE: &str = "Day 15: Rambunctious Recitation";
const FIRST_TURNS: u32 = 2020;
const SECOND_TURNS: u32 = 30000000;

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    let first = nth_spoken_number(&data.data, FIRST_TURNS);
    println!("The {FIRST_TURNS}th spoken is {first}");

    let second = nth_spoken_number(&data.data, SECOND_TURNS);
    println!("The {SECOND_TURNS}th spoken is {second}");

    Ok(())
}

fn parse(s: &str) -> Result<CommaSep<u32>> {
    Ok(s.parse()?)
}

/// Compute the nth term using a Vec as the backing memoization:
///
/// Use a Vec of fixed length max of (prefix values, turns) + 1
/// For any `number`, the value at the index `number` means:
/// 0 => number was never spoken,
/// t => number was spoken on turn n
fn nth_spoken_number(prefix: &[u32], n: u32) -> u32 {
    let size = prefix.iter().copied().max().unwrap_or_default().max(n);
    let mut spoken: Vec<u32> = vec![0; (size as usize) + 1];

    let mut current = 0;
    let mut index = 0;
    prefix.iter().for_each(|&number| {
        index += 1;
        spoken[number as usize] = index;
        current = number;
    });

    (index..n).for_each(|turn| {
        if let Some(last) = spoken.get_mut(current as usize) {
            let spoken_at = *last;
            current = if spoken_at != 0 { turn - spoken_at } else { 0 };
            *last = turn;
        }
    });

    current
}

#[cfg(test)]
mod tests;
