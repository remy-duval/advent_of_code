use commons::error::Result;
use commons::{ensure, WrapErr};

pub const TITLE: &str = "Day 6: Tuning Trouble";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data)?;
    println!("1. Found the start of packet marker at position {first}");
    let second = second_part(&data)?;
    println!("2. Found the start of message marker at position {second}");

    Ok(())
}

fn first_part(stream: &[u8]) -> Result<usize> {
    find_marker(stream, 4)
}

fn second_part(stream: &[u8]) -> Result<usize> {
    find_marker(stream, 14)
}

fn find_marker(stream: &[u8], count: usize) -> Result<usize> {
    let bits = stream.iter().map(|c| 1 << (*c as u32));
    // Find 'count' distinct consecutive bits by running a bitset of the last 'count' bits
    // At each step:
    // - XOR the next bit to add it to the set
    // - XOR the last bit before the 'count' window to remove it from the set
    // - Because XOR removes if present, a bit will only be set if it is present exactly once
    // - We have found the wanted position when there are 'count' bits in the bitset
    bits.clone()
        .zip(std::iter::repeat(0).take(count).chain(bits))
        .scan(0u32, |bitset, (next_included_bit, next_excluded_bit)| {
            *bitset ^= next_included_bit;
            *bitset ^= next_excluded_bit;
            Some(*bitset)
        })
        .position(|bitset| bitset.count_ones() >= count as u32)
        .map(|position| position + 1)
        .wrap_err_with(|| format!("end of stream without finding {count} distinct bits"))
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<u8>> {
    let line = s.lines().next().wrap_err("Empty input")?;
    line.chars()
        .map(|c| {
            ensure!(c.is_ascii_lowercase(), "non alphabetic character");
            Ok(c as u8 - b'a')
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/06.txt");
    const MAIN: &str = include_str!("../inputs/06.txt");

    #[test]
    fn first_part_example_1() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data).unwrap(), 7);
    }

    #[test]
    fn first_part_example_2() {
        let data = parse("bvwbjplbgvbhsrlpgdmjqwftvncz".into()).unwrap();
        assert_eq!(first_part(&data).unwrap(), 5);
    }

    #[test]
    fn first_part_example_3() {
        let data = parse("nppdvjthqldpwncqszvftbrmjlhg".into()).unwrap();
        assert_eq!(first_part(&data).unwrap(), 6);
    }

    #[test]
    fn first_part_example_4() {
        let data = parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into()).unwrap();
        assert_eq!(first_part(&data).unwrap(), 10);
    }

    #[test]
    fn first_part_example_5() {
        let data = parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into()).unwrap();
        assert_eq!(first_part(&data).unwrap(), 11);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data).unwrap(), 1757);
    }

    #[test]
    fn second_part_example_1() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 19);
    }

    #[test]
    fn second_part_example_2() {
        let data = parse("bvwbjplbgvbhsrlpgdmjqwftvncz".into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 23);
    }

    #[test]
    fn second_part_example_3() {
        let data = parse("nppdvjthqldpwncqszvftbrmjlhg".into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 23);
    }

    #[test]
    fn second_part_example_4() {
        let data = parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 29);
    }

    #[test]
    fn second_part_example_5() {
        let data = parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 26);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data).unwrap(), 2950);
    }
}
