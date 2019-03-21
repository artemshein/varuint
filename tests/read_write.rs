use std::io::Cursor;
use varuint::{ReadVarint, VarintBaseType, WriteVarint};

fn test_varuint<T: VarintBaseType>(v: T, size: usize)
where
    Cursor<Vec<u8>>: WriteVarint<T> + ReadVarint<T>,
{
    assert_eq!(size, v.varint_size());
    let mut cur = Cursor::new(Vec::new());
    assert_eq!(size, cur.write_varint(v).unwrap());
    assert_eq!(size, cur.position() as usize);
    cur.set_position(0);
    assert_eq!(v, cur.read_varint().unwrap());
}

#[test]
fn test_all() {
    test_varuint(0u8, 1);
    test_varuint(240u8, 1);
    test_varuint(0u16, 1);
    test_varuint(240u16, 1);
    test_varuint(0u32, 1);
    test_varuint(240u32, 1);
    test_varuint(0u64, 1);
    test_varuint(240u64, 1);
    test_varuint(0u128, 1);
    test_varuint(240u128, 1);

    test_varuint(241u8, 2);
    test_varuint(2031u16, 2);
    test_varuint(241u16, 2);
    test_varuint(2031u32, 2);
    test_varuint(241u32, 2);
    test_varuint(2031u64, 2);
    test_varuint(241u64, 2);
    test_varuint(2031u128, 2);
    test_varuint(241u128, 2);

    test_varuint(2032u16, 3);
    test_varuint(67567u32, 3);
    test_varuint(2032u32, 3);
    test_varuint(67567u64, 3);
    test_varuint(2032u64, 3);
    test_varuint(67567u128, 3);
    test_varuint(2032u128, 3);

    test_varuint(67568u32, 4);
    test_varuint(16777215u32, 4);
    test_varuint(67568u64, 4);
    test_varuint(16777215u64, 4);
    test_varuint(67568u128, 4);
    test_varuint(16777215u128, 4);

    test_varuint(16777216u64, 5);
    test_varuint(4294967295u64, 5);
    test_varuint(16777216u128, 5);
    test_varuint(4294967295u128, 5);

    test_varuint(4294967296u64, 6);
    test_varuint(1099511627775u64, 6);
    test_varuint(4294967296u128, 6);
    test_varuint(1099511627775u128, 6);

    test_varuint(1099511627776u64, 7);
    test_varuint(281474976710655u64, 7);
    test_varuint(1099511627776u128, 7);
    test_varuint(281474976710655u128, 7);

    test_varuint(281474976710656u64, 8);
    test_varuint(72057594037927935u64, 8);
    test_varuint(281474976710656u128, 8);
    test_varuint(72057594037927935u128, 8);

    test_varuint(72057594037927936u128, 9);
    test_varuint(u128::from(u64::max_value()), 9);

    test_varuint(u128::from(u64::max_value()) + 1, 17);
    test_varuint(u128::max_value(), 17);
}
