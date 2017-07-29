Variable length signed and unsigned integer types. Types support up to 64-bit integers (128-bit integers support will be added when Rust has the i128 and u128 types).

Encoding rules are based on [SQLite 4 Varuint type](https://sqlite.org/src4/doc/trunk/www/varint.wiki) with modifications for future support of 128-bit long integers in the future.
Varint is encoded using the [Protobuf ZigZag approach](https://developers.google.com/protocol-buffers/docs/encoding#signed-integers) and reusing Varuint as a storage.

Unlike the Protobuf encoding rules Varuint needs the first byte only to find out the length of the whole value. Microbenchmarks say that it is a lot faster.

See Varuint and Varint for more details.

# How to use

Add dependency to `Cargo.toml`:
```cargo
[dependencies]
varuint = "0.1"
```

Update you project:
```rust
extern crate varuint;

use std::mem;

use varuint::{Varuint, VarType};
use std::io::Read;


fn test_varuint(v: u64, size: usize) {
  let v = Varuint(v);
  assert_eq!(size, v.size_hint());
  let mut arr: [u8; 9] = unsafe { mem::uninitialized() };
  {
    let mut buf = &mut arr as &mut [u8];
    assert_eq!(size, v.write(&mut buf).unwrap());
  }
  let mut buf: &[u8] = &arr;
  let mut read: &mut Read = &mut buf;
  assert_eq!(v, Varuint::read(read).unwrap());
}

fn main() {
  test_varuint(0, 1);
  test_varuint(240, 1);

  test_varuint(241, 2);
  test_varuint(2031, 2);

  test_varuint(2032, 3);
  test_varuint(67567, 3);

  test_varuint(67568, 4);
  test_varuint(16777215, 4);

  test_varuint(16777216, 5);
  test_varuint(4294967295, 5);

  test_varuint(4294967296, 6);
  test_varuint(1099511627775, 6);

  test_varuint(1099511627776, 7);
  test_varuint(281474976710655, 7);

  test_varuint(281474976710656, 8);
  test_varuint(72057594037927935, 8);

  test_varuint(72057594037927936, 9);
  test_varuint(u64::max_value(), 9);
}
```
