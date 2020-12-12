//! An implementation of a 2D vector to be used as a grid
//!
//! Note that the indexing operations on the grid are based on isize instead of usize
//! to allow easier increment/decrement operation on them

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Index, IndexMut};

/// A basic 2 dimension Vec for representing a grid
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Grid<T> {
    /// The data, stored in a single line
    storage: Vec<T>,
    /// The width of each line in the Grid
    width: usize,
}

impl<T> Grid<T> {
    /// Create a new Grid, with the given width and line capacity, but no elements inside
    ///
    /// ### Arguments
    /// * `width` - The width of a line in the Grid
    /// * `height_capacity` - The number of lines to allocate space for
    ///
    /// ### Examples
    ///
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let created: Grid<usize> = Grid::new(5, 4);
    ///
    /// assert_eq!(created.width(), 5);
    /// assert_eq!(created.height(), 0);
    /// assert_eq!(created.flattened(), &[]);
    /// ```
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
    /// * `height_capacity` - The number of lines to create
    /// * `f` - The function to compute the element for the index (x, y)
    ///
    /// ### Examples
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let created = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    ///
    /// assert_eq!(created.width(), 5);
    /// assert_eq!(created.height(), 4);
    /// assert_eq!(
    ///     created.flattened(),
    ///     &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    /// )
    /// ```
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
    ///
    /// ### Examples
    ///
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let created = Grid::fill(2, 2, 3);
    ///
    /// assert_eq!(created.width(), 2);
    /// assert_eq!(created.height(), 2);
    /// assert_eq!(
    ///     created.flattened(),
    ///     &[3, 3, 3, 3]
    /// )
    /// ```
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
    ///
    /// ### Examples
    ///
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let created: Grid<usize> = Grid::with_default(2, 2);
    ///
    /// assert_eq!(created.width(), 2);
    /// assert_eq!(created.height(), 2);
    /// assert_eq!(
    ///     created.flattened(),
    ///     &[0, 0, 0, 0]
    /// )
    /// ```
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
    ///
    /// ### Examples
    ///
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut grid: Grid<usize> = Grid::new(5, 3);
    /// grid.add_line();
    /// grid.add_line();
    /// let mut added_line = grid.add_line();
    /// added_line[2] = 69;
    ///
    /// assert_eq!(
    ///     grid.flattened(),
    ///     &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0]
    /// )
    /// ```
    pub fn add_line(&mut self) -> &mut [T] {
        self.fill_line(|_| T::default())
    }
}

impl<T> Grid<T> {
    /// The width of this Grid
    ///
    /// ### Examples
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let vec: Grid<usize> = Grid::new(5, 0);
    /// assert_eq!(vec.width(), 5);
    /// ```
    pub fn width(&self) -> usize {
        self.width
    }

    /// The height of this Grid
    ///
    /// ### Examples
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    ///
    /// assert_eq!(vec.height(), 4);
    /// ```
    pub fn height(&self) -> usize {
        self.storage.len() / self.width
    }

    /// The (`width`, `height`) of this Grid
    /// The height of this Grid
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    ///
    /// assert_eq!(vec.size(), (5, 4));
    /// ```
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
    ///
    /// ### Examples
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    ///
    /// assert_eq!(8, *vec.get((3, 1)).unwrap());
    /// assert_eq!(19, *vec.get((4, 3)).unwrap())
    /// ```
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
    ///
    /// ### Examples
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    ///
    /// let inside = vec.get_mut((3, 1)).unwrap();
    /// *inside = 50;
    /// assert_eq!(50, *vec.get((3, 1)).unwrap());
    /// ```
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
    ///
    /// ### Examples
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    ///
    /// assert_eq!(
    ///     &[15, 16, 17, 18, 19],
    ///     vec.get_line(3).unwrap()
    /// );
    /// ```
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
    /// ### Examples
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    ///
    /// let inside = vec.get_line_mut(3).unwrap();
    /// inside[2] = 2;
    /// assert_eq!(
    ///     &[15, 16, 2, 18, 19],
    ///     vec.get_line(3).unwrap()
    /// );
    /// ```
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
    ///
    /// ### Examples
    ///
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut vec = Grid::new(5, 1);
    ///
    /// vec.push_line(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(1, vec.height());
    /// assert_eq!(&[1, 2, 3, 4, 5], vec.get_line(0).unwrap());
    /// ```
    ///
    /// ```should_panic
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut vec = Grid::new(5, 1);
    ///
    /// vec.push_line(vec![1, 2, 3, 4]); // Too short
    /// ```
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
    ///
    /// ### Examples
    ///
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut grid = Grid::new(5, 3);
    /// grid.fill_line(|i| i);
    /// grid.fill_line(|i| 2 * i);
    /// let added_line = grid.fill_line(|i| 3 * i);
    /// added_line[2] = 500;
    ///
    /// assert_eq!(
    ///     grid.flattened(),
    ///     &[0, 1, 2, 3, 4, 0, 2, 4, 6, 8, 0, 3, 500, 9, 12]
    /// )
    /// ```
    pub fn fill_line(&mut self, mut produce: impl FnMut(usize) -> T) -> &mut [T] {
        let start = self.storage.len();
        self.storage.reserve(self.width as usize);
        for i in 0..self.width {
            self.storage.push(produce(i));
        }

        &mut self.storage[start..(start + self.width as usize)]
    }

