use std::convert::From;
use std::ops::{AddAssign, Mul};
use std::ops::{Generator, GeneratorState};

use super::position::Position;
use super::Weight;

#[derive(Debug)]
pub struct Traversal<T> {
    nz: usize,
    ny: usize,
    nx: usize,
    dz: usize,
    dy: usize,
    dx: usize,
    m: usize,
    a: Vec<T>,
    ix: usize,
    zero: T,
}

/// Traversal of the data.
///
/// # Note
/// The maximum traversal is limited by 6 steps (in either dimension).
impl<T: Default + Copy> Traversal<T> {
    pub fn new(nz: usize, ny: usize, nx: usize) -> Self {
        let dx = 1;
        let dy = nx + 1;
        let dz = dy * (ny + 1);

        let sum = nx * ny * 6 * 6; // limit for furthest possible neighbour is set to 6
        let m = sum.next_power_of_two() - 1;
        let a = vec![Default::default(); m + 1];
        let ix = 0;
        let zero = Default::default();

        Traversal {
            nz,
            ny,
            nx,
            dz,
            dy,
            dx,
            m,
            a,
            ix,
            zero,
        }
    }
    pub fn advance(&mut self, z: i32, y: i32, x: i32) {
        let n = self.dz as i32 * z + self.dy as i32 * y + self.dx as i32 * x;
        self.push(&Default::default(), n);
    }
    pub fn push(&mut self, val: &T, mut n: i32) {
        while n > 0 {
            self.a[self.ix & self.m] = *val;
            self.ix += 1;
            n -= 1;
        }
    }
    pub fn fetch(&self, z: i32, y: i32, x: i32) -> &T {
        let pos = self.ix as i32 - (self.dz as i32 * z + self.dy as i32 * y + self.dx as i32 * x);
        &self.a[(pos & self.m as i32) as usize]
    }
}

#[deprecated(since="0.1.0", note="Use 'neighbours' instead")]
#[allow(deprecated)]
pub fn predict<T: Copy + AddAssign<<T as Mul>::Output> + Default + From<i32> + Mul>(
    data: &Vec<T>,
    at: usize,
    traversal: &mut Traversal<T>,
    weights: &Vec<(i32, Position)>,
) -> T {
    let mut data_ix = 0usize;
    let maximas = predict_maximas(&weights);
    traversal.advance(maximas.z, 0, 0);
    'outer: for _ in 0..traversal.nz {
        traversal.advance(0, maximas.y, 0);
        for _ in 0..traversal.ny {
            traversal.advance(0, 0, maximas.x);
            for _ in 0..traversal.nx {
                let a = &data[data_ix];
                if data_ix == at {
                    break 'outer;
                }
                traversal.push(a, 1);
                data_ix += 1;
            }
        }
    }
    let mut result = Default::default();
    for (w, p) in weights {
        result += T::from(*w) * *traversal.fetch(p.z, p.y, p.x);
    }
    result
}

#[deprecated(
    since = "0.1.0",
    note = "Use 'furthest_neighbour_per_dimension' instead"
)]
fn predict_maximas(values: &Vec<(i32, Position)>) -> Position {
    let max_x = values.iter().map(|(_, x)| x.x).max();
    let max_y = values.iter().map(|(_, x)| x.y).max();
    let max_z = values.iter().map(|(_, x)| x.z).max();

    let x = match max_x {
        Some(i) => {
            if i != 0 {
                i
            } else {
                1
            }
        }
        _ => 1,
    };
    let y = match max_y {
        Some(i) => {
            if i != 0 {
                i
            } else {
                1
            }
        }
        _ => 1,
    };
    let z = match max_z {
        Some(i) => {
            if i != 0 {
                i
            } else {
                1
            }
        }
        _ => 1,
    };

    Position { x, y, z }
}

pub struct Predictor<T> {
    pub traversal: Traversal<T>,
    pub weights: Vec<Weight>,
    pub data: Vec<T>,
}

pub mod predictors {
    use super::{Position, Weight};

    pub fn get_lastvalue() -> Vec<Weight> {
        vec![Weight {
            coeff: 1,
            pos: Position { x: 1, y: 0, z: 0 },
        }]
    }

