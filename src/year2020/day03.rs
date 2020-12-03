use std::str::FromStr;

pub struct Day03;

impl crate::Problem for Day03 {
    type Input = Forest;
    type Err = std::convert::Infallible;
    const TITLE: &'static str = "Day 3: Toboggan Trajectory";

    fn solve(data: Self::Input) -> Result<(), Self::Err> {
        println!(
            "Using a slope of (3, 1), you will encounter {} trees",
            first_part(&data)
        );
        println!(
            "The product trees numbers at slopes (1, 1), (3, 1), (5, 1), (7, 1) and (1, 2) is {}",
            second_part(&data)
        );
        Ok(())
    }
}

fn first_part(data: &Forest) -> usize {
    count_tree_at_slope(data, (3, 1))
}

fn second_part(data: &Forest) -> usize {
    let a: usize = count_tree_at_slope(data, (1, 1));
    let b: usize = count_tree_at_slope(data, (3, 1));
    let c: usize = count_tree_at_slope(data, (5, 1));
    let d: usize = count_tree_at_slope(data, (7, 1));
    let e: usize = count_tree_at_slope(data, (1, 2));

    a * b * c * d * e
}

/// Count the number of trees encountered while descending the forest with the given slope
///
/// ### Arguments
/// * `forest` - The Forest to traverse
/// * `slope` - The slope at which to descend as a tuple (horizontal, vertical)
///
/// ### Returns
/// The number of trees that were found on the slope
fn count_tree_at_slope(forest: &Forest, slope: (usize, usize)) -> usize {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;
    while y < forest.trees.len() {
        if forest.has_tree_at((x, y)) {
            trees += 1;
        }
        x += slope.0;
        y += slope.1;
    }
    trees
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Forest {
    trees: Vec<Vec<bool>>,
}

impl Forest {
    /// Check if a tree is at the given position.
    /// If the horizontal position is larger than the Forest holds, it wraps around
    ///
    /// ### Arguments
    /// * `position` - The position on the grid (horizontal, vertical)
    ///
    /// ### Returns
    /// true if a tree is at the given position, false if not
    fn has_tree_at(&self, position: (usize, usize)) -> bool {
        if let Some(tree_line) = self.trees.get(position.1) {
            let x: usize = position.0 as usize % tree_line.len();
            tree_line[x]
        } else {
            false
        }
    }
}

impl FromStr for Forest {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<Vec<bool>> = s
            .lines()
            .map(|line| line.chars().map(|char| char == '#').collect())
            .collect();

        Ok(Forest { trees })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = include_str!("test_resources/03-A.txt");
    const B: &str = include_str!("test_resources/03-B.txt");

    #[test]
    fn first_part_test_a() {
        let forest: Forest = A.parse().unwrap();
        assert_eq!(7, first_part(&forest));
    }

    #[test]
    fn first_part_test_b() {
        let forest: Forest = B.parse().unwrap();
        assert_eq!(286, first_part(&forest));
    }

    #[test]
    fn second_part_test_a() {
        let forest: Forest = A.parse().unwrap();
        assert_eq!(336, second_part(&forest));
    }

    #[test]
    fn second_part_test_b() {
        let forest: Forest = B.parse().unwrap();
        assert_eq!(3638606400, second_part(&forest));
    }
}
