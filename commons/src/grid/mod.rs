//! An implementation of a 2D vector to be used as a grid
//!
//! Note that the indexing operations on the grid are based on isize instead of usize
//! to allow easier increment/decrement operation on them

use std::ops::{Index, IndexMut};

pub use point::Direction;
pub use point::Point;

pub mod iter;
pub mod point;

/// A basic 2 dimension Vec for representing a grid
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Grid<T> {
    storage: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    /// Create a new Grid, with the given width and line capacity, but no elements inside
    ///
    /// ### Arguments
    /// * `width` - The width of a line in the Grid
    /// * `height_capacity` - The number of lines to allocate space for
    pub fn new(width: usize, height_capacity: usize) -> Self {
        Self {
            storage: Vec::with_capacity(height_capacity * width),
            width,
        }
    }

    /// Build a new Grid, with each element computed from the given closure
    ///
    /// ### Arguments
    /// * `width` - The width of a line in the Grid
    /// * `height` - The number of lines to create
    /// * `f` - The function to compute the element for the index (x, y)
    pub fn tabulate(width: usize, height: usize, mut f: impl FnMut((usize, usize)) -> T) -> Self {
        let mut created = Self::new(width, height);
        for y in 0..height {
            for x in 0..width {
                created.storage.push(f((x, y)));
            }
        }

        created
    }

    /// Convert a Vec into a Grid by specifying the width of the lines
    /// The elements at the end of the Vec that would not be a full line are dropped
    pub fn from_vec(width: usize, mut storage: Vec<T>) -> Self {
        let rest = storage.len().rem_euclid(width);
        storage.truncate(storage.len() - rest);
        Self { storage, width }
    }
}

impl<T: Clone> Grid<T> {
    /// Build a new Grid filled with clones of the given element
    ///
    /// ### Argument
    /// * `width` - The width of a line in the Grid
    /// * `height_capacity` - The number of lines to create
    /// * `element` - The element to duplicate for each position
    pub fn fill(width: usize, height: usize, element: T) -> Self {
        Self {
            storage: vec![element; width * height],
            width,
        }
    }
}

impl<T: Default + Clone> Grid<T> {
    /// Build a new Grid filled with the default values of a type
    ///
    /// ### Argument
    /// * `width` - The width of a line in the Grid
    /// * `height_capacity` - The number of lines to create
    pub fn with_default(width: usize, height: usize) -> Self {
        Self::fill(width, height, T::default())
    }
}

impl<T: Default> Grid<T> {
    /// Add a new line to the Grid, filled with the type default value
    /// and return a mutable to it
    ///
    /// ### Returns
    /// A mutable slice of the inserted line
    pub fn insert_default_line(&mut self) -> &mut [T] {
        self.insert_filled_line(|_| T::default())
    }
}

impl<T> Grid<T> {
    /// The width of this Grid
    pub fn width(&self) -> usize {
        self.width
    }

    /// The height of this Grid
    pub fn height(&self) -> usize {
        self.storage.len() / self.width
    }

    /// The (`width`, `height`) of this Grid
    pub fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    /// Get a reference to the element in the Grid at the given position
    ///
    /// ### Arguments
    /// * `x` - The horizontal position of the element
    /// * `y` - The vertical position of the element
    ///
    /// ### Returns
    /// None if the given position does not exist
    /// Some of a reference to the element if it is found
    pub fn get(&self, (x, y): (isize, isize)) -> Option<&T> {
        if x < 0 || x >= self.width as isize || y < 0 {
            None
        } else {
            self.storage.get(y as usize * self.width + x as usize)
        }
    }

    /// Get a mutable reference to the element in the Grid at the given position
    ///
    /// ### Arguments
    /// * `x` - The horizontal position of the element
    /// * `y` - The vertical position of the element
    ///
    /// ### Returns
    /// None if the given position does not exist
    /// Some of a reference to the element if it is found
    pub fn get_mut(&mut self, (x, y): (isize, isize)) -> Option<&mut T> {
        if x < 0 || x >= self.width as isize || y < 0 {
            None
        } else {
            self.storage.get_mut(y as usize * self.width + x as usize)
        }
    }

    /// Get a reference to the line at the given height
    ///
    /// ### Arguments
    /// * `y` - The vertical position of the line
    ///
    /// ### Returns
    /// None if the line does not exist
    /// Some of a reference to the line if it is found
    pub fn get_line(&self, y: isize) -> Option<&[T]> {
        let start = y as usize * self.width;
        let end = start + self.width;
        if end <= self.storage.len() {
            Some(&self.storage[start..end])
        } else {
            None
        }
    }

    /// Get a mutable reference to the line at the given height
    ///
    /// ### Arguments
    /// * `y` - The vertical position of the line
    ///
    /// ### Returns
    /// None if the line does not exist
    /// Some of a mutable reference to the line if it is found
    pub fn get_line_mut(&mut self, y: isize) -> Option<&mut [T]> {
        let start = y as usize * self.width;
        let end = start + self.width;
        if end <= self.storage.len() {
            Some(&mut self.storage[start..end])
        } else {
            None
        }
    }

