use std::{
    ops::{Index, IndexMut},
    vec::IntoIter,
};

/// A value that represents a grid index
pub type Idx = (usize, usize);

/// A grid with a fixed width and height that stores values using arrays.
///
/// This will generally be faster than a standard grid, since it stores values on the stack rather than the heap.
/// The consequence of storing values in this manner, however, is that the contained values must be more restricted.
///
/// For example, to create an `ArrayGrid` using `new()`, the type of T must implement `Copy`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrayGrid<T, const W: usize, const H: usize>([[Option<T>; W]; H]);

impl<T: Copy, const W: usize, const H: usize> ArrayGrid<T, W, H> {
    /// Creates a new empty grid
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::<u8, 3, 3>::new();
    ///
    /// let b = ArrayGrid::from([
    ///     [None, None, None],
    ///     [None, None, None],
    ///     [None, None, None],
    /// ]);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub const fn new() -> Self {
        Self([[None; W]; H])
    }
    /// Creates a new grid filled with the provided value through copying
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::<_, 3, 3>::new_with(true);
    ///
    /// let b = ArrayGrid::from([
    ///     [true, true, true],
    ///     [true, true, true],
    ///     [true, true, true],
    /// ]);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub const fn new_with(value: T) -> Self {
        Self([[Some(value); W]; H])
    }

    /// Transposes the grid, swapping its rows and columns
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [1, 4, 7],
    ///     [2, 5, 8],
    ///     [3, 6, 9],
    /// ]);
    ///
    /// let b = ArrayGrid::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ])
    /// .transpose();
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn transpose(self) -> ArrayGrid<T, H, W> {
        let mut grid = ArrayGrid::new();

        for (y, row) in self.0.into_iter().enumerate() {
            for (x, option) in row.into_iter().enumerate() {
                grid[(y, x)] = option;
            }
        }

        grid
    }
    /// Rotates the grid to the left
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [2, 4],
    ///     [1, 3],
    /// ]);
    ///
    /// let b = ArrayGrid::from([
    ///     [1, 2],
    ///     [3, 4],
    /// ])
    /// .rotate_left();
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn rotate_left(mut self) -> ArrayGrid<T, H, W> {
        self.flip_x();
        self.transpose()
    }
    /// Rotates the grid to the right
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [3, 1],
    ///     [4, 2],
    /// ]);
    ///
    /// let b = ArrayGrid::from([
    ///     [1, 2],
    ///     [3, 4],
    /// ])
    /// .rotate_right();
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn rotate_right(mut self) -> ArrayGrid<T, H, W> {
        self.flip_y();
        self.transpose()
    }
}

impl<T: PartialEq, const W: usize, const H: usize> ArrayGrid<T, W, H> {
    /// Returns `true` if the grid contains the provided value
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::from([
    ///     [1, 2],
    ///     [3, 4],
    /// ]);
    ///
    /// assert!(a.contains(&1));
    /// assert!(a.contains(&2));
    /// assert!(a.contains(&3));
    /// assert!(a.contains(&4));
    /// assert!(!a.contains(&5));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|v| match v {
            Some(v) => v == value,
            None => false,
        })
    }
}

impl<T: Ord, const W: usize, const H: usize> ArrayGrid<T, W, H> {
    /// Sorts the grid
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [1, 2],
    ///     [3, 4],
    /// ]);
    ///
    /// let mut b = ArrayGrid::from([
    ///     [4, 3],
    ///     [2, 1],
    /// ]);
    ///
    /// b.sort();
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn sort(&mut self) {
        self.0.iter_mut().for_each(|r| r.sort());
        self.0.sort();
    }
    /// Sorts the grid, but may not preserve order of equal elements
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [1, 2],
    ///     [3, 4],
    /// ]);
    ///
    /// let mut b = ArrayGrid::from([
    ///     [4, 3],
    ///     [2, 1],
    /// ]);
    ///
    /// b.sort_unstable();
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn sort_unstable(&mut self) {
        self.0.iter_mut().for_each(|r| r.sort_unstable());
        self.0.sort_unstable();
    }
}

