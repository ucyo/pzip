#![feature(generators, generator_trait)]
#![feature(uniform_paths)]

/// pzip - predicted zip
///
/// # pzip
/// A compression library for floating point data
pub mod mapping;
pub mod position;
pub mod testing;
pub mod traversal;

use std::io;
use testing::{FileToBeCompressed, Source};
use testing::{CompressedFile, Sink};
use traversal::{Traversal, Predictor};
use position::Position;
use traversal::{GeneratorIteratorAdapter, predictions};

#[allow(dead_code)]
pub struct Shape {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

#[allow(dead_code)]
pub struct Setup<T> {
    source: testing::Source<T>,
    shape: Shape,
    weights: Vec<(i32, Position)>
}

impl Setup<f64> {

    pub fn new(input: &String,
               shape: Shape,
               weights: Vec<(i32, Position)>
    ) -> Self
    {
        let source: Source<f64> = Source::new(input);
        Setup {source, shape, weights}
    }

    fn to_predictor(mut self) -> Predictor {
        self.source.load().expect("Error while loading data");
        let traversal = Traversal::new(self.shape.z, self.shape.y, self.shape.x);
        Predictor {traversal, weights:self.weights, data:self.source.data} // fix for f32
    }

    pub fn write(self, output: &String) -> () {
        let p = self.to_predictor();
        let generator_iterator = GeneratorIteratorAdapter(predictions(p));
        let results: Vec<f64> = generator_iterator.collect();
        let mut tmp: Vec<u8> = Vec::new();
        for n in results {
                let _ = tmp.write_f64::<LittleEndian>(n);
        }
        use byteorder::{LittleEndian, WriteBytesExt};
        use std::io::{Write, BufWriter};
        use std::fs::File;

        let mut output = BufWriter::new(File::create(output).unwrap());
        output.write_all(tmp.as_slice()).unwrap();
    }
}
