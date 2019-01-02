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

#[derive(Debug)]
pub struct Storage {
    data: Vec<f64>,
    ix:   usize,
    size: usize,
    dims: (usize,usize,usize)
}

impl Storage {
    pub fn new(size: usize, shape: (usize, usize, usize), default: f64) -> Self {
        let data = vec![default;size];
        let ix   = 0_usize;

        let nx = 1;
        let ny = &nx * shape.0;
        let nz = &ny * shape.1;
        // let nt = &nz * shape.2;

        Storage{data, ix, size, dims:(nx,ny,nz)}
    }
    fn push(&mut self, value: f64, mut n: usize) {
        while n > 0 {
            self.data[self.ix%self.size] = value;
            self.ix += 1;
            n -= 1;
        }
    }
    fn pos(&self, x: usize, y:usize, z:usize) -> f64 {
        let pos = x*self.dims.0 + y*self.dims.1 + z * self.dims.2 + 1;
        println!("{:?}", pos);
        self.data[((self.ix - pos) %self.size)]
    }
}


#[allow(dead_code, unused_variables)]
fn at3(shape: (usize, usize, usize), ix: usize, position: (usize, usize, usize), data: &Vec<f64>, default: f64) -> f64 {
    assert!(ix < data.len(), "Index position {} > data size {}", ix, data.len());

    let nx = 1;
    let ny = &nx * shape.0;
    let nz = &ny * shape.1;
    let nt = &nz * shape.2;

    let temp_array_size = position.0 * nx + position.1 * ny + position.2 * nz;
    let mut temp_array = Storage::new(nz, shape, default);
    let mut pos = 0_usize;
    'outer: for i in 0..nz {
        temp_array.push(default, ny);
        for j in 0..ny {
            temp_array.push(default, nx);
            for k in 0..ny {
                if pos == ix {
                    break 'outer
                }
                temp_array.push(data[pos], 1);
                pos += 1;
            }
        }
    };
    temp_array.pos(position.0,position.1,position.2)
}

#[allow(dead_code, unused_variables)]
fn at_default2(shape: (usize, usize), ix: usize, position: (usize, usize), data: &Vec<f64>, default: f64) -> f64 {
    assert!(ix < data.len(), "Index position {} > data size {}", ix, data.len());

    let nx = 1;
    let ny = &nx * shape.0;
    let nz = &ny * shape.1;

    // build temporary array to save the values needed for future predictions.
    // the size is the furthest point used for prediction:
    // - in this case given by position (being the single value to be used
    //   in the prediction)
    let size = position.0 * nx + position.1 * ny;
    let mut temporary_array: Vec<f64> = vec![0f64; size];

    // fill out all the values till the current position. Since this
    // elements should have been seen by now if this was not a fake test case
    for i in 0..ix {
        temporary_array[i%size] = data[i]
    }

    // if ix position is smaller than the furthest point needed for prediction
    // return the default value. Otherwise return the value in the temporary
    // array
    if ix < size {
        return default
    } else {
        let pos = ix as i32 - size as i32;
        let ixposition = pos as usize % size;
        return temporary_array[ixposition]
    }
    // TODO: it might be more efficient to use XOR instead of modulo?
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_two_dimensions() {
        let default = 88f64;
        let shape = (4,4);
        let position = (0, 1);
        let data = vec![ 0.0, 1.0, 2.0, 3.0,
                         4.0, 5.0, 6.0, 7.0,
                         8.0, 9.0,10.0,11.0,
                        12.0,13.0,14.0,15.0];

        assert_eq!( at_default2(shape,  5, position, &data, default) ,  1f64);
        assert_eq!( at_default2(shape, 14, position, &data, default) , 10f64);
        assert_eq!( at_default2(shape, 10, position, &data, default) ,  6f64);
        assert_eq!( at_default2(shape, 11, position, &data, default) ,  7f64);
        assert_eq!( at_default2(shape,  3, position, &data, default) ,  default);
        assert_eq!( at_default2(shape,  0, position, &data, default) ,  default);
    }

    #[test]
    #[should_panic]
    fn test_two_dimensions_panic() {
        let default = 88f64;
        let shape = (4,4);
        let position = (0, 1);
        let data = vec![ 0.0, 1.0, 2.0, 3.0,
                         4.0, 5.0, 6.0, 7.0,
                         8.0, 9.0,10.0,11.0,
                        12.0,13.0,14.0,15.0];
        at_default2(shape, 18, position, &data, default);
    }

    #[test]
    fn test_three_dimensions() {
        let default = 88f64;
        let shape = (3,3,3);
        let position = (0,1,0);
        let data = vec![  0.0,  1.0,  2.0,
                          3.0,  4.0,  5.0,
                          6.0,  7.0,  8.0,

                          9.0, 10.0, 11.0,
                         12.0, 13.0, 14.0,
                         15.0, 16.0, 17.0,

                         18.0, 19.0, 20.0,
                         21.0, 22.0, 23.0,
                         24.0, 25.0, 26.0,
                         ];

        assert_eq!( at3(shape,  22, position, &data, default), 19f64);
        assert_eq!( at3(shape,  13, position, &data, default), 10f64);
        assert_eq!( at3(shape,  17, position, &data, default), 14f64);
        assert_eq!( at3(shape, 21, position, &data, default), 18f64);
    }

    #[test]
    fn test_three_dimensions_default() {
        let default = 88f64;
        let shape = (3,3,3);
        let position = (0,1,0);
        let data = vec![  0.0,  1.0,  2.0,
                          3.0,  4.0,  5.0,
                          6.0,  7.0,  8.0,

                          9.0, 10.0, 11.0,
                         12.0, 13.0, 14.0,
                         15.0, 16.0, 17.0,

                         18.0, 19.0, 20.0,
                         21.0, 22.0, 23.0,
                         24.0, 25.0, 26.0,
                         ];

        assert_eq!( at3(shape,  9, position, &data, default), default);
        assert_eq!( at3(shape, 18, position, &data, default), default);
        assert_eq!( at3(shape, 20, position, &data, default), default);
        assert_eq!( at3(shape,  2, position, &data, default), default);
        assert_eq!( at3(shape, 10, position, &data, default), default);
        assert_eq!( at3(shape,  1, position, &data, default), default);
        assert_eq!( at3(shape, 10, position, &data, default), default);
        assert_eq!( at3(shape, 11, position, &data, default), default);
    }
}
