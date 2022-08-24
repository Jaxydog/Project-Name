use std::{
    ops::{Index, IndexMut},
    vec::IntoIter,
};

use super::{Grid, Idx};

/// A grid with a fixed width and height that stores values using arrays.
///
/// This will generally be faster than a standard grid, since it stores values on the stack rather than
/// the heap. The consequence of storing values in this manner, however, is that the contained values
/// must be more restricted.
///
/// For example, to create an `ArrayGrid` using `new()`, the type of T must implement `Copy`.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrayGrid<T, const W: usize, const H: usize>([[Option<T>; W]; H]);

impl<T, const W: usize, const H: usize> ArrayGrid<T, W, H> {
    /// Creates a new empty grid
    pub const fn new() -> Self
    where
        T: Copy,
    {
        Self([[None; W]; H])
    }
    /// Creates a new grid filled with the value provided by the callback
    pub fn new_from<F: Fn() -> T>(f: F) -> Self
    where
        T: Copy,
    {
        Self([[Some(f()); W]; H])
    }
    /// Creates a new grid filled with the provided value through copying
    pub const fn new_with(value: T) -> Self
    where
        T: Copy,
    {
        Self([[Some(value); W]; H])
    }

    /// Returns a grid of the same size as `Self`, with function `f` applied to each value in order
    pub fn map<U, F: Fn(Option<T>) -> Option<U>>(self, f: F) -> ArrayGrid<U, W, H> {
        ArrayGrid(self.0.map(|r| r.map(&f)))
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each `Some` value in order
    pub fn map_some<U, F: Fn(T) -> U>(self, f: F) -> ArrayGrid<U, W, H> {
        self.map(|o| o.map(&f))
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each `None` value in order
    pub fn map_none<F: Fn() -> T>(self, f: F) -> Self {
        self.map(|o| o.or_else(|| Some(f())))
    }
    /// Returns a grid of the same size as `Self`, filled with the provided value through cloning
    pub fn fill<U: Clone>(self, value: U) -> ArrayGrid<U, W, H> {
        self.map(|_| Some(value.clone()))
    }
    /// Returns a grid of the same size as `Self`, replacing all `Some` values with the provided value through cloning
    pub fn fill_some<U: Clone>(self, value: U) -> ArrayGrid<U, W, H> {
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
    pub fn transpose(self) -> ArrayGrid<T, H, W>
    where
        T: Copy,
    {
        let mut grid = ArrayGrid::new();

        for (y, row) in self.0.into_iter().enumerate() {
            for (x, option) in row.into_iter().enumerate() {
                grid[(y, x)] = option;
            }
        }

        grid
    }
    /// Rotates the grid to the left
    pub fn rotate_left(mut self) -> ArrayGrid<T, H, W>
    where
        T: Copy,
    {
        self.flip_x();
        self.transpose()
    }
    /// Rotates the grid to the right
    pub fn rotate_right(mut self) -> ArrayGrid<T, H, W>
    where
        T: Copy,
    {
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

impl<T, const W: usize, const H: usize> From<[[T; W]; H]> for ArrayGrid<T, W, H> {
    fn from(array: [[T; W]; H]) -> Self {
        Self(array.map(|r| r.map(|v| Some(v))))
    }
}

impl<T, const W: usize, const H: usize> From<[[Option<T>; W]; H]> for ArrayGrid<T, W, H> {
    fn from(array: [[Option<T>; W]; H]) -> Self {
        Self(array)
    }
}

impl<'i, T: 'i, const W: usize, const H: usize> Grid<'i, T> for ArrayGrid<T, W, H> {
    type Iter = Iter<'i, T, W, H>;
    type IterMut = IterMut<'i, T, W, H>;

    fn size(&self) -> Idx {
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

impl<T, const W: usize, const H: usize> Index<Idx> for ArrayGrid<T, W, H> {
    type Output = Option<T>;

    fn index(&self, (x, y): Idx) -> &Self::Output {
        &self.0[y][x]
    }
}

impl<T, const W: usize, const H: usize> IndexMut<Idx> for ArrayGrid<T, W, H> {
    fn index_mut(&mut self, (x, y): Idx) -> &mut Self::Output {
        &mut self.0[y][x]
    }
}

impl<T, const W: usize, const H: usize> IntoIterator for ArrayGrid<T, W, H> {
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

impl<T: Copy + Default, const W: usize, const H: usize> Default for ArrayGrid<T, W, H> {
    fn default() -> Self {
        Self::new_with(T::default())
    }
}

/// Custom iterator that iterates over an `ArrayGrid`
pub struct Iter<'i, T, const W: usize, const H: usize>(Idx, &'i [[Option<T>; W]; H]);

impl<'i, T, const W: usize, const H: usize> Iter<'i, T, W, H> {
    /// Creates a new iterator using the provided slice
    const fn new(slice: &'i [[Option<T>; W]; H]) -> Self {
        Self((0, 0), slice)
    }
}

impl<'i, T, const W: usize, const H: usize> ExactSizeIterator for Iter<'i, T, W, H> {}

impl<'i, T, const W: usize, const H: usize> Iterator for Iter<'i, T, W, H> {
    type Item = &'i Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.0;

        if x < W && y < H {
            if x < W {
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
        (W * H, Some(W * H))
    }
}

/// Custom mutable iterator that iterates over an `ArrayGrid`
pub struct IterMut<'i, T, const W: usize, const H: usize>(Idx, &'i mut [[Option<T>; W]; H]);

impl<'i, T, const W: usize, const H: usize> IterMut<'i, T, W, H> {
    /// Creates a new iterator using the provided slice
    fn new(slice: &'i mut [[Option<T>; W]; H]) -> Self {
        Self((0, 0), slice)
    }
}

impl<'i, T, const W: usize, const H: usize> ExactSizeIterator for IterMut<'i, T, W, H> {}

impl<'i, T, const W: usize, const H: usize> Iterator for IterMut<'i, T, W, H> {
    type Item = &'i mut Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.0;

        if x < W && y < H {
            if x < W {
                self.0 .0 += 1;
            } else {
                self.0 .1 += 1;
                self.0 .0 = 0;
            }

            if y < self.1.len() {
                unsafe {
                    let row = self.1.as_mut_ptr().add(y);

                    if x < row.as_ref().map_or(0, |v| v.len()) {
                        return row.as_mut()?.as_mut_ptr().add(x).as_mut();
                    }
                }
            }
        }

        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (W * H, Some(W * H))
    }
}
