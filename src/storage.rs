
#[derive(Debug)]
pub struct Storage {
    pub data: Vec<f64>,
    pub ix:   usize,
    pub and:  usize,
    pub dims: Dimension,
    pub default: f64,
}


impl Storage {
    pub fn new(shape: Shape, maxposition : &position::Position) -> Self {
        let nx = 1;
        let ny = nx * (shape.x + maxposition.x);
        let nz = ny * (shape.y + maxposition.y);

        let mut size = nx + ny + nz;
        while size & (size + 1) != 0{
            size = size | size + 1
        };
        let data = vec![0f64;size+1];
        let ix   = 0_usize;

        Storage{data, ix, and:size, dims:Dimension{x:nx,y:ny,z:nz}, default: 0f64}
    }

    fn push(&mut self, value: f64, mut n: usize) {
        println!("Pushing {}x{}", n, value);
        while n > 0 {
            self.data[self.ix & self.and] = value;
            self.ix += 1;
            n -= 1;
        }
    }

    fn adv(&mut self, z:usize, y:usize, x: usize){
        let n = z * self.dims.z + y * self.dims.y + x*self.dims.x;
        self.push(self.default, n);
    }

    fn pos(&self, z: usize, y:usize, x:usize) -> f64 {
        let pos = self.ix - (x*self.dims.x + y*(self.dims.y+1) + z * (self.dims.z+self.dims.y));
        self.data[pos & self.and]
    }
}
