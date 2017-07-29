/*!

 Variable length signed and unsigned integer types.
 Types support up to 64-bit integers (128-bit integers support will be added when Rust has the
 i128 and u128 types).

 Encoding rules are based on [SQLite 4 Varuint type](https://sqlite.org/src4/doc/trunk/www/varint.wiki)
 with modifications for future support of 128-bit long integers.
 Varint is encoded using the [Protobuf ZigZag approach](https://developers.google.com/protocol-buffers/docs/encoding#signed-integers)
 and reuses `Varuint` as a storage.

 Unlike the Protobuf encoding rules `Varuint` needs the first byte only to find out the length of the
 whole value. Microbenchmarks say that it is a lot faster.

 See `Varuint` and `Varint` for more details.

*/
mod varuint;

pub use varuint::{Serializable, Deserializable, Varuint, Varint};