impl<T, const W: usize, const H: usize> ArrayGrid<T, W, H> {
    /// Returns the array grid's size
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::<u8, 3, 4>::new();
    ///
    /// assert_eq!((3, 4), grid.size());
    /// ```
    pub const fn size(&self) -> (usize, usize) {
        let height = self.0.len();
        let width = if height >= 1 { self.0[0].len() } else { 0 };

        (width, height)
    }
    /// Returns the array grid's width
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::<u8, 3, 4>::new();
    ///
    /// assert_eq!(3, grid.width());
    /// ```
    pub const fn width(&self) -> usize {
        self.size().0
    }
    /// Returns the array grid's height
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::<u8, 3, 4>::new();
    ///
    /// assert_eq!(4, grid.height());
    /// ```
    pub const fn height(&self) -> usize {
        self.size().1
    }
    /// Returns the array grid's total capacity
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::<u8, 3, 4>::new();
    ///
    /// assert_eq!(12, grid.capacity());
    /// ```
    pub const fn capacity(&self) -> usize {
        let (width, height) = self.size();
        width * height
    }
    /// Returns `true` if the grid contains the provided index
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::<u8, 3, 4>::new();
    ///
    /// assert!(grid.includes((1, 2)));
    /// assert!(!grid.includes((4, 20)));
    /// ```
    pub const fn includes(&self, (x, y): Idx) -> bool {
        let (width, height) = self.size();
        x < width && y < height
    }

    /// Returns a reference to the value at the provided index, if present
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// assert_eq!(Some(8), grid.get((1, 2)));
    /// ```
    pub fn get(&self, index: Idx) -> Option<&T> {
        self.includes(index)
            .then_some(self[index].as_ref())
            .flatten()
    }
    /// Returns a reference to the value at the provided index, if present
    ///
    /// # Examples
    /// ```rust
    /// let mut grid = ArrayGrid::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// assert_eq!(Some(8), grid.get_mut((1, 2)));
    /// ```
    pub fn get_mut(&mut self, index: Idx) -> Option<&mut T> {
        self.includes(index)
            .then_some(self[index].as_mut())
            .flatten()
    }
    /// Sets the value at the given index to the provided value, returning the previous value is present
    ///
    /// # Examples
    /// ```rust
    /// let mut grid = ArrayGrid::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// assert_eq!(Some(8), grid.insert((1, 2), 12));
    /// assert_eq!(Some(12), grid.insert((1, 2)));
    /// ```
    pub fn insert(&mut self, index: Idx, value: T) -> Option<T> {
        let old = self.remove(index);
        self[index] = Some(value);
        old
    }
    /// Removes the value from the given index and returns it, if present
    ///
    /// # Examples
    /// ```rust
    /// let mut grid = ArrayGrid::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// assert_eq!(Some(8), grid.remove((1, 2)));
    /// assert_eq!(None, grid.get((1, 2)));
    /// ```
    pub fn remove(&mut self, index: Idx) -> Option<T> {
        self.includes(index).then(|| self[index].take()).flatten()
    }

