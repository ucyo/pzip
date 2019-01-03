// /// pzip - predicted zip
// ///
// /// # pzip
// /// A compression library for floating point data
// ///
// /// # Source
// /// The source gives information/reads input file to be compressed.
// ///
// /// # Sink
// /// The sink struct gives information/writes to output file.

#![feature(generators, generator_trait)]

pub mod testing;
pub mod position;
pub mod traversal;

// #[derive(Debug)]
// pub struct Storage {
//     a: Vec<f64>,
//     ix:   usize,
//     and:  usize,
//     dims: Dimension,
//     default: f64,
// }

// type Dimension = position::Position;
// type Shape = position::Position;


// impl Storage {
//     pub fn new(shape: Shape, maxposition : &position::Position) -> Self {
//         let nx = 1;
//         let ny = nx * (shape.x + maxposition.x);
//         let nz = ny * (shape.y + maxposition.y);

//         let mut size = nx + ny + nz;
//         while size & (size + 1) != 0{
//             size = size | size + 1
//         };
//         let data = vec![0f64;size+1];
//         let ix   = 0_usize;

//         Storage{data, ix, and:size, dims:Dimension{x:nx,y:ny,z:nz}, default: 0f64}
//     }

//     fn push(&mut self, value: f64, mut n: usize) {
//         println!("Pushing {}x{}", n, value);
//         while n > 0 {
//             self.data[self.ix & self.and] = value;
//             self.ix += 1;
//             n -= 1;
//         }
//     }

//     fn adv(&mut self, z:usize, y:usize, x: usize){
//         let n = z * self.dims.z + y * self.dims.y + x*self.dims.x;
//         self.push(self.default, n);
//     }

//     fn pos(&self, z: usize, y:usize, x:usize) -> f64 {
//         let pos = self.ix - (x*self.dims.x + y*(self.dims.y+1) + z * (self.dims.z+self.dims.y));
//         self.data[pos & self.and]
//     }
// }


// #[allow(dead_code)]
// fn at3(store: &mut Storage, ix: usize, data: &Vec<f64>, pos: &position::Position) -> f64 {
//     let mut data_ix = 0usize;
//     store.adv(1, 0, 0);
//     println!("{:?}", store.data);
//     'outer: for _ in 0..store.dims.z {
//         store.adv(0, 1, 0);
//         println!("{:?}", store.data);
//         for _ in 0..store.dims.y {
//             store.adv(0, 0, 1);
//             println!("{:?}", store.data);
//             for _ in 0..store.dims.y {
//                 println!("{:?}", store);
//                 let result = data[data_ix];
//                 if data_ix == ix {
//                     break 'outer;
//                 }
//                 store.push(result, 1);
//                 data_ix += 1;
//             }
//         }
//     }
//     store.pos(pos.z,pos.y,pos.x)
// }

// #[allow(dead_code, unused_variables)]
// fn at_default2(shape: (usize, usize), ix: usize, position: (usize, usize), data: &Vec<f64>, default: f64) -> f64 {
//     assert!(ix < data.len(), "Index position {} > data size {}", ix, data.len());

//     let nx = 1;
//     let ny = &nx * shape.0;
//     let nz = &ny * shape.1;

//     // build temporary array to save the values needed for future predictions.
//     // the size is the furthest point used for prediction:
//     // - in this case given by position (being the single value to be used
//     //   in the prediction)
//     let size = position.0 * nx + position.1 * ny;
//     let mut temporary_array: Vec<f64> = vec![0f64; size];

//     // fill out all the values till the current position. Since this
//     // elements should have been seen by now if this was not a fake test case
//     for i in 0..ix {
//         temporary_array[i%size] = data[i]
//     }

//     // if ix position is smaller than the furthest point needed for prediction
//     // return the default value. Otherwise return the value in the temporary
//     // array
//     if ix < size {
//         return default
//     } else {
//         let pos = ix as i32 - size as i32;
//         let ixposition = pos as usize % size;
//         return temporary_array[ixposition]
//     }
//     // TODO: it might be more efficient to use XOR instead of modulo?
// }

// #[allow(unused_imports)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_two_dimensions() {
//         let default = 88f64;
//         let shape = (4,4);
//         let position = (0, 1);
//         let data = vec![ 0.0, 1.0, 2.0, 3.0,
//                          4.0, 5.0, 6.0, 7.0,
//                          8.0, 9.0,10.0,11.0,
//                         12.0,13.0,14.0,15.0];

//         assert_eq!( at_default2(shape,  5, position, &data, default) ,  1f64);
//         assert_eq!( at_default2(shape, 14, position, &data, default) , 10f64);
//         assert_eq!( at_default2(shape, 10, position, &data, default) ,  6f64);
//         assert_eq!( at_default2(shape, 11, position, &data, default) ,  7f64);
//         assert_eq!( at_default2(shape,  3, position, &data, default) ,  default);
//         assert_eq!( at_default2(shape,  0, position, &data, default) ,  default);
//     }

//     #[test]
//     #[should_panic(expected = "Index position")]
//     fn test_two_dimensions_panic() {
//         let default = 88f64;
//         let shape = (4,4);
//         let position = (0, 1);
//         let data = vec![ 0.0, 1.0, 2.0, 3.0,
//                          4.0, 5.0, 6.0, 7.0,
//                          8.0, 9.0,10.0,11.0,
//                         12.0,13.0,14.0,15.0];
//         at_default2(shape, 18, position, &data, default);
//     }

