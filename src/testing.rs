use super::*;


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

pub struct So<T> {file: fs::File, data: Vec<T>}
pub struct Si<T> {file: fs::File, data: PhantomData<T>}


impl FileToBeCompressed<u8> for So<u8> {

    fn new(filename: &String) -> Self {
        let file = fs::File::open(filename).unwrap();
        let data : Vec<u8> = Vec::new();
        So{file, data}
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

impl CompressedFile<u8> for Si<u8> {

    // REFACTOR: Change filename to fs::path::Path type
    fn new(filename: &String) -> Self {
        let file = fs::File::create(&filename).unwrap();
        Si{file, data: PhantomData}
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
