use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    vec::IntoIter,
};

/// Value used for indexing into a grid
pub type Idx = (usize, usize);
/// Result of indexing into a grid
pub type Result<T> = std::result::Result<T, OutOfBoundsError>;

/// Raised when a provided grid index is outside of grid boundaries
#[derive(Debug)]
pub struct OutOfBoundsError(pub Idx, pub Idx);

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "out of bounds; max {:?}, given {:?}", self.0, self.1)
    }
}

/// A grid with variable size that is allocated on the heap, generally slower than a `FixedGrid` but more lenient
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T>(Vec<Vec<Option<T>>>);

impl<T> Grid<T> {
    /// Returns the width of the grid
    pub fn width(&self) -> usize {
        self.0.len()
    }
    /// Returns the height of the grid
    pub fn height(&self) -> usize {
        if self.width() >= 1 {
            self.0[0].len()
        } else {
            0
        }
    }
    /// Returns the boundaries of the grid
    pub fn bounds(&self) -> Idx {
        (self.width(), self.height())
    }
    /// Returns the total size of the grid
    pub fn capacity(&self) -> usize {
        self.width() * self.height()
    }

    /// Returns `true` if the grid contains the provided index
    pub fn contains_index(&self, index: Idx) -> bool {
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
    pub fn map<U, F: Copy + Fn(&T) -> U>(self, f: F) -> Grid<U> {
        Grid(
            self.0
                .into_iter()
                .map(|r| r.into_iter().map(|o| o.as_ref().map(f)).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }
    /// Returns a grid of the same size as `self`, filled with the provided value
    pub fn fill<U: Clone>(self, value: U) -> Grid<U> {
        Grid(
            self.0
                .into_iter()
                .map(|r| {
                    r.into_iter()
                        .map(|o| o.as_ref().map(|_| value.clone()))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
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
    pub fn iter(&self) -> IntoIter<((usize, usize), &Option<T>)> {
        let mut vector = Vec::with_capacity(self.capacity());

        for (y, row) in self.0.iter().enumerate() {
            for (x, option) in row.iter().enumerate() {
                vector.push(((x, y), option));
            }
        }

        vector.into_iter()
    }
    /// Returns a mutable iterator over values of a grid
    pub fn iter_mut(&mut self) -> IntoIter<((usize, usize), &mut Option<T>)> {
        let mut vector = Vec::with_capacity(self.capacity());

        for (y, row) in self.0.iter_mut().enumerate() {
            for (x, option) in row.iter_mut().enumerate() {
                vector.push(((x, y), option));
            }
        }

        vector.into_iter()
    }
}

impl<T: Clone> Grid<T> {
    /// Creates a new empty grid
    pub fn new_empty(width: usize, height: usize) -> Self {
        Self(vec![vec![None; width]; height])
    }
    /// Creates a new grid filled with the provided value
    pub fn new_with(width: usize, height: usize, value: T) -> Self {
        Self(vec![vec![Some(value); width]; height])
    }

    /// Transposes the grid (swaps rows and columns)
    pub fn transpose(self) -> Self {
        let mut grid = Self::new_empty(self.height(), self.width());

        for (y, row) in self.0.into_iter().enumerate() {
            for (x, option) in row.into_iter().enumerate() {
                if let Some(value) = option {
                    grid.set((x, y), value.clone()).ok();
                }
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
        self.iter()
            .any(|(_, o)| o.as_ref().map_or(false, |v| v == value))
    }
}

impl<T: Ord> Grid<T> {
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

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
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

impl<T> From<Vec<Vec<Option<T>>>> for Grid<T> {
    fn from(grid: Vec<Vec<Option<T>>>) -> Self {
        Self(grid)
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(grid: Vec<Vec<T>>) -> Self {
        Self(
            grid.into_iter()
                .map(|r| r.into_iter().map(|o| Some(o)).collect())
                .collect(),
        )
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

impl<T: Clone> IntoIterator for Grid<T> {
    type Item = Option<T>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().flatten().collect::<Vec<_>>().into_iter()
    }
}
