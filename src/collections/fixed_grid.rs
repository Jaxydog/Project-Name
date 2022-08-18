use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    vec::IntoIter,
};

use super::grid::{Idx, OutOfBoundsError, Result};

/// An iterator over values of a grid
pub struct Iter<'i, T, const W: usize, const H: usize> {
    index: (usize, usize),
    slice: &'i [[Option<T>; W]; H],
}

impl<'i, T, const W: usize, const H: usize> Iterator for Iter<'i, T, W, H> {
    type Item = &'i T;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.index;

        if x < W && y < H {
            if x < W {
                self.index.0 += 1;
            } else {
                self.index.1 += 1;
                self.index.0 = 0;
            }

            self.slice[y][x].as_ref()
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let passed = self.index.0 + (W * self.index.1);
        ((W * H) - passed, Some(W * H))
    }
}

impl<'i, T, const W: usize, const H: usize> ExactSizeIterator for Iter<'i, T, W, H> {}

/// A mutable iterator over values of a grid
pub struct IterMut<'i, T, const W: usize, const H: usize> {
    index: (usize, usize),
    slice: &'i mut [[Option<T>; W]; H],
}

impl<'i, T, const W: usize, const H: usize> Iterator for IterMut<'i, T, W, H> {
    type Item = &'i mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.index;

        if x < W && y < H {
            if x < W {
                self.index.0 += 1;
            } else {
                self.index.1 += 1;
                self.index.0 = 0;
            }

            if y < self.slice.len() {
                let row_ptr = self.slice.as_mut_ptr();

                unsafe {
                    let row = row_ptr.add(y);

                    if x < row.as_ref().map_or(0, |v| v.len()) {
                        let val_ptr = row.as_mut()?.as_mut_ptr();

                        return (*val_ptr.add(x)).as_mut();
                    }
                }
            }
        }

        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let passed = self.index.0 + (W * self.index.1);
        ((W * H) - passed, Some(W * H))
    }
}

impl<'i, T, const W: usize, const H: usize> ExactSizeIterator for IterMut<'i, T, W, H> {}

/// A grid with a fixed width and height, generally faster than a regular `Grid` but more strict
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedGrid<T, const W: usize, const H: usize>([[Option<T>; W]; H]);

impl<T, const W: usize, const H: usize> FixedGrid<T, W, H> {
    /// Returns the width of the grid
    pub const fn width(&self) -> usize {
        self.0.len()
    }
    /// Returns the height of the grid
    pub const fn height(&self) -> usize {
        if self.width() >= 1 {
            self.0[0].len()
        } else {
            0
        }
    }
    /// Returns the boundaries of the grid
    pub const fn bounds(&self) -> Idx {
        (self.width(), self.height())
    }
    /// Returns the total size of the grid
    pub const fn capacity(&self) -> usize {
        self.width() * self.height()
    }

    /// Returns `true` if the grid contains the provided index
    pub const fn contains_index(&self, index: Idx) -> bool {
        index.0 < self.width() && index.1 < self.height()
    }

    /// Returns a reference to the value at the given index, if present
    pub fn get(&self, index: Idx) -> Result<Option<&T>> {
        if self.contains_index(index) {
            Ok(self[index].as_ref())
        } else {
            Err(OutOfBoundsError(self.bounds(), index))
        }
    }
    /// Returns a mutable reference to the value at the given index, if present
    pub fn get_mut(&mut self, index: Idx) -> Result<Option<&mut T>> {
        if self.contains_index(index) {
            Ok(self[index].as_mut())
        } else {
            Err(OutOfBoundsError(self.bounds(), index))
        }
    }
    /// Sets the value at the given position in the grid to the provided value, returning the previous value if present
    pub fn set(&mut self, index: Idx, value: T) -> Result<Option<T>> {
        if self.contains_index(index) {
            let old = self[index].take();
            self[index] = Some(value);
            Ok(old)
        } else {
            Err(OutOfBoundsError(self.bounds(), index))
        }
    }

    /// Returns a grid of the same size as `self`, with function `f` applied to each value in order
    pub fn map<U, F: Copy + Fn(&T) -> U>(self, f: F) -> FixedGrid<U, W, H> {
        FixedGrid(self.0.map(|r| r.map(|o| o.as_ref().map(f))))
    }
    /// Returns a grid of the same size as `self`, filled with the provided value
    pub fn fill<U: Clone>(self, value: U) -> FixedGrid<U, W, H> {
        FixedGrid(self.0.map(|r| r.map(|o| o.as_ref().map(|_| value.clone()))))
    }

    /// Reverses each row of the grid
    pub fn flip_x(&mut self) {
        self.0.iter_mut().for_each(|r| r.reverse());
    }
    /// Reverses each column of the grid
    pub fn flip_y(&mut self) {
        self.0.reverse();
    }

