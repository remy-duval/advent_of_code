use commons::error::Result;
use commons::parse::sep_by_empty_lines;

pub const TITLE: &str = "Day 13: Point of Incidence";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. The sum of reflection coordinates is {first}");
    let second = second_part(&data);
    println!("2. The sum of reflection coordinates with the mistake fixed is {second}");

    Ok(())
}

#[derive(Debug)]
struct Pattern {
    width: usize,
    height: usize,
    row_major: Vec<bool>,
    column_major: Vec<bool>,
}

fn first_part(patterns: &[Pattern]) -> usize {
    summarize::<false>(patterns)
}

fn second_part(patterns: &[Pattern]) -> usize {
    summarize::<true>(patterns)
}

fn summarize<const FIX: bool>(patterns: &[Pattern]) -> usize {
    let check_matching = |a: &[bool], b: &[bool], done_fixing: &mut bool| -> bool {
        if a == b {
            true
        } else if !*done_fixing {
            *done_fixing = true;
            a.iter().zip(b).filter(|(a, b)| a != b).count() == 1
        } else {
            false
        }
    };
    let find_reflection = |grid: &[bool], width: usize| -> Option<usize> {
        let mut mid = width;
        loop {
            let mut a = mid - width;
            let mut b = mid;
            let mut done_fixing = !FIX;
            while check_matching(
                grid.get(a..(a + width))?,
                grid.get(b..(b + width))?,
                &mut done_fixing,
            ) {
                b += width;
                if a < width || b >= grid.len() {
                    if done_fixing {
                        return Some(mid / width);
                    } else {
                        break;
                    }
                }
                a -= width;
            }
            mid += width;
        }
    };

    patterns
        .iter()
        .filter_map(|pat| {
            find_reflection(&pat.column_major, pat.height)
                .or_else(|| find_reflection(&pat.row_major, pat.width).map(|r| r * 100))
        })
        .sum()
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Vec<Pattern>> {
    Ok(sep_by_empty_lines(s.as_ref())
        .filter_map(|block| {
            let width = block.lines().next()?.len();
            let height = block.lines().count();
            let mut row_major = vec![false; width * height];
            let mut column_major = row_major.clone();
            block.lines().enumerate().for_each(|(row, line)| {
                line.bytes().enumerate().for_each(|(column, char)| {
                    if char == b'#' {
                        row_major[row * width + column] = true;
                        column_major[column * height + row] = true;
                    }
                });
            });

            Some(Pattern {
                width,
                height,
                row_major,
                column_major,
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/13.txt");
    const MAIN: &str = include_str!("../inputs/13.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 405);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 30_535);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 400);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 30_844);
    }
}
