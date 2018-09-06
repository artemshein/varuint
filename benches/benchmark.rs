#[macro_use]
extern crate criterion;
extern crate varuint;

use std::{mem, io::Read};

use criterion::Criterion;

use varuint::*;

fn serialize_varuint(v: u128) {
    let v = Varuint(v);
    let mut arr: [u8; 17] = unsafe { mem::uninitialized() };
    let mut buf = &mut arr as &mut [u8];
    let _ = v.serialize(&mut buf).unwrap();
}

fn serialize_varint(v: i128) {
    let v = Varint(v);
    let mut arr: [u8; 17] = unsafe { mem::uninitialized() };
    let mut buf = &mut arr as &mut [u8];
    let _ = v.serialize(&mut buf).unwrap();
}

fn serialize_1_benchmark(c: &mut Criterion) {
    c.bench_function("ser 1", |b| b.iter(|| serialize_varuint(25)));
}

fn serialize_17_benchmark(c: &mut Criterion) {
    c.bench_function("ser 17", |b| b.iter(|| serialize_varuint(u128::max_value())));
}

criterion_group!(benches, serialize_1_benchmark, serialize_17_benchmark);
criterion_main!(benches);
