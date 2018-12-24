/// pzip - predicted zip
///
/// # pzip
/// A compression library for floating point data

#[derive(Debug)]
pub struct Source {
    source: String,
}

#[derive(Debug)]
pub struct Sink {
    sink: String,
}

impl Source {

    pub fn new(source: String) -> Source {
        Source { source }
    }

    pub fn memory_read(&self) -> Result<(), String>{
        panic!("Reading into memory did not work.")
    }

    pub fn write(&self, sink: &Sink) -> Result<(), String>{
        panic!("Writing into sink did not work.")
    }
}

impl Sink {

    pub fn new(sink: String) -> Sink {
        Sink { sink }
    }
}

#[cfg(test)]
mod tests {
    use super::*;  // add lib into scope
    #[test]
    #[ignore]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
