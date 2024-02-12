use crate::{ReadVarint, Varint, VarintSizeHint, WriteVarint};
use std::io::{Read, Result, Write};

/// Trait for serializable types
pub trait Serializable {
    /// Get a hint of encoded value byte-length
    fn size_hint(&self) -> usize;
    /// Serialize a value, returns bytes written
    fn serialize(&self, w: &mut dyn Write) -> Result<usize>;
}

/// Trait for deserializable types
pub trait Deserializable: Sized {
    /// Deserialize value from a `Read`
    fn deserialize(r: &mut dyn Read) -> Result<Self>;
}

impl Deserializable for Varint<u8> {
    fn deserialize(r: &mut dyn Read) -> Result<Self> {
        Ok(Self(r.read_varint()?))
    }
}

impl Deserializable for Varint<u16> {
    fn deserialize(r: &mut dyn Read) -> Result<Self> {
        Ok(Self(r.read_varint()?))
    }
}

impl Deserializable for Varint<u32> {
    fn deserialize(r: &mut dyn Read) -> Result<Self> {
        Ok(Self(r.read_varint()?))
    }
}

impl Deserializable for Varint<u64> {
    fn deserialize(r: &mut dyn Read) -> Result<Self> {
        Ok(Self(r.read_varint()?))
    }
}

impl Deserializable for Varint<u128> {
    fn deserialize(r: &mut dyn Read) -> Result<Self> {
        Ok(Self(r.read_varint()?))
    }
}

impl Serializable for Varint<u8> {
    fn size_hint(&self) -> usize {
        self.0.varint_size()
    }

    fn serialize(&self, w: &mut dyn Write) -> Result<usize> {
        w.write_varint(self.0)
    }
}

impl Serializable for Varint<u16> {
    fn size_hint(&self) -> usize {
        self.0.varint_size()
    }

    fn serialize(&self, w: &mut dyn Write) -> Result<usize> {
        w.write_varint(self.0)
    }
}

impl Serializable for Varint<u32> {
    fn size_hint(&self) -> usize {
        self.0.varint_size()
    }

    fn serialize(&self, w: &mut dyn Write) -> Result<usize> {
        w.write_varint(self.0)
    }
}

impl Serializable for Varint<u64> {
    fn size_hint(&self) -> usize {
        self.0.varint_size()
    }

    fn serialize(&self, w: &mut dyn Write) -> Result<usize> {
        w.write_varint(self.0)
    }
}

impl Serializable for Varint<u128> {
    fn size_hint(&self) -> usize {
        self.0.varint_size()
    }

    fn serialize(&self, w: &mut dyn Write) -> Result<usize> {
        w.write_varint(self.0)
    }
}

impl Serializable for Varint<i8> {
    #[inline]
    fn size_hint(&self) -> usize {
        self.0.varint_size()
    }

    #[inline]
    fn serialize(&self, w: &mut dyn Write) -> Result<usize> {
        w.write_varint(self.0)
    }
}

impl Deserializable for Varint<i8> {
    fn deserialize(r: &mut dyn Read) -> Result<Self> {
        Ok(Self(r.read_varint()?))
    }
}

impl Serializable for Varint<i16> {
    #[inline]
    fn size_hint(&self) -> usize {
        self.0.varint_size()
    }

    #[inline]
    fn serialize(&self, w: &mut dyn Write) -> Result<usize> {
        w.write_varint(self.0)
    }
}

impl Deserializable for Varint<i16> {
    fn deserialize(r: &mut dyn Read) -> Result<Self> {
        Ok(Self(r.read_varint()?))
    }
}

impl Serializable for Varint<i32> {
    #[inline]
    fn size_hint(&self) -> usize {
        self.0.varint_size()
    }

    #[inline]
    fn serialize(&self, w: &mut dyn Write) -> Result<usize> {
        w.write_varint(self.0)
    }
}

impl Deserializable for Varint<i32> {
    fn deserialize(r: &mut dyn Read) -> Result<Self> {
        Ok(Self(r.read_varint()?))
    }
}

impl Serializable for Varint<i64> {
    #[inline]
    fn size_hint(&self) -> usize {
        self.0.varint_size()
    }

    #[inline]
    fn serialize(&self, w: &mut dyn Write) -> Result<usize> {
        w.write_varint(self.0)
    }
}

impl Deserializable for Varint<i64> {
    fn deserialize(r: &mut dyn Read) -> Result<Self> {
        Ok(Self(r.read_varint()?))
    }
}

impl Serializable for Varint<i128> {
    #[inline]
    fn size_hint(&self) -> usize {
        self.0.varint_size()
    }

    #[inline]
    fn serialize(&self, w: &mut dyn Write) -> Result<usize> {
        w.write_varint(self.0)
    }
}

impl Deserializable for Varint<i128> {
    fn deserialize(r: &mut dyn Read) -> Result<Self> {
        Ok(Self(r.read_varint()?))
    }
}