    /// Returns an iterator over the grid
    ///
    /// # Examples
    /// ```rust
    /// let mut grid = ArrayGrid::from([
    ///     [1,2,3],
    ///     [4,5,6],
    ///     [7,8,9],
    /// ])
    /// .iter();
    ///
    /// assert_eq!(Some(1), grid.next());
    /// assert_eq!(Some(2), grid.next());
    /// assert_eq!(Some(3), grid.next());
    /// assert_eq!(Some(4), grid.next());
    /// ```
    pub fn iter(&self) -> Iter<T, W, H> {
        Iter::new(&self.0)
    }
    /// Returns a mutable iterator over the grid
    ///
    /// # Examples
    /// ```rust
    /// let mut grid = ArrayGrid::from([
    ///     [1,2,3],
    ///     [4,5,6],
    ///     [7,8,9],
    /// ])
    /// .iter_mut();
    ///
    /// assert_eq!(Some(1), grid.next());
    /// assert_eq!(Some(2), grid.next());
    /// assert_eq!(Some(3), grid.next());
    /// assert_eq!(Some(4), grid.next());
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T, W, H> {
        IterMut::new(&mut self.0)
    }
    /// Returns an iterator over the grid's rows
    ///
    /// # Examples
    /// ```rust
    /// let mut grid = ArrayGrid::from([
    ///     [1,2,3],
    ///     [4,5,6],
    ///     [7,8,9],
    /// ])
    /// .rows();
    ///
    /// assert_eq!(Some([Some(1),Some(2),Some(3)]), grid.next());
    /// assert_eq!(Some([Some(4),Some(5),Some(6)]), grid.next());
    /// assert_eq!(Some([Some(7),Some(8),Some(9)]), grid.next());
    /// ```
    pub fn rows(&self) -> IntoIter<Vec<&Option<T>>> {
        let mut vector = Vec::with_capacity(H);

        for rows in self.0.iter() {
            let mut row = Vec::with_capacity(W);

            for option in rows.iter() {
                row.push(option);
            }

            vector.push(row);
        }

        vector.into_iter()
    }
    /// Returns an iterator over the grid's columns
    ///
    /// # Examples
    /// ```rust
    /// let mut grid = ArrayGrid::from([
    ///     [1,2,3],
    ///     [4,5,6],
    ///     [7,8,9],
    /// ])
    /// .columns();
    ///
    /// assert_eq!(Some([Some(1),Some(4),Some(7)]), grid.next());
    /// assert_eq!(Some([Some(2),Some(5),Some(8)]), grid.next());
    /// assert_eq!(Some([Some(3),Some(6),Some(9)]), grid.next());
    /// ```
    pub fn columns(&self) -> IntoIter<Vec<&Option<T>>> {
        let mut vector = Vec::with_capacity(H);

        for x in 0..self.width() {
            let mut column = Vec::with_capacity(H);

            for y in 0..self.height() {
                column.push(&self[(x, y)]);
            }

            vector.push(column);
        }

        vector.into_iter()
    }
    /// Returns an iterator over the grid that also contains each value's position
    ///
    /// # Examples
    /// ```rust
    /// let mut grid = ArrayGrid::from([
    ///     [1,2,3],
    ///     [4,5,6],
    ///     [7,8,9],
    /// ])
    /// .enumerate();
    ///
    /// assert_eq!(Some((0, 0), 1), grid.next());
    /// assert_eq!(Some((1, 0), 2), grid.next());
    /// assert_eq!(Some((2, 0), 3), grid.next());
    /// assert_eq!(Some((0, 1), 4), grid.next());
    /// ```
    pub fn enumerate(&self) -> IntoIter<((usize, usize), &Option<T>)> {
        let mut vector = Vec::with_capacity(W * H);

        for (y, row) in self.0.iter().enumerate() {
            for (x, option) in row.iter().enumerate() {
                vector.push(((x, y), option));
            }
        }

        vector.into_iter()
    }
    /// Returns a mutable iterator over the grid
    ///
    /// This method allocates references into the heap
    ///
    /// # Examples
    /// ```rust
    /// let mut grid = ArrayGrid::from([
    ///     [1,2,3],
    ///     [4,5,6],
    ///     [7,8,9],
    /// ])
    /// .enumerate_mut();
    ///
    /// assert_eq!(Some(1), grid.next());
    /// assert_eq!(Some(2), grid.next());
    /// assert_eq!(Some(3), grid.next());
    /// assert_eq!(Some(4), grid.next());
    /// ```
    pub fn enumerate_mut(&mut self) -> IntoIter<((usize, usize), &mut Option<T>)> {
        let mut vector = Vec::with_capacity(W * H);

        for (y, row) in self.0.iter_mut().enumerate() {
            for (x, option) in row.iter_mut().enumerate() {
                vector.push(((x, y), option));
            }
        }

        vector.into_iter()
    }