    /// Shifts the grid to the left by the specified number of cells. Any number higher than the grid's width will be ignored
    pub fn shift_left(&mut self, cells: usize) {
        let cells = cells.min(self.width());
        self.0.iter_mut().for_each(|r| r.rotate_left(cells));
    }
    /// Shifts the grid to the right by the specified number of cells. Any number higher than the grid's width will be ignored
    pub fn shift_right(&mut self, cells: usize) {
        let cells = cells.min(self.width());
        self.0.iter_mut().for_each(|r| r.rotate_right(cells));
    }
    /// Shifts the grid upwards by the specified number of cells. Any number higher than the grid's height will be ignored
    pub fn shift_up(&mut self, cells: usize) {
        let cells = cells.min(self.height());
        self.0.rotate_left(cells);
    }
    /// Shifts the grid downwards by the specified number of cells. Any number higher than the grid's height will be ignored
    pub fn shift_down(&mut self, cells: usize) {
        let cells = cells.min(self.height());
        self.0.rotate_right(cells);
    }

    /// Returns an iterator over values of a grid
    pub const fn iter(&self) -> Iter<T, W, H> {
        Iter {
            index: (0, 0),
            slice: &self.0,
        }
    }
    /// Returns a mutable iterator over values of a grid
    pub fn iter_mut(&mut self) -> IterMut<T, W, H> {
        IterMut {
            index: (0, 0),
            slice: &mut self.0,
        }
    }
}

impl<T: Copy, const W: usize, const H: usize> FixedGrid<T, W, H> {
    /// Creates a new empty grid
    pub const fn new_empty() -> Self {
        Self([[None; W]; H])
    }
    /// Creates a new grid filled with the provided value
    pub const fn new_with(value: T) -> Self {
        Self([[Some(value); W]; H])
    }

    /// Transposes the grid (swaps rows and columns)
    pub fn transpose(self) -> FixedGrid<T, H, W> {
        let mut grid = FixedGrid::new_empty();

        for (y, row) in self.0.into_iter().enumerate() {
            for (x, option) in row.into_iter().enumerate() {
                if let Some(value) = option {
                    grid.set((x, y), value).ok();
                }
            }
        }

        grid
    }
    /// Rotates the grid to the left
    pub fn rotate_left(mut self) -> FixedGrid<T, H, W> {
        self.flip_x();
        self.transpose()
    }
    /// Rotates the grid to the right
    pub fn rotate_right(mut self) -> FixedGrid<T, H, W> {
        self.flip_y();
        self.transpose()
    }
}

impl<T: PartialEq, const W: usize, const H: usize> FixedGrid<T, W, H> {
    /// Returns `true` if the grid contains the provided value
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|v| v == value)
    }
}

impl<T: Ord, const W: usize, const H: usize> FixedGrid<T, W, H> {
    /// Sorts the grid
    pub fn sort(&mut self) {
        self.0.iter_mut().for_each(|r| r.sort());
        self.0.sort();
    }
    /// Sorts the grid, but may not preserve order of equal elements
    pub fn sort_unstable(&mut self) {
        self.0.iter_mut().for_each(|r| r.sort_unstable());
        self.0.sort_unstable();
    }
}

impl<T: Copy + Default, const W: usize, const H: usize> Default for FixedGrid<T, W, H> {
    fn default() -> Self {
        Self::new_with(T::default())
    }
}

impl<T: Display, const W: usize, const H: usize> Display for FixedGrid<T, W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            let row = row
                .iter()
                .map(|o| o.as_ref().map_or("None".to_string(), ToString::to_string))
                .collect::<Vec<_>>()
                .join(" ");

            writeln!(f, "{}", row)?;
        }

        Ok(())
    }
}

impl<T, const W: usize, const H: usize> From<[[Option<T>; W]; H]> for FixedGrid<T, W, H> {
    fn from(array: [[Option<T>; W]; H]) -> Self {
        Self(array)
    }
}

impl<T, const W: usize, const H: usize> From<[[T; W]; H]> for FixedGrid<T, W, H> {
    fn from(array: [[T; W]; H]) -> Self {
        Self(array.map(|r| r.map(|v| Some(v))))
    }
}

impl<T, const W: usize, const H: usize> Index<Idx> for FixedGrid<T, W, H> {
    type Output = Option<T>;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

impl<T, const W: usize, const H: usize> IndexMut<Idx> for FixedGrid<T, W, H> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index.1][index.0]
    }
}

impl<T: Clone, const W: usize, const H: usize> IntoIterator for FixedGrid<T, W, H> {
    type Item = Option<T>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .map(|r| r.to_vec())
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
    }
}
