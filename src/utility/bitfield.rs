/// Maximum number of flags possible on a `BitField`
pub const MAX_NORMAL_FLAGS: u64 = 64;
/// Maximum number of flags possible on a `LargeBitField`
pub const MAX_LARGE_FLAGS: u128 = 128;

/// Occurs when trying to use a flag that does not fit within a bit field
#[derive(Debug)]
pub struct FlagTooLarge<N>(N, N);

/// Common methods for bit field implementations
pub trait BitFieldResolvable<N> {
    /// Returns `true` if the bit field contains the provided flag
    fn contains<T: Into<N>>(&self, flag: T) -> Result<bool, FlagTooLarge<N>>;
    /// Inserts the provided flag into the bit field
    fn insert<T: Into<N>>(&mut self, flag: T) -> Result<(), FlagTooLarge<N>>;
    /// Removes the provided flag into the bit field
    fn remove<T: Into<N>>(&mut self, flag: T) -> Result<(), FlagTooLarge<N>>;

    /// Returns `true` if every provided flag is present within the bit field
    fn contains_all<T: Clone + Into<N>>(&self, flags: &[T]) -> Result<bool, FlagTooLarge<N>> {
        for flag in flags {
            if !self.contains(flag.clone())? {
                return Ok(false);
            }
        }

        Ok(true)
    }
    /// Returns `true` if any provided flag is present within the bit field
    fn contains_any<T: Clone + Into<N>>(&self, flags: &[T]) -> Result<bool, FlagTooLarge<N>> {
        for flag in flags {
            if self.contains(flag.clone())? {
                return Ok(true);
            }
        }

        Ok(false)
    }
    /// Inserts the provided flag into the bit field
    fn insert_all<T: Clone + Into<N>>(&mut self, flags: &[T]) -> Result<(), FlagTooLarge<N>> {
        for flag in flags {
            self.insert(flag.clone())?;
        }

        Ok(())
    }
    /// Removes the provided flag into the bit field
    fn remove_all<T: Clone + Into<N>>(&mut self, flags: &[T]) -> Result<(), FlagTooLarge<N>> {
        for flag in flags {
            self.remove(flag.clone())?;
        }

        Ok(())
    }
}

/// Bit field that may contain up to 64 flags
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitField(u64);

impl BitFieldResolvable<u64> for BitField {
    fn contains<T: Into<u64>>(&self, flag: T) -> Result<bool, FlagTooLarge<u64>> {
        let flag = flag.into();

        if flag >= MAX_NORMAL_FLAGS {
            Err(FlagTooLarge(MAX_NORMAL_FLAGS, flag))
        } else {
            Ok(self.0 & (1 << flag) != 0)
        }
    }
    fn insert<T: Into<u64>>(&mut self, flag: T) -> Result<(), FlagTooLarge<u64>> {
        let flag = flag.into();

        if flag >= MAX_NORMAL_FLAGS {
            Err(FlagTooLarge(MAX_NORMAL_FLAGS, flag))
        } else {
            self.0 |= 1 << flag;
            Ok(())
        }
    }
    fn remove<T: Into<u64>>(&mut self, flag: T) -> Result<(), FlagTooLarge<u64>> {
        let flag = flag.into();

        if flag >= MAX_NORMAL_FLAGS {
            Err(FlagTooLarge(MAX_NORMAL_FLAGS, flag))
        } else {
            self.0 ^= 1 << flag;
            Ok(())
        }
    }
}

/// Bit field that may contain up to 128 flags
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LargeBitField(u128);

impl BitFieldResolvable<u128> for LargeBitField {
    fn contains<T: Into<u128>>(&self, flag: T) -> Result<bool, FlagTooLarge<u128>> {
        let flag = flag.into();

        if flag >= MAX_LARGE_FLAGS {
            Err(FlagTooLarge(MAX_LARGE_FLAGS, flag))
        } else {
            Ok(self.0 & (1 << flag) != 0)
        }
    }
    fn insert<T: Into<u128>>(&mut self, flag: T) -> Result<(), FlagTooLarge<u128>> {
        let flag = flag.into();

        if flag >= MAX_LARGE_FLAGS {
            Err(FlagTooLarge(MAX_LARGE_FLAGS, flag))
        } else {
            self.0 |= 1 << flag;
            Ok(())
        }
    }
    fn remove<T: Into<u128>>(&mut self, flag: T) -> Result<(), FlagTooLarge<u128>> {
        let flag = flag.into();

        if flag >= MAX_LARGE_FLAGS {
            Err(FlagTooLarge(MAX_LARGE_FLAGS, flag))
        } else {
            self.0 ^= 1 << flag;
            Ok(())
        }
    }
}