    /// Returns a grid of the same size as `Self`, with function `f` applied to each value in order
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::from([
    ///     [1,2,3],
    ///     [4,5,6],
    ///     [7,8,9],
    /// ])
    /// .map(|option| match option {
    ///     Some(n) => Some(n + 1),
    ///     None => 1,
    /// });
    ///
    /// assert_eq!(Some(2), grid.get((0, 0)));
    /// ```
    pub fn map<U, F: Fn(Option<T>) -> Option<U>>(self, f: F) -> ArrayGrid<U, W, H> {
        ArrayGrid(self.0.map(|r| r.map(&f)))
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each `Some` value in order
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::from([
    ///     [Some(1), None, Some(3)],
    ///     [None, Some(5), None],
    ///     [Some(7), None, Some(9)],
    /// ])
    /// .map_some(|v| v + 1);
    ///
    /// assert_eq!(Some(6), grid.get((0, 2)));
    /// ```
    pub fn map_some<U, F: Fn(T) -> U>(self, f: F) -> ArrayGrid<U, W, H> {
        self.map(|o| o.map(&f))
    }
    /// Returns a grid of the same size as `Self`, with function `f` applied to each `None` value in order
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::from([
    ///     [Some(1), None, Some(3)],
    ///     [None, Some(5), None],
    ///     [Some(7), None, Some(9)],
    /// ])
    /// .map_none(|| 0);
    ///
    /// assert_eq!(Some(0), grid.get((0, 1)));
    /// ```
    pub fn map_none<F: Fn() -> T>(self, f: F) -> Self {
        self.map(|o| o.or_else(|| Some(f())))
    }
    /// Returns a grid of the same size as `Self`, filled with the provided value through cloning
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::from([
    ///     [Some(1), None, Some(3)],
    ///     [None, Some(5), None],
    ///     [Some(7), None, Some(9)],
    /// ])
    /// .fill(false);
    ///
    /// assert_eq!(Some(false), grid.get((1, 1)));
    /// assert_eq!(Some(false), grid.get((2, 1)));
    /// ```
    pub fn fill<U: Clone>(self, value: U) -> ArrayGrid<U, W, H> {
        self.map(|_| Some(value.clone()))
    }
    /// Returns a grid of the same size as `Self`, replacing all `Some` values with the provided value through cloning
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::from([
    ///     [Some(1), None, Some(3)],
    ///     [None, Some(5), None],
    ///     [Some(7), None, Some(9)],
    /// ])
    /// .fill_some(true);
    ///
    /// assert_eq!(Some(true), grid.get((1, 1)));
    /// assert_eq!(None, grid.get((2, 1)));
    /// ```
    pub fn fill_some<U: Clone>(self, value: U) -> ArrayGrid<U, W, H> {
        self.map_some(|_| value.clone())
    }
    /// Returns a grid of the same size as `Self`, replacing all `None` values with the provided value throuhg cloning
    ///
    /// # Examples
    /// ```rust
    /// let grid = ArrayGrid::from([
    ///     [Some(1), None, Some(3)],
    ///     [None, Some(5), None],
    ///     [Some(7), None, Some(9)],
    /// ])
    /// .fill_none(0);
    ///
    /// assert_eq!(Some(5), grid.get((1, 1)));
    /// assert_eq!(Some(0), grid.get((2, 1)));
    /// ```
    pub fn fill_none(self, value: T) -> Self
    where
        T: Clone,
    {
        self.map_none(|| value.clone())
    }

    /// Reverses each row of the grid
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [1, 2],
    ///     [3, 4]
    /// ]);
    ///
    /// let mut b = ArrayGrid::from([
    ///     [2, 1],
    ///     [4, 3],
    /// ]);
    ///
    /// b.flip_x();
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn flip_x(&mut self) {
        self.0.iter_mut().for_each(|r| r.reverse());
    }
    /// Reverses each column of the grid
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [1, 2],
    ///     [3, 4]
    /// ]);
    ///
    /// let mut b = ArrayGrid::from([
    ///     [3, 4],
    ///     [1, 2],
    /// ]);
    ///
    /// b.flip_y();
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn flip_y(&mut self) {
        self.0.reverse();
    }
    /// Shifts the grid to the left by the specified number of cells.
    ///
    /// Any number higher than the grid's width will be ignored
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// let mut b = ArrayGrid::from([
    ///     [3, 1, 2],
    ///     [6, 4, 5],
    ///     [8, 7, 8],
    /// ]);
    ///
    /// b.shift_left(1);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn shift_left(&mut self, cells: usize) {
        let cells = cells.min(self.width());
        self.0.iter_mut().for_each(|r| r.rotate_left(cells));
    }
    /// Shifts the grid to the right by the specified number of cells.
    ///
    /// Any number higher than the grid's width will be ignored
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [3, 1, 2],
    ///     [6, 4, 5],
    ///     [8, 7, 8],
    /// ]);
    ///
    /// let mut b = ArrayGrid::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// b.shift_left(1);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn shift_right(&mut self, cells: usize) {
        let cells = cells.min(self.width());
        self.0.iter_mut().for_each(|r| r.rotate_right(cells));
    }
    /// Shifts the grid upwards by the specified number of cells.
    ///
    /// Any number higher than the grid's height will be ignored
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// let mut b = ArrayGrid::from([
    ///     [7, 8, 9],
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    /// ]);
    ///
    /// b.shift_up(1);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn shift_up(&mut self, cells: usize) {
        let cells = cells.min(self.height());
        self.0.rotate_left(cells);
    }
    /// Shifts the grid downwards by the specified number of cells.
    ///
    /// Any number higher than the grid's height will be ignored
    ///
    /// # Examples
    /// ```rust
    /// let a = ArrayGrid::from([
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    /// ]);
    ///
    /// let mut b = ArrayGrid::from([
    ///     [4, 5, 6],
    ///     [7, 8, 9],
    ///     [1, 2, 3],
    /// ]);
    ///
    /// b.shift_down(1);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn shift_down(&mut self, cells: usize) {
        let cells = cells.min(self.height());
        self.0.rotate_right(cells);
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
