/// pzip - predicted zip
///
/// # pzip
/// A compression library for floating point data

use std::marker::PhantomData;
use std::io::{self};
use std::fs;
use byteorder::{self, ReadBytesExt, LittleEndian};

pub struct Source<T> {
    file: fs::File,
    phantom: PhantomData<T>
}

impl<T> Source<T> {

    pub fn new(filename: String) -> Result<Source<T>, io::Error> {
        let file = fs::File::open(filename)?;
        Ok(Source{file, phantom: PhantomData})
    }
}

impl Source<u8> {
    pub fn get(&mut self) -> u8 {
        let val = self.file.read_u8().unwrap();
        val
    }
}

impl Source<f32> {
    pub fn get(&mut self) -> f32 {
        let val = self.file.read_f32::<LittleEndian>().unwrap();
        val
    }
}

impl Source<f64> {
    pub fn get(&mut self) -> f64 {
        let val = self.file.read_f64::<LittleEndian>().unwrap();
        val
    }
}
