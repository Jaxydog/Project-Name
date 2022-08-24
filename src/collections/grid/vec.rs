use std::{
    ops::{Index, IndexMut},
    vec::IntoIter,
};

use super::{Grid, Idx};

/// A grid with a variable width and height that stores values using `Vec`s.
///
/// This will generally be slower than an `ArrayGrid`, however it comes with the benefit of having
/// looser requirements for the values stored within the grid.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecGrid<T>(Vec<Vec<Option<T>>>);

impl<T> VecGrid<T> {
    /// Creates a new empty grid
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(height);

        for _ in 0..height {
            let mut row = Vec::with_capacity(width);

            for _ in 0..width {
                row.push(None);
            }

            grid.push(row);
        }

        Self(grid)
    }
    /// Creates a new grid filled with the value provided by the given closure
    pub fn new_from<F: Fn() -> T>(width: usize, height: usize, f: F) -> Self {
        let mut grid = Vec::with_capacity(height);

        for _ in 0..height {
            let mut row = Vec::with_capacity(width);

            for _ in 0..width {
                row.push(Some(f()));
            }

            grid.push(row);
        }

        Self(grid)
    }
    /// Creates a new grid filled with the provided value through cloning
    pub fn new_with(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        Self::new_from(width, height, || value.clone())
    }
    /// Resizes the grid to the provided dimensions
    pub fn resize(&mut self, width: usize, height: usize) {
        self.0.iter_mut().for_each(|row| {
            row.resize_with(width, || None);
        });
        self.0.resize_with(height, || {
            let mut vec = Vec::with_capacity(width);

            for _ in 0..width {
                vec.push(None);
            }

            vec
        });
    }

    /// Returns a grid of the same size as `Self`, with function `f` applied to each value in order
    pub fn map<U, F: Fn(Option<T>) -> Option<U>>(self, f: F) -> VecGrid<U> {
        VecGrid(
            self.0
                .into_iter()
                .map(|r| r.into_iter().map(&f).collect())
                .collect(),
        )
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each `Some` value in order
    pub fn map_some<U, F: Fn(T) -> U>(self, f: F) -> VecGrid<U> {
        self.map(|o| o.map(&f))
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each `None` value in order
    pub fn map_none<F: Fn() -> T>(self, f: F) -> Self {
        self.map(|o| o.or_else(|| Some(f())))
    }
    /// Returns a grid of the same size as `Self`, filled with the provided value through cloning
    pub fn fill<U: Clone>(self, value: U) -> VecGrid<U> {
        self.map(|_| Some(value.clone()))
    }
    /// Returns a grid of the same size as `Self`, replacing all `Some` values with the provided value through cloning
    pub fn fill_some<U: Clone>(self, value: U) -> VecGrid<U> {
        self.map_some(|_| value.clone())
    }
    /// Returns a grid of the same size as `Self`, replacing all `None` values with the provided value throuhg cloning
    pub fn fill_none(self, value: T) -> Self
    where
        T: Clone,
    {
        self.map_none(|| value.clone())
    }

    /// Reverses each row of the grid
    pub fn flip_x(&mut self) {
        self.0.iter_mut().for_each(|r| r.reverse());
    }
    /// Reverses each column of the grid
    pub fn flip_y(&mut self) {
        self.0.reverse();
    }
    /// Shifts the grid to the left by the specified number of cells.
    ///
    /// Any number higher than the grid's width will be ignored.
    pub fn shift_left(&mut self, cells: usize) {
        let cells = cells.min(self.width());
        self.0.iter_mut().for_each(|r| r.rotate_left(cells));
    }
    /// Shifts the grid to the right by the specified number of cells.
    ///
    /// Any number higher than the grid's width will be ignored.
    pub fn shift_right(&mut self, cells: usize) {
        let cells = cells.min(self.width());
        self.0.iter_mut().for_each(|r| r.rotate_right(cells));
    }
    /// Shifts the grid upwards by the specified number of cells.
    ///
    /// Any number higher than the grid's height will be ignored.
    pub fn shift_up(&mut self, cells: usize) {
        let cells = cells.min(self.height());
        self.0.rotate_left(cells);
    }
    /// Shifts the grid downwards by the specified number of cells.
    ///
    /// Any number higher than the grid's height will be ignored.
    pub fn shift_down(&mut self, cells: usize) {
        let cells = cells.min(self.height());
        self.0.rotate_right(cells);
    }

    /// Transposes the grid, swapping its rows and columns
    pub fn transpose(self) -> Self {
        let mut grid = Self::new(self.height(), self.width());

        for (y, row) in self.0.into_iter().enumerate() {
            for (x, option) in row.into_iter().enumerate() {
                grid[(y, x)] = option;
            }
        }

        grid
    }
    /// Rotates the grid to the left
    pub fn rotate_left(mut self) -> Self {
        self.flip_x();
        self.transpose()
    }
    /// Rotates the grid to the right
    pub fn rotate_right(mut self) -> Self {
        self.flip_y();
        self.transpose()
    }

    /// Sorts the grid
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.0.iter_mut().for_each(|r| r.sort());
        self.0.sort();
    }
    /// Sorts the grid, but may not preserve order of equal elements
    pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.0.iter_mut().for_each(|r| r.sort_unstable());
        self.0.sort_unstable();
    }
}