    /// Add a new line to the Grid
    ///
    /// ### Arguments
    /// * `line` The line to add the the Grid, it must be exactly `width` in length
    ///
    /// ### Panics
    /// If the line is shorter or longer than the vec `width`
    pub fn push_line(&mut self, line: Vec<T>) {
        if let Err(line) = self.try_push_line(line) {
            panic!(
                "Mismatched line length {} and grid width {}",
                line.len(),
                self.width
            );
        }
    }

    /// Add a new line to the Grid using a closure to compute each element
    /// and return a mutable reference to it
    ///
    /// ### Arguments
    /// * `produce` - A closure from the `x` value of the element in the line to its value
    ///
    /// ### Returns
    /// A mutable slice of the inserted line
    pub fn insert_filled_line(&mut self, mut produce: impl FnMut(usize) -> T) -> &mut [T] {
        let start = self.storage.len();
        self.storage.reserve(self.width);
        for i in 0..self.width {
            self.storage.push(produce(i));
        }

        &mut self.storage[start..(start + self.width)]
    }

    /// Add a new line to the Grid (if it is exactly `width` in length
    ///
    /// ### Arguments
    /// * `line` The line to add the the Grid, it must be exactly `width` in length
    ///
    /// ### Returns
    /// Err if the line is shorter or longer than the vec `width`
    pub fn try_push_line(&mut self, line: Vec<T>) -> Result<(), Vec<T>> {
        if line.len() == self.width {
            self.storage.extend(line);
            Ok(())
        } else {
            Err(line)
        }
    }

    /// An iterator on the flattened content of the Grid
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.storage.iter()
    }

    /// An iterator on the mutable flattened content of the Grid
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.storage.iter_mut()
    }

    /// An iterator on the lines of the Grid as slices
    pub fn lines(&self) -> iter::LineIterator<'_, T> {
        iter::LineIterator::new(self)
    }

    /// An iterator over the points in the Grid
    pub fn indices(&self) -> iter::Indices {
        iter::Indices::new(self)
    }

    /// An iterator on the points and their values in the Grid
    pub fn indexed_values(&self) -> iter::IndexedValues<'_, T> {
        iter::IndexedValues::new(self)
    }

    /// An iterator on the points in the Grid that belong to a given half line
    ///
    /// ### Arguments
    /// * `from` - The first point of the half-line
    /// * `step` - The increment to apply to get the following point from the previous one
    ///
    /// ### Panics
    /// If `increment` is (0, 0) as this would be an infinite iterator
    pub fn half_line(&self, from: (isize, isize), step: (isize, isize)) -> iter::HalfLine<'_, T> {
        iter::HalfLine::new(self, from, step)
    }
}

impl<T> From<(usize, Vec<T>)> for Grid<T> {
    fn from((width, storage): (usize, Vec<T>)) -> Self {
        Self::from_vec(width, storage)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.lines() {
            for elt in line {
                elt.fmt(f)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> AsRef<[T]> for Grid<T> {
    fn as_ref(&self) -> &[T] {
        self.storage.as_ref()
    }
}

impl<T> AsMut<[T]> for Grid<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.storage.as_mut()
    }
}

impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        match self.get(index) {
            None => panic!("Index {index:?} out of bounds"),
            Some(value) => value,
        }
    }
}

impl<T> IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        match self.get_mut(index) {
            None => panic!("Index {index:?} out of bounds"),
            Some(value) => value,
        }
    }
}

impl<T> Index<isize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: isize) -> &Self::Output {
        match self.get_line(index) {
            None => panic!("Line index {index} out of bounds"),
            Some(value) => value,
        }
    }
}

