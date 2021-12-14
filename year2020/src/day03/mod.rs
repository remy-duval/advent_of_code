use commons::eyre::Result;

pub const TITLE: &str = "Day 3: Toboggan Trajectory";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
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

fn parse(s: &str) -> Result<Forest> {
    let trees: Vec<Vec<bool>> = s
        .lines()
        .map(|line| line.chars().map(|char| char == '#').collect())
        .collect();

    Ok(Forest { trees })
}

fn first_part(data: &Forest) -> usize {
    count_tree_at_slope(data, (3, 1))
}

fn second_part(data: &Forest) -> usize {
    count_tree_at_slope(data, (1, 1))
        * count_tree_at_slope(data, (3, 1))
        * count_tree_at_slope(data, (5, 1))
        * count_tree_at_slope(data, (7, 1))
        * count_tree_at_slope(data, (1, 2))
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

struct Forest {
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
        self.trees
            .get(position.1)
            .and_then(|tree_line| tree_line.get(position.0 % tree_line.len()))
            .map_or(false, |&b| b)
    }
}

#[cfg(test)]
mod tests;
