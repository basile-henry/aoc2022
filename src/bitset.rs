use core::mem::size_of;
use core::ops::{BitAnd, BitOr, BitOrAssign, Shl};
use num::{one, zero, One, Zero};

#[allow(dead_code)]
pub(crate) type U128Set = BitSet<u128>;
pub(crate) type U64Set = BitSet<u64>;
pub(crate) type U32Set = BitSet<u32>;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BitSet<T>(T);

impl<T> BitSet<T>
where
    T: BitAnd<Output = T>,
    T: BitOr<Output = T>,
    T: BitOrAssign,
    T: Shl<u8, Output = T>,
    T: Zero,
    T: One,
    T: PartialOrd,
    T: Copy,
{
    pub fn empty() -> Self {
        BitSet(zero())
    }

    pub fn contains(&self, item: u8) -> bool {
        debug_assert!((item as usize) < size_of::<T>() * 8);
        (self.0 & one::<T>().shl(item)) > zero()
    }

    pub fn insert(&mut self, item: u8) {
        debug_assert!((item as usize) < size_of::<T>() * 8);
        self.0 |= one::<T>() << item;
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    pub fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn iter(&'_ self) -> impl Iterator<Item = u8> + '_ {
        (0..(size_of::<T>() * 8) as u8).filter(|x| self.contains(*x))
    }

    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.iter().count()
    }

    /// Insert only new elements to the set
    /// Returns true if it successfully inserted the whole iterator
    /// Returns early without consuming the whole iterator otherwise
    pub fn insert_only_new(&mut self, mut iter: impl Iterator<Item = u8>) -> bool {
        iter.try_fold(self, |s, e| {
            if s.contains(e) {
                None
            } else {
                s.insert(e);
                Some(s)
            }
        })
        .is_some()
    }
}

impl<U> FromIterator<u8> for BitSet<U>
where
    U: BitAnd<Output = U>,
    U: BitOr<Output = U>,
    U: BitOrAssign,
    U: Shl<u8, Output = U>,
    U: Zero,
    U: One,
    U: PartialOrd,
    U: Copy,
{
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut set = Self::empty();

        for x in iter {
            set.insert(x);
        }

        set
    }
}