impl<T> IndexMut<isize> for Grid<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        match self.get_line_mut(index) {
            None => panic!("Line index {index} out of bounds"),
            Some(value) => value,
        }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let created: Grid<usize> = Grid::new(5, 4);
        assert_eq!(created.width(), 5);
        assert_eq!(created.height(), 0);
        assert_eq!(created.as_ref(), &[]);
    }

    #[test]
    fn tabulate() {
        let created = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        assert_eq!(created.width(), 5);
        assert_eq!(created.height(), 4);
        assert_eq!(
            created.as_ref(),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
        )
    }

    #[test]
    fn fill() {
        let created = Grid::fill(2, 2, 3);
        assert_eq!(created.width(), 2);
        assert_eq!(created.height(), 2);
        assert_eq!(created.as_ref(), &[3, 3, 3, 3])
    }

    #[test]
    fn with_default() {
        let created: Grid<usize> = Grid::with_default(2, 2);
        assert_eq!(created.width(), 2);
        assert_eq!(created.height(), 2);
        assert_eq!(created.as_ref(), &[0, 0, 0, 0])
    }

    #[test]
    fn insert_default_line() {
        let mut grid: Grid<usize> = Grid::new(5, 3);
        grid.insert_default_line();
        grid.insert_default_line();
        let added_line = grid.insert_default_line();
        added_line[2] = 69;
        assert_eq!(
            grid.as_ref(),
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0]
        )
    }

    #[test]
    fn width() {
        let vec: Grid<usize> = Grid::new(5, 0);
        assert_eq!(vec.width(), 5);
    }

    #[test]
    fn height() {
        let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        assert_eq!(vec.height(), 4);
    }

    #[test]
    fn size() {
        let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        assert_eq!(vec.size(), (5, 4));
    }

    #[test]
    fn get() {
        let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        assert_eq!(8, *vec.get((3, 1)).unwrap());
        assert_eq!(19, *vec.get((4, 3)).unwrap())
    }

    #[test]
    fn get_mut() {
        let mut vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        let inside = vec.get_mut((3, 1)).unwrap();
        *inside = 50;
        assert_eq!(50, *vec.get((3, 1)).unwrap());
    }

    #[test]
    fn get_line() {
        let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        assert_eq!(&[15, 16, 17, 18, 19], vec.get_line(3).unwrap());
    }

    #[test]
    fn get_line_mut() {
        let mut vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        let inside = vec.get_line_mut(3).unwrap();
        inside[2] = 2;
        assert_eq!(&[15, 16, 2, 18, 19], vec.get_line(3).unwrap());
    }

    #[test]
    fn push_line() {
        let mut vec = Grid::new(5, 1);
        vec.push_line(vec![1, 2, 3, 4, 5]);
        assert_eq!(1, vec.height());
        assert_eq!(&[1, 2, 3, 4, 5], vec.get_line(0).unwrap());
    }

    #[test]
    #[should_panic]
    fn push_line_too_short() {
        let mut vec = Grid::new(5, 1);
        vec.push_line(vec![1, 2, 3, 4]); // Too short
    }

    #[test]
    fn insert_filled_line() {
        let mut grid = Grid::new(5, 3);
        grid.insert_filled_line(|i| i);
        grid.insert_filled_line(|i| 2 * i);
        let added_line = grid.insert_filled_line(|i| 3 * i);
        added_line[2] = 500;
        assert_eq!(
            grid.as_ref(),
            &[0, 1, 2, 3, 4, 0, 2, 4, 6, 8, 0, 3, 500, 9, 12]
        )
    }

    #[test]
    fn try_push_line() {
        let mut vec = Grid::new(5, 1);
        vec.try_push_line(vec![1, 2, 3, 4, 5]).unwrap(); // Ok
        vec.try_push_line(vec![1, 2, 3, 4]).unwrap_err(); // Too short
        vec.try_push_line(vec![1, 2, 3, 4, 5, 6]).unwrap_err(); // Too long
        assert_eq!(1, vec.height());
        assert_eq!(&[1, 2, 3, 4, 5], vec.get_line(0).unwrap());
    }

    #[test]
    fn lines() {
        let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        let mut lines = vec.lines();
        assert_eq!(lines.next().unwrap(), &[0, 1, 2, 3, 4]);
        assert_eq!(lines.next().unwrap(), &[5, 6, 7, 8, 9]);
        assert_eq!(lines.next().unwrap(), &[10, 11, 12, 13, 14]);
        assert_eq!(lines.next().unwrap(), &[15, 16, 17, 18, 19]);
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn indices() {
        let vec: Grid<u8> = Grid::with_default(2, 2);
        let mut keys = vec.indices();
        assert_eq!(keys.next(), Some((0, 0)));
        assert_eq!(keys.next(), Some((1, 0)));
        assert_eq!(keys.next(), Some((0, 1)));
        assert_eq!(keys.next(), Some((1, 1)));
        assert_eq!(keys.next(), None);
    }

    #[test]
    fn indexed_values() {
        let vec: Grid<u8> = Grid::with_default(2, 2);
        let mut keys = vec.indexed_values();
        assert_eq!(keys.next(), Some(((0, 0), &0)));
        assert_eq!(keys.next(), Some(((1, 0), &0)));
        assert_eq!(keys.next(), Some(((0, 1), &0)));
        assert_eq!(keys.next(), Some(((1, 1), &0)));
        assert_eq!(keys.next(), None);
    }

    #[test]
    fn half_line() {
        let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        let mut points = vec.half_line((2, 2), (1, -1));
        assert_eq!(points.next(), Some(((2, 2), &12)));
        assert_eq!(points.next(), Some(((3, 1), &8)));
        assert_eq!(points.next(), Some(((4, 0), &4)));
        assert_eq!(points.next(), None);
    }

    #[test]
    #[should_panic]
    fn half_line_with_zero_step() {
        let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
        vec.half_line((2, 2), (0, 0));
    }
}
