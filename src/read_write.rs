use std::io::Result;
use std::io::{Read, Write};
use std::{mem, ptr};

/// Varuint size hinting trait
pub trait VarintSizeHint {
    fn varint_size(self) -> usize;
}

impl VarintSizeHint for u8 {
    fn varint_size(self) -> usize {
        if self <= 240 {
            1
        } else {
            2
        }
    }
}

impl VarintSizeHint for u16 {
    fn varint_size(self) -> usize {
        if self <= 240 {
            1
        } else if self <= 2031 {
            2
        } else {
            3
        }
    }
}

impl VarintSizeHint for u32 {
    fn varint_size(self) -> usize {
        if self <= 240 {
            1
        } else if self <= 2031 {
            2
        } else if self <= 67567 {
            3
        } else if self <= 16_777_215 {
            4
        } else {
            5
        }
    }
}

impl VarintSizeHint for u64 {
    fn varint_size(self) -> usize {
        if self <= 240 {
            1
        } else if self <= 2031 {
            2
        } else if self <= 67567 {
            3
        } else if self <= 16_777_215 {
            4
        } else if self <= 4_294_967_295 {
            5
        } else if self <= 1_099_511_627_775 {
            6
        } else if self <= 281_474_976_710_655 {
            7
        } else if self <= 72_057_594_037_927_935 {
            8
        } else {
            9
        }
    }
}

impl VarintSizeHint for u128 {
    fn varint_size(self) -> usize {
        if self <= 240 {
            1
        } else if self <= 2031 {
            2
        } else if self <= 67567 {
            3
        } else if self <= 16_777_215 {
            4
        } else if self <= 4_294_967_295 {
            5
        } else if self <= 1_099_511_627_775 {
            6
        } else if self <= 281_474_976_710_655 {
            7
        } else if self <= 72_057_594_037_927_935 {
            8
        } else if self <= 18_446_744_073_709_551_615 {
            9
        } else {
            17
        }
    }
}

impl VarintSizeHint for i8 {
    fn varint_size(self) -> usize {
        varint_to_varuint_8(self).varint_size()
    }
}

impl VarintSizeHint for i16 {
    fn varint_size(self) -> usize {
        varint_to_varuint_16(self).varint_size()
    }
}

impl VarintSizeHint for i32 {
    fn varint_size(self) -> usize {
        varint_to_varuint_32(self).varint_size()
    }
}

impl VarintSizeHint for i64 {
    fn varint_size(self) -> usize {
        varint_to_varuint_64(self).varint_size()
    }
}

impl VarintSizeHint for i128 {
    fn varint_size(self) -> usize {
        varint_to_varuint_128(self).varint_size()
    }
}

pub trait WriteVarint<T> {
    fn write_varint(&mut self, v: T) -> Result<usize>;
}

impl<T: Write + ?Sized> WriteVarint<u8> for T {
    fn write_varint(&mut self, v: u8) -> Result<usize> {
        let mut buf: [u8; 2] = unsafe { mem::uninitialized() };
        let size = v.varint_size();
        match size {
            1 => buf[0] = v,
            2 => {
                buf[0] = 241;
                buf[1] = (v - 240) as u8;
            }
            _ => unreachable!(),
        }
        self.write(&buf[0..size])
    }
}

impl<T: Write + ?Sized> WriteVarint<u16> for T {
    fn write_varint(&mut self, v: u16) -> Result<usize> {
        let mut buf: [u8; 3] = unsafe { mem::uninitialized() };
        let size = v.varint_size();
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
            _ => unreachable!(),
        }
        self.write(&buf[0..size])
    }
}

impl<T: Write + ?Sized> WriteVarint<u32> for T {
    fn write_varint(&mut self, v: u32) -> Result<usize> {
        let mut buf: [u8; 5] = unsafe { mem::uninitialized() };
        let size = v.varint_size();
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
                write_value_32(&mut buf[1..=3], v);
            }
            5 => {
                buf[0] = 250;
                unsafe {
                    ptr::copy_nonoverlapping(&v as *const _ as *const u8, &mut buf[1], 4);
                }
            }
            _ => unreachable!(),
        }
        self.write(&buf[0..size])
    }
}

