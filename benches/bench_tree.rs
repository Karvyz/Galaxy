use crate::universe::tree::Tree;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    let tree = Tree::new(10);
    c.bench_function("tree 10", |b| b.iter(|| tree.count()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
