/// pzip - predicted zip
///
/// # pzip
/// A compression library for floating point data
///
/// # Source
/// The source gives information/reads input file to be compressed.
///
/// # Sink
/// The sink struct gives information/writes to output file.

use std::io::{self, Read};
use std::fs;
use std::marker::PhantomData;
use std::io::prelude::*;
use byteorder::{self, WriteBytesExt, ReadBytesExt, ByteOrder, LittleEndian};

pub mod testing;

pub struct Source<T> {
    file: fs::File,
    data: Vec<T>
}

pub struct Sink<T> {
    file : fs::File,
    data : PhantomData<T>
}

impl<T> Source<T> {

    // REFACTOR: Change filename to fs::path::Path type
    pub fn new(filename: &String) -> Result<Source<T>, io::Error> {
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

    pub fn load(&mut self) -> Result<usize, io::Error>{
        let mut bytes : Vec<u8> = Vec::new();
        let size = self.file.read_to_end(&mut bytes)?;

        self.data = bytes;
        Ok(size)
    }
}

impl Source<f32> {
    pub fn get(&mut self) -> f32 {
        let val = self.file.read_f32::<LittleEndian>().unwrap();
        val
    }

    pub fn load(&mut self) -> Result<usize, io::Error>{
        let mut bytes : Vec<u8> = Vec::new();
        let size = self.file.read_to_end(&mut bytes)?;

        if size % 4 != 0 {
            panic!("Can not be read into f64");
        }
        let mut data = vec![0_f32; size/4];
        LittleEndian::read_f32_into_unchecked(&bytes, &mut data);
        self.data = data;
        Ok(self.data.len())
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

impl<T> Sink<T> {

    // REFACTOR: Change filename to fs::path::Path type
    pub fn new(filename: &String) -> Result<Sink<T>, io::Error> {
        let file = fs::File::create(&filename)?;
        Ok(Sink{file, data: PhantomData})
    }

    pub fn flush(&mut self) -> Result<(), io::Error>{
        self.file.flush()?;
        Ok(())
    }
}


impl Sink<u8> {

    pub fn put(&mut self, value: u8) -> Result<(), io::Error> {
        self.file.write_u8(value).expect("Wrong writing value");
        Ok(())
    }

    pub fn put_all(&mut self, values: &[u8]) -> Result<(), io::Error>{
        self.file.write(values).expect("");
        Ok(())
    }
}


impl Sink<f32> {
    pub fn put(&mut self, value: f32) -> Result<(), io::Error> {
        let mut buf = [0_u8; 4];
        LittleEndian::write_f32(&mut buf, value);
        self.file.write(&buf).expect("Wrong writing value");
        Ok(())
    }

    pub fn put_all(&mut self, values: &[f32]) -> Result<(), io::Error> {
        for &n in values {
            // self.file.write_f32::<LittleEndian>(n)?;
            self.put(n)?;
        }
        Ok(())
    }
}


impl Sink<f64> {
    pub fn put(&mut self, value: f64) -> Result<(), io::Error> {
        let mut buf = [0_u8; 8];
        LittleEndian::write_f64(&mut buf, value);
        self.file.write(&buf).expect("Wrong writing value");
        Ok(())
    }

    pub fn put_all(&mut self, values: &[f64]) -> Result<(), io::Error> {
        for &n in values {
            self.put(n)?;
        }
        Ok(())
    }
}