impl<T: Write + ?Sized> WriteVarint<u64> for T {
    fn write_varint(&mut self, v: u64) -> Result<usize> {
        let mut buf: [u8; 9] = unsafe { mem::uninitialized() };
        let size = v.varint_size();
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
                write_value_64(&mut buf[1..=3], v);
            }
            5 => {
                buf[0] = 250;
                unsafe {
                    ptr::copy_nonoverlapping(&v as *const _ as *const u8, &mut buf[1], 4);
                }
            }
            6 => {
                buf[0] = 251;
                write_value_64(&mut buf[1..=5], v);
            }
            7 => {
                buf[0] = 252;
                write_value_64(&mut buf[1..=6], v);
            }
            8 => {
                buf[0] = 253;
                write_value_64(&mut buf[1..=7], v);
            }
            9 => {
                buf[0] = 254;
                unsafe {
                    ptr::copy_nonoverlapping(&v as *const _ as *const u8, &mut buf[1], 8);
                }
            }
            _ => unreachable!(),
        }
        self.write(&buf[0..size])
    }
}

impl<T: Write + ?Sized> WriteVarint<u128> for T {
    fn write_varint(&mut self, v: u128) -> Result<usize> {
        let mut buf: [u8; 17] = unsafe { mem::uninitialized() };
        let size = v.varint_size();
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
                write_value_128(&mut buf[1..=3], v);
            }
            5 => {
                buf[0] = 250;
                unsafe {
                    ptr::copy_nonoverlapping(&v as *const _ as *const u8, &mut buf[1], 4);
                }
            }
            6 => {
                buf[0] = 251;
                write_value_128(&mut buf[1..=5], v);
            }
            7 => {
                buf[0] = 252;
                write_value_128(&mut buf[1..=6], v);
            }
            8 => {
                buf[0] = 253;
                write_value_128(&mut buf[1..=7], v);
            }
            9 => {
                buf[0] = 254;
                unsafe {
                    ptr::copy_nonoverlapping(&v as *const _ as *const u8, &mut buf[1], 8);
                }
            }
            17 => {
                buf[0] = 255;
                unsafe {
                    ptr::copy_nonoverlapping(&v as *const _ as *const u8, &mut buf[1], 16);
                }
            }
            _ => unreachable!(),
        }
        self.write(&buf[0..size])
    }
}

impl<T: Write + ?Sized> WriteVarint<i8> for T {
    fn write_varint(&mut self, v: i8) -> Result<usize> {
        self.write_varint(varint_to_varuint_8(v))
    }
}

impl<T: Write + ?Sized> WriteVarint<i16> for T {
    fn write_varint(&mut self, v: i16) -> Result<usize> {
        self.write_varint(varint_to_varuint_16(v))
    }
}

impl<T: Write + ?Sized> WriteVarint<i32> for T {
    fn write_varint(&mut self, v: i32) -> Result<usize> {
        self.write_varint(varint_to_varuint_32(v))
    }
}

impl<T: Write + ?Sized> WriteVarint<i64> for T {
    fn write_varint(&mut self, v: i64) -> Result<usize> {
        self.write_varint(varint_to_varuint_64(v))
    }
}

impl<T: Write + ?Sized> WriteVarint<i128> for T {
    fn write_varint(&mut self, v: i128) -> Result<usize> {
        self.write_varint(varint_to_varuint_128(v))
    }
}

pub trait ReadVarint<T> {
    fn read_varint(&mut self) -> Result<T>;
}

impl<T: Read + ?Sized> ReadVarint<u8> for T {
    fn read_varint(&mut self) -> Result<u8> {
        let mut buf: [u8; 2] = unsafe { mem::uninitialized() };
        self.read_exact(&mut buf[0..1])?;
        let length = match buf[0] {
            v @ 0...240 => return Ok(v),
            241...247 => 2,
            _ => unreachable!(),
        };
        self.read_exact(&mut buf[1..length])?;
        Ok(match length {
            2 => 240u8 + buf[1],
            _ => unreachable!(),
        })
    }
}

impl<T: Read + ?Sized> ReadVarint<u16> for T {
    fn read_varint(&mut self) -> Result<u16> {
        let mut buf: [u8; 3] = unsafe { mem::uninitialized() };
        self.read_exact(&mut buf[0..1])?;
        let length = match buf[0] {
            v @ 0...240 => return Ok(u16::from(v)),
            241...247 => 2,
            248 => 3,
            _ => unreachable!(),
        };
        self.read_exact(&mut buf[1..length])?;
        Ok(match length {
            2 => 240u16 + 256u16 * (u16::from(buf[0]) - 241u16) + u16::from(buf[1]),
            3 => 2032u16 + 256u16 * u16::from(buf[1]) + u16::from(buf[2]),
            _ => unreachable!(),
        })
    }
}