    pub fn get_lorenz() -> Vec<Weight> {
        vec![
            Weight {
                coeff: 1,
                pos: Position { x: 1, y: 1, z: 1 },
            },
            Weight {
                coeff: 1,
                pos: Position { x: 0, y: 0, z: 1 },
            },
            Weight {
                coeff: 1,
                pos: Position { x: 0, y: 1, z: 0 },
            },
            Weight {
                coeff: 1,
                pos: Position { x: 1, y: 0, z: 0 },
            },
            Weight {
                coeff: -1,
                pos: Position { x: 1, y: 1, z: 0 },
            },
            Weight {
                coeff: -1,
                pos: Position { x: 0, y: 1, z: 1 },
            },
            Weight {
                coeff: -1,
                pos: Position { x: 1, y: 0, z: 1 },
            },
        ]
    }
}

#[deprecated(since = "0.1.0", note = "use 'neighbours' instead")]
pub fn predictions<'a, T: AddAssign<<T as Mul>::Output> + Copy + Default + Mul + From<i16>>(
    p: &'a mut Predictor<T>,
) -> impl Generator<Yield = T, Return = ()> + 'a {
    move || {
        let mut data_ix = 0usize;
        while data_ix < p.data.len() {
            p.traversal.advance(1, 0, 0);
            for _ in 0..p.traversal.nz {
                p.traversal.advance(0, 1, 0);
                for _ in 0..p.traversal.ny {
                    p.traversal.advance(0, 0, 1);
                    for _ in 0..p.traversal.nx {
                        let a = &p.data[data_ix];

                        let mut result = T::default();
                        for w in &p.weights {
                            let coeff =
                                *p.traversal.fetch(w.pos.z, w.pos.y, w.pos.x) * T::from(w.coeff);
                            result += coeff;
                        }
                        yield result;
                        p.traversal.push(a, 1);
                        data_ix += 1;
                    }
                }
            }
        }
    }
}

/// Searches for the furthest neighbout in each dimension.
/// This method is important for the number of 0s added in the advance
/// function of the traversal.
fn furthest_neighbour_per_dimension(values: &Vec<Position>) -> Position {
    let max_x = values.iter().map(|x| x.x).max();
    let max_y = values.iter().map(|x| x.y).max();
    let max_z = values.iter().map(|x| x.z).max();

    let x = match max_x {
        Some(i) => {
            if i != 0 {
                i
            } else {
                1
            }
        }
        _ => 1,
    };
    let y = match max_y {
        Some(i) => {
            if i != 0 {
                i
            } else {
                1
            }
        }
        _ => 1,
    };
    let z = match max_z {
        Some(i) => {
            if i != 0 {
                i
            } else {
                1
            }
        }
        _ => 1,
    };

    Position { x, y, z }
}

#[deprecated(since="0.1.0", note="Please use 'single_neighbours_with_ring' instead.")]
pub fn neighbours<'a, T: AddAssign<<T as Mul>::Output> + Copy + Default + Mul + From<i16>>(
    mut traversal: Traversal<T>,
    data: &'a Vec<T>,
    neighbours: &'a Vec<Position>,
) -> impl Generator<Yield = Vec<T>, Return = ()> + 'a {
    move || {
        let mut data_ix = 0usize;
        let max = furthest_neighbour_per_dimension(&neighbours);
        while data_ix < data.len() {
            traversal.advance(max.z, 0, 0);
            for _ in 0..traversal.nz {
                traversal.advance(0, max.y, 0);
                for _ in 0..traversal.ny {
                    traversal.advance(0, 0, max.x);
                    for _ in 0..traversal.nx {
                        let a = &data[data_ix];
                        let mut result: Vec<T> = Vec::new();
                        for p in neighbours.iter() {
                            result.push(*traversal.fetch(p.z, p.y, p.x));
                        }
                        yield result;
                        traversal.push(a, 1);
                        data_ix += 1;
                    }
                }
            }
        }
    }
}

pub struct GeneratorIteratorAdapter<G>(pub G);

