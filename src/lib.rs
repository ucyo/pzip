#![feature(generators, generator_trait)]

/// pzip - predicted zip
///
/// # pzip
/// A compression library for floating point data
pub mod mapping;
pub mod position;
pub mod testing;
pub mod traversal;

#[allow(dead_code)]
pub struct Shape {
    x: usize,
    y: usize,
    z: usize
}

pub type Position = position::Position;

#[allow(dead_code)]
pub struct Setup<T> {
    source: testing::Source<T>,
    shape: Shape,
    weights: Vec<(i32, position::Position)>
}

impl<T> Setup<T> {
    pub fn new(input: &String, shape: Shape, weights: Vec<(i32, position::Position)>) {
        unimplemented!();
    }
    pub fn write(output: &String) {
        unimplemented!();
    }
    pub fn write_bytes(output: &String) {
        unimplemented!();
    }
}
