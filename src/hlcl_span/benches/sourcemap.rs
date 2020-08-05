use criterion::{black_box, criterion_group, criterion_main, Criterion};

use hlcl_span::sourcemap::SourceMap;

fn alloc_speed(bench: &mut Criterion) {
    bench.bench_function("alloc_speed", |bench| bench.iter(|| {
        SourceMap::with_capacity(10000)
    }));
}

criterion_group!(benches, alloc_speed);
criterion_main!(benches);