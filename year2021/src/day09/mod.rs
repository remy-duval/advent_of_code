use commons::eyre::{eyre, Report, Result};
use commons::grid::{Direction, Grid, Point};
use commons::Problem;
use hashbrown::HashSet;

pub struct Day;

impl Problem for Day {
    type Input = HeightMap;
    const TITLE: &'static str = "Day 9: Smoke Basin";

    fn solve(data: Self::Input) -> Result<()> {
        println!("1. Low point risk: {}", first_part(&data));
        println!("1. Basin size product: {}", second_part(&data));

        Ok(())
    }
}

fn first_part(map: &HeightMap) -> usize {
    map.low_points().map(|(_, next)| next as usize + 1).sum()
}

fn second_part(map: &HeightMap) -> usize {
    let (a, b, c) = map
        .basin_sizes()
        .fold((0, 0, 0), |(max1, max2, max3), next| {
            if next > max1 {
                (next, max1, max2)
            } else if next > max2 {
                (max1, next, max2)
            } else if next > max3 {
                (max1, max2, next)
            } else {
                (max1, max2, max3)
            }
        });

    a * b * c
}

pub struct HeightMap {
    grid: Grid<u8>,
}

impl HeightMap {
    /// An iterator over all points adjacent to the given one
    fn adjacent((i_x, i_y): (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
        Direction::ALL.into_iter().map(move |direction| {
            let Point { x, y } = direction.offset::<isize>();
            (i_x + x, i_y + y)
        })
    }

    /// An iterator over all low points of the grid (local minimum)
    fn low_points(&self) -> impl Iterator<Item = ((isize, isize), u8)> + '_ {
        self.grid
            .indexed_values()
            .filter(|&(index, value)| {
                Self::adjacent(index).all(|point| {
                    if let Some(adjacent) = self.grid.get(point) {
                        adjacent > value
                    } else {
                        true
                    }
                })
            })
            .map(|(index, value)| (index, *value))
    }

    /// An iterator over all basin sizes (regions around a local minimum delimited by 9s)
    fn basin_sizes(&self) -> impl Iterator<Item = usize> + '_ {
        let mut set = HashSet::with_capacity(128);
        let mut stack = Vec::with_capacity(128);
        self.low_points().map(move |(low_index, initial)| {
            // Do a DFS of points of the basin
            set.clear();
            stack.clear();
            stack.push((low_index, initial));
            set.insert(low_index);
            while let Some((point, current)) = stack.pop() {
                Self::adjacent(point).for_each(|adjacent| {
                    if let Some(value) = self.grid.get(adjacent).copied() {
                        if value > current && value < 9 && set.insert(adjacent) {
                            stack.push((adjacent, value));
                        }
                    }
                });
            }

            set.len()
        })
    }
}

impl std::str::FromStr for HeightMap {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let first = lines.next().ok_or_else(|| eyre!("Missing first line"))?;
        let width = first.chars().count();
        let mut storage = Vec::with_capacity(width * width);
        first
            .chars()
            .chain(lines.flat_map(move |f| f.chars()))
            .try_for_each(|c| -> Result<()> {
                let i = c.to_digit(10).ok_or_else(|| eyre!("Bad digit {}", c))?;
                storage.push(i as u8);
                Ok(())
            })?;

        let grid = Grid::from_vec(width, storage);
        Ok(Self { grid })
    }
}

#[cfg(test)]
mod tests;
