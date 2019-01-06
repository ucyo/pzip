use byteorder::{self, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs;
use std::io::prelude::*;
use std::io::{self, Read};
use std::marker::PhantomData;

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
    fn put_all(&mut self, value: &[T]) -> Result<(), io::Error>;
}

pub struct Source<T> {
    file: fs::File,
    data: Vec<T>,
}
pub struct Sink<T> {
    file: fs::File,
    data: PhantomData<T>,
}

impl FileToBeCompressed<u8> for Source<u8> {
    fn new(filename: &String) -> Self {
        let file = fs::File::open(filename).unwrap();
        let data: Vec<u8> = Vec::new();
        Source { file, data }
    }

    fn ix(&self, position: usize) -> &u8 {
        &self.data[position]
    }

    fn get(&mut self) -> u8 {
        self.file.read_u8().unwrap()
    }

    fn load(&mut self) -> Result<usize, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let length = self.file.read_to_end(&mut bytes)?;
        self.data = bytes;
        Ok(length)
    }
}

impl FileToBeCompressed<f32> for Source<f32> {
    fn new(filename: &String) -> Self {
        let file = fs::File::open(filename).unwrap();
        let data: Vec<f32> = Vec::new();
        Source { file, data }
    }

    fn ix(&self, position: usize) -> &f32 {
        &self.data[position]
    }

    fn get(&mut self) -> f32 {
        self.file.read_f32::<LittleEndian>().unwrap()
    }

    fn load(&mut self) -> Result<usize, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let size = self.file.read_to_end(&mut bytes)?;

        if size % 4 != 0 {
            panic!("Can not be read into f64");
        }
        let mut data = vec![0_f32; size / 4];
        LittleEndian::read_f32_into_unchecked(&bytes, &mut data);
        self.data = data;
        Ok(self.data.len())
    }
}

impl FileToBeCompressed<f64> for Source<f64> {
    fn new(filename: &String) -> Self {
        let file = fs::File::open(filename).unwrap();
        let data: Vec<f64> = Vec::new();
        Source { file, data }
    }

    fn ix(&self, position: usize) -> &f64 {
        &self.data[position]
    }

    fn get(&mut self) -> f64 {
        self.file.read_f64::<LittleEndian>().unwrap()
    }

    fn load(&mut self) -> Result<usize, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let size = self.file.read_to_end(&mut bytes)?;

        if size % 4 != 0 {
            panic!("Can not be read into f64");
        }
        let mut data = vec![0_f64; size / 8];
        LittleEndian::read_f64_into_unchecked(&bytes, &mut data);
        self.data = data;
        Ok(self.data.len())
    }
}

impl CompressedFile<u8> for Sink<u8> {
    // REFACTOR: Change filename to fs::path::Path type
    fn new(filename: &String) -> Self {
        let file = fs::File::create(&filename).unwrap();
        Sink {
            file,
            data: PhantomData,
        }
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.file.flush()?;
        Ok(())
    }

    fn put(&mut self, value: u8) -> Result<(), io::Error> {
        self.file.write_u8(value)?;
        Ok(())
    }

    fn put_all(&mut self, values: &[u8]) -> Result<(), io::Error> {
        self.file.write(values)?;
        Ok(())
    }
}

impl CompressedFile<f32> for Sink<f32> {
    // REFACTOR: Change filename to fs::path::Path type
    fn new(filename: &String) -> Self {
        let file = fs::File::create(&filename).unwrap();
        Sink {
            file,
            data: PhantomData,
        }
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.file.flush()?;
        Ok(())
    }

    fn put(&mut self, value: f32) -> Result<(), io::Error> {
        let mut buf = [0_u8; 4];
        LittleEndian::write_f32(&mut buf, value);
        self.file.write(&buf)?;
        Ok(())
    }

    fn put_all(&mut self, values: &[f32]) -> Result<(), io::Error> {
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
        Sink {
            file,
            data: PhantomData,
        }
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.file.flush()?;
        Ok(())
    }

    fn put(&mut self, value: f64) -> Result<(), io::Error> {
        let mut buf = [0_u8; 8];
        LittleEndian::write_f64(&mut buf, value);
        self.file.write(&buf)?;
        Ok(())
    }

