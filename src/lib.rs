//! Variable length signed and unsigned integer types.
//! Types support up to 128-bit integers, both are encoded to 1-17 bytes.
//!
//! Encoding rules are based on [SQLite 4 Varuint type](https://!sqlite.org/src4/doc/trunk/www/varint.wiki)
//! with modifications for support of 128-bit long integers.
//! Signed integers are encoded using the [Protobuf ZigZag approach](https://!developers.google.com/protocol-buffers/docs/encoding#signed-integers)
//! and reuses unsigned integer as a storage.
//!
//! Unlike the Protobuf encoding rules `Varint` needs the first byte only to find out the length of the
//! whole value. Microbenchmarks say that it is a lot faster.
//!
//! ## How to use
//!
//! Add dependency to your Cargo.toml:
//!
//! ```cargo
//! [dependencies]
//! varuint = "0.6"
//! ```
//!
//! Add imports to your code:
//!
//! ```rust,no_run
//!
//! use varuint::{Varint, Serializable, Deserializable};
//! ```
//!
//! Use it:
//!
//! ```rust,no_run
//! use std::mem;
//! use std::io::Cursor;
//!
//! use varuint::*;
//!
//! fn main() {
//!     let mut cursor = Cursor::new(vec![]);
//!     let _ = cursor.write_varint(1u8).unwrap();
//!     let _ = cursor.write_varint(-300i16).unwrap();
//!     let v = Varint(-56_782i128);
//!     let _ = v.serialize(&mut cursor).unwrap();
//!     cursor.set_position(0);
//!     assert_eq!(1u8, ReadVarint::<u8>::read_varint(&mut cursor).unwrap());
//!     assert_eq!(-300i16, ReadVarint::<i16>::read_varint(&mut cursor).unwrap());
//!     assert_eq!(v, Varint::<i128>::deserialize(&mut cursor).unwrap());
//! }
//! ```
//!
//! ## Encoding rules
//!
//! Encoding rules for unsinged integer `Varint` are (assuming value is `V`):
//!
//!   * If `V<=240` then output a single byte `A0` equal to `V`.
//!   * If `V<=2031` then output `A0` as `(V-240)/256 + 241` and `A1` as `(V-240)%256`.
//!   * If `V<=67567` then output `A0` as `248`, `A1` as `(V-2032)/256`, and `A2` as `(V-2032)%256`.
//!   * If `V<=16_777_215` then output `A0` as `249` and `A1` through `A3` as a little-endian 3-byte integer.
//!   * If `V<=4_294_967_295` then output `A0` as `250` and `A1..A4` as a little-endian 4-byte integer.
//!   * If `V<=1_099_511_627_775` then output `A0` as `251` and `A1..A5` as a little-endian 5-byte integer.
//!   * If `V<=281_474_976_710_655` then output `A0` as `252` and `A1..A6` as a little-endian 6-byte integer.
//!   * If `V<=72_057_594_037_927_935` then output `A0` as `253` and `A1..A7` as a little-endian 7-byte integer.
//!   * If `V<=9_223_372_036_854_775_807` then output `A0` as `254` and `A1..A8` as a little-endian 8-byte integer.
//!   * Otherwise output `A0` as `255` and `A1..A16` as a little-endian 16-byte integer.
//!
//! Signed integer `Varint` converted to the unsigned integer `Varint` in the first place and then encoded as an unsigned integer.
//! Conversion method makes values closer to 0 to take less space.
//! See [Protobuf docs](https://!developers.google.com/protocol-buffers/docs/encoding#signed-integers)
//! for details.
mod read_write;
mod ser_deser;
mod varint;

pub use crate::read_write::{ReadVarint, VarintSizeHint, WriteVarint};
pub use crate::ser_deser::{Deserializable, Serializable};
pub use crate::varint::{Varint, VarintBaseType};