//     #[test]
//     fn test_three_dimensions_position_y() {
//         let default = 0f64;
//         let shape = Shape{x:3,y:3,z:3};
//         let maxposition = position::Position{x:2,y:1,z:0};
//         let mut store = Storage::new(shape, &maxposition);

//         let data = vec![  0.0,  1.0,  2.0,
//                           3.0,  4.0,  5.0,
//                           6.0,  7.0,  8.0,

//                           9.0, 10.0, 11.0,
//                          12.0, 13.0, 14.0,
//                          15.0, 16.0, 17.0,

//                          18.0, 19.0, 20.0,
//                          21.0, 22.0, 23.0,
//                          24.0, 25.0, 26.0,
//                          ];

//         assert_eq!( at3(&mut store,  9, &data, &maxposition), default);
//         // assert_eq!( at3(&mut store, 18, &data, &maxposition), default);
//         // assert_eq!( at3(&mut store, 20, &data, &maxposition), default);
//         // assert_eq!( at3(&mut store,  2, &data, &maxposition), default);
//         // assert_eq!( at3(&mut store, 10, &data, &maxposition), default);
//         // assert_eq!( at3(&mut store,  1, &data, &maxposition), default);
//         // assert_eq!( at3(&mut store, 10, &data, &maxposition), default);
//         // assert_eq!( at3(&mut store, 11, &data, &maxposition), default);
//         // assert_eq!( at3(&mut store,  4, &data,  &maxposition),  1f64);
//         // assert_eq!( at3(&mut store, 13, &data, &maxposition), 10f64);
//         // assert_eq!( at3(&mut store, 17, &data, &maxposition), 14f64);
//         // assert_eq!( at3(&mut store, 21, &data, &maxposition), 18f64);
//     }

//     //   #[test]
//     // fn test_three_dimensions_position_x() {
//     //     let default = 0f64;
//     //     let shape = Shape{x:3,y:3,z:3};
//     //     let maxposition = position::Position{x:1,y:1,z:0};
//     //     let mut store = Storage::new(shape, &maxposition);

//     //     let data = vec![  0.0,  1.0,  2.0,
//     //                       3.0,  4.0,  5.0,
//     //                       6.0,  7.0,  8.0,

//     //                       9.0, 10.0, 11.0,
//     //                      12.0, 13.0, 14.0,
//     //                      15.0, 16.0, 17.0,

//     //                      18.0, 19.0, 20.0,
//     //                      21.0, 22.0, 23.0,
//     //                      24.0, 25.0, 26.0,
//     //                      ];

//     //     // assert_eq!( at3(&mut store,  9, &data, &maxposition), default);
//     //     // assert_eq!( at3(&mut store, 18, &data, &maxposition), default);
//     //     // assert_eq!( at3(&mut store, 20, &data, &maxposition), default);
//     //     // assert_eq!( at3(&mut store,  2, &data, &maxposition), default);
//     //     // assert_eq!( at3(&mut store, 10, &data, &maxposition), default);
//     //     // assert_eq!( at3(&mut store,  1, &data, &maxposition), default);
//     //     // assert_eq!( at3(&mut store, 10, &data, &maxposition), default);
//     //     // assert_eq!( at3(&mut store, 11, &data, &maxposition), default);
//     //     // assert_eq!( at3(&mut store, 4, &data,  &maxposition),  3f64);
//     //     assert_eq!( at3(&mut store, 25, &data, &maxposition), 12f64);
//     //     // assert_eq!( at3(&mut store, 17, &data, &maxposition), 16f64);
//     //     // assert_eq!( at3(&mut store, 21, &data, &maxposition), default);
//     // }

//     // #[test]
//     // fn test_three_dimensions_default_position_z() {
//     //     let default = 88f64;
//     //     let shape = (3,3,3);
//     //     let position = (0,0,1);
//     //     let data = vec![  0.0,  1.0,  2.0,
//     //                       3.0,  4.0,  5.0,
//     //                       6.0,  7.0,  8.0,

//     //                       9.0, 10.0, 11.0,
//     //                      12.0, 13.0, 14.0,
//     //                      15.0, 16.0, 17.0,

//     //                      18.0, 19.0, 20.0,
//     //                      21.0, 22.0, 23.0,
//     //                      24.0, 25.0, 26.0,
//     //                      ];

//     //     assert_eq!( at3(shape,  1, position, &data, default), default);
//     //     assert_eq!( at3(shape,  0, position, &data, default), default);
//     //     assert_eq!( at3(shape,  2, position, &data, default), default);
//     //     assert_eq!( at3(shape,  6, position, &data, default), default);
//     //     assert_eq!( at3(shape,  7, position, &data, default), default);
//     //     assert_eq!( at3(shape,  8, position, &data, default), default);
//     //     assert_eq!( at3(shape,  9, position, &data, default), default);
//     //     assert_eq!( at3(shape,  5, position, &data, default), default);

//     //     assert_eq!( at3(shape, 22, position, &data, default), 13f64-1.);
//     //     assert_eq!( at3(shape, 17, position, &data, default), 8f64-1.);
//     //     assert_eq!( at3(shape, 25, position, &data, default), 16f64-1.);
//     //     assert_eq!( at3(shape, 16, position, &data, default), 7f64-1.);
//     //     assert_eq!( at3(shape, 13, position, &data, default), 4f64-1.);
//     // }
// }
