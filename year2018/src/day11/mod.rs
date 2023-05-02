use itertools::iproduct;

use commons::grid::Grid;
use commons::Result;

pub const TITLE: &str = "Day 11: Chronal Charge";

pub fn run(raw: String) -> Result<()> {
    let serial_number = raw.parse()?;
    let grid = PartialSumGrid::new(serial_number);
    let (x, y) = first_part(&grid);
    println!("The 3x3 square with the highest power is {x},{y}");
    let (x, y, s) = second_part(&grid);
    println!("The square with the highest power is {x},{y},{s}");
    Ok(())
}

fn first_part(grid: &PartialSumGrid) -> (isize, isize) {
    let (x, y, size) = grid.maximum_square(3, 3);
    assert_eq!(size, 3);
    (x, y)
}

fn second_part(grid: &PartialSumGrid) -> (isize, isize, isize) {
    grid.maximum_square(2, 30)
}

/// A [`Summed Area Table`] of the power grid to facilitate the square sums
///
/// [`Summed Area Table`]: https://en.wikipedia.org/wiki/Summed-area_table
struct PartialSumGrid {
    grid: Grid<i32>,
}

impl PartialSumGrid {
    const WIDTH: usize = 300;

    /// Build the partial sum grid for a serial number
    fn new(serial_number: isize) -> Self {
        // Compute the power level of a fuel cell
        fn power_level(serial_number: isize, (x, y): (isize, isize)) -> i32 {
            let id = x + 10;
            let digits = (id * y + serial_number) * id;
            let digit = (digits / 100) % 10;
            digit as i32 - 5
        }

        let mut grid = Grid::with_default(Self::WIDTH, Self::WIDTH);
        (0..(Self::WIDTH as isize)).for_each(|y| {
            (0..(Self::WIDTH as isize)).for_each(|x| {
                let power = power_level(serial_number, (x + 1, y + 1));
                let left = grid.get((x - 1, y)).map_or(0, |&x| x);
                let top = grid.get((x, y - 1)).map_or(0, |&x| x);
                let top_left = grid.get((x - 1, y - 1)).map_or(0, |&x| x);

                grid[(x, y)] = power + top + left - top_left;
            });
        });

        Self { grid }
    }

    /// Compute the sum of a square of size `size`
    ///
    /// ### Arguments
    /// * `(x, y)` - The top left corner of the square
    /// * `size` - The size of the square
    ///
    /// ### Returns
    /// None if the square is out of bound of the grid
    /// Some(sum) containing the sum of all elements in the square
    fn sum(&self, (x, y): (isize, isize), size: isize) -> i32 {
        let start_x = x - 1;
        let end_x = start_x + size;
        let start_y = y - 1;
        let end_y = start_y + size;

        // Sum from (x, y) to (x + size - 1, y + size -1) can be computed using the grid:
        // A = (x - 1, y - 1)
        // B = (x + size - 1, y - 1)
        // B = (x - 1, y + size - 1)
        // D = (x + size - 1, y + size - 1)
        // The sum is equal to PSUM(A) + PSUM(D) - PSUM(C) - PSUM(B)
        let top_left = self.grid.get((start_x, start_y)).map_or(0, |&x| x);
        let bottom_left = self.grid.get((end_x, start_y)).map_or(0, |&x| x);
        let top_right = self.grid.get((start_x, end_y)).map_or(0, |&x| x);
        let bottom_right = self.grid.get((end_x, end_y)).map_or(0, |&x| x);

        top_left + bottom_right - bottom_left - top_right
    }

    /// Find the square with the maximum total power in the grid
    fn maximum_square(&self, min_size: isize, max_size: isize) -> (isize, isize, isize) {
        let width = Self::WIDTH as isize;
        (min_size..=max_size)
            .filter_map(|size| {
                iproduct!(0..(width - size), 0..(width - size))
                    .map(|(x, y)| ((x, y, size), self.sum((x, y), size)))
                    .max_by_key(|(_, sum)| *sum)
            })
            .max_by_key(|(_, sum)| *sum)
            .map_or((0, 0, 0), |((x, y, s), _)| (x + 1, y + 1, s))
    }
}

#[cfg(test)]
mod tests;
