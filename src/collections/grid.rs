use std::ops::{Index, IndexMut};

pub mod array;
pub mod vec;

/// Value that can be used to index into the grid
pub type Idx = (usize, usize);

/// Base trait for implementing custom grid types
pub trait Grid<'i, T: 'i>: Index<Idx, Output = Option<T>> + IndexMut<Idx> {
    /// Return type of the `iter` method
    type Iter: Iterator<Item = &'i Option<T>>;
    /// Return type of the `iter_mut` method
    type IterMut: Iterator<Item = &'i mut Option<T>>;

    /// Returns the size of the grid
    fn size(&self) -> Idx;
    /// Returns the width of the grid
    fn width(&self) -> usize {
        self.size().0
    }
    /// Returns the height of the grid
    fn height(&self) -> usize {
        self.size().1
    }
    /// Returns the total capacity of the grid
    fn capacity(&self) -> usize {
        self.width() * self.height()
    }

    /// Returns `true` if the grid contains the provided index
    fn contains_index(&self, index: Idx) -> bool {
        let (w, h) = self.size();
        let (x, y) = index;

        x < w && y < h
    }
    /// Returns `true` if the grid contains the given value
    fn contains_value(&'i self, value: T) -> bool
    where
        T: Clone + PartialEq,
    {
        self.iter().any(|v| v == &Some(value.clone()))
    }
    /// Returns `true` if the grid contains the given value at the provided index
    fn contains(&self, index: Idx, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.get(index) == Some(value)
    }

    /// Returns a reference to the value at the provided index
    fn get(&self, index: Idx) -> Option<&T> {
        self.contains_index(index)
            .then_some(self[index].as_ref())
            .flatten()
    }
    /// Returns a mutable reference to the value at the provided index
    fn get_mut(&mut self, index: Idx) -> Option<&mut T> {
        self.contains_index(index)
            .then_some(self[index].as_mut())
            .flatten()
    }

    /// Inserts the given value into the provided index, returning the old value
    fn insert(&mut self, index: Idx, value: T) -> Option<T> {
        let old = self.remove(index);
        self[index] = Some(value);
        old
    }

    /// Removes the value at the provided index, returning it
    fn remove(&mut self, index: Idx) -> Option<T> {
        self[index].take()
    }

    /// Returns an iterator over values of the grid
    fn iter(&'i self) -> Self::Iter;
    /// Returns a mutable iterator over values of the grid
    fn iter_mut(&'i mut self) -> Self::IterMut;

    /// Returns a list of every possible index within the grid
    fn indexes(&self) -> Vec<(usize, usize)> {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .collect()
    }
}
