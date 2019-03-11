#![feature(generators, generator_trait)]
#![feature(uniform_paths)]

pub mod config;
pub mod gen;
/// pzip - predicted zip
///
/// # pzip
/// A compression library for floating point data
// pub mod mapping;
pub mod position;
pub mod predictors;
pub mod ptraversal;
pub mod testing;
pub mod transform;
pub mod traversal;
pub mod residual;

use position::Position;
use testing::{FileToBeCompressed, Source};
use transform::{Byte, Compact, Inter, Intra};
use transform::{ByteMapping, CompactMapping, InterMapping, IntraMapping};

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

use predictors::{Ignorant, PredictorTrait};
pub struct Setup<T> {
    source: testing::Source<T>,
    shape: Position,
    predictor: Ignorant<T>,
}

impl Setup<f64> {
    pub fn new(input: &String, shape: Position, predictor: Ignorant<f64>) -> Self {
        let source: Source<f64> = Source::new(input);
        Setup {
            source,
            shape,
            predictor,
        }
    }

    pub fn write(&mut self, h: Inter, k: Intra, b: Byte, output: &String) {
        self.source.load().expect("Wrong loading");
        let results = self
            .predictor
            .consume(&self.source.data, &self.shape, false);
        let diff: Vec<u64> = results
            .iter()
            .map(|a| h.to_u64(*a))
            .zip(self.source.data.iter().map(|a| h.to_u64(*a)))
            .map(|(a, b)| k.to_new_u64(a) ^ k.to_new_u64(b))
            .collect();
        let mut tmp: Vec<u8> = Vec::new();
        dbg!(&diff);
        for n in diff {
            let _ = tmp.write_u64::<LittleEndian>(n);
        }
        use byteorder::{LittleEndian, WriteBytesExt};
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let tmp: Vec<u8> = tmp.iter().map(|a| b.to_u8(*a)).collect();
        let mut output = BufWriter::new(File::create(output).unwrap());
        output.write_all(tmp.as_slice()).unwrap();
    }
}

impl Setup<f32> {
    pub fn new(input: &String, shape: Position, predictor: Ignorant<f32>) -> Self {
        let source: Source<f32> = Source::new(input);
        Setup {
            source,
            shape,
            predictor,
        }
    }

    pub fn write(&mut self, h: Inter, k: Intra, b: Byte, c: Compact, ring: bool, output: &String) {
        self.source.load().expect("Wrong loading");
        let results = self.predictor.consume(&self.source.data, &self.shape, ring);
        let diff: Vec<u32> = results
            .iter()
            .map(|a| h.to_u32(*a))
            .zip(self.source.data.iter().map(|a| h.to_u32(*a)))
            .map(|(a, b)| k.to_new_u32(a) ^ k.to_new_u32(b))
            .collect();
        let diff = c.compact_u32(diff);
        let mut tmp: Vec<u8> = Vec::new();
        for n in diff {
            let _ = tmp.write_u32::<LittleEndian>(n);
        }
        use byteorder::{LittleEndian, WriteBytesExt};
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let mut output = BufWriter::new(File::create(output).unwrap());

        let tmp: Vec<u8> = tmp.iter().map(|a| b.to_u8(*a)).collect();
        output.write_all(tmp.as_slice()).unwrap();
    }
}
