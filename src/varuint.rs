use std::{fmt, ptr, ops::{Deref, DerefMut}, mem, io::{Read, Write, Result}};

/// Trait for serializable types
pub trait Serializable {
    /// Get a hint of encoded value byte-length
    fn size_hint(&self) -> usize;
    /// Serialize a value, returns bytes written
    fn serialize(&self, w: &mut Write) -> Result<usize>;
}

/// Trait for deserializable types
pub trait Deserializable : Sized {
    /// Deserialize value from a `Read`
    fn deserialize(r: &mut Read) -> Result<Self>;
}

/// Variable length unsigned integer.
/// 
/// # Examples
/// 
/// ```rust
/// use std::mem;
/// 
/// use varuint::{Varuint, Serializable, Deserializable};
/// use std::io::Read;
/// 
/// 
/// fn test_varuint(v: u128, size: usize) {
///  let v = Varuint(v);
///  assert_eq!(size, v.size_hint());
///  let mut arr: [u8; 17] = unsafe { mem::uninitialized() };
///  {
///      let mut buf = &mut arr as &mut [u8];
///      assert_eq!(size, v.serialize(&mut buf).unwrap());
///  }
///  let mut buf: &[u8] = &arr;
///  let mut read: &mut Read = &mut buf;
///  assert_eq!(v, Varuint::deserialize(read).unwrap());
/// }
/// 
/// test_varuint(0, 1);
/// test_varuint(240, 1);
/// 
/// test_varuint(241, 2);
/// test_varuint(2031, 2);
/// 
/// test_varuint(2032, 3);
/// test_varuint(67567, 3);
/// 
/// test_varuint(67568, 4);
/// test_varuint(16777215, 4);
/// 
/// test_varuint(16777216, 5);
/// test_varuint(4294967295, 5);
/// 
/// test_varuint(4294967296, 6);
/// test_varuint(1099511627775, 6);
/// 
/// test_varuint(1099511627776, 7);
/// test_varuint(281474976710655, 7);
/// 
/// test_varuint(281474976710656, 8);
/// test_varuint(72057594037927935, 8);
/// 
/// test_varuint(72057594037927936, 9);
/// test_varuint(u64::max_value().into(), 9);
///
/// test_varuint(u64::max_value() as u128 + 1, 17);
/// test_varuint(u128::max_value(), 17);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Varuint(pub u128);

