use criterion::{Criterion, criterion_group, criterion_main};

criterion_main!(benches);
criterion_group!(benches, test_bench);

fn test_bench(c: &mut Criterion) {

}

