use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Bound, Div,
    DivAssign, Mul, MulAssign, Neg, RangeBounds, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign,
    Sub, SubAssign,
};

/// Represents a value that can be set to any value within a given range
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct RangedValue<N, R: RangeBounds<N>>(N, R);

impl<N, R: RangeBounds<N>> RangedValue<N, R> {
    /// Returns a reference to the ranged value's value
    ///
    /// # Examples
    /// ```rust
    /// let number = RangedValue::new(0..=100, 50);
    ///
    /// assert_eq!(50, number.value());
    /// ```
    pub const fn value(&self) -> &N {
        &self.0
    }
    /// Returns a mutable reference to the ranged value's value
    ///
    /// # Examples
    /// ```rust
    /// let number = RangedValue::new(0..=100, 50);
    ///
    /// number.value_mut() += 20;
    ///
    /// assert_eq!(70, number.value());
    /// ```
    pub fn value_mut(&mut self) -> &mut N {
        &mut self.0
    }
    /// Returns a reference to the ranged value's range start
    ///
    /// # Examples
    /// ```rust
    /// let number = RangedValue::new(0..=100, 50);
    ///
    /// assert_eq!(Some(0), number.start());
    /// ```
    pub fn start(&self) -> Option<&N> {
        match self.1.start_bound() {
            Bound::Included(n) | Bound::Excluded(n) => Some(n),
            Bound::Unbounded => None,
        }
    }
    /// Returns a reference to the ranged value's range end
    ///
    /// # Examples
    /// ```rust
    /// let number = RangedValue::new(0..=100, 50);
    ///
    /// assert_eq!(Some(100), number.end());
    /// ```
    pub fn end(&self) -> Option<&N> {
        match self.1.end_bound() {
            Bound::Included(n) | Bound::Excluded(n) => Some(n),
            Bound::Unbounded => None,
        }
    }
}

impl<N: PartialOrd, R: RangeBounds<N>> RangedValue<N, R> {
    /// Returns `true` if the provided value is within the ranged value's range
    ///
    /// # Examples
    /// ```rust
    /// let number = RangedValue::new(0..=100, 0);
    ///
    /// assert!(number.contains(25));
    /// assert!(number.contains(75));
    /// assert!(!number.contains(101));
    /// ```
    pub fn contains(&self, value: &N) -> bool {
        self.1.contains(value)
    }
    /// Returns `true` if the provided value is under the ranged value's range start
    ///
    /// # Examples
    /// ```rust
    /// let number = RangedValue::new(0..=100, 0);
    ///
    /// assert!(number.underflows(-1));
    /// assert!(!number.underflows(101));
    /// ```
    pub fn underflows(&self, value: &N) -> bool {
        self.start().map_or(false, |s| s > value)
    }
    /// Returns `true` if the provided value is over the ranged value's range end
    ///
    /// # Examples
    /// ```rust
    /// let number = RangedValue::new(0..=100, 0);
    ///
    /// assert!(number.overflows(101));
    /// assert!(!number.overflows(-1));
    /// ```
    pub fn overflows(&self, value: &N) -> bool {
        self.end().is_some() && !self.contains(value) && !self.underflows(value)
    }
}

impl<N: Clone + PartialOrd, R: RangeBounds<N>> RangedValue<N, R> {
    /// Creates a new range number, defaulting to the lowest bound if the provided value is invalid
    ///
    /// # Examples
    /// ```rust
    /// let number = RangedValue::new(0..=100, 50);
    ///
    /// assert_eq!(50, number.value());
    /// assert_eq!(Some(0), number.start());
    /// assert_eq!(Some(100), number.end());
    /// ```
    pub fn new(range: R, value: N) -> Self {
        if range.contains(&value) {
            Self(value, range)
        } else {
            let default = match range.start_bound() {
                Bound::Included(n) | Bound::Excluded(n) => Some(n),
                Bound::Unbounded => None,
            }
            .map(ToOwned::to_owned);

            Self(default.unwrap_or(value), range)
        }
    }

    /// Performs an operation on the ranged value
    ///
    /// # Examples
    /// ```rust
    /// let number = RangedValue::new(0..=100, 50);
    ///
    /// assert_eq!(20, number.operate(|v| v - 30));
    /// ```
    pub fn operate<F: FnOnce(N) -> N>(&self, f: F) -> N {
        let n = f(self.value().clone());

        if self.underflows(&n) {
            self.start().unwrap_or_else(|| self.value()).to_owned()
        } else if self.overflows(&n) {
            self.end().unwrap_or_else(|| self.value()).to_owned()
        } else {
            n
        }
    }
    /// Performs an operation on the ranged value and assigns the result
    ///
    /// # Examples
    /// ```rust
    /// let mut number = RangedValue::new(0..=100, 50);
    ///
    /// number.assign(|v| v - 12);
    ///
    /// assert_eq!(38, number.value());
    /// ```
    pub fn assign<F: FnOnce(N) -> N>(&mut self, f: F) {
        self.0 = self.operate(f);
    }
}

impl<N, R> Add<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Add<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn add(self, rhs: N) -> Self::Output {
        self.operate(|n| n.add(rhs))
    }
}

impl<N, R> AddAssign<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Add<Output = N> + AddAssign,
    R: RangeBounds<N>,
{
    fn add_assign(&mut self, rhs: N) {
        self.assign(|n| n.add(rhs));
    }
}

impl<N, R> BitAnd<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + BitAnd<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn bitand(self, rhs: N) -> Self::Output {
        self.operate(|n| n.bitand(rhs))
    }
}

impl<N, R> BitAndAssign<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + BitAnd<Output = N> + BitAndAssign,
    R: RangeBounds<N>,
{
    fn bitand_assign(&mut self, rhs: N) {
        self.assign(|n| n.bitand(rhs));
    }
}

impl<N, R> BitOr<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + BitOr<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn bitor(self, rhs: N) -> Self::Output {
        self.operate(|n| n.bitor(rhs))
    }
}

impl<N, R> BitOrAssign<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + BitOr<Output = N> + BitOrAssign,
    R: RangeBounds<N>,
{
    fn bitor_assign(&mut self, rhs: N) {
        self.assign(|n| n.bitor(rhs));
    }
}

impl<N, R> BitXor<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + BitXor<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn bitxor(self, rhs: N) -> Self::Output {
        self.operate(|n| n.bitxor(rhs))
    }
}

impl<N, R> BitXorAssign<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + BitXor<Output = N> + BitXorAssign,
    R: RangeBounds<N>,
{
    fn bitxor_assign(&mut self, rhs: N) {
        self.assign(|n| n.bitxor(rhs));
    }
}

impl<N, R> Div<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Div<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn div(self, rhs: N) -> Self::Output {
        self.operate(|n| n.div(rhs))
    }
}

impl<N, R> DivAssign<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Div<Output = N> + DivAssign,
    R: RangeBounds<N>,
{
    fn div_assign(&mut self, rhs: N) {
        self.assign(|n| n.div(rhs));
    }
}

impl<N, R> Mul<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Mul<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn mul(self, rhs: N) -> Self::Output {
        self.operate(|n| n.mul(rhs))
    }
}

impl<N, R> MulAssign<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Mul<Output = N> + MulAssign,
    R: RangeBounds<N>,
{
    fn mul_assign(&mut self, rhs: N) {
        self.assign(|n| n.mul(rhs));
    }
}

impl<N, R> Neg for RangedValue<N, R>
where
    N: Clone + PartialOrd + Neg<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn neg(self) -> Self::Output {
        self.operate(Neg::neg)
    }
}

impl<N, R> Rem<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Rem<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn rem(self, rhs: N) -> Self::Output {
        self.operate(|n| n.rem(rhs))
    }
}

impl<N, R> RemAssign<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Rem<Output = N> + RemAssign,
    R: RangeBounds<N>,
{
    fn rem_assign(&mut self, rhs: N) {
        self.assign(|n| n.rem(rhs));
    }
}

impl<N, R> Shl<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Shl<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn shl(self, rhs: N) -> Self::Output {
        self.operate(|n| n.shl(rhs))
    }
}

impl<N, R> ShlAssign<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Shl<Output = N> + ShlAssign,
    R: RangeBounds<N>,
{
    fn shl_assign(&mut self, rhs: N) {
        self.assign(|n| n.shl(rhs));
    }
}

impl<N, R> Shr<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Shr<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn shr(self, rhs: N) -> Self::Output {
        self.operate(|n| n.shr(rhs))
    }
}

impl<N, R> ShrAssign<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Shr<Output = N> + ShrAssign,
    R: RangeBounds<N>,
{
    fn shr_assign(&mut self, rhs: N) {
        self.assign(|n| n.shr(rhs));
    }
}

impl<N, R> Sub<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Sub<Output = N>,
    R: RangeBounds<N>,
{
    type Output = N;

    fn sub(self, rhs: N) -> Self::Output {
        self.operate(|n| n.sub(rhs))
    }
}

impl<N, R> SubAssign<N> for RangedValue<N, R>
where
    N: Clone + PartialOrd + Sub<Output = N> + SubAssign,
    R: RangeBounds<N>,
{
    fn sub_assign(&mut self, rhs: N) {
        self.assign(|n| n.sub(rhs));
    }
}
