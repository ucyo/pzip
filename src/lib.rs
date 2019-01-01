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


pub mod testing;

#[allow(dead_code, unused_variables)]
fn at(shape: (usize, usize), ix: usize, position: (usize, usize), data: &Vec<f64>) -> f64 {
    0f64
}

#[allow(unused_imports)]
mod tests {
    use super::at;

    #[test]
    fn test_two_dimensions() {
        let shape = (4,4);
        let position = (1,1);
        let data = vec![ 0.0, 1.0, 2.0, 3.0,
                        4.0, 5.0, 6.0, 7.0,
                        8.0, 9.0,10.0,11.0,
                        12.0,13.0,14.0,15.0];

        assert_eq!( at(shape,  5, position, &data) ,  1f64);
        assert_eq!( at(shape, 14, position, &data) , 10f64);
        assert_eq!( at(shape, 10, position, &data) ,  6f64);
        assert_eq!( at(shape, 11, position, &data) ,  7f64);
    }
}
