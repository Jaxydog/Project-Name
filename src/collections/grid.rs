use std::{
    ops::{Index, IndexMut},
    slice,
    vec::IntoIter,
};

/// Represents a grid index
pub type Idx = (usize, usize);

/// An iterator over the values of the grid
pub struct Iter<'i, T> {
    index: (usize, usize),
    slice: &'i [Vec<Option<T>>],
}

impl<'i, T> Iter<'i, T> {
    /// Creates a new iterator
    pub const fn new(slice: &'i [Vec<Option<T>>]) -> Self {
        Self {
            index: (0, 0),
            slice,
        }
    }

    /// Returns the height of the iterator
    const fn height(&self) -> usize {
        self.slice.len()
    }
    /// Returns the width of the iterator
    fn width(&self) -> usize {
        if self.height() >= 1 {
            self.slice[0].len()
        } else {
            0
        }
    }
}

impl<'i, T> Iterator for Iter<'i, T> {
    type Item = &'i Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.index;

        if x < self.width() && y < self.height() {
            if x < self.width() {
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
pub struct IterMut<'i, T> {
    index: (usize, usize),
    slice: &'i mut [Vec<Option<T>>],
}

impl<'i, T> IterMut<'i, T> {
    /// Creates a new mutable iterator
    pub fn new(slice: &'i mut [Vec<Option<T>>]) -> Self {
        Self {
            index: (0, 0),
            slice,
        }
    }

    /// Returns the height of the iterator
    const fn height(&self) -> usize {
        self.slice.len()
    }
    /// Returns the width of the iterator
    fn width(&self) -> usize {
        if self.height() >= 1 {
            self.slice[0].len()
        } else {
            0
        }
    }
}

impl<'i, T> Iterator for IterMut<'i, T> {
    type Item = &'i mut Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.index;

        if x < self.width() && y < self.height() {
            if x < self.width() {
                self.index.0 += 1;
            } else {
                self.index.1 += 1;
                self.index.0 = 0;
            }

            if y < self.height() {
                unsafe {
                    let ptr = self.slice.as_mut_ptr();
                    let row = ptr.add(y);

                    if x < row.as_ref().map_or(0, Vec::len) {
                        let ptr = row.as_mut()?.as_mut_ptr();
                        return ptr.add(x).as_mut();
                    }
                }
            }
        }

        None
    }
}

/// A grid that is allocated on the heap but does not have a fixed size
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T>(Vec<Vec<Option<T>>>);

impl<T> Grid<T> {
    /// Creates a new empty grid
    pub fn new(width: usize, height: usize) -> Self {
        let mut vector = Vec::with_capacity(height);

        for _ in 0..height {
            let mut row = Vec::with_capacity(width);

            for _ in 0..width {
                row.push(None);
            }

            vector.push(row);
        }

        Self(vector)
    }

    /// Returns the grid's height
    pub fn height(&self) -> usize {
        self.0.len()
    }
    /// Returns the grid's width
    pub fn width(&self) -> usize {
        if self.height() >= 1 {
            self.0[0].len()
        } else {
            0
        }
    }
    /// Returns the grid's capacity
    pub fn capacity(&self) -> usize {
        self.width() * self.height()
    }

    /// Returns `true` if the grid contains the provided index
    pub fn includes(&self, index: Idx) -> bool {
        index.0 < self.width() && index.1 < self.height()
    }

    /// Returns a reference to the value at the given index, if present
    pub fn get(&self, index: Idx) -> Option<&T> {
        if self.includes(index) {
            self[index].as_ref()
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
    pub fn map<U, F: Fn(Option<T>) -> Option<U>>(self, f: F) -> Grid<U> {
        Grid(
            self.0
                .into_iter()
                .map(|r| r.into_iter().map(&f).collect())
                .collect(),
        )
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each `Some(...)` value in order
    pub fn map_some<U, F: Fn(&T) -> U>(self, f: F) -> Grid<U> {
        self.map(|o| if let Some(v) = o { Some(f(&v)) } else { None })
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each `None` value in order
    pub fn map_none<U, F: Fn() -> Option<T>>(self, f: F) -> Self {
        self.map(|o| if o.is_none() { f() } else { o })
    }
    /// Returns a grid of the same size as `Self`, replacing all values with the provided value through cloning
    pub fn fill<U: Clone>(self, value: U) -> Grid<U> {
        self.map(|_| Some(value.clone()))
    }
    /// Returns a grid of the same size as `Self`, replacing each `Some(...)` value with the provided value through cloning
    pub fn replace<U: Clone>(self, value: U) -> Grid<U> {
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
    pub fn iter(&self) -> Iter<T> {
        Iter::new(&self.0)
    }
    /// Returns a mutable iterator over the grid
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(&mut self.0)
    }
    /// Returns an iterator over the grid's rows
    pub fn rows(&self) -> slice::Iter<Vec<Option<T>>> {
        self.0.iter()
    }
    /// Returns a mutable iterator over the grid's rows
    pub fn rows_mut(&mut self) -> slice::IterMut<Vec<Option<T>>> {
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

impl<T: Clone> Grid<T> {
    /// Creates a new grid filled with the provided value
    pub fn new_with(width: usize, height: usize, value: T) -> Self {
        Self::new(width, height).fill(value)
    }

    /// Transposes the grid, swapping rows and columns
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
}

impl<T: PartialEq> Grid<T> {
    /// Returns `true` if the grid contains the provided value
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|v| match v {
            Some(v) => v == value,
            None => false,
        })
    }
}

impl<T: Ord> Grid<T> {
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

impl<T> From<Vec<Vec<Option<T>>>> for Grid<T> {
    fn from(vector: Vec<Vec<Option<T>>>) -> Self {
        Self(vector)
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(vector: Vec<Vec<T>>) -> Self {
        let width = vector.iter().map(Vec::len).min().unwrap_or_default();

        vector
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .take(width)
                    .map(|v| Some(v))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl<T> Index<Idx> for Grid<T> {
    type Output = Option<T>;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

impl<T> IndexMut<Idx> for Grid<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index.1][index.0]
    }
}

impl<T> IntoIterator for Grid<T> {
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