impl<G> Iterator for GeneratorIteratorAdapter<G>
where
    G: Generator<Return = ()>,
{
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { self.0.resume() } {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    #[test]
    fn test_fetch() {
        let data = vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
            16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0,
        ];
        let tr = Traversal::new(3, 3, 3);

        let mut weights: Vec<Position> = Vec::new();
        weights.push(Position { x: 0, y: 1, z: 1 });

        let results: Vec<f64> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).map(|x|x[0]).collect();

        assert_eq!(results[20], 0f64);
        assert_eq!(results[17], 5f64);
        assert_eq!(results[11], 0f64);
    }

    #[test]
    fn get_all_predictions() {
        let data = vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
            16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0,
        ];
        let tr = Traversal::new(3, 3, 3);

        let mut weights: Vec<Position> = Vec::new();
        weights.push(Position { x: 1, y: 0, z: 0 });

        let generator_iterator = GeneratorIteratorAdapter(neighbours(tr, &data, &weights));
        let results: Vec<f64> = generator_iterator.map(|x| x[0]).collect();
        assert_eq!(
            results,
            vec![
                0.0, 0.0, 1.0, 0.0, 3.0, 4.0, 0.0, 6.0, 7.0, 0.0, 9.0, 10.0, 0.0, 12.0, 13.0, 0.0,
                15.0, 16.0, 0.0, 18.0, 19.0, 0.0, 21.0, 22.0, 0.0, 24.0, 25.0
            ]
        );

        for (i, c) in results.iter().enumerate() {
            println!("{}: {}", i, c);
        }
    }

    #[test]
    #[ignore]
    fn extended_fetch_test_for_traversal() {
        let data = vec![
            0.0, 1.0, 2.0,
            3.0, 4.0, 5.0,
            6.0, 7.0, 8.0,

            9.0, 10.0, 11.0,
            12.0, 13.0, 14.0,
            15.0, 16.0, 17.0,

            18.0, 19.0, 20.0,
            21.0, 22.0, 23.0,
            24.0, 25.0, 26.0,
        ];

        {
            let tr = Traversal::new(3, 3, 3);
            let mut weights: Vec<Position> = Vec::new();
            weights.push(Position { x: 1, y: 0, z: 2 });

            let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();
            assert_eq!(result[20], vec![1f64]);
            assert_eq!(result[17], vec![0f64]);
            assert_eq!(result[26], vec![7f64]);
            assert_eq!(result[18], vec![0f64]);
            assert_eq!(result[9], vec![0f64]);
            assert_eq!(result[13], vec![0f64]);
            assert_eq!(result[5], vec![0f64]);
            assert_eq!(result[19], vec![0f64]);
            assert_eq!(result[11], vec![0f64]);
            assert_eq!(result[10], vec![0f64]);
            assert_eq!(result[9], vec![0f64]);
            assert_eq!(result[1], vec![0f64]);
            assert_eq!(result[22], vec![3f64]);
        }

        {   let tr = Traversal::new(3, 3, 3);
            let mut weights: Vec<Position> = Vec::new();
            weights.push(Position { x: 1, y: 2, z: 0 });


            let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();
            assert_eq!(result[19], vec![0f64]);
            assert_eq!(result[20], vec![0f64]);
            assert_eq!(result[17], vec![10f64]);
            assert_eq!(result[26], vec![19f64]);
            assert_eq!(result[18], vec![0f64]);
            assert_eq!(result[9], vec![0f64]);
            assert_eq!(result[13], vec![0f64]);
            assert_eq!(result[5], vec![0f64]);
            assert_eq!(result[25], vec![18f64]);
            assert_eq!(result[18], vec![0f64]);
            assert_eq!(result[16], vec![9f64]);
        }

        {   let tr = Traversal::new(3, 3, 3);
            let mut weights: Vec<(Position)> = Vec::new();
            weights.push(Position { x: 3, y: 3, z: 0 });

            let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();
            assert_eq!(result[19], vec![0f64]);
            assert_eq!(result[20], vec![0f64]);
            assert_eq!(result[17], vec![0f64]);
            assert_eq!(result[26], vec![0f64]);
            assert_eq!(result[18], vec![0f64]);
            assert_eq!(result[9], vec![ 0f64]);
            assert_eq!(result[13], vec![0f64]);
            assert_eq!(result[5], vec![ 0f64]);
            assert_eq!(result[25], vec![0f64]);
            assert_eq!(result[18], vec![0f64]);
            assert_eq!(result[16], vec![0f64]);
        }

        {
            let tr = Traversal::new(3, 3, 3);
            let mut weights: Vec<Position> = Vec::new();
            weights.push(Position { x: 2, y: 1, z: 1 });

            let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();
            assert_eq!(result[24], vec![0f64]);
            assert_eq!(result[26], vec![18f64]);
        }

        {
            let mut tr = Traversal::new(3, 3, 3);
            let mut weights: Vec<(i32, Position)> = Vec::new();
            weights.push((1, Position { x: 2, y: 1, z: 1 }));

            let result = predict(&data, 26, &mut tr, &weights);
            {
                let look = &tr.a[..];
                let x = 3;
            }
            assert_eq!(result, 12f64);
        }
    }

    // #[test]
    // fn test_for_negative_positions(){

    //     let data = vec![
    //         0.0, 1.0, 2.0,
    //         3.0, 4.0, 5.0,
    //         6.0, 7.0, 8.0,

    //         9.0, 10.0, 11.0,
    //         12.0, 13.0, 14.0,
    //         15.0, 16.0, 17.0,

    //         18.0, 19.0, 20.0,
    //         21.0, 22.0, 23.0,
    //         24.0, 25.0, 26.0,
    //     ];

    //     {
    //         let tr = Traversal::new(3, 3, 3);
    //         let mut weights: Vec<Position> = Vec::new();
    //         weights.push(Position { x: -1, y: 1, z: 0 });

    //         let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();
    //         assert_eq!(result[1], vec![0f64]);
    //         assert_eq!(result[10], vec![0f64]);
    //         assert_eq!(result[11], vec![0f64]);
    //         assert_eq!(result[13], vec![11f64]);
    //         assert_eq!(result[17], vec![0f64]);
    //         assert_eq!(result[18], vec![0f64]);
    //         assert_eq!(result[19], vec![0f64]);
    //         assert_eq!(result[2], vec![0f64]);
    //         assert_eq!(result[20], vec![0f64]);
    //         assert_eq!(result[22], vec![20f64]);
    //         assert_eq!(result[26], vec![0f64]);
    //         assert_eq!(result[5], vec![0f64]);
    //         assert_eq!(result[9], vec![0f64]);
    //         assert_eq!(result[25], vec![23f64]);
    //         assert_eq!(result[12], vec![10f64]);
    //         assert_eq!(result[16], vec![14f64]);
    //         assert_eq!(result[21], vec![19f64]);
    //         assert_eq!(result[3], vec![1f64]);
    //         assert_eq!(result[7], vec![5f64]);
    //     }

    //     {   let tr = Traversal::new(3, 3, 3);
    //         let mut weights: Vec<Position> = Vec::new();
    //         weights.push(Position { x: 0, y: -1, z: 1 });


    //         let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();
    //         assert_eq!(result[13], vec![7f64]);
    //         assert_eq!(result[10], vec![4f64]);
    //         assert_eq!(result[11], vec![5f64]);
    //         assert_eq!(result[16], vec![0f64]);
    //         assert_eq!(result[17], vec![0f64]);
    //         assert_eq!(result[18], vec![12f64]);
    //         assert_eq!(result[19], vec![13f64]);
    //         assert_eq!(result[20], vec![14f64]);
    //         assert_eq!(result[25], vec![0f64]);
    //         assert_eq!(result[26], vec![0f64]);
    //         assert_eq!(result[5], vec![ 0f64]);
    //         assert_eq!(result[9], vec![ 3f64]);
    //     }

    // }

    // #[test]
    // fn test_negative_positions() {
    //     let data = vec![
    //         0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
    //         16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0,
    //     ];
    //     let tr = Traversal::new(3, 3, 3);

    //     let mut weights: Vec<Position> = Vec::new();
    //     weights.push(Position { x: -1, y: 1, z: 0 });

    //         let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();
    //     assert_eq!(result[20], vec![0f64]);
    //     assert_eq!(result[17], vec![0f64]);
    //     assert_eq!(result[11], vec![0f64]);
    //     assert_eq!(result[21], vec![19f64]);
    //     assert_eq!(result[22], vec![20f64]);
    //     assert_eq!(result[15], vec![13f64]);
    //     assert_eq!(result[ 8], vec![0f64]);
    //     assert_eq!(result[25], vec![23f64]);
    //     assert_eq!(result[4], vec![2f64]);
    // }

    // #[test]
    // fn test_negative_positions_further_away() {
    //     let data = vec![
    //         0.0, 1.0, 2.0,
    //         3.0, 4.0, 5.0,
    //         6.0, 7.0, 8.0,

    //         9.0, 10.0, 11.0,
    //         12.0, 13.0, 14.0,
    //         15.0,16.0, 17.0,

    //         18.0, 19.0, 20.0,
    //         21.0, 22.0, 23.0,
    //         24.0, 25.0, 26.0,
    //     ];
    //     let tr = Traversal::new(3, 3, 3);

    //     let mut weights: Vec<Position> = Vec::new();
    //     weights.push(Position { x: -1, y: 1, z: 1 });

    //     let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();
    //     assert_eq!(result[20], vec![0f64]);
    //     assert_eq!(result[17], vec![0f64]);
    //     assert_eq!(result[11], vec![0f64]);
    //     assert_eq!(result[21], vec![10f64]);
    //     assert_eq!(result[22], vec![11f64]);
    //     assert_eq!(result[15], vec![4f64]);
    //     assert_eq!(result[ 8], vec![0f64]);
    //     assert_eq!(result[25], vec![14f64]);
    //     assert_eq!(result[4], vec![ 0f64]);
    // }

    // #[test]
    // fn test_negative_distance_for_x_max_1() {
    //     let data = vec![
    //         0.0, 1.0, 2.0,
    //         3.0, 4.0, 5.0,
    //         6.0, 7.0, 8.0,

    //         9.0, 10.0, 11.0,
    //         12.0, 13.0, 14.0,
    //         15.0,16.0, 17.0,

    //         18.0, 19.0, 20.0,
    //         21.0, 22.0, 23.0,
    //         24.0, 25.0, 26.0,
    //     ];
    //     {   let tr = Traversal::new(3, 3, 3);
    //         let mut weights: Vec<Position> = Vec::new();
    //         weights.push(Position { x: -1, y: -1, z: 1 });

    //         let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();

    //         assert_eq!(result[12], vec![7f64]);
    //         assert_eq!(result[13], vec![8f64]);
    //         assert_eq!(result[15], vec![0f64]);
    //         assert_eq!(result[16], vec![0f64]);
    //         assert_eq!(result[17], vec![0f64]);
    //         assert_eq!(result[18], vec![13f64]);
    //         assert_eq!(result[19], vec![14f64]);
    //         assert_eq!(result[20], vec![0f64]);
    //         assert_eq!(result[21], vec![16f64]);
    //         assert_eq!(result[24], vec![0f64]);
    //         assert_eq!(result[25], vec![0f64]);
    //         assert_eq!(result[26], vec![0f64]);
    //         assert_eq!(result[3], vec![0f64]);
    //         assert_eq!(result[5], vec![0f64]);
    //         assert_eq!(result[6], vec![0f64]);
    //         assert_eq!(result[9], vec![4f64]);
    //     }
    // }

    // #[test]
    // fn test_negative_distance_for_x_ge_1() {
    //     let data = vec![
    //         0.0, 1.0, 2.0,
    //         3.0, 4.0, 5.0,
    //         6.0, 7.0, 8.0,

    //         9.0, 10.0, 11.0,
    //         12.0, 13.0, 14.0,
    //         15.0,16.0, 17.0,

    //         18.0, 19.0, 20.0,
    //         21.0, 22.0, 23.0,
    //         24.0, 25.0, 26.0,
    //     ];
    //     {   let tr = Traversal::new(3, 3, 3);
    //         let mut weights: Vec<Position> = Vec::new();
    //         weights.push(Position { x:  0, y: 0, z: 1 });
    //         weights.push(Position { x:  0, y: -1, z: 1 });
    //         weights.push(Position { x:  0, y: 1, z: 1 });
    //         weights.push(Position { x:  -1, y: 0, z: 2 });
    //         weights.push(Position { x: -2, y: 1, z: 0 });
    //         weights.push(Position { x: -1, y: 1, z: 0 });
    //         weights.push(Position { x: -1, y: 0, z: 1 });
    //         weights.push(Position { x: 1, y: 0, z: 1 });
    //         // weights.push(Position { x: -1, y: 2, z: 0 }); // TODO: If I don't get a hit it is wrong

    //         let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();

    //         assert_eq!(result[21], vec![12.0, 15.0, 9.0, 4.0, 20.0, 19.0, 13.0, 0.0]);
    //         assert_eq!(result[24], vec![15.0, 0.0, 12.0, 7.0, 23.0, 22.0, 16.0, 0.0]);
    //         assert_eq!(result[7],  vec![0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0]);
    //         assert_eq!(result[16], vec![7.0, 0.0, 4.0, 0.0, 0.0, 14.0, 8.0, 6.0]);
    //         assert_eq!(result[17], vec![8.0, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 7.0]);
    //     }
    // }


    // #[test]
    // fn test_negative_distance_groups() {
    //     let data = vec![
    //         0.0, 1.0, 2.0,
    //         3.0, 4.0, 5.0,
    //         6.0, 7.0, 8.0,

    //         9.0, 10.0, 11.0,
    //         12.0, 13.0, 14.0,
    //         15.0,16.0, 17.0,

    //         18.0, 19.0, 20.0,
    //         21.0, 22.0, 23.0,
    //         24.0, 25.0, 26.0,
    //     ];
    //     {   let tr = Traversal::new(3, 3, 3);
    //         let mut weights: Vec<Position> = Vec::new();
    //         weights.push(Position { x: 2, y: 1, z: 1 });
    //         weights.push(Position { x: -2, y: 1, z: 1 });

    //         let result: Vec<Vec<f64>> = GeneratorIteratorAdapter(neighbours(tr, &data, &weights)).collect();

    //         assert_eq!(result[24], vec![0.0,14.0]);
    //     }
    // }
}
