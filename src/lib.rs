//! Variable length signed and unsigned integer types.
//! Types support up to 128-bit integers and encoded to 1-17 bytes.
//!
//! Encoding rules are based on [SQLite 4 Varuint type](https://!sqlite.org/src4/doc/trunk/www/varint.wiki)
//! with modifications for support of 128-bit long integers.
//! Varint is encoded using the [Protobuf ZigZag approach](https://!developers.google.com/protocol-buffers/docs/encoding#signed-integers)
//! and reuses `Varuint` as a storage.
//!
//! Unlike the Protobuf encoding rules `Varuint` needs the first byte only to find out the length of the
//! whole value. Microbenchmarks say that it is a lot faster.
//!
//! ## How to use
//!
//! Add dependency to your Cargo.toml:
//!
//! ```cargo
//! [dependencies]
//! varuint = "0.4"
//! ```
//!
//! Add imports to your code:
//!
//! ```rust,no_run
//!
//! use varuint::{Varint, Varuint, Serializable, Deserializable};
//! ```
//!
//! Use it:
//!
//! ```rust,no_run
//! use std::mem;
//! use std::io::Read;
//!
//! use varuint::*;
//!
//! fn test_varint(v: i128, size: usize) {
//!     let v = Varint(v);
//!     assert_eq!(size, v.size_hint());
//!     let mut arr: [u8; 17] = unsafe { mem::uninitialized() };
//!     {
//!         let mut buf = &mut arr as &mut [u8];
//!         assert_eq!(size, v.serialize(&mut buf).unwrap());
//!     }
//!     let mut buf: &[u8] = &arr;
//!     let mut read: &mut Read = &mut buf;
//!     assert_eq!(v, Varint::deserialize(read).unwrap());
//! }
//!
//! fn main() {
//!     test_varint(0, 1);
//!     test_varint(1, 1);
//!     test_varint(-1, 1);
//! }
//! ```
//!
//! ## Encoding rules
//!
//! Encoding rules for `Varuint` are (assuming value is `V`):
//!
//!   * If `V<=240` then output a single byte `A0` equal to `V`.
//!   * If `V<=2031` then output `A0` as `(V-240)/256 + 241` and `A1` as `(V-240)%256`.
//!   * If `V<=67567` then output `A0` as `248`, `A1` as `(V-2032)/256`, and `A2` as `(V-2032)%256`.
//!   * If `V<=16777215` then output `A0` as `249` and `A1` through `A3` as a little-endian 3-byte integer.
//!   * If `V<=4294967295` then output `A0` as `250` and `A1..A4` as a little-ending 4-byte integer.
//!   * If `V<=1099511627775` then output `A0` as `251` and `A1..A5` as a little-ending 5-byte integer.
//!   * If `V<=281474976710655` then output `A0` as `252` and `A1..A6` as a little-ending 6-byte integer.
//!   * If `V<=72057594037927935` then output `A0` as `253` and `A1..A7` as a little-ending 7-byte integer.
//!   * If `V<=9223372036854775807` then output `A0` as `254` and `A1..A8` as a little-ending 8-byte integer.
//!   * Otherwise output `A0` as `255` and `A1..A16` as a little-endian 16-byte integer.
//!
//! `Varint` converted to the `Varuint` in the first place and then encoded as an unsigned integer.
//! Conversion method makes values closer to 0 take less space.
//! See [Protobuf docs](https://!developers.google.com/protocol-buffers/docs/encoding#signed-integers)
//! for details.
mod varuint;

#[macro_use]
#[cfg(feature = "serde-support")]
extern crate serde_derive;
#[cfg(feature = "serde-support")]
extern crate serde;


pub use varuint::{Serializable, Deserializable, Varuint, Varint};