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
    zero: f64
}

pub fn next_power_2(mut val: usize) -> usize {
    while val & (val + 1) != 0 {
        val = val | (val + 1)
    }
    val + 1
}

impl Traversal {
    pub fn new(nz:usize, ny:usize, nx: usize) -> Traversal {
        let dx = 1;
        let dy = nx + 1;
        let dz = dy * (ny + 1);

        let sum = dz + dy + dx - 1;
        let m = next_power_2(sum) - 1;
        let a = vec![0f64; m + 1];
        let ix = 0;
        let zero = 0f64;

        Traversal{nz,ny,nx,dz,dy,dx,m,a,ix,zero}
    }
    pub fn advance(&mut self, z: usize, y:usize, x:usize) {
        let n = self.dz * z + self.dy * y + self.dx * x;
        self.push(self.zero, n);
    }
    pub fn push(&mut self, val:f64, mut n: usize) {
        println!("Pushing {}x{}", n, val);
        while n > 0 {
            self.a[self.ix & self.m] = val;
            self.ix += 1;
            n -= 1;
        }
    }
    pub fn fetch(self, z: usize, y:usize, x:usize) -> f64 {
        let pos = self.ix - (self.dz * z + self.dy * y + self.dx * x);
        self.a[pos & self.m]
    }
}

pub fn predict(data: &Vec<f64>, at: usize, mut traversal: Traversal) -> f64 {
    let mut data_ix = 0usize;
    traversal.advance(1, 0, 0);
    'outer:
    for _ in 0..traversal.nz {
        traversal.advance(0, 1, 0);
        for _ in 0..traversal.ny {
            traversal.advance(0, 0, 1);
            for _ in 0..traversal.nx {
                let a = data[data_ix];
                println!("Pushed {:?}", traversal);
                if data_ix == at {
                    break 'outer
                }
                traversal.push(a, 1);
                data_ix += 1;
            }
        }
    }
    traversal.fetch(1, 1, 1)
}

mod tests {
    use super::*;
    #[test]
    fn test_fetch(){
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
        let tr = Traversal::new(3, 3, 3);
        assert_eq!( predict(&data, 24, tr), 0f64);
    }
}
