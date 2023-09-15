use commons::error::Result;
use commons::WrapErr;

pub const TITLE: &str = "Day 20: Grove Positioning System";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(data.clone());
    println!("1. Coordinates sum is {first}");
    let second = second_part(data);
    println!("2. Improved coordinates sum is {second}");

    Ok(())
}

fn first_part(mut values: Vec<Element>) -> isize {
    mix(&mut values);
    extract_answer(&values)
}

fn second_part(mut values: Vec<Element>) -> isize {
    values.iter_mut().for_each(|v| v.value *= 811589153);
    (0..10).for_each(|_| mix(&mut values));
    extract_answer(&values)
}

fn extract_answer(values: &[Element]) -> isize {
    let zero = values.iter().position(|e| e.value == 0).unwrap_or_default();
    let one = (0..1000).fold(zero, |i, _| values[i].next);
    let two = (0..1000).fold(one, |i, _| values[i].next);
    let three = (0..1000).fold(two, |i, _| values[i].next);
    values[one].value + values[two].value + values[three].value
}

fn mix(values: &mut [Element]) {
    for start in 0..values.len() {
        do_move(values, start);
    }
}

fn do_move(values: &mut [Element], start: usize) {
    let count = values[start].value;
    // Modulo LEN - 1 as the value is "in flight" and so does not count at that moment
    let moves = count.abs() % (values.len() as isize - 1);
    let end = match count {
        0 => return,
        c if c > 0 => (0..moves).fold(start, |i, _| values[i].next),
        _ => (0..(moves + 1)).fold(start, |i, _| values[i].previous),
    };

    let after_end = std::mem::replace(&mut values[end].next, start);
    let after_start = std::mem::replace(&mut values[start].next, after_end);
    let before_start = std::mem::replace(&mut values[start].previous, end);
    values[before_start].next = after_start;
    values[after_start].previous = before_start;
    values[after_end].previous = start;
}

#[derive(Debug, Clone)]
struct Element {
    previous: usize,
    next: usize,
    value: isize,
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Element>> {
    let count = s.lines().count() - 1;
    s.lines()
        .enumerate()
        .map(|(pos, num)| {
            num.trim()
                .parse()
                .wrap_err_with(|| format!("Could not parse {s}"))
                .map(|value| {
                    let (previous, next) = if pos == 0 {
                        (count, pos + 1)
                    } else if pos == count {
                        (pos - 1, 0)
                    } else {
                        (pos - 1, pos + 1)
                    };
                    Element {
                        previous,
                        next,
                        value,
                    }
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/20.txt");
    const MAIN: &str = include_str!("../inputs/20.txt");

    fn print(values: &[Element]) -> String {
        use itertools::Itertools;
        (0..values.len())
            .scan(0, |i, _| {
                let current = &values[*i];
                *i = current.next;
                Some(current.value)
            })
            .join(", ")
    }

    #[test]
    fn move_test_1() {
        let mut values = parse("4\n5\n6\n1\n7\n8\n9".into()).unwrap();
        assert_eq!(print(&values), "4, 5, 6, 1, 7, 8, 9");
        do_move(&mut values, 3);
        assert_eq!(print(&values), "4, 5, 6, 7, 1, 8, 9");
    }

    #[test]
    fn move_test_2() {
        let mut values = parse("4\n-2\n5\n6\n7\n8\n9".into()).unwrap();
        assert_eq!(print(&values), "4, -2, 5, 6, 7, 8, 9");
        do_move(&mut values, 1);
        assert_eq!(print(&values), "4, 5, 6, 7, 8, -2, 9");
    }

    #[test]
    fn move_test_3() {
        let mut values = parse("4\n-1\n5\n6\n7\n8\n9".into()).unwrap();
        assert_eq!(print(&values), "4, -1, 5, 6, 7, 8, 9");
        do_move(&mut values, 1);
        assert_eq!(print(&values), "4, 5, 6, 7, 8, 9, -1");
    }

    #[test]
    fn move_test_4() {
        let mut values = parse("4\n-1\n5\n6\n7\n8\n-9".into()).unwrap();
        assert_eq!(print(&values), "4, -1, 5, 6, 7, 8, -9");
        do_move(&mut values, 6);
        assert_eq!(print(&values), "4, -1, 5, -9, 6, 7, 8");
    }

    #[test]
    fn moves_test() {
        let mut values = parse(EXAMPLE.into()).unwrap();
        assert_eq!(print(&values), "1, 2, -3, 3, -2, 0, 4");
        do_move(&mut values, 0);
        assert_eq!(print(&values), "1, -3, 3, -2, 0, 4, 2");
        do_move(&mut values, 1);
        assert_eq!(print(&values), "1, -3, 2, 3, -2, 0, 4");
        do_move(&mut values, 2);
        assert_eq!(print(&values), "1, 2, 3, -2, -3, 0, 4");
        do_move(&mut values, 3);
        assert_eq!(print(&values), "1, 2, -2, -3, 0, 3, 4");
        do_move(&mut values, 4);
        assert_eq!(print(&values), "1, 2, -3, 0, 3, 4, -2");
        do_move(&mut values, 5);
        assert_eq!(print(&values), "1, 2, -3, 0, 3, 4, -2");
        do_move(&mut values, 6);
        assert_eq!(print(&values), "1, 2, -3, 4, 0, 3, -2");
    }

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(data), 3);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(data), 7278);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(data), 1_623_178_306);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(data), 14_375_678_667_089);
    }
}
