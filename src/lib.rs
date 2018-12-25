/// pzip - predicted zip
///
/// # pzip
/// A compression library for floating point data

use std::io::{self, Read};
use std::fs;
use byteorder::{self, ReadBytesExt, ByteOrder, LittleEndian};

pub struct Source<T> {
    file: fs::File,
    data: Vec<T>
}

impl<T> Source<T> {

    pub fn new(filename: String) -> Result<Source<T>, io::Error> {
        let file = fs::File::open(filename)?;
        let data : Vec<T> = Vec::new();
        Ok(Source{file, data})
    }

    pub fn ix(&self, position: usize) -> &T {
        &self.data[position]
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

    pub fn load(&mut self) -> Result<usize, io::Error>{
        let mut bytes : Vec<u8> = Vec::new();
        let size = self.file.read_to_end(&mut bytes)?;

        if size % 8 != 0 {
            panic!("Can not be read into f64");
        }
        let mut data = vec![0_f64; size/8];
        LittleEndian::read_f64_into_unchecked(&bytes, &mut data);
        self.data = data;

        Ok(self.data.len())
    }
}