impl<T: Read + ?Sized> ReadVarint<u32> for T {
    fn read_varint(&mut self) -> Result<u32> {
        let mut buf: [u8; 5] = unsafe { mem::uninitialized() };
        self.read_exact(&mut buf[0..1])?;
        let length = match buf[0] {
            v @ 0...240 => return Ok(u32::from(v)),
            241...247 => 2,
            248 => 3,
            249 => 4,
            250 => 5,
            _ => unreachable!(),
        };
        self.read_exact(&mut buf[1..length])?;
        Ok(match length {
            2 => 240u32 + 256u32 * (u32::from(buf[0]) - 241u32) + u32::from(buf[1]),
            3 => 2032u32 + 256u32 * u32::from(buf[1]) + u32::from(buf[2]),
            4 => read_value_32(&buf[1..=3]),
            5 => {
                let mut v: u32 = unsafe { mem::uninitialized() };
                unsafe {
                    ptr::copy_nonoverlapping(&buf[1], &mut v as *mut _ as *mut u8, 4);
                }
                v
            }
            _ => unreachable!(),
        })
    }
}

impl<T: Read + ?Sized> ReadVarint<u64> for T {
    fn read_varint(&mut self) -> Result<u64> {
        let mut buf: [u8; 9] = unsafe { mem::uninitialized() };
        self.read_exact(&mut buf[0..1])?;
        let length = match buf[0] {
            v @ 0...240 => return Ok(u64::from(v)),
            241...247 => 2,
            248 => 3,
            249 => 4,
            250 => 5,
            251 => 6,
            252 => 7,
            253 => 8,
            254 => 9,
            _ => unreachable!(),
        };
        self.read_exact(&mut buf[1..length])?;
        Ok(match length {
            2 => 240u64 + 256u64 * (u64::from(buf[0]) - 241u64) + u64::from(buf[1]),
            3 => 2032u64 + 256u64 * u64::from(buf[1]) + u64::from(buf[2]),
            4 => read_value_64(&buf[1..=3]),
            5 => {
                let mut v: u32 = unsafe { mem::uninitialized() };
                unsafe {
                    ptr::copy_nonoverlapping(&buf[1], &mut v as *mut _ as *mut u8, 4);
                }
                u64::from(v)
            }
            6 => read_value_64(&buf[1..=5]),
            7 => read_value_64(&buf[1..=6]),
            8 => read_value_64(&buf[1..=7]),
            9 => {
                let mut v: u64 = unsafe { mem::uninitialized() };
                unsafe {
                    ptr::copy_nonoverlapping(&buf[1], &mut v as *mut _ as *mut u8, 8);
                }
                v
            }
            _ => unreachable!(),
        })
    }
}

impl<T: Read + ?Sized> ReadVarint<u128> for T {
    fn read_varint(&mut self) -> Result<u128> {
        let mut buf: [u8; 17] = unsafe { mem::uninitialized() };
        self.read_exact(&mut buf[0..1])?;
        let length = match buf[0] {
            v @ 0...240 => return Ok(u128::from(v)),
            241...247 => 2,
            248 => 3,
            249 => 4,
            250 => 5,
            251 => 6,
            252 => 7,
            253 => 8,
            254 => 9,
            255 => 17,
        };
        self.read_exact(&mut buf[1..length])?;
        Ok(match length {
            2 => 240u128 + 256u128 * (u128::from(buf[0]) - 241u128) + u128::from(buf[1]),
            3 => 2032u128 + 256u128 * u128::from(buf[1]) + u128::from(buf[2]),
            4 => read_value_128(&buf[1..=3]),
            5 => {
                let mut v: u32 = unsafe { mem::uninitialized() };
                unsafe {
                    ptr::copy_nonoverlapping(&buf[1], &mut v as *mut _ as *mut u8, 4);
                }
                u128::from(v)
            }
            6 => read_value_128(&buf[1..=5]),
            7 => read_value_128(&buf[1..=6]),
            8 => read_value_128(&buf[1..=7]),
            9 => {
                let mut v: u64 = unsafe { mem::uninitialized() };
                unsafe {
                    ptr::copy_nonoverlapping(&buf[1], &mut v as *mut _ as *mut u8, 8);
                }
                u128::from(v)
            }
            17 => {
                let mut v: u128 = unsafe { mem::uninitialized() };
                unsafe {
                    ptr::copy_nonoverlapping(&buf[1], &mut v as *mut _ as *mut u8, 16);
                }
                v
            }
            _ => unreachable!(),
        })
    }
}

