/// Benchmarking methods using tips
/// https://bheisler.github.io/criterion.rs/book/getting_started.html
/// and
/// http://seenaburns.com/benchmarking-rust-with-cargo-bench/

#[macro_use]
extern crate criterion;

use criterion::Criterion;
use pzip::position::Position;
use pzip::Weight;
use pzip::traversal::{predictions, GeneratorIteratorAdapter, Predictor, Traversal};

fn get_all_predictions() {
    let data = vec![
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0,
    ];
    let tr = Traversal::new(3, 3, 3);

    let mut weights: Vec<Weight> = Vec::new();
    weights.push(Weight{coeff: 1, pos: Position { x: 1, y: 0, z: 0 }});
    weights.push(Weight{coeff: 1, pos: Position { x: 1, y: 1, z: 0 }});
    weights.push(Weight{coeff: 1, pos: Position { x: 1, y: 0, z: 1 }});
    weights.push(Weight{coeff: 1, pos: Position { x: 0, y: 1, z: 1 }});
    weights.push(Weight{coeff: 1, pos: Position { x: 1, y: 0, z: 1 }});
    weights.push(Weight{coeff: 1, pos: Position { x: 0, y: 0, z: 1 }});
    weights.push(Weight{coeff: 1, pos: Position { x: 0, y: 1, z: 0 }});

    let mut p = Predictor {
        traversal: tr,
        weights: weights,
        data: data,
    };

    let generator_iterator = GeneratorIteratorAdapter(predictions(&mut p));
    let _: Vec<f64> = generator_iterator.collect();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("all_pred", |b| b.iter(|| get_all_predictions()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
