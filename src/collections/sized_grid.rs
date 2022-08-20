use std::{
    ops::{Index, IndexMut},
    slice,
    vec::IntoIter,
};

use super::grid::Idx;

/// An iterator over the values of the grid
pub struct Iter<'i, T, const W: usize, const H: usize> {
    index: (usize, usize),
    slice: &'i [[Option<T>; W]; H],
}

impl<'i, T, const W: usize, const H: usize> Iter<'i, T, W, H> {
    /// Creates a new iterator
    pub const fn new(slice: &'i [[Option<T>; W]; H]) -> Self {
        Self {
            index: (0, 0),
            slice,
        }
    }
}

impl<'i, T, const W: usize, const H: usize> ExactSizeIterator for Iter<'i, T, W, H> {}

impl<'i, T, const W: usize, const H: usize> Iterator for Iter<'i, T, W, H> {
    type Item = &'i Option<T>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let capacity = W * H;
        (capacity, Some(capacity))
    }
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.index;

        if x < W && y < H {
            if x < W {
                self.index.0 += 1;
            } else {
                self.index.1 += 1;
                self.index.0 = 0;
            }

            Some(&self.slice[y][x])
        } else {
            None
        }
    }
}

/// A mutable iterator over the values of the grid
pub struct IterMut<'i, T, const W: usize, const H: usize> {
    index: (usize, usize),
    slice: &'i mut [[Option<T>; W]; H],
}

impl<'i, T, const W: usize, const H: usize> IterMut<'i, T, W, H> {
    /// Creates a new mutable iterator
    pub fn new(slice: &'i mut [[Option<T>; W]; H]) -> Self {
        Self {
            index: (0, 0),
            slice,
        }
    }
}

impl<'i, T, const W: usize, const H: usize> ExactSizeIterator for IterMut<'i, T, W, H> {}

impl<'i, T, const W: usize, const H: usize> Iterator for IterMut<'i, T, W, H> {
    type Item = &'i mut Option<T>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let capacity = W * H;
        (capacity, Some(capacity))
    }
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
                let ptr = self.slice.as_mut_ptr();

                unsafe {
                    let row = ptr.add(y);

                    if x < row.as_ref().map_or(0, |v| v.len()) {
                        let ptr = row.as_mut()?.as_mut_ptr();
                        return ptr.add(x).as_mut();
                    }
                }
            }
        }

        None
    }
}

/// A grid with a fixed width and height, generally faster than a regular `Grid`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SizedGrid<T, const W: usize, const H: usize>([[Option<T>; W]; H]);

impl<T, const W: usize, const H: usize> SizedGrid<T, W, H> {
    /// Returns the grid's height
    pub const fn height(&self) -> usize {
        self.0.len()
    }
    /// Returns the grid's width
    pub const fn width(&self) -> usize {
        if self.height() >= 1 {
            self.0[0].len()
        } else {
            0
        }
    }
    /// Returns the grid's capacity
    pub const fn capacity(&self) -> usize {
        self.width() * self.height()
    }

    /// Returns `true` if the grid contains the provided index
    pub const fn includes(&self, index: Idx) -> bool {
        index.0 < self.width() && index.1 < self.height()
    }

