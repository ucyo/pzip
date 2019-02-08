use criterion::{criterion_group, criterion_main, Criterion, Fun};
use rand::distributions::{Standard};
use rand::{thread_rng, Rng};


use pzip::ptraversal::{single_neighbours_grouped_no_ring};
use pzip::position::Position as Coordinate;
use pzip::traversal::{neighbours as neighboursfn, GeneratorIteratorAdapter, Traversal};

fn random_number_generator_f32(min: f32, max: f32, size: usize) -> Vec<f32> {
    let v: Vec<f32> = thread_rng().sample_iter(&Standard).take(size).collect();
    let v = v.iter().map(|x| x * max + min).collect();
    v
}
fn random_number_generator_f64(min: f64, max: f64, size: usize) -> Vec<f64> {
    let v: Vec<f64> = thread_rng().sample_iter(&Standard).take(size).collect();
    let v = v.iter().map(|x| x * max + min).collect();
    v
}
fn prepare_data_f32(shape: &Coordinate) -> Vec<f32> {
    let size = shape.x * shape.y * shape.z;
    random_number_generator_f32(0f32, 1_000f32, size as usize)
}

fn prepare_functions() -> Vec<Fun<(pzip::position::Position, Vec<pzip::position::Position>)>> {
    let neighbours_grouped_no_ring = Fun::new("PA Grouped NO ring", |b, (shape, neighbours)| {
        let data = prepare_data_f32(&shape);
        b.iter(|| { let _: Vec<Vec<f32>> = GeneratorIteratorAdapter(single_neighbours_grouped_no_ring(&shape, &neighbours, &data)).collect(); ()});
    });
    let former_implementation = Fun::new("Normal NO ring", |b, (shape, neighbours)| {
        let data = prepare_data_f32(&shape);
        b.iter(|| {let tr = Traversal::new(shape.x as usize, shape.y as usize, shape.z as usize); let _: Vec<Vec<f32>> = GeneratorIteratorAdapter(neighboursfn(tr, &data, &neighbours)).collect(); ()});
    });

    vec![neighbours_grouped_no_ring, former_implementation]
}

//neighbourhood & size
fn small_small_comparison(c: &mut Criterion) {
    let funcs = prepare_functions();
    let shape = Coordinate{x:55, y:35, z:70};
    let mut neihgbours: Vec<Coordinate> = Vec::new();
    neihgbours.push(Coordinate { x: 1, y: 0, z: 0 });
    c.bench_functions("No Ring", funcs, (shape, neihgbours));
}

fn small_medium_comparison(c: &mut Criterion) {
    let funcs = prepare_functions();
    let shape = Coordinate{x:950, y:350, z:70};
    let mut neihgbours: Vec<Coordinate> = Vec::new();
    neihgbours.push(Coordinate { x: 1, y: 0, z: 0 });
    c.bench_functions("No Ring", funcs, (shape, neihgbours));
}

fn small_big_comparison(c: &mut Criterion) {
    let funcs = prepare_functions();
    let shape = Coordinate{x:1250, y:550, z:300};
    let mut neihgbours: Vec<Coordinate> = Vec::new();
    neihgbours.push(Coordinate { x: 1, y: 0, z: 0 });
    c.bench_functions("No Ring", funcs, (shape, neihgbours));
}
fn medium_small_comparison(c: &mut Criterion) {
    let funcs = prepare_functions();
    let shape = Coordinate{x:55, y:35, z:70};
    let mut neihgbours: Vec<Coordinate> = Vec::new();
    neihgbours.push(Coordinate { x: 1, y: 0, z: 0 });
    neihgbours.push(Coordinate { x: 1, y: 2, z: 0 });
    neihgbours.push(Coordinate { x: 2, y: 5, z: 0 });
    neihgbours.push(Coordinate { x: 4, y: 2, z: 1 });
    c.bench_functions("No Ring", funcs, (shape, neihgbours));
}


