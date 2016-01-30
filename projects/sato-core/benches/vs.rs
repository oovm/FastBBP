use chudnovsky::RamanujanL1;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn power_1(c: &mut Criterion) {
    let mut g = c.benchmark_group("π 10¹");
    g.bench_function("L1-J7", |b| b.iter(|| RamanujanL1::J163.run(black_box(10))));
    g.bench_function("L1-J11", |b| b.iter(|| RamanujanL1::J163.run(black_box(10))));
    g.bench_function("L1-J19", |b| b.iter(|| RamanujanL1::J163.run(black_box(10))));
    g.bench_function("L1-J43", |b| b.iter(|| RamanujanL1::J163.run(black_box(100))));
    g.bench_function("L1-J67", |b| b.iter(|| RamanujanL1::J163.run(black_box(1000))));
    g.bench_function("L1-J163", |b| b.iter(|| RamanujanL1::J163.run(black_box(10000))));
    g.finish()
}

fn power_2(c: &mut Criterion) {
    let mut g = c.benchmark_group("π 10²");
    g.bench_function("L1-J163", |b| b.iter(|| RamanujanL1::J163.run(black_box(10))));
    // .bench_function("L1 J163", |b| b.iter(|| RamanujanL1::J163.run(black_box(100))))
    // .bench_function("L1 J163", |b| b.iter(|| RamanujanL1::J163.run(black_box(1000))))
    // .bench_function("L1 J163", |b| b.iter(|| RamanujanL1::J163.run(black_box(10000))))
    g.finish()
}

fn power_3(c: &mut Criterion) {
    let mut g = c.benchmark_group("π 10³");
    g.bench_function("L1-J163", |b| b.iter(|| RamanujanL1::J163.run(black_box(10))));
    // .bench_function("L1 J163", |b| b.iter(|| RamanujanL1::J163.run(black_box(100))))
    // .bench_function("L1 J163", |b| b.iter(|| RamanujanL1::J163.run(black_box(1000))))
    // .bench_function("L1 J163", |b| b.iter(|| RamanujanL1::J163.run(black_box(10000))))
    g.finish()
}

criterion_group!(benches, power_1, power_2, power_3);
criterion_main!(benches);