    /// Returns a reference to the value at the given index, if present
    pub const fn get(&self, index: Idx) -> Option<&T> {
        if self.includes(index) {
            self.0[index.1][index.0].as_ref()
        } else {
            None
        }
    }
    /// Returns a reference to the value at the given index, if present
    pub fn get_mut(&mut self, index: Idx) -> Option<&mut T> {
        if self.includes(index) {
            self.0[index.1][index.0].as_mut()
        } else {
            None
        }
    }
    /// Sets the value at the given index to the provided value, returning the previous value if present
    pub fn set(&mut self, index: Idx, value: T) -> Option<T> {
        if self.includes(index) {
            let old = self[index].take();
            self[index] = Some(value);
            old
        } else {
            None
        }
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each value in order
    pub fn map<U, F: Fn(Option<T>) -> Option<U>>(self, f: F) -> SizedGrid<U, W, H> {
        SizedGrid(self.0.map(|r| r.map(&f)))
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each `Some(...)` value in order
    pub fn map_some<U, F: Fn(&T) -> U>(self, f: F) -> SizedGrid<U, W, H> {
        SizedGrid(self.0.map(|r| r.map(|o| o.as_ref().map(&f))))
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each `None` value in order
    pub fn map_none<U, F: Fn() -> Option<T>>(self, f: F) -> Self {
        Self(self.0.map(|r| r.map(|o| if o.is_none() { f() } else { o })))
    }
    /// Returns a grid of the same size as `Self`, replacing all values with the provided value through cloning
    pub fn fill<U: Clone>(self, value: U) -> SizedGrid<U, W, H> {
        self.map(|_| Some(value.clone()))
    }
    /// Returns a grid of the same size as `Self`, replacing each `Some(...)` value with the provided value through cloning
    pub fn replace<U: Clone>(self, value: U) -> SizedGrid<U, W, H> {
        self.map_some(|_| value.clone())
    }

    /// Reverses each row of the grid
    pub fn flip_x(&mut self) {
        self.rows_mut().for_each(|v| v.reverse());
    }
    /// Reverses each column of the grid
    pub fn flip_y(&mut self) {
        self.0.reverse();
    }

    /// Shifts the grid to the left by the specified number of cells. Any number higher than the grid's width will be ignored
    pub fn shift_left(&mut self, cells: usize) {
        let cells = cells.min(self.width());
        self.rows_mut().for_each(|r| r.rotate_left(cells));
    }
    /// Shifts the grid to the right by the specified number of cells. Any number higher than the grid's width will be ignored
    pub fn shift_right(&mut self, cells: usize) {
        let cells = cells.min(self.width());
        self.rows_mut().for_each(|r| r.rotate_right(cells));
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

    /// Returns an iterator over the grid
    pub const fn iter(&self) -> Iter<T, W, H> {
        Iter::new(&self.0)
    }
    /// Returns a mutable iterator over the grid
    pub fn iter_mut(&mut self) -> IterMut<T, W, H> {
        IterMut::new(&mut self.0)
    }
    /// Returns an iterator over the grid's rows
    pub fn rows(&self) -> slice::Iter<[Option<T>; W]> {
        self.0.iter()
    }
    /// Returns a mutable iterator over the grid's rows
    pub fn rows_mut(&mut self) -> slice::IterMut<[Option<T>; W]> {
        self.0.iter_mut()
    }
    /// Returns an iterator over the grid that also contains each value's position
    pub fn enumerate(&self) -> IntoIter<((usize, usize), &Option<T>)> {
        let mut vector = Vec::with_capacity(self.capacity());

        for (y, row) in self.rows().enumerate() {
            for (x, option) in row.iter().enumerate() {
                vector.push(((x, y), option));
            }
        }

        vector.into_iter()
    }
    /// Returns a mutable iterator over the grid that also contains each value's position
    pub fn enumerate_mut(&mut self) -> IntoIter<((usize, usize), &mut Option<T>)> {
        let mut vector = Vec::with_capacity(self.capacity());

        for (y, row) in self.rows_mut().enumerate() {
            for (x, option) in row.iter_mut().enumerate() {
                vector.push(((x, y), option));
            }
        }

        vector.into_iter()
    }
}

impl<T: Copy, const W: usize, const H: usize> SizedGrid<T, W, H> {
    /// Creates a new empty grid
    pub const fn new() -> Self {
        Self([[None; W]; H])
    }
    /// Creates a new grid filled with the provided value
    pub const fn new_with(value: T) -> Self {
        Self([[Some(value); W]; H])
    }

    /// Transposes the grid, swapping rows and columns
    pub fn transpose(self) -> SizedGrid<T, H, W> {
        let mut grid = SizedGrid::new();

        for (y, row) in self.0.into_iter().enumerate() {
            for (x, option) in row.into_iter().enumerate() {
                grid[(y, x)] = option;
            }
        }

        grid
    }
    /// Rotates the grid to the left
    pub fn rotate_left(mut self) -> SizedGrid<T, H, W> {
        self.flip_x();
        self.transpose()
    }
    /// Rotates the grid to the right
    pub fn rotate_right(mut self) -> SizedGrid<T, H, W> {
        self.flip_y();
        self.transpose()
    }
}

impl<T: PartialEq, const W: usize, const H: usize> SizedGrid<T, W, H> {
    /// Returns `true` if the grid contains the provided value
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|v| match v {
            Some(v) => v == value,
            None => false,
        })
    }
}

impl<T: Ord, const W: usize, const H: usize> SizedGrid<T, W, H> {
    /// Sorts the grid
    pub fn sort(&mut self) {
        self.rows_mut().for_each(|r| r.sort());
        self.0.sort();
    }
    /// Sorts the grid, but may not preserve order of equal elements
    pub fn sort_unstable(&mut self) {
        self.rows_mut().for_each(|r| r.sort_unstable());
        self.0.sort_unstable();
    }
}

impl<T, const W: usize, const H: usize> From<[[Option<T>; W]; H]> for SizedGrid<T, W, H> {
    fn from(array: [[Option<T>; W]; H]) -> Self {
        Self(array)
    }
}

impl<T, const W: usize, const H: usize> From<[[T; W]; H]> for SizedGrid<T, W, H> {
    fn from(array: [[T; W]; H]) -> Self {
        Self(array.map(|r| r.map(|v| Some(v))))
    }
}

impl<T, const W: usize, const H: usize> Index<Idx> for SizedGrid<T, W, H> {
    type Output = Option<T>;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

impl<T, const W: usize, const H: usize> IndexMut<Idx> for SizedGrid<T, W, H> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index.1][index.0]
    }
}

impl<T, const W: usize, const H: usize> IntoIterator for SizedGrid<T, W, H> {
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

impl<T: Copy + Default, const W: usize, const H: usize> Default for SizedGrid<T, W, H> {
    fn default() -> Self {
        Self::new_with(T::default())
    }
}
