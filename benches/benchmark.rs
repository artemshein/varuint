#[macro_use]
extern crate criterion;
extern crate varuint;

use criterion::Criterion;

use varuint::*;

fn serialize_varint<T>(v: T) where for<'a> &'a mut [u8]: varuint::WriteVarint<T> {
    let mut arr: [u8; 17] = [0; 17];
    {
        let mut buf = &mut arr as &mut [u8];
        let _ = buf.write_varint(v).unwrap();
    }
}

fn serialize_varint_128(v: u128) {
    let v = Varint(v);
    let mut arr: [u8; 17] = [0; 17];
    let mut buf = &mut arr as &mut [u8];
    let _ = v.serialize(&mut buf).unwrap();
}

fn serialize_1_benchmark(c: &mut Criterion) {
    c.bench_function("ser 1", |b| b.iter(|| serialize_varint(25u8)));
    c.bench_function("ser 1 (128)", |b| b.iter(|| serialize_varint_128(25)));
}

fn serialize_5_benchmark(c: &mut Criterion) {
    c.bench_function("ser 5", |b| {
        b.iter(|| serialize_varint(u32::max_value()))
    });
    c.bench_function("ser 5 (128)", |b| {
        b.iter(|| serialize_varint_128(u32::max_value() as u128))
    });
}

fn serialize_9_benchmark(c: &mut Criterion) {
    c.bench_function("ser 9", |b| {
        b.iter(|| serialize_varint(u64::max_value()))
    });
    c.bench_function("ser 9 (128)", |b| {
        b.iter(|| serialize_varint_128(u64::max_value() as u128))
    });
}

fn serialize_17_benchmark(c: &mut Criterion) {
    c.bench_function("ser 17", |b| {
        b.iter(|| serialize_varint_128(u128::max_value()))
    });
}

criterion_group!(
    benches,
    serialize_1_benchmark,
    serialize_5_benchmark,
    serialize_9_benchmark,
    serialize_17_benchmark
);
criterion_main!(benches);