impl<'i, T: 'i> Grid<'i, T> for VecGrid<T> {
    type Iter = Iter<'i, T>;
    type IterMut = IterMut<'i, T>;

    fn size(&self) -> super::Idx {
        let height = self.0.len();
        let width = (height > 0).then_some(self.0[0].len()).unwrap_or(0);

        (width, height)
    }
    fn iter(&'i self) -> Self::Iter {
        Iter::new(&self.0)
    }
    fn iter_mut(&'i mut self) -> Self::IterMut {
        IterMut::new(&mut self.0)
    }
}

impl<T> From<Vec<Vec<T>>> for VecGrid<T> {
    fn from(vec: Vec<Vec<T>>) -> Self {
        Self(
            vec.into_iter()
                .map(|r| r.into_iter().map(|v| Some(v)).collect())
                .collect(),
        )
    }
}

impl<T> From<Vec<Vec<Option<T>>>> for VecGrid<T> {
    fn from(vec: Vec<Vec<Option<T>>>) -> Self {
        Self(vec)
    }
}

impl<T> Index<Idx> for VecGrid<T> {
    type Output = Option<T>;

    fn index(&self, (x, y): Idx) -> &Self::Output {
        &self.0[y][x]
    }
}

impl<T> IndexMut<Idx> for VecGrid<T> {
    fn index_mut(&mut self, (x, y): Idx) -> &mut Self::Output {
        &mut self.0[y][x]
    }
}

impl<T> IntoIterator for VecGrid<T> {
    type Item = Option<T>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vector = Vec::with_capacity(self.capacity());

        for row in self.0 {
            for value in row {
                vector.push(value);
            }
        }

        vector.into_iter()
    }
}

/// Custom iterator that iterates over a `VecGrid`
pub struct Iter<'i, T>(Idx, &'i [Vec<Option<T>>]);

impl<'i, T> Iter<'i, T> {
    /// Creates a new iterator using the provided slice
    const fn new(slice: &'i [Vec<Option<T>>]) -> Self {
        Self((0, 0), slice)
    }
}

impl<'i, T> ExactSizeIterator for Iter<'i, T> {}

impl<'i, T> Iterator for Iter<'i, T> {
    type Item = &'i Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.0;
        let height = self.1.len();
        let width = (height >= 1).then_some(self.1[0].len()).unwrap_or(0);

        if x < width && y < height {
            if x < width {
                self.0 .0 += 1;
            } else {
                self.0 .1 += 1;
                self.0 .0 = 0;
            }

            return Some(&self.1[y][x]);
        }

        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let height = self.1.len();
        let width = (height >= 1).then_some(self.1[0].len()).unwrap_or(0);

        (width * height, Some(width * height))
    }
}

/// Custom mutable iterator that iterates over a `VecGrid`
pub struct IterMut<'i, T>(Idx, &'i mut [Vec<Option<T>>]);

impl<'i, T> IterMut<'i, T> {
    /// Creates a new iterator using the provided slice
    fn new(slice: &'i mut [Vec<Option<T>>]) -> Self {
        Self((0, 0), slice)
    }
}

impl<'i, T> ExactSizeIterator for IterMut<'i, T> {}

impl<'i, T> Iterator for IterMut<'i, T> {
    type Item = &'i mut Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.0;
        let height = self.1.len();
        let width = (height >= 1).then_some(self.1[0].len()).unwrap_or(0);

        if x < width && y < height {
            if x < width {
                self.0 .0 += 1;
            } else {
                self.0 .1 += 1;
                self.0 .0 = 0;
            }

            if y < height {
                unsafe {
                    let row = self.1.as_mut_ptr().add(y);

                    if x < row.as_ref().map_or(0, Vec::len) {
                        return row.as_mut()?.as_mut_ptr().add(x).as_mut();
                    }
                }
            }
        }

        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let height = self.1.len();
        let width = (height >= 1).then_some(self.1[0].len()).unwrap_or(0);

        (width * height, Some(width * height))
    }
}
