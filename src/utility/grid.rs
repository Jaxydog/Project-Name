use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    vec::IntoIter,
};

/// Contains errors that can be encountered while working with the grid
#[derive(Debug)]
pub enum Error {
    OutOfBounds(usize, usize),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    /// Returns the grid's total horizontal capacity
    pub fn width(&self) -> usize {
        self.as_vec().get(0).map_or(0, std::vec::Vec::len)
    }
    /// Returns the grid's total vertical capacity
    pub fn height(&self) -> usize {
        self.as_vec().len()
    }
    /// Returns the grid's total capacity
    pub fn capacity(&self) -> usize {
        self.width() * self.height()
    }
    /// Returns `true` if the provided coordinates are within the grid
    pub fn contains_coords(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }
    /// Returns a reference to the grid as a vector
    pub const fn as_vec(&self) -> &Vec<Vec<T>> {
        &self.0
    }
    /// Returns a mutable reference to the grid as a vector
    pub fn as_vec_mut(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.0
    }

    /// Flips the grid horizontally
    pub fn flip_x(&mut self) {
        for row in self.as_vec_mut() {
            row.reverse();
        }
    }
    /// Flips the grid vertically
    pub fn flip_y(&mut self) {
        self.as_vec_mut().reverse();
    }
    /// Returns a reference to the value at the provided coordinates
    pub fn get(&self, x: usize, y: usize) -> Result<&T, Error> {
        if self.contains_coords(x, y) {
            Ok(&self[y][x])
        } else {
            Err(Error::OutOfBounds(x, y))
        }
    }
    /// Returns a mutable reference to the value at the provided coordinates
    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut T, Error> {
        if self.contains_coords(x, y) {
            Ok(&mut self[y][x])
        } else {
            Err(Error::OutOfBounds(x, y))
        }
    }
    /// Sets the provided coordinates to the given value, and returns the previous value if present
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), Error> {
        if self.contains_coords(x, y) {
            self[y][x] = value;
            Ok(())
        } else {
            Err(Error::OutOfBounds(x, y))
        }
    }
    /// Returns an iterator over the values within the grid
    pub fn iter(&self) -> IntoIter<((usize, usize), &T)> {
        let mut items = Vec::new();

        for (y, row) in self.0.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                items.push(((x, y), value));
            }
        }

        items.into_iter()
    }
    /// Returns a mutable iterator over the values within the grid
    pub fn iter_mut(&mut self) -> IntoIter<((usize, usize), &mut T)> {
        let mut items = Vec::new();

        for (y, row) in self.0.iter_mut().enumerate() {
            for (x, value) in row.iter_mut().enumerate() {
                items.push(((x, y), value));
            }
        }

        items.into_iter()
    }
    /// Shifts the grid to the left by the given number of values
    pub fn shift_left(&mut self, by: usize) -> Result<(), Error> {
        if by < self.width() {
            for row in self.as_vec_mut() {
                row.rotate_left(by);
            }
            Ok(())
        } else {
            Err(Error::OutOfBounds(by, 0))
        }
    }
    /// Shifts the grid to the right by the given number of values
    pub fn shift_right(&mut self, by: usize) -> Result<(), Error> {
        if by < self.width() {
            for row in self.as_vec_mut() {
                row.rotate_right(by);
            }
            Ok(())
        } else {
            Err(Error::OutOfBounds(by, 0))
        }
    }
    /// Shifts the grid upwards by the given number of values
    pub fn shift_up(&mut self, by: usize) -> Result<(), Error> {
        if by < self.height() {
            self.as_vec_mut().rotate_left(by);
            Ok(())
        } else {
            Err(Error::OutOfBounds(0, by))
        }
    }
    /// Shifts the grid downwards by the given number of values
    pub fn shift_down(&mut self, by: usize) -> Result<(), Error> {
        if by < self.height() {
            self.as_vec_mut().rotate_right(by);
            Ok(())
        } else {
            Err(Error::OutOfBounds(0, by))
        }
    }
}

impl<T: Clone> Grid<T> {
    /// Creates a new grid
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        let mut rows = Vec::new();

        for _ in 0..height {
            let mut row = Vec::new();

            for _ in 0..width {
                row.push(fill.clone());
            }

            rows.push(row);
        }

        Self(rows)
    }
    /// Transposes the grid (swaps columns and rows)
    #[allow(clippy::needless_range_loop)]
    pub fn transpose(&mut self) {
        let mut transposed = Vec::new();

        for x in 0..self.width() {
            let mut column = Vec::new();

            for y in 0..self.height() {
                column.push(self[y][x].clone());
            }

            transposed.push(column);
        }

        self.0 = transposed;
    }
    /// Rotates the grid 90 degrees counter-clockwise
    pub fn rotate_left(&mut self) {
        self.transpose();
        self.flip_y();
    }
    /// Rotates the grid 90 degrees clockwise
    pub fn rotate_right(&mut self) {
        self.transpose();
        self.flip_x();
    }
    /// Rotates the grid 180 degrees
    pub fn rotate_twice(&mut self) {
        self.flip_x();
        self.flip_y();
    }
    /// Fills the grid with the provided value
    pub fn fill(&mut self, value: T) {
        let mut rows = Vec::new();

        for _ in 0..self.height() {
            let mut row = Vec::new();

            for _ in 0..self.width() {
                row.push(value.clone());
            }

            rows.push(row);
        }

        self.0 = rows;
    }
    /// Returns a copy of the grid with values that have been converted to another type
    #[allow(unused_must_use)]
    pub fn map<F: Fn(&T) -> U, U: Clone>(&self, f: F) -> Grid<U> {
        let mut grid = Grid::new(self.width(), self.height(), f(&self.as_vec()[0][0]));

        for ((x, y), value) in self.iter() {
            grid.set(x, y, f(value));
        }

        grid
    }
}

impl<T: PartialEq> Grid<T> {
    /// Returns `true` if the grid contains the provided value
    pub fn has(&self, value: &T) -> bool {
        self.as_vec().iter().any(|v| v.iter().any(|t| t == value))
    }
    /// Returns the coordinates of the provided value, if present
    pub fn find(&self, value: &T) -> Option<(usize, usize)> {
        self.to_owned()
            .iter()
            .find(|(_, v)| v == &value)
            .map(|(c, _)| c)
    }
}

impl<T: Ord> Grid<T> {
    /// Sorts the grid
    pub fn sort(&mut self) {
        for row in self.as_vec_mut() {
            row.sort();
        }
        self.as_vec_mut().sort();
    }
    /// Sorts the grid, but may not preserve the order of equal elements
    pub fn sort_unstable(&mut self) {
        for row in self.as_vec_mut() {
            row.sort_unstable();
        }
        self.as_vec_mut().sort_unstable();
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(list: Vec<Vec<T>>) -> Self {
        Self(list)
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_vec()[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_vec_mut()[index]
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = ((usize, usize), T);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut items = Vec::new();

        for (y, row) in self.0.into_iter().enumerate() {
            for (x, value) in row.into_iter().enumerate() {
                items.push(((x, y), value));
            }
        }

        items.into_iter()
    }
}

impl<T: Clone + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for vector in self.as_vec().clone() {
            for value in vector {
                s.push_str(format!("{}", value).as_str());
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}
