use chudnovsky::RamanujanL1;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn level1_j163(c: &mut Criterion) {
    let mut g = c.benchmark_group("L1-J163");
    g.bench_with_input(BenchmarkId::new("π", "10¹"), &2, |b, i| b.iter(|| RamanujanL1::J163.run(*i)));
    g.bench_with_input(BenchmarkId::new("π", "10²"), &8, |b, i| b.iter(|| RamanujanL1::J163.run(*i)));
    g.bench_with_input(BenchmarkId::new("π", "10³"), &72, |b, i| b.iter(|| RamanujanL1::J163.run(*i)));
    g.bench_with_input(BenchmarkId::new("π", "10⁴"), &715, |b, i| b.iter(|| RamanujanL1::J163.run(*i)));
    g.finish()
}

fn level1_j67(c: &mut Criterion) {
    let mut g = c.benchmark_group("L1-J67");
    g.bench_with_input(BenchmarkId::new("π", "10¹"), &2, |b, i| b.iter(|| RamanujanL1::J67.run(*i)));
    g.bench_with_input(BenchmarkId::new("π", "10²"), &14, |b, i| b.iter(|| RamanujanL1::J67.run(*i)));
    g.bench_with_input(BenchmarkId::new("π", "10³"), &134, |b, i| b.iter(|| RamanujanL1::J67.run(*i)));
    g.bench_with_input(BenchmarkId::new("π", "10⁴"), &1334, |b, i| b.iter(|| RamanujanL1::J67.run(*i)));
    g.finish()
}

fn level1_j19(c: &mut Criterion) {
    let mut g = c.benchmark_group("L1-J19");
    g.bench_with_input(BenchmarkId::new("π", "10¹"), &4, |b, i| b.iter(|| RamanujanL1::J19.run(black_box(*i))));
    g.bench_with_input(BenchmarkId::new("π", "10²"), &37, |b, i| b.iter(|| RamanujanL1::J19.run(black_box(*i))));
    g.bench_with_input(BenchmarkId::new("π", "10³"), &368, |b, i| b.iter(|| RamanujanL1::J19.run(black_box(*i))));
    g.bench_with_input(BenchmarkId::new("π", "10⁴"), &3677, |b, i| b.iter(|| RamanujanL1::J19.run(black_box(*i))));
    g.finish()
}

criterion_group!(benches, level1_j163, level1_j67, level1_j19);
criterion_main!(benches);