    fn put_all(&mut self, values: &[f64]) -> Result<(), io::Error> {
        for &n in values {
            self.put(n)?;
        }
        Ok(())
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn trait_write_f32_to_file() {
        let filename = String::from("/tmp/output.raw");
        let v = 213.232_f32;

        let mut sink: Sink<f32> = Sink::new(&filename);
        sink.put(v).expect("Writing unsuccessfull");
        sink.flush().expect("Writing unsuccessfull");

        let mut source: Source<f32> = Source::new(&filename);
        let value = source.get();

        std::fs::remove_file(&filename).expect("Error");
        assert_eq!(v, value)
    }

    #[test]
    fn trait_write_f32s_to_file() {
        let filename = String::from("/tmp/output.raw");
        let values = [213.236_f32, 839.9482_f32, 94.32_f32];

        let mut sink: Sink<f32> = Sink::new(&filename);
        sink.put_all(&values).expect("Writing unsuccessfull");
        sink.flush().expect("Writing unsuccessfull");

        let mut source: Source<f32> = Source::new(&filename);
        source.load().expect("Load unsuccessfull");

        for i in 0..3 {
            assert_eq!(source.ix(i), &values[i])
        }
        std::fs::remove_file(&filename).expect("Error");
    }

    #[test]
    fn trait_read_first_f32_from_file() {
        let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

        let mut source: Source<f32> = Source::new(&filename);
        let first = source.get();
        assert_eq!(first, 160.57284545898_f32)
    }

    #[test]
    fn trait_read_f32_from_file() {
        let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

        let mut source: Source<f32> = Source::new(&filename);
        source.load().expect("Error loading the data");

        let expected = [
            160.57284545898_f32,
            160.47055053711_f32,
            160.36930847168_f32,
        ];
        for i in 0..3 {
            assert_eq!(source.ix(i), &expected[i]);
        }
    }

    #[test]
    fn trait_write_f64_to_file() {
        let filename = String::from("/tmp/output.raw");
        let v = 213.232_f64;

        let mut sink: Sink<f64> = Sink::new(&filename);
        sink.put(v).expect("Writing unsuccessfull");
        sink.flush().expect("Writing unsuccessfull");

        let mut source: Source<f64> = Source::new(&filename);
        let value = source.get();

        std::fs::remove_file(&filename).expect("Error");
        assert_eq!(v, value)
    }

    #[test]
    fn trait_write_f64s_to_file() {
        let filename = String::from("/tmp/output.raw");
        let values = [324234.423234_f64, 9291.822_f64, 1.23131_f64];

        let mut sink: Sink<f64> = Sink::new(&filename);
        sink.put_all(&values).expect("Writing unsuccessfull");
        sink.flush().expect("Writing unsuccessfull");

        let mut source: Source<f64> = Source::new(&filename);
        source.load().expect("Load unsuccessfull");

        for i in 0..3 {
            assert_eq!(source.ix(i), &values[i])
        }
        std::fs::remove_file(&filename).expect("Error");
    }

    #[test]
    fn trait_read_first_f64_from_file() {
        let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

        let mut source: Source<f64> = Source::new(&filename);
        let first = source.get();
        assert_eq!(first, 2.318024477526355e+15_f64)
    }

    #[test]
    fn trait_read_f64_from_file() {
        let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

        let mut source: Source<f64> = Source::new(&filename);
        source.load().expect("Error loading the data");

        let expected = [
            2.318024477526355e+15_f64,
            2.2897421178755255e+15_f64,
            2.262535647532541e+15_f64,
        ];
        for i in 0..3 {
            assert_eq!(source.ix(i), &expected[i]);
        }
    }

    #[test]
    fn trait_write_byte_to_file() {
        let filename = String::from("/tmp/output.raw");
        let v = 213_u8;

        let mut sink: Sink<u8> = Sink::new(&filename);
        sink.put(v).expect("Writing unsuccessfull");
        sink.flush().expect("Writing unsuccessfull");

        let mut source: Source<u8> = Source::new(&filename);
        let value = source.get();

        std::fs::remove_file(&filename).expect("Error");
        assert_eq!(v, value)
    }

    #[test]
    fn trait_write_bytes_to_file() {
        let filename = String::from("/tmp/output.raw");
        let values = [123_u8, 193_u8, 201_u8];

        let mut sink: Sink<u8> = Sink::new(&filename);
        sink.put_all(&values).expect("Writing unsuccessfull");
        sink.flush().expect("Writing unsuccessfull");

        let mut source: Source<u8> = Source::new(&filename);
        source.load().expect("Load unsuccessfull");

        for i in 0..3 {
            assert_eq!(source.ix(i), &values[i])
        }
        std::fs::remove_file(&filename).expect("Error");
    }

    #[test]
    fn trait_read_first_byte_from_file() {
        let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

        let mut source: Source<u8> = Source::new(&filename);
        let first = source.get();
        assert_eq!(first, 166u8)
    }

    #[test]
    fn trait_read_bytes_from_file() {
        let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

        let mut source: Source<u8> = Source::new(&filename);
        source.load().expect("Error loading the data");

        let expected = [166_u8, 146_u8, 32_u8];
        for i in 0..3 {
            assert_eq!(source.ix(i), &expected[i]);
        }
    }

}
