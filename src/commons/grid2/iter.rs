use std::cmp::Ordering;
use std::convert::TryFrom;

use super::Grid;

/// An iterator over the lines of the Grid
pub struct LineIterator<'a, T> {
    inner: &'a Grid<T>,
    start: isize,
    end: isize,
}

impl<'a, T> LineIterator<'a, T> {
    pub fn new(inner: &'a Grid<T>) -> Self {
        Self {
            inner,
            start: 0,
            end: inner.height() as isize,
        }
    }
}

/// An iterator over the points of the Grid
pub struct Indices {
    start: (isize, isize),
    end: (isize, isize),
    width: isize,
}

impl Indices {
    pub fn new<T>(grid: &Grid<T>) -> Self {
        Self {
            start: (0, 0),
            end: (grid.width() as isize - 1, grid.height() as isize - 1),
            width: grid.width as isize,
        }
    }
}

/// An iterator over the points of a Grid
pub struct IndexedValues<'a, T> {
    keys: Indices,
    values: std::slice::Iter<'a, T>,
}

impl<'a, T> IndexedValues<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self {
            keys: Indices::new(grid),
            values: grid.iter(),
        }
    }
}

/// An iterator on a Grid that yields points on an half-line on it
pub struct HalfLine<'a, T> {
    inner: &'a Grid<T>,
    start: (isize, isize),
    increment: (isize, isize),
}

impl<'a, T> HalfLine<'a, T> {
    /// Build a new iterator that will iterate from points at start by increments
    ///
    /// ### Arguments
    /// * `start` - The first point of the half-line
    /// * `increment` - The increment to apply to get the following point from the previous one
    ///
    /// ### Panics
    /// If `increment` is (0, 0) as this would be an infinite iterator
    pub fn new(inner: &'a Grid<T>, start: (isize, isize), increment: (isize, isize)) -> Self {
        assert!(
            increment.0 != 0 || increment.1 != 0,
            "Increment for an half-line can't be (0, 0)"
        );
        Self {
            inner,
            start,
            increment,
        }
    }
}

impl<'a, T> Iterator for LineIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let from = self.start as usize * self.inner.width;
            let to = from + self.inner.width;
            self.start += 1;
            self.inner.storage.get(from..to)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length: usize = usize::try_from(self.end - self.start).unwrap_or_default();
        (length, Some(length))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.start += n as isize;
        self.next()
    }
}

impl<'a, T> DoubleEndedIterator for LineIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;
            let from = self.end as usize * self.inner.width;
            let to = from + self.inner.width;
            self.inner.storage.get(from..to)
        } else {
            None
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.end -= n as isize;
        self.next_back()
    }
}

impl<'a, T> ExactSizeIterator for LineIterator<'a, T> {}

impl Indices {
    /// True if there are no remaining elements in this iterator
    fn check_empty(&self) -> bool {
        match self.start.1.cmp(&self.end.1) {
            Ordering::Less => false,
            Ordering::Equal => self.start.0 > self.end.0,
            Ordering::Greater => true,
        }
    }

    /// Move a point nth steps in a direction (negative to go backward)
    fn move_point(&self, point: (isize, isize), n: isize) -> (isize, isize) {
        let x = point.0 + n;
        match x.div_euclid(self.width) {
            0 => (x, point.1),
            n => (x - n * self.width, point.1 + n),
        }
    }
}

impl Iterator for Indices {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.check_empty() {
            None
        } else {
            let current = self.start;
            self.start.0 += 1;
            if self.start.0 >= self.width {
                self.start.0 = 0;
                self.start.1 += 1;
            }
            Some(current)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.end.1 - self.start.1;
        let width = self.end.0 - self.start.0 + 1;
        let remaining = usize::try_from(length * self.width + width).unwrap_or_default();

        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.start = self.move_point(self.start, n as isize);
        self.next()
    }
}