impl<T: Read + ?Sized> ReadVarint<i8> for T {
    fn read_varint(&mut self) -> Result<i8> {
        Ok(varuint_to_varint_8(self.read_varint()?))
    }
}

impl<T: Read + ?Sized> ReadVarint<i16> for T {
    fn read_varint(&mut self) -> Result<i16> {
        Ok(varuint_to_varint_16(self.read_varint()?))
    }
}

impl<T: Read + ?Sized> ReadVarint<i32> for T {
    fn read_varint(&mut self) -> Result<i32> {
        Ok(varuint_to_varint_32(self.read_varint()?))
    }
}

impl<T: Read + ?Sized> ReadVarint<i64> for T {
    fn read_varint(&mut self) -> Result<i64> {
        Ok(varuint_to_varint_64(self.read_varint()?))
    }
}

impl<T: Read + ?Sized> ReadVarint<i128> for T {
    fn read_varint(&mut self) -> Result<i128> {
        Ok(varuint_to_varint_128(self.read_varint()?))
    }
}

#[inline]
fn varint_to_varuint_8(v: i8) -> u8 {
    ((v << 1) ^ (v >> 7)) as u8
}

#[inline]
fn varuint_to_varint_8(v: u8) -> i8 {
    ((v >> 1) as i8) ^ -((v & 1) as i8)
}

#[inline]
fn varint_to_varuint_16(v: i16) -> u16 {
    ((v << 1) ^ (v >> 15)) as u16
}

#[inline]
fn varuint_to_varint_16(v: u16) -> i16 {
    ((v >> 1) as i16) ^ -((v & 1) as i16)
}

#[inline]
fn varint_to_varuint_32(v: i32) -> u32 {
    ((v << 1) ^ (v >> 31)) as u32
}

#[inline]
fn varuint_to_varint_32(v: u32) -> i32 {
    ((v >> 1) as i32) ^ -((v & 1) as i32)
}

#[inline]
fn varint_to_varuint_64(v: i64) -> u64 {
    ((v << 1) ^ (v >> 63)) as u64
}

#[inline]
fn varuint_to_varint_64(v: u64) -> i64 {
    ((v >> 1) as i64) ^ -((v & 1) as i64)
}

#[inline]
fn varint_to_varuint_128(v: i128) -> u128 {
    ((v << 1) ^ (v >> 127)) as u128
}

#[inline]
fn varuint_to_varint_128(v: u128) -> i128 {
    ((v >> 1) as i128) ^ -((v & 1) as i128)
}

#[inline(always)]
fn write_value_32(buf: &mut [u8], v: u32) {
    for (i, val) in buf.iter_mut().enumerate() {
        *val = (v >> (8 * i)) as u8;
    }
}

#[inline(always)]
fn write_value_64(buf: &mut [u8], v: u64) {
    for (i, val) in buf.iter_mut().enumerate() {
        *val = (v >> (8 * i)) as u8;
    }
}

#[inline(always)]
fn write_value_128(buf: &mut [u8], v: u128) {
    for (i, val) in buf.iter_mut().enumerate() {
        *val = (v >> (8 * i)) as u8;
    }
}

#[inline(always)]
fn read_value_128(buf: &[u8]) -> u128 {
    let mut v = 0;
    for (i, val) in buf.iter().enumerate() {
        v |= u128::from(*val) << (8 * i);
    }
    v
}

#[inline(always)]
fn read_value_64(buf: &[u8]) -> u64 {
    let mut v = 0;
    for (i, val) in buf.iter().enumerate() {
        v |= u64::from(*val) << (8 * i);
    }
    v
}

#[inline(always)]
fn read_value_32(buf: &[u8]) -> u32 {
    let mut v = 0;
    for (i, val) in buf.iter().enumerate() {
        v |= u32::from(*val) << (8 * i);
    }
    v
}
