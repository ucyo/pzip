
use std::io::{self, Read};
use std::fs;
use std::marker::PhantomData;
use std::io::prelude::*;
use byteorder::{self, WriteBytesExt, ReadBytesExt, ByteOrder, LittleEndian};


pub trait FileToBeCompressed<T> {
    fn new(filename: &String) -> Self;
    fn ix(&self, position: usize) -> &T;
    fn get(&mut self) -> T;
    fn load(&mut self) -> Result<usize, io::Error>;
}

pub trait CompressedFile<T> {
    fn new(filename: &String) -> Self;
    fn flush(&mut self) -> Result<(), io::Error>;
    fn put(&mut self, value: T) -> Result<(), io::Error>;
    fn put_all(&mut self, value: &[T])  -> Result<(), io::Error>;
}

pub struct Source<T> {file: fs::File, data: Vec<T>}
pub struct Sink<T> {file: fs::File, data: PhantomData<T>}


impl FileToBeCompressed<u8> for Source<u8> {

    fn new(filename: &String) -> Self {
        let file = fs::File::open(filename).unwrap();
        let data : Vec<u8> = Vec::new();
        Source{file, data}
    }

    fn ix(&self, position: usize) -> &u8 {
        &self.data[position]
    }

    fn get(&mut self) -> u8 {
        self.file.read_u8().unwrap()
    }

    fn load(&mut self) -> Result<usize, io::Error>{
        let mut bytes : Vec<u8> = Vec::new();
        let length = self.file.read_to_end(&mut bytes)?;
        self.data = bytes;
        Ok(length)
    }

}

impl FileToBeCompressed<f32> for Source<f32> {

    fn new(filename: &String) -> Self {
        let file = fs::File::open(filename).unwrap();
        let data : Vec<f32> = Vec::new();
        Source{file, data}
    }

    fn ix(&self, position: usize) -> &f32 {
        &self.data[position]
    }

    fn get(&mut self) -> f32 {
        self.file.read_f32::<LittleEndian>().unwrap()
    }

    fn load(&mut self) -> Result<usize, io::Error>{
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

impl FileToBeCompressed<f64> for Source<f64> {

    fn new(filename: &String) -> Self {
        let file = fs::File::open(filename).unwrap();
        let data : Vec<f64> = Vec::new();
        Source{file, data}
    }

    fn ix(&self, position: usize) -> &f64 {
        &self.data[position]
    }

    fn get(&mut self) -> f64 {
        self.file.read_f64::<LittleEndian>().unwrap()
    }

    fn load(&mut self) -> Result<usize, io::Error>{
        let mut bytes : Vec<u8> = Vec::new();
        let size = self.file.read_to_end(&mut bytes)?;

        if size % 4 != 0 {
            panic!("Can not be read into f64");
        }
        let mut data = vec![0_f64; size/8];
        LittleEndian::read_f64_into_unchecked(&bytes, &mut data);
        self.data = data;
        Ok(self.data.len())
    }

}


impl CompressedFile<u8> for Sink<u8> {

    // REFACTOR: Change filename to fs::path::Path type
    fn new(filename: &String) -> Self {
        let file = fs::File::create(&filename).unwrap();
        Sink{file, data: PhantomData}
    }

    fn flush(&mut self)  -> Result<(), io::Error> {
       self.file.flush()?;
       Ok(())
    }

    fn put(&mut self, value: u8) -> Result<(), io::Error> {
        self.file.write_u8(value)?;
        Ok(())
    }

    fn put_all(&mut self, values: &[u8])  -> Result<(), io::Error> {
        self.file.write(values)?;
        Ok(())
    }
}


impl CompressedFile<f32> for Sink<f32> {

    // REFACTOR: Change filename to fs::path::Path type
    fn new(filename: &String) -> Self {
        let file = fs::File::create(&filename).unwrap();
        Sink{file, data: PhantomData}
    }

    fn flush(&mut self)  -> Result<(), io::Error> {
       self.file.flush()?;
       Ok(())
    }

    fn put(&mut self, value: f32) -> Result<(), io::Error> {
        let mut buf = [0_u8; 4];
        LittleEndian::write_f32(&mut buf, value);
        self.file.write(&buf)?;
        Ok(())
    }

    fn put_all(&mut self, values: &[f32])  -> Result<(), io::Error> {
       for &n in values {
            self.put(n)?;
        }
        Ok(())
    }
}


impl CompressedFile<f64> for Sink<f64> {

    // REFACTOR: Change filename to fs::path::Path type
    fn new(filename: &String) -> Self {
        let file = fs::File::create(&filename).unwrap();
        Sink{file, data: PhantomData}
    }

    fn flush(&mut self)  -> Result<(), io::Error> {
       self.file.flush()?;
       Ok(())
    }

    fn put(&mut self, value: f64) -> Result<(), io::Error> {
        let mut buf = [0_u8; 8];
        LittleEndian::write_f64(&mut buf, value);
        self.file.write(&buf)?;
        Ok(())
    }

    fn put_all(&mut self, values: &[f64])  -> Result<(), io::Error> {
       for &n in values {
            self.put(n)?;
        }
        Ok(())
    }
}
