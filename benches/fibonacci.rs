use std::time::Duration;

use bench_vtables::{
    fibonacci::{
        fibonacci, make_fibonacci_inline_sieve, make_fibonacci_multiptr,
        make_fibonacci_packed_sieve, make_fibonacci_vtable,
    },
    inline_sieve_table::InlineSievePtr,
    multi_ptrs::MultiVPtr,
    packed_sieve_table::PackedSievePtr,
    v_table::VPtr,
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in [10u64].iter() {
        group.bench_with_input(BenchmarkId::new("PackedSievePtr_2trait", i), i, |b, i| {
            let table = black_box(make_fibonacci_packed_sieve::<_, 2, 5>());
            let ptr = PackedSievePtr::new(&table);
            b.iter(|| fibonacci(black_box(ptr), black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("PackedSievePtr_3trait", i), i, |b, i| {
            let table = black_box(make_fibonacci_packed_sieve::<_, 3, 4>());
            let ptr = PackedSievePtr::new(&table);
            b.iter(|| fibonacci(black_box(ptr), black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("PackedSievePtr_4trait", i), i, |b, i| {
            let table = black_box(make_fibonacci_packed_sieve::<_, 4, 3>());
            let ptr = PackedSievePtr::new(&table);
            b.iter(|| fibonacci(black_box(ptr), black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("PackedSievePtr_5trait", i), i, |b, i| {
            let table = black_box(make_fibonacci_packed_sieve::<_, 5, 2>());
            let ptr = PackedSievePtr::new(&table);
            b.iter(|| fibonacci(black_box(ptr), black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("InlineSievePtr", i), i, |b, i| {
            let table = black_box(make_fibonacci_inline_sieve());
            let ptr = InlineSievePtr::new(&table);
            b.iter(|| fibonacci(black_box(ptr), black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("VPtr", i), i, |b, i| {
            let table = black_box(make_fibonacci_vtable());
            let ptr = VPtr::new(&table);
            b.iter(|| fibonacci(black_box(ptr), black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("MultiVPtr_2trait", i), i, |b, i| {
            let table = black_box(make_fibonacci_multiptr::<_, 2, 5>());
            let ptr = MultiVPtr::new(&table);
            b.iter(|| fibonacci(black_box(ptr), black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("MultiVPtr_3trait", i), i, |b, i| {
            let table = black_box(make_fibonacci_multiptr::<_, 3, 4>());
            let ptr = MultiVPtr::new(&table);
            b.iter(|| fibonacci(black_box(ptr), black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("MultiVPtr_4trait", i), i, |b, i| {
            let table = black_box(make_fibonacci_multiptr::<_, 4, 3>());
            let ptr = MultiVPtr::new(&table);
            b.iter(|| fibonacci(black_box(ptr), black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("MultiVPtr_5trait", i), i, |b, i| {
            let table = black_box(make_fibonacci_multiptr::<_, 5, 2>());
            let ptr = MultiVPtr::new(&table);
            b.iter(|| fibonacci(black_box(ptr), black_box(*i)))
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(60));
    targets = bench_fibs
);
criterion_main!(benches);
