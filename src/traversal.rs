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

impl<T: Default + Copy> Traversal<T> {
    pub fn new(nz: usize, ny: usize, nx: usize) -> Self {
        let dx = 1;
        let dy = nx + 1;
        let dz = dy * (ny + 1);

        let sum = dz + dy + dx - 1;
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
    pub fn advance(&mut self, z: usize, y: usize, x: usize) {
        let n = self.dz * z + self.dy * y + self.dx * x;
        self.push(&Default::default(), n);
    }
    pub fn push(&mut self, val: &T, mut n: usize) {
        while n > 0 {
            self.a[self.ix & self.m] = *val;
            self.ix += 1;
            n -= 1;
        }
    }
    pub fn fetch(&self, z: usize, y: usize, x: usize) -> &T {
        let pos = self.ix - (self.dz * z + self.dy * y + self.dx * x);
        &self.a[pos & self.m]
    }
}

pub fn predict<T: Copy + AddAssign<<T as Mul>::Output> + Default + From<i32> + Mul>(
    data: &Vec<T>,
    at: usize,
    traversal: &mut Traversal<T>,
    weights: &Vec<(i32, Position)>,
) -> T {
    let mut data_ix = 0usize;
    traversal.advance(1, 0, 0);
    'outer: for _ in 0..traversal.nz {
        traversal.advance(0, 1, 0);
        for _ in 0..traversal.ny {
            traversal.advance(0, 0, 1);
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

pub struct Predictor<T> {
    pub traversal: Traversal<T>,
    pub weights: Vec<Weight>,
    pub data: Vec<T>,
}

pub mod predictors {
    use super::{Weight, Position};

    pub fn get_lastvalue() -> Vec<Weight> {
        vec![Weight{
            coeff:1, pos: Position{x:1, y:0, z:0}
        }]
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
        let mut tr = Traversal::new(3, 3, 3);

        let mut weights: Vec<(i32, Position)> = Vec::new();
        weights.push((1, Position { x: 0, y: 1, z: 1 }));

        assert_eq!(predict(&data, 20, &mut tr, &weights), 0f64);
        assert_eq!(predict(&data, 17, &mut tr, &weights), 5f64);
        assert_eq!(predict(&data, 11, &mut tr, &weights), 0f64);
    }

    #[test]
    fn get_all_predictions() {
        let data = vec![
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
            16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0,
        ];
        let tr = Traversal::new(3, 3, 3);

        let mut weights: Vec<Weight> = Vec::new();
        weights.push(Weight {
            coeff: 1,
            pos: Position { x: 1, y: 0, z: 0 },
        });

        let mut p = Predictor {
            traversal: tr,
            weights: weights,
            data: data,
        };

        let generator_iterator = GeneratorIteratorAdapter(predictions(&mut p));
        let results: Vec<f64> = generator_iterator.collect();
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
}
