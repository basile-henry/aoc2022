use core::mem::size_of;
use core::ops::{BitAnd, BitOr, BitOrAssign, Shl};
use num::{one, zero, One, Zero};

#[allow(dead_code)]
pub(crate) type U128Set = BitSet<u128>;
pub(crate) type U64Set = BitSet<u64>;
pub(crate) type U32Set = BitSet<u32>;

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

    #[allow(dead_code)]
    pub fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn iter(&'_ self) -> impl Iterator<Item = u8> + '_ {
        (0..(size_of::<T>() * 8) as u8).filter(|x| self.contains(*x))
    }

    pub fn count(&self) -> usize {
        self.iter().count()
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