    /// Add a new line to the Grid (if it is exactly `width` in length
    ///
    /// ### Arguments
    /// * `line` The line to add the the Grid, it must be exactly `width` in length
    ///
    /// ### Returns
    /// Err if the line is shorter or longer than the vec `width`
    ///
    /// ### Examples
    ///
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let mut vec = Grid::new(5, 1);
    ///
    /// vec.try_push_line(vec![1, 2, 3, 4, 5]).unwrap(); // Ok
    /// vec.try_push_line(vec![1, 2, 3, 4]).unwrap_err(); // Too short
    /// vec.try_push_line(vec![1, 2, 3, 4, 5, 6]).unwrap_err(); // Too long
    /// assert_eq!(1, vec.height());
    /// assert_eq!(&[1, 2, 3, 4, 5], vec.get_line(0).unwrap());
    /// ```
    pub fn try_push_line(&mut self, line: Vec<T>) -> Result<(), Vec<T>> {
        if line.len() == self.width {
            self.storage.extend(line);
            Ok(())
        } else {
            Err(line)
        }
    }

    /// The data of this Grid in a single line
    pub fn flattened(&self) -> &[T] {
        &self.storage
    }

    /// The mutable data of this Grid in a single line
    pub fn flattened_mut(&mut self) -> &mut [T] {
        &mut self.storage
    }

    /// An iterator on the flattened content of the Grid
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.storage.iter()
    }

    /// An iterator on the mutable flattened content of the Grid
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.storage.iter_mut()
    }

    /// An iterator on the lines of the Grid
    ///
    /// ### Examples
    ///
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    /// let mut lines = vec.lines();
    ///
    /// assert_eq!(lines.next().unwrap(), &[0, 1, 2, 3, 4]);
    /// assert_eq!(lines.next().unwrap(), &[5, 6, 7, 8, 9]);
    /// assert_eq!(lines.next().unwrap(), &[10, 11, 12, 13, 14]);
    /// assert_eq!(lines.next().unwrap(), &[15, 16, 17, 18, 19]);
    /// assert_eq!(lines.next(), None);
    /// ```
    pub fn lines(&self) -> LineIterator<'_, T> {
        LineIterator {
            inner: &self,
            current: 0,
        }
    }

    /// An iterator on the points in the Grid
    ///
    /// ### Examples
    ///
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let vec: Grid<u8> = Grid::with_default(2, 2);
    /// let mut keys = vec.key_values();
    ///
    /// assert_eq!(keys.next(), Some(((0, 0), &0)));
    /// assert_eq!(keys.next(), Some(((1, 0), &0)));
    /// assert_eq!(keys.next(), Some(((0, 1), &0)));
    /// assert_eq!(keys.next(), Some(((1, 1), &0)));
    /// assert_eq!(keys.next(), None);
    /// ```
    pub fn key_values(&self) -> KeyValues<'_, T> {
        KeyValues {
            inner: self.iter(),
            x: 0,
            y: 0,
            width: self.width as isize,
        }
    }

    /// An iterator on the points in the Grid that belong to a given half line
    ///
    /// ### Arguments
    /// * `from` - The first point of the half-line
    /// * `step` - The increment to apply to get the following point from the previous one
    ///
    /// ### Examples
    ///
    /// ```
    /// use advent_of_code::commons::grid2::Grid;
    /// let vec = Grid::tabulate(5, 4, |(x, y)| x + 5 * y);
    /// let mut points = vec.half_line((2, 2), (1, -1));
    ///
    /// assert_eq!(points.next(), Some(((2, 2), &12)));
    /// assert_eq!(points.next(), Some(((3, 1), &8)));
    /// assert_eq!(points.next(), Some(((4, 0), &4)));
    /// assert_eq!(points.next(), None);
    /// ```
    pub fn half_line(&self, from: (isize, isize), step: (isize, isize)) -> HalfLine<'_, T> {
        HalfLine {
            inner: &self,
            current: from,
            increment: step,
        }
    }
}

/// An iterator over the lines of the Grid
pub struct LineIterator<'a, T> {
    inner: &'a Grid<T>,
    current: isize,
}

impl<'a, T> Iterator for LineIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        self.inner.get_line(self.current - 1)
    }
}

/// An iterator over the points of a Grid
pub struct KeyValues<'a, T> {
    inner: std::slice::Iter<'a, T>,
    x: isize,
    y: isize,
    width: isize,
}

impl<'a, T> Iterator for KeyValues<'a, T> {
    type Item = ((isize, isize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let found = self.inner.next()?;
        let current = (self.x, self.y);
        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        Some((current, found))
    }
}

/// An iterator on a Grid that yields points on an half-line on it
pub struct HalfLine<'a, T> {
    inner: &'a Grid<T>,
    current: (isize, isize),
    increment: (isize, isize),
}

impl<'a, T> Iterator for HalfLine<'a, T> {
    type Item = ((isize, isize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let found = self.inner.get(self.current)?;
        let current = self.current;
        self.current.0 += self.increment.0;
        self.current.1 += self.increment.1;

        Some((current, found))
    }
}

impl<T> From<(usize, Vec<T>)> for Grid<T> {
    fn from((width, storage): (usize, Vec<T>)) -> Self {
        Self::from_vec(width, storage)
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for line in self.lines() {
            for elt in line {
                write!(f, "{}", elt)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> AsRef<[T]> for Grid<T> {
    fn as_ref(&self) -> &[T] {
        self.flattened()
    }
}

impl<T> AsMut<[T]> for Grid<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.flattened_mut()
    }
}

impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        match self.get(index) {
            None => panic!("Index {:?} out of bounds", index),
            Some(value) => value,
        }
    }
}

impl<T> IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        match self.get_mut(index) {
            None => panic!("Index {:?} out of bounds", index),
            Some(value) => value,
        }
    }
}

impl<T> Index<isize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: isize) -> &Self::Output {
        match self.get_line(index) {
            None => panic!("Line index {} out of bounds", index),
            Some(value) => value,
        }
    }
}

impl<T> IndexMut<isize> for Grid<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        match self.get_line_mut(index) {
            None => panic!("Line index {} out of bounds", index),
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
