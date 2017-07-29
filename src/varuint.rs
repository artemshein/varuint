use std::io::{self, Read, Write, Result};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::fmt;

/// Variable length signed and unsigned integer types.
/// Types support up to 64-bit integers (128-bit integers support will be added when Rust has the
/// i128 and u128 types).
///
/// See [`Varuint`] and [`Varint`] for more details.
///

pub trait VarType {
    fn size_hint(&self) -> usize;
    fn write(&self, w: &mut Write) -> Result<usize>;
    fn write_buf(&self, buf: &mut [u8]) -> usize;
}

/// Variable length unsigned integer.
///
/// ```
/// use std::mem;
///
/// use varuint::{Varuint, VarType};
/// use std::io::Read;
///
///
/// fn test_varuint(v: u64, size: usize) {
///     let v = Varuint(v);
///     let mut arr: [u8; 9] = unsafe { mem::uninitialized() };
///     {
///         let mut buf = &mut arr as &mut [u8];
///         assert_eq!(size, v.write(&mut buf).unwrap());
///     }
///     let mut buf: &[u8] = &arr;
///     let mut read: &mut Read = &mut buf;
///     assert_eq!(v, Varuint::read(read).unwrap());
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
/// test_varuint(u64::max_value(), 9);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Varuint(pub u64);

impl fmt::Display for Varuint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Varuint {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Varuint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Varuint {
    pub fn read(mut r: &mut Read) -> Result<Varuint> {
        let mut buf: [u8; 9] = unsafe { mem::uninitialized() };
        r.read_exact(&mut buf[0..1])?;
        let length = match buf[0] {
            v @ 0...240 => return Ok(Varuint(v as u64)),
            241...247 => 2,
            248 => 3,
            249 => 4,
            250 => 5,
            251 => 6,
            252 => 7,
            253 => 8,
            254 => 9,
            255 => return Err(io::ErrorKind::InvalidData.into()), // not supported yet
            _ => unreachable!()
        };
        r.read_exact(&mut buf[1..length])?;
        Ok(Varuint(match length {
            2 => 240u64 + 256u64 * (buf[0] as u64 - 241u64) + buf[1] as u64,
            3 => 2032u64 + 256u64 * buf[1] as u64 + buf[2] as u64,
            4 => ((buf[1] as u64) << 16) | ((buf[2] as u64) << 8) | buf[3] as u64,
            5 => ((buf[1] as u64) << 24) | ((buf[2] as u64) << 16) | ((buf[3] as u64) << 8) | buf[4] as u64,
            6 => ((buf[1] as u64) << 32) | ((buf[2] as u64) << 24) | ((buf[3] as u64) << 16) | ((buf[4] as u64) << 8) | buf[5] as u64,
            7 => ((buf[1] as u64) << 40) | ((buf[2] as u64) << 32) | ((buf[3] as u64) << 24) | ((buf[4] as u64) << 16) | ((buf[5] as u64) << 8) | buf[6] as u64,
            8 => ((buf[1] as u64) << 48) | ((buf[2] as u64) << 40) | ((buf[3] as u64) << 32) | ((buf[4] as u64) << 24) | ((buf[5] as u64) << 16) | ((buf[6] as u64) << 8) | buf[7] as u64,
            9 => ((buf[1] as u64) << 56) | ((buf[2] as u64) << 48) | ((buf[3] as u64) << 40) | ((buf[4] as u64) << 32) | ((buf[5] as u64) << 24) | ((buf[6] as u64) << 16) | ((buf[7] as u64) << 8) | buf[8] as u64,
            _ => unreachable!()
        }))
    }
}

impl VarType for Varuint {

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
        } else {
            9
        }
        // u128 is not supported yet
    }

    fn write(&self, w: &mut Write) -> Result<usize> {
        let mut buf: [u8; 9] = unsafe { mem::uninitialized() };
        let size = self.write_buf(&mut buf as &mut [u8]);
        w.write(&buf[0..size])
    }

    fn write_buf(&self, mut buf: &mut [u8]) -> usize {
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
                buf[1] = (v >> 16) as u8;
                buf[2] = (v >> 8) as u8;
                buf[3] = v as u8;
            }
            5 => {
                buf[0] = 250;
                buf[1] = (v >> 24) as u8;
                buf[2] = (v >> 16) as u8;
                buf[3] = (v >> 8) as u8;
                buf[4] = v as u8;
            }
            6 => {
                buf[0] = 251;
                buf[1] = (v >> 32) as u8;
                buf[2] = (v >> 24) as u8;
                buf[3] = (v >> 16) as u8;
                buf[4] = (v >> 8) as u8;
                buf[5] = v as u8;
            }
            7 => {
                buf[0] = 252;
                buf[1] = (v >> 40) as u8;
                buf[2] = (v >> 32) as u8;
                buf[3] = (v >> 24) as u8;
                buf[4] = (v >> 16) as u8;
                buf[5] = (v >> 8) as u8;
                buf[6] = v as u8;
            }
            8 => {
                buf[0] = 253;
                buf[1] = (v >> 48) as u8;
                buf[2] = (v >> 40) as u8;
                buf[3] = (v >> 32) as u8;
                buf[4] = (v >> 24) as u8;
                buf[5] = (v >> 16) as u8;
                buf[6] = (v >> 8) as u8;
                buf[7] = v as u8;
            }
            9 => {
                buf[0] = 254;
                buf[1] = (v >> 56) as u8;
                buf[2] = (v >> 48) as u8;
                buf[3] = (v >> 40) as u8;
                buf[4] = (v >> 32) as u8;
                buf[5] = (v >> 24) as u8;
                buf[6] = (v >> 16) as u8;
                buf[7] = (v >> 8) as u8;
                buf[8] = v as u8;
            }
            _ => unreachable!()
        };
        size
    }

}

/// Variable length signed integer.
///
/// ```
/// use std::mem;
///
/// use varuint::{Varint, VarType};
/// use std::io::Read;
///
///
/// fn test_varint(v: i64, size: usize) {
///     let v = Varint(v);
///     let mut arr: [u8; 9] = unsafe { mem::uninitialized() };
///     {
///         let mut buf = &mut arr as &mut [u8];
///         assert_eq!(size, v.write(&mut buf).unwrap());
///     }
///     let mut buf: &[u8] = &arr;
///     let mut read: &mut Read = &mut buf;
///     assert_eq!(v, Varint::read(read).unwrap());
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
/// test_varint(i64::min_value(), 9);
/// test_varint(i64::max_value(), 9);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
pub struct Varint(pub i64);

impl fmt::Display for Varint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Varint {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Varint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[inline]
fn varint_to_varuint(v: i64) -> u64 {
    ((v << 1) ^ (v >> 63)) as u64
}

#[inline]
fn varuint_to_varint(v: u64) -> i64 {
    let sign_shifted = (v & 1) as i64;
    let val_shifted = v ^ (((sign_shifted << 63) >> 63) as u64);
    ((val_shifted >> 1) ^ ((sign_shifted << 63) as u64)) as i64
}

impl VarType for Varint {

    fn size_hint(&self) -> usize {
        Varuint(varint_to_varuint(self.0)).size_hint()
    }

    fn write(&self, w: &mut Write) -> Result<usize> {
        Varuint(varint_to_varuint(self.0)).write(w)
    }

    fn write_buf(&self, mut buf: &mut [u8]) -> usize {
        Varuint(varint_to_varuint(self.0)).write_buf(buf)
    }

}

impl Varint {
    pub fn read(mut r: &mut Read) -> Result<Varint> {
        Ok(Varint(varuint_to_varint(*Varuint::read(r)?)))
    }
}

