use criterion::{criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};

use coco_span::sourcemap::SourceMap;

fn alloc_speed(bench: &mut Criterion) {
    static KB: usize = 1024;

    let mut group = bench.benchmark_group("alloc_speed");
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| SourceMap::with_length(size));
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(500);
    targets = alloc_speed
}

criterion_main!(benches);