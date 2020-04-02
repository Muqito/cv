use criterion::{criterion_group, criterion_main, Criterion};
use space::{Bits512, Hamming};
use std::path::Path;

type Descriptor = Hamming<Bits512>;

fn image_to_kps(path: impl AsRef<Path>) -> (Vec<akaze::Keypoint>, Vec<Descriptor>) {
    akaze::extract_path(path, akaze::Config::new(0.01)).unwrap()
}

fn extract(c: &mut Criterion) {
    c.bench_function("extract", |b| b.iter(|| image_to_kps("res/0000000000.png")));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = extract
);
criterion_main!(benches);