impl fmt::Display for Varuint {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Varuint {
    type Target = u128;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Varuint {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u8> for Varuint {
    #[inline]
    fn from(v: u8) -> Self {
        Varuint(v as u128)
    }
}

impl From<u16> for Varuint {
    #[inline]
    fn from(v: u16) -> Self {
        Varuint(v as u128)
    }
}

impl From<u32> for Varuint {
    #[inline]
    fn from(v: u32) -> Self {
        Varuint(v as u128)
    }
}

impl From<u64> for Varuint {
    #[inline]
    fn from(v: u64) -> Self {
        Varuint(v as u128)
    }
}

impl From<u128> for Varuint {
    #[inline]
    fn from(v: u128) -> Self {
        Varuint(v)
    }
}

impl Default for Varuint {
    #[inline]
    fn default() -> Self {
        Varuint(0u128)
    }
}

#[inline(always)]
fn write_value(buf: &mut [u8], v: u128, size: usize) {
    for i in 0..size {
        buf[i] = (v >> (8 * i)) as u8;
    }
}

impl Varuint {
    fn serialize_buf(&self, buf: &mut [u8]) -> usize {
        let size = self.size_hint();
        let v = self.0;
        match size {
            1 => buf[0] = v as u8,
            2 => {
                buf[0] = ((v - 240) / 256 + 241) as u8;
                buf[1] = ((v - 240) % 256) as u8;
            }
            3 => {
                buf[0] = 248;
                buf[1] = ((v - 2032) / 256) as u8;
                buf[2] = ((v - 2032) % 256) as u8;
            }
            4 => {
                buf[0] = 249;
                write_value(&mut buf[1..], v, 3);
            }
            5 => {
                buf[0] = 250;
                unsafe { ptr::copy_nonoverlapping(&v as *const _ as *const u32, &mut buf[1] as *mut u8 as *mut u32, 1); }
            }
            6 => {
                buf[0] = 251;
                write_value(&mut buf[1..], v, 5);
            }
            7 => {
                buf[0] = 252;
                write_value(&mut buf[1..], v, 6);
            }
            8 => {
                buf[0] = 253;
                write_value(&mut buf[1..], v, 7);
            }
            9 => {
                buf[0] = 254;
                unsafe { ptr::copy_nonoverlapping(&v as *const _ as *const u64, &mut buf[1] as *mut u8 as *mut u64, 1); }
            }
            17 => {
                buf[0]  = 255;
                unsafe { ptr::copy_nonoverlapping(&v as *const _ as *const u128, &mut buf[1] as *mut u8 as *mut u128, 1); }
            }
            _ => unreachable!()
        };
        size
    }
}

#[inline(always)]
fn read_value(buf: &[u8], size: usize) -> u128 {
    let mut v = 0;
    for i in 0..size {
        v |= (buf[i] as u128) << (8 * i);
    }
    v
}

impl Deserializable for Varuint {

    fn deserialize(r: &mut Read) -> Result<Varuint> {
        let mut buf: [u8; 17] = unsafe { mem::uninitialized() };
        r.read_exact(&mut buf[0..1])?;
        let length = match buf[0] {
            v @ 0...240 => return Ok(Varuint(v as u128)),
            241...247 => 2,
            248 => 3,
            249 => 4,
            250 => 5,
            251 => 6,
            252 => 7,
            253 => 8,
            254 => 9,
            255 => 17,
            _ => unreachable!()
        };
        r.read_exact(&mut buf[1..length])?;
        Ok(Varuint(match length {
            2 => 240u128 + 256u128 * (buf[0] as u128 - 241u128) + buf[1] as u128,
            3 => 2032u128 + 256u128 * buf[1] as u128 + buf[2] as u128,
            4 => read_value(&buf[1..], 3),
            5 => {
                let mut v: u32 = unsafe { mem::uninitialized() };
                unsafe { ptr::copy_nonoverlapping(&buf[1] as *const _ as *const u32, &mut v, 1); }
                v as u128
            },
            6 => read_value(&buf[1..], 5),
            7 => read_value(&buf[1..], 6),
            8 => read_value(&buf[1..], 7),
            9 => {
                let mut v: u64 = unsafe { mem::uninitialized() };
                unsafe { ptr::copy_nonoverlapping(&buf[1] as *const _ as *const u64, &mut v, 1); }
                v as u128
            },
            17 => {
                let mut v: u128 = unsafe { mem::uninitialized() };
                unsafe { ptr::copy_nonoverlapping(&buf[1] as *const _ as *const u128, &mut v, 1); }
                v
            },
            _ => unreachable!()
        }))
    }
}

impl Serializable for Varuint {

    fn size_hint(&self) -> usize {
        let v = self.0;
        if v <= 240 {
            1
        } else if v <= 2031 {
            2
        } else if v <= 67567 {
            3
        } else if v <= 16777215 {
            4
        } else if v <= 4294967295 {
            5
        } else if v <= 1099511627775 {
            6
        } else if v <= 281474976710655 {
            7
        } else if v <= 72057594037927935 {
            8
        } else if v <= 18446744073709551615 {
            9
        } else {
            17
        }
    }

    fn serialize(&self, w: &mut Write) -> Result<usize> {
        let mut buf: [u8; 17] = unsafe { mem::uninitialized() };
        let size = self.serialize_buf(&mut buf as &mut [u8]);
        w.write(&buf[0..size])
    }
}

/// Variable length signed integer.
/// 
/// # Examples
/// 
/// ```rust
/// use std::mem;
/// 
/// use varuint::{Varint, Serializable, Deserializable};
/// use std::io::Read;
/// 
/// 
/// fn test_varint(v: i128, size: usize) {
///     println!("{}", v);
///     let v = Varint(v);
///     assert_eq!(size, v.size_hint());
///     let mut arr: [u8; 17] = unsafe { mem::uninitialized() };
///     {
///         let mut buf = &mut arr as &mut [u8];
///         assert_eq!(size, v.serialize(&mut buf).unwrap());
///     }
///     let mut buf: &[u8] = &arr;
///     let mut read: &mut Read = &mut buf;
///     assert_eq!(v, Varint::deserialize(read).unwrap());
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
pub struct Varint(pub i128);

impl fmt::Display for Varint {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Varint {
    type Target = i128;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Varint {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<i8> for Varint {
    #[inline]
    fn from(i: i8) -> Self {
        Varint(i.into())
    }
}

impl From<i16> for Varint {
    #[inline]
    fn from(i: i16) -> Self {
        Varint(i.into())
    }
}

impl From<i32> for Varint {
    #[inline]
    fn from(i: i32) -> Self {
        Varint(i.into())
    }
}

impl From<i64> for Varint {
    #[inline]
    fn from(i: i64) -> Self {
        Varint(i.into())
    }
}

impl From<i128> for Varint {
    #[inline]
    fn from(v: i128) -> Self {
        Varint(v)
    }
}

impl Default for Varint {
    #[inline]
    fn default() -> Self {
        Varint(0)
    }
}

#[inline]
fn varint_to_varuint(v: i128) -> u128 {
    ((v << 1) ^ (v >> 127)) as u128
}

#[inline]
fn varuint_to_varint(v: u128) -> i128 {
    ((v >> 1) as i128) ^ -((v & 1) as i128)
}

impl Serializable for Varint {

    #[inline]
    fn size_hint(&self) -> usize {
        Varuint(varint_to_varuint(self.0)).size_hint()
    }

    #[inline]
    fn serialize(&self, w: &mut Write) -> Result<usize> {
        Varuint(varint_to_varuint(self.0)).serialize(w)
    }
}

impl Deserializable for Varint {

    fn deserialize(r: &mut Read) -> Result<Varint> {
        Ok(Varint(varuint_to_varint(*Varuint::deserialize(r)?)))
    }
}

