use crate::VarintSizeHint;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

pub trait VarintBaseType:
    Copy + Clone + PartialEq + PartialOrd + Eq + Ord + fmt::Debug + VarintSizeHint
{
}
impl<T: Copy + Clone + PartialEq + PartialOrd + Eq + Ord + fmt::Debug + VarintSizeHint>
    VarintBaseType for T
{
}

/// Variable length signed integer.
///
/// # Examples
///
/// ```rust
/// use std::mem;
///
/// use varuint::{Varint, Serializable, Deserializable};
///
/// fn test_varint(v: i128, size: usize) {
///     println!("{}", v);
///     let v = Varint(v);
///     assert_eq!(size, v.size_hint());
///     let mut arr: [u8; 17] = unsafe { mem::uninitialized() };
///     assert_eq!(size, v.serialize(&mut (&mut arr as &mut [u8])).unwrap());
///     assert_eq!(v, Varint::deserialize(&mut (&arr as &[u8])).unwrap());
/// }
///
/// test_varint(0, 1);
/// test_varint(1, 1);
/// test_varint(-1, 1);
///
/// test_varint(-120, 1);
/// test_varint(120, 1);
///
/// test_varint(-2031/2, 2);
/// test_varint(2031/2, 2);
///
/// test_varint(-67567/2, 3);
/// test_varint(67567/2, 3);
///
/// test_varint(-16777215/2, 4);
/// test_varint(16777215/2, 4);
///
/// test_varint(-4294967295/2, 5);
/// test_varint(4294967295/2, 5);
///
/// test_varint(-1099511627775/2, 6);
/// test_varint(1099511627775/2, 6);
///
/// test_varint(-281474976710655/2, 7);
/// test_varint(281474976710655/2, 7);
///
/// test_varint(-72057594037927935/2, 8);
/// test_varint(72057594037927935/2, 8);
///
/// test_varint(i64::min_value().into(), 9);
/// test_varint(i64::max_value().into(), 9);
///
/// test_varint(i128::min_value(), 17);
/// test_varint(i128::max_value(), 17);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Varint<T: VarintBaseType>(pub T);

impl<T: VarintBaseType + fmt::Display> fmt::Display for Varint<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: VarintBaseType> Deref for Varint<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: VarintBaseType> DerefMut for Varint<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<i8> for Varint<i8> {
    #[inline]
    fn from(i: i8) -> Self {
        Self(i)
    }
}

impl From<i8> for Varint<i16> {
    #[inline]
    fn from(i: i8) -> Self {
        Self(i16::from(i))
    }
}

impl From<i8> for Varint<i32> {
    #[inline]
    fn from(i: i8) -> Self {
        Self(i32::from(i))
    }
}

impl From<i8> for Varint<i64> {
    #[inline]
    fn from(i: i8) -> Self {
        Self(i64::from(i))
    }
}

impl From<i8> for Varint<i128> {
    #[inline]
    fn from(i: i8) -> Self {
        Self(i128::from(i))
    }
}

impl From<i16> for Varint<i16> {
    #[inline]
    fn from(i: i16) -> Self {
        Self(i)
    }
}

impl From<i16> for Varint<i32> {
    #[inline]
    fn from(i: i16) -> Self {
        Self(i32::from(i))
    }
}

impl From<i16> for Varint<i64> {
    #[inline]
    fn from(i: i16) -> Self {
        Self(i64::from(i))
    }
}

impl From<i16> for Varint<i128> {
    #[inline]
    fn from(i: i16) -> Self {
        Self(i128::from(i))
    }
}

impl From<i32> for Varint<i32> {
    #[inline]
    fn from(i: i32) -> Self {
        Self(i)
    }
}

impl From<i32> for Varint<i64> {
    #[inline]
    fn from(i: i32) -> Self {
        Self(i64::from(i))
    }
}

impl From<i32> for Varint<i128> {
    #[inline]
    fn from(i: i32) -> Self {
        Self(i128::from(i))
    }
}

impl From<i64> for Varint<i64> {
    #[inline]
    fn from(i: i64) -> Self {
        Self(i)
    }
}

impl From<i64> for Varint<i128> {
    #[inline]
    fn from(i: i64) -> Self {
        Self(i128::from(i))
    }
}

impl From<i128> for Varint<i128> {
    #[inline]
    fn from(v: i128) -> Self {
        Self(v)
    }
}

impl From<u8> for Varint<u128> {
    #[inline]
    fn from(v: u8) -> Self {
        Self(u128::from(v))
    }
}

impl From<u16> for Varint<u128> {
    #[inline]
    fn from(v: u16) -> Self {
        Self(u128::from(v))
    }
}

impl From<u32> for Varint<u128> {
    #[inline]
    fn from(v: u32) -> Self {
        Self(u128::from(v))
    }
}

impl From<u64> for Varint<u128> {
    #[inline]
    fn from(v: u64) -> Self {
        Self(u128::from(v))
    }
}

impl From<u128> for Varint<u128> {
    #[inline]
    fn from(v: u128) -> Self {
        Self(v)
    }
}

impl Default for Varint<u128> {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}

impl Default for Varint<i8> {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}

impl Default for Varint<i16> {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}

impl Default for Varint<i32> {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}

impl Default for Varint<i64> {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}

impl Default for Varint<i128> {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}
