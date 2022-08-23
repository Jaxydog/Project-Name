use std::array::IntoIter;

/// Represents one of four possible sides of a generator tile
#[repr(usize)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    #[default]
    Top = 0,
    Left = 1,
    Right = 2,
    Bottom = 3,
}

impl Side {
    /// Returns the side opposite to this one
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!(Side::Left.opposite(), Side::Right);
    /// ```
    pub const fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
        }
    }
    /// Returns the side closest to the provided `end` value, relative to the `start` value
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!(Side::relative_x(-1, 1), Side::Left);
    /// ```
    pub const fn relative_x(start: usize, end: usize) -> Self {
        if start <= end {
            Self::Left
        } else {
            Self::Right
        }
    }
    /// Returns the side closest to the provided `end` value, relative to the `start` value
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!(Side::relative_y(-1, 1), Side::Top);
    /// ```
    pub const fn relative_y(start: usize, end: usize) -> Self {
        if start <= end {
            Self::Top
        } else {
            Self::Bottom
        }
    }
    /// Returns the side closest to the provided `end` position, relative to the `start` position
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!(Side::relative((0, 0), (0, 1)), Side::Top);
    /// assert_eq!(Side::relative((12, 0), (-40, 0)), Side::Right);
    /// ```
    pub const fn relative(start: (usize, usize), end: (usize, usize)) -> Self {
        if start.0 == end.0 {
            Self::relative_y(start.1, end.1)
        } else {
            Self::relative_x(start.0, end.0)
        }
    }
}

impl From<Side> for usize {
    fn from(side: Side) -> Self {
        side as Self
    }
}

impl IntoIterator for Side {
    type Item = Self;
    type IntoIter = IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        let mut base = [Self::Top, Self::Left, Self::Right, Self::Bottom];
        base.sort_unstable();
        base.rotate_left(self.into());
        base.into_iter()
    }
}

/// Represents one of four possible rotations of a generator tile
#[repr(usize)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rotation {
    #[default]
    D0 = 0,
    D90 = 1,
    D180 = 2,
    D270 = 3,
}

impl From<Rotation> for usize {
    fn from(rotation: Rotation) -> Self {
        rotation as Self
    }
}

impl IntoIterator for Rotation {
    type Item = Self;
    type IntoIter = IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        let mut base = [Self::D0, Self::D90, Self::D180, Self::D270];
        base.sort_unstable();
        base.rotate_left(self.into());
        base.into_iter()
    }
}
