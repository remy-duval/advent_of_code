use commons::error::Result;
use commons::WrapErr;

pub const TITLE: &str = "Day 8: Treetop Tree House";

pub fn run(raw: String) -> Result<()> {
    let data = parse(raw.into())?;
    let first = first_part(&data);
    println!("1. There are {first} visible trees");
    let second = second_part(&data);
    println!("2. The maximum possible scenic score is {second}");

    Ok(())
}

#[derive(Debug, Clone)]
struct Trees {
    width: usize,
    data: Vec<i8>,
    visibilities: Vec<Visibility>,
}

/// The maximum height of neighbours in each direction
#[derive(Debug, Clone, Default, Copy)]
struct Visibility {
    up: i8,
    right: i8,
    left: i8,
    down: i8,
    visible: bool,
}

fn first_part(trees: &Trees) -> usize {
    trees.visibilities.iter().filter(|v| v.visible).count()
}

fn second_part(trees: &Trees) -> u32 {
    let width = trees.width;
    let height = trees.data.len() / width;
    let indices = (0..height).flat_map(|i| (0..width).map(move |j| (i, j)));
    trees
        .visibilities
        .iter()
        .zip(trees.data.iter().copied().zip(indices))
        // Invisible trees will have a small view distance in each direction, leading to a low score
        .filter(|(v, _)| v.visible)
        // Boundaries will always be visible but have a score of 0
        .filter(|(_, (_, (i, j)))| (1..(height - 1)).contains(i) && (1..(width - 1)).contains(j))
        .map(|(view, (tree, (i, j)))| {
            // Find the view distance of the tree in each direction
            // If it is visible in that direction, then it is the distance to the edge
            // Otherwise march toward the edge until an obstacle is found
            let up = if tree > view.up {
                i
            } else {
                (0..i)
                    .rev()
                    .position(|i| trees.data[i * width + j] >= tree)
                    .map_or(0, |x| x + 1)
            };
            let left = if tree > view.left {
                j
            } else {
                (0..j)
                    .rev()
                    .position(|j| trees.data[i * width + j] >= tree)
                    .map_or(0, |x| x + 1)
            };
            let right = if tree > view.right {
                width - j - 1
            } else {
                ((j + 1)..width)
                    .position(|j| trees.data[i * width + j] >= tree)
                    .map_or(0, |x| x + 1)
            };
            let down = if tree > view.down {
                height - i - 1
            } else {
                ((i + 1)..height)
                    .position(|i| trees.data[i * width + j] >= tree)
                    .map_or(0, |x| x + 1)
            };

            (up * left * right * down) as u32
        })
        .max()
        .unwrap_or_default()
}

fn parse(s: std::borrow::Cow<'static, str>) -> Result<Trees> {
    let width = s.lines().next().wrap_err("Empty input")?.len();
    let data: Vec<_> = s
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| (c as u8 - b'0') as i8)
        .collect();

    let mut visibilities = vec![
        Visibility {
            up: -1,
            right: -1,
            left: -1,
            down: -1,
            visible: false
        };
        data.len()
    ];

    // Propagate maximum neighbour height up of each tree
    for i in width..data.len() {
        let up = i - width;
        let max_height = data[up].max(visibilities[up].up);
        visibilities[i].up = max_height;
    }

    // Propagate maximum neighbour height right of each tree
    for (i, step) in (0..data.len()).rev().zip((0..width).cycle()) {
        if step == 0 {
            continue;
        } else {
            let right = i + 1;
            let max_height = data[right].max(visibilities[right].right);
            visibilities[i].right = max_height;
        }
    }

    // Propagate maximum neighbour height left of each tree
    for (i, step) in (0..data.len()).zip((0..width).cycle()) {
        if step == 0 {
            continue;
        } else {
            let left = i - 1;
            let max_height = data[left].max(visibilities[left].left);
            visibilities[i].left = max_height;
        }
    }

    // Propagate maximum neighbour height down of each tree
    for i in (0..(data.len() - width)).rev() {
        let down = i + width;
        let max_height = data[down].max(visibilities[down].down);
        visibilities[i].down = max_height;
    }

    // Visibility can now be computed for each tree
    for (&tree, view) in data.iter().zip(visibilities.iter_mut()) {
        let visible = tree > view.up || tree > view.left || tree > view.right || tree > view.down;
        view.visible = visible;
    }

    Ok(Trees {
        width,
        data,
        visibilities,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/08.txt");
    const MAIN: &str = include_str!("../inputs/08.txt");

    #[test]
    fn first_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(first_part(&data), 21);
    }

    #[test]
    fn first_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(first_part(&data), 1_733);
    }

    #[test]
    fn second_part_example() {
        let data = parse(EXAMPLE.into()).unwrap();
        assert_eq!(second_part(&data), 8);
    }

    #[test]
    fn second_part_main() {
        let data = parse(MAIN.into()).unwrap();
        assert_eq!(second_part(&data), 284_648);
    }
}
