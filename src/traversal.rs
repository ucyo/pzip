use super::position::Position;

#[derive(Debug)]
pub struct Traversal {
    nz: usize,
    ny: usize,
    nx: usize,
    dz: usize,
    dy: usize,
    dx: usize,
    m: usize,
    a: Vec<f64>,
    ix: usize,
    zero: f64,
}

pub fn next_power_2(mut val: usize) -> usize {
    while val & (val + 1) != 0 {
        val = val | (val + 1)
    }
    val + 1
}

impl Traversal {
    pub fn new(nz: usize, ny: usize, nx: usize) -> Traversal {
        let dx = 1;
        let dy = nx + 1;
        let dz = dy * (ny + 1);

        let sum = dz + dy + dx - 1;
        let m = next_power_2(sum) - 1;
        let a = vec![0f64; m + 1];
        let ix = 0;
        let zero = 0f64;

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
        self.push(self.zero, n);
    }
    pub fn push(&mut self, val: f64, mut n: usize) {
        while n > 0 {
            self.a[self.ix & self.m] = val;
            self.ix += 1;
            n -= 1;
        }
    }
    pub fn fetch(&self, z: usize, y: usize, x: usize) -> f64 {
        let pos = self.ix - (self.dz * z + self.dy * y + self.dx * x);
        self.a[pos & self.m]
    }
}

pub fn predict(
    data: &Vec<f64>,
    at: usize,
    traversal: &mut Traversal,
    weights: &Vec<(i32, Position)>,
) -> f64 {
    let mut data_ix = 0usize;
    traversal.advance(1, 0, 0);
    'outer: for _ in 0..traversal.nz {
        traversal.advance(0, 1, 0);
        for _ in 0..traversal.ny {
            traversal.advance(0, 0, 1);
            for _ in 0..traversal.nx {
                let a = data[data_ix];
                println!("Pushed {:?}", traversal);
                if data_ix == at {
                    break 'outer;
                }
                traversal.push(a, 1);
                data_ix += 1;
            }
        }
    }
    let mut result = 0f64;
    for (w, p) in weights {
        result += *w as f64 * traversal.fetch(p.z, p.y, p.x);
    }
    result
}

use std::ops::{Generator, GeneratorState};

pub struct Predictor {
    pub traversal: Traversal,
    pub weights: Vec<(i32, Position)>,
    pub data: Vec<f64>,
}
#[allow(dead_code)]
pub fn predictions(mut p: Predictor) -> impl Generator<Yield = f64, Return = ()> {
    move || {
        let mut data_ix = 0usize;
        while data_ix < p.data.len() {
            p.traversal.advance(1, 0, 0);
            for _ in 0..p.traversal.nz {
                p.traversal.advance(0, 1, 0);
                for _ in 0..p.traversal.ny {
                    p.traversal.advance(0, 0, 1);
                    for _ in 0..p.traversal.nx {
                        let a = p.data[data_ix];

                        // This needs to be bench tested using criterion:
                        // Which implementation is faster than the other?

                        // method 1
                        yield p
                            .weights
                            .iter()
                            .map(|(w, f)| *w as f64 * p.traversal.fetch(f.z, f.y, f.x))
                            .sum();

                        //method 2
                        // let mut result = 0f64;
                        // for (w,pi) in &p.weights {
                        //     result += *w as f64 * p.traversal.fetch(pi.z, pi.y, pi.x);
                        // }
                        // yield result;

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

        let mut weights: Vec<(i32, Position)> = Vec::new();
        weights.push((1, Position { x: 1, y: 0, z: 0 }));

        let p = Predictor {
            traversal: tr,
            weights: weights,
            data: data,
        };

        let generator_iterator = GeneratorIteratorAdapter(predictions(p));
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