fn medium_medium_comparison(c: &mut Criterion) {
    let funcs = prepare_functions();
    let shape = Coordinate{x:950, y:350, z:70};
    let mut neihgbours: Vec<Coordinate> = Vec::new();
    neihgbours.push(Coordinate { x: 1, y: 0, z: 0 });
    neihgbours.push(Coordinate { x: 1, y: 2, z: 0 });
    neihgbours.push(Coordinate { x: 2, y: 5, z: 0 });
    neihgbours.push(Coordinate { x: 4, y: 2, z: 1 });
    c.bench_functions("No Ring", funcs, (shape, neihgbours));
}

fn medium_big_comparison(c: &mut Criterion) {
    let funcs = prepare_functions();
    let shape = Coordinate{x:1250, y:550, z:300};
    let mut neihgbours: Vec<Coordinate> = Vec::new();
    neihgbours.push(Coordinate { x: 1, y: 0, z: 0 });
    neihgbours.push(Coordinate { x: 1, y: 2, z: 0 });
    neihgbours.push(Coordinate { x: 2, y: 5, z: 0 });
    neihgbours.push(Coordinate { x: 4, y: 2, z: 1 });
    c.bench_functions("No Ring", funcs, (shape, neihgbours));
}

fn big_small_comparison(c: &mut Criterion) {
    let funcs = prepare_functions();
    let shape = Coordinate{x:55, y:35, z:70};
    let mut neihgbours: Vec<Coordinate> = Vec::new();
    neihgbours.push(Coordinate { x: 1, y: 0, z: 0 });
    neihgbours.push(Coordinate { x: 1, y: 2, z: 0 });
    neihgbours.push(Coordinate { x: 2, y: 5, z: 0 });
    neihgbours.push(Coordinate { x: 0, y: 1, z: 1 });
    neihgbours.push(Coordinate { x: 2, y: 2, z: 3 });
    neihgbours.push(Coordinate { x: 6, y: 1, z: 2 });
    neihgbours.push(Coordinate { x: 2, y: 3, z: 0 });
    neihgbours.push(Coordinate { x: 4, y: 2, z: 1 });
    c.bench_functions("No Ring", funcs, (shape, neihgbours));
}
fn big_medium_comparison(c: &mut Criterion) {
    let funcs = prepare_functions();
    let shape = Coordinate{x:950, y:350, z:70};
    let mut neihgbours: Vec<Coordinate> = Vec::new();
    neihgbours.push(Coordinate { x: 1, y: 0, z: 0 });
    neihgbours.push(Coordinate { x: 1, y: 2, z: 0 });
    neihgbours.push(Coordinate { x: 2, y: 5, z: 0 });
    neihgbours.push(Coordinate { x: 0, y: 1, z: 1 });
    neihgbours.push(Coordinate { x: 2, y: 2, z: 3 });
    neihgbours.push(Coordinate { x: 6, y: 1, z: 2 });
    neihgbours.push(Coordinate { x: 2, y: 3, z: 0 });
    neihgbours.push(Coordinate { x: 4, y: 2, z: 1 });
    c.bench_functions("No Ring", funcs, (shape, neihgbours));
}

fn big_big_comparison(c: &mut Criterion) {
    let funcs = prepare_functions();
    let shape = Coordinate{x:1250, y:550, z:300};
    let mut neihgbours: Vec<Coordinate> = Vec::new();
    neihgbours.push(Coordinate { x: 1, y: 0, z: 0 });
    neihgbours.push(Coordinate { x: 1, y: 2, z: 0 });
    neihgbours.push(Coordinate { x: 2, y: 5, z: 0 });
    neihgbours.push(Coordinate { x: 0, y: 1, z: 1 });
    neihgbours.push(Coordinate { x: 2, y: 2, z: 3 });
    neihgbours.push(Coordinate { x: 6, y: 1, z: 2 });
    neihgbours.push(Coordinate { x: 2, y: 3, z: 0 });
    neihgbours.push(Coordinate { x: 4, y: 2, z: 1 });
    c.bench_functions("No Ring", funcs, (shape, neihgbours));
}

criterion_group!(
    analysis,
    // big_big_comparison,
    // big_medium_comparison,
    // big_small_comparison,
    // medium_big_comparison,
    // medium_medium_comparison,
    // medium_small_comparison,
    // small_big_comparison,
    // small_medium_comparison,
    small_small_comparison,
);

criterion_main!(analysis);