impl DoubleEndedIterator for Indices {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.check_empty() {
            None
        } else {
            let current = self.end;
            self.end.0 -= 1;
            if self.end.0 < 0 {
                self.end.0 = self.width - 1;
                self.end.1 -= 1;
            }
            Some(current)
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.end = self.move_point(self.end, -1 * (n as isize));
        self.next_back()
    }
}

impl ExactSizeIterator for Indices {}

impl<'a, T> Iterator for IndexedValues<'a, T> {
    type Item = ((isize, isize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.keys.next().zip(self.values.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.values.size_hint()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.keys.nth(n).zip(self.values.nth(n))
    }
}

impl<'a, T> DoubleEndedIterator for IndexedValues<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.keys.next_back().zip(self.values.next_back())
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.keys.nth_back(n).zip(self.values.nth_back(n))
    }
}

impl<'a, T> ExactSizeIterator for IndexedValues<'a, T> {}

impl<'a, T> Iterator for HalfLine<'a, T> {
    type Item = ((isize, isize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let found = self.inner.get(self.start)?;
        let current = self.start;
        self.start.0 += self.increment.0;
        self.start.1 += self.increment.1;

        Some((current, found))
    }
}

#[cfg(test)]
mod tests {
    use crate::commons::grid2::Grid;

    #[test]
    fn line_iterator() {
        let grid = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        let mut lines = grid.lines();

        // Testing the next iteration
        assert_eq!(lines.len(), 4);
        assert_eq!(lines.next().unwrap(), &[0, 1, 2, 3, 4]);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines.next().unwrap(), &[5, 6, 7, 8, 9]);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines.next().unwrap(), &[10, 11, 12, 13, 14]);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines.next().unwrap(), &[15, 16, 17, 18, 19]);
        assert_eq!(lines.len(), 0);
        assert!(lines.next().is_none());
        assert_eq!(lines.len(), 0);

        // Testing the nth optimization
        let mut lines = grid.lines();
        assert_eq!(lines.nth(3).unwrap(), &[15, 16, 17, 18, 19]);
        assert!(lines.next().is_none());
        assert_eq!(lines.len(), 0);

        // If the nth goes past the end
        let mut lines = grid.lines();
        assert!(lines.nth(4).is_none());
        assert!(lines.next().is_none());
        assert_eq!(lines.len(), 0);

        // Next back
        let mut lines = grid.lines();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines.next_back().unwrap(), &[15, 16, 17, 18, 19]);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines.next_back().unwrap(), &[10, 11, 12, 13, 14]);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines.next_back().unwrap(), &[5, 6, 7, 8, 9]);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines.next_back().unwrap(), &[0, 1, 2, 3, 4]);
        assert_eq!(lines.len(), 0);
        assert!(lines.next_back().is_none());
        assert_eq!(lines.len(), 0);

        // Nth back optimization
        let mut lines = grid.lines();
        assert_eq!(lines.nth_back(3).unwrap(), &[0, 1, 2, 3, 4]);
        assert!(lines.next_back().is_none());
        assert_eq!(lines.len(), 0);

        // Nth back goes past the start of the iterator
        let mut lines = grid.lines();
        assert!(lines.nth_back(4).is_none());
        assert!(lines.next_back().is_none());
        assert_eq!(lines.len(), 0);

        // Mixing next and next_back should not produce inconsistent results
        let mut lines = grid.lines();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines.next_back().unwrap(), &[15, 16, 17, 18, 19]);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines.next().unwrap(), &[0, 1, 2, 3, 4]);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines.next_back().unwrap(), &[10, 11, 12, 13, 14]);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines.next().unwrap(), &[5, 6, 7, 8, 9]);
        assert_eq!(lines.len(), 0);
        assert!(lines.next().is_none());
        assert_eq!(lines.len(), 0);
        assert!(lines.next_back().is_none());
        assert_eq!(lines.len(), 0);
    }

    #[test]
    fn indices_iterator() {
        let grid = Grid::tabulate(2, 2, |(x, y)| x + y);

        // Testing the next iteration
        let mut keys = grid.indices();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys.next(), Some((0, 0)));
        assert_eq!(keys.len(), 3);
        assert_eq!(keys.next(), Some((1, 0)));
        assert_eq!(keys.len(), 2);
        assert_eq!(keys.next(), Some((0, 1)));
        assert_eq!(keys.len(), 1);
        assert_eq!(keys.next(), Some((1, 1)));
        assert_eq!(keys.len(), 0);
        assert_eq!(keys.next(), None);
        assert_eq!(keys.len(), 0);

        // Testing the nth optimization
        let mut keys = grid.indices();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys.nth(3), Some((1, 1)));
        assert_eq!(keys.len(), 0);
        assert!(keys.next().is_none());
        assert_eq!(keys.len(), 0);

        // If nth goes past the end
        let mut keys = grid.indices();
        assert_eq!(keys.len(), 4);
        assert!(keys.nth(4).is_none());
        assert_eq!(keys.len(), 0);
        assert!(keys.next().is_none());
        assert_eq!(keys.len(), 0);

        // Testing the next back iteration
        let mut keys = grid.indices();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys.next_back(), Some((1, 1)));
        assert_eq!(keys.len(), 3);
        assert_eq!(keys.next_back(), Some((0, 1)));
        assert_eq!(keys.len(), 2);
        assert_eq!(keys.next_back(), Some((1, 0)));
        assert_eq!(keys.len(), 1);
        assert_eq!(keys.next_back(), Some((0, 0)));
        assert_eq!(keys.len(), 0);
        assert_eq!(keys.next_back(), None);
        assert_eq!(keys.len(), 0);

        // Testing nth_back
        let mut keys = grid.indices();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys.nth_back(3), Some((0, 0)));
        assert_eq!(keys.len(), 0);
        assert!(keys.next_back().is_none());
        assert_eq!(keys.len(), 0);

        // If nth_back goes past the end of the array
        let mut keys = grid.indices();
        assert_eq!(keys.len(), 4);
        assert!(keys.nth_back(4).is_none());
        assert_eq!(keys.len(), 0);
        assert!(keys.next_back().is_none());
        assert_eq!(keys.len(), 0);

        // Mixing next and next_back should not produce inconsistent results
        let mut keys = grid.indices();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys.next_back(), Some((1, 1)));
        assert_eq!(keys.len(), 3);
        assert_eq!(keys.next(), Some((0, 0)));
        assert_eq!(keys.len(), 2);
        assert_eq!(keys.next_back(), Some((0, 1)));
        assert_eq!(keys.len(), 1);
        assert_eq!(keys.next(), Some((1, 0)));
        assert_eq!(keys.len(), 0);
        assert_eq!(keys.next(), None);
        assert_eq!(keys.len(), 0);
        assert_eq!(keys.next_back(), None);
        assert_eq!(keys.len(), 0);

        // Using the returned keys to index the Grid
        let mut keys = grid.indices();
        assert_eq!(grid[keys.next_back().unwrap()], 2);
        assert_eq!(grid[keys.next().unwrap()], 0);
        assert_eq!(grid[keys.next_back().unwrap()], 1);
        assert_eq!(grid[keys.next_back().unwrap()], 1);
    }

    #[test]
    fn indexed_values_iterator() {
        let grid = Grid::tabulate(2, 2, |(x, y)| x + y);

        // Testing the next iteration
        let mut keys = grid.indexed_values();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys.next(), Some(((0, 0), &0)));
        assert_eq!(keys.len(), 3);
        assert_eq!(keys.next(), Some(((1, 0), &1)));
        assert_eq!(keys.len(), 2);
        assert_eq!(keys.next(), Some(((0, 1), &1)));
        assert_eq!(keys.len(), 1);
        assert_eq!(keys.next(), Some(((1, 1), &2)));
        assert_eq!(keys.len(), 0);
        assert_eq!(keys.next(), None);
        assert_eq!(keys.len(), 0);

        // Testing the nth optimization
        let mut keys = grid.indexed_values();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys.nth(3), Some(((1, 1), &2)));
        assert_eq!(keys.len(), 0);
        assert!(keys.next().is_none());
        assert_eq!(keys.len(), 0);

        // If nth goes past the end
        let mut keys = grid.indexed_values();
        assert_eq!(keys.len(), 4);
        assert!(keys.nth(4).is_none());
        assert_eq!(keys.len(), 0);
        assert!(keys.next().is_none());
        assert_eq!(keys.len(), 0);

        // Testing the next back iteration
        let mut keys = grid.indexed_values();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys.next_back(), Some(((1, 1), &2)));
        assert_eq!(keys.len(), 3);
        assert_eq!(keys.next_back(), Some(((0, 1), &1)));
        assert_eq!(keys.len(), 2);
        assert_eq!(keys.next_back(), Some(((1, 0), &1)));
        assert_eq!(keys.len(), 1);
        assert_eq!(keys.next_back(), Some(((0, 0), &0)));
        assert_eq!(keys.len(), 0);
        assert_eq!(keys.next_back(), None);
        assert_eq!(keys.len(), 0);

        // Testing nth_back
        let mut keys = grid.indexed_values();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys.nth_back(3), Some(((0, 0), &0)));
        assert_eq!(keys.len(), 0);
        assert!(keys.next_back().is_none());
        assert_eq!(keys.len(), 0);

        // If nth_back goes past the end of the array
        let mut keys = grid.indexed_values();
        assert_eq!(keys.len(), 4);
        assert!(keys.nth_back(4).is_none());
        assert_eq!(keys.len(), 0);
        assert!(keys.next_back().is_none());
        assert_eq!(keys.len(), 0);

        // Mixing next and next_back should not produce inconsistent results
        let mut keys = grid.indexed_values();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys.next_back(), Some(((1, 1), &2)));
        assert_eq!(keys.len(), 3);
        assert_eq!(keys.next(), Some(((0, 0), &0)));
        assert_eq!(keys.len(), 2);
        assert_eq!(keys.next_back(), Some(((0, 1), &1)));
        assert_eq!(keys.len(), 1);
        assert_eq!(keys.next(), Some(((1, 0), &1)));
        assert_eq!(keys.len(), 0);
        assert_eq!(keys.next(), None);
        assert_eq!(keys.len(), 0);
        assert_eq!(keys.next_back(), None);
        assert_eq!(keys.len(), 0);
    }

    #[test]
    fn half_line_iterator() {
        let grid = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);

        let mut points = grid.half_line((0, 0), (1, 0));
        assert_eq!(points.next(), Some(((0, 0), &0)));
        assert_eq!(points.next(), Some(((1, 0), &1)));
        assert_eq!(points.next(), Some(((2, 0), &2)));
        assert_eq!(points.next(), Some(((3, 0), &3)));
        assert_eq!(points.next(), Some(((4, 0), &4)));
        assert_eq!(points.next(), None);

        let mut points = grid.half_line((0, 0), (0, 1));
        assert_eq!(points.next(), Some(((0, 0), &0)));
        assert_eq!(points.next(), Some(((0, 1), &5)));
        assert_eq!(points.next(), Some(((0, 2), &10)));
        assert_eq!(points.next(), Some(((0, 3), &15)));
        assert_eq!(points.next(), None);

        let mut points = grid.half_line((2, 2), (1, -1));
        assert_eq!(points.next(), Some(((2, 2), &12)));
        assert_eq!(points.next(), Some(((3, 1), &8)));
        assert_eq!(points.next(), Some(((4, 0), &4)));
        assert_eq!(points.next(), None);

        let mut points = grid.half_line((0, 1), (3, 2));
        assert_eq!(points.next(), Some(((0, 1), &5)));
        assert_eq!(points.next(), Some(((3, 3), &18)));
        assert_eq!(points.next(), None);
    }
}
