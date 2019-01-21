#![feature(generators, generator_trait)]
#![feature(uniform_paths)]

pub mod config;
/// pzip - predicted zip
///
/// # pzip
/// A compression library for floating point data
pub mod mapping;
pub mod position;
pub mod testing;
pub mod traversal;

use position::Position;
use testing::{FileToBeCompressed, Source};
use traversal::{predictions, GeneratorIteratorAdapter};
use traversal::{Predictor, Traversal};
use mapping::{Intramapping, Intermapping, ByteMapping};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Shape {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub struct Weight {
    pub coeff: i16,
    pub pos: Position,
}

#[allow(dead_code)]
pub struct Setup<T> {
    source: testing::Source<T>,
    shape: Shape,
    weights: Vec<Weight>,
}

impl Setup<f64> {
    pub fn new(input: &String, shape: Shape, weights: Vec<Weight>) -> Self {
        let source: Source<f64> = Source::new(input);
        Setup {
            source,
            shape,
            weights,
        }
    }

    fn to_predictor(mut self) -> Predictor<f64> {
        self.source.load().expect("Error while loading data");
        let traversal = Traversal::new(self.shape.z, self.shape.y, self.shape.x);
        Predictor {
            traversal,
            weights: self.weights,
            data: self.source.data,
        }
    }

    pub fn write<H: Intermapping, K: Intramapping, B: ByteMapping>(self, output: &String) -> () {
        let mut p = self.to_predictor();
        let generator_iterator = GeneratorIteratorAdapter(predictions(&mut p));
        let results: Vec<f64> = generator_iterator.collect();
        let diff: Vec<u64> = results.iter()
                                    .map(|a| H::to_u64(*a))
                                    .zip(p.data.iter().map(|a|H::to_u64(*a)))
                                    .map(|(a,b)| K::to_new_u64(a) ^ K::to_new_u64(b))  // TODO eliminate dereferencing
                                    .collect();
        let mut tmp: Vec<u8> = Vec::new();
        for n in diff {
            let _ = tmp.write_u64::<LittleEndian>(n);
        }
        use byteorder::{LittleEndian, WriteBytesExt};
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let tmp : Vec<u8> = tmp.iter().map(|a| B::to_u8(*a)).collect();
        let mut output = BufWriter::new(File::create(output).unwrap());
        output.write_all(tmp.as_slice()).unwrap();
    }
}

impl Setup<f32> {
    pub fn new(input: &String, shape: Shape, weights: Vec<Weight>) -> Self {
        let source: Source<f32> = Source::new(input);
        Setup {
            source,
            shape,
            weights,
        }
    }

    fn to_predictor(mut self) -> Predictor<f32> {
        self.source.load().expect("Error while loading data");
        let traversal = Traversal::new(self.shape.z, self.shape.y, self.shape.x);
        Predictor {
            traversal,
            weights: self.weights,
            data: self.source.data,
        } // fix for f32
    }

    pub fn write<H: Intermapping, K: Intramapping, B: ByteMapping>(self, output: &String) -> () {
        let mut p = self.to_predictor();
        let generator_iterator = GeneratorIteratorAdapter(predictions(&mut p));
        let results: Vec<f32> = generator_iterator.collect();
        let diff: Vec<u32> = results.iter().map(|a| H::to_u32(*a))
                                    .zip(p.data.iter().map(|a| H::to_u32(*a)))
                                    .map(|(a,b)|  K::to_new_u32(a)^K::to_new_u32(b)) // TODO eliminate dereferencing
                                    .collect();
        let mut tmp: Vec<u8> = Vec::new();
        for n in diff {
            let _ = tmp.write_u32::<LittleEndian>(n);
        }
        use byteorder::{LittleEndian, WriteBytesExt};
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let mut output = BufWriter::new(File::create(output).unwrap());

        let tmp : Vec<u8> = tmp.iter().map(|a| B::to_u8(*a)).collect();
        output.write_all(tmp.as_slice()).unwrap();
    }
}
