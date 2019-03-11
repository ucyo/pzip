pub trait ResidualTrait {
    fn residual(&self, truth: &u32, prediction: &u32) -> u32;
    fn truth(&self, residual: &u32, prediction: &u32) -> u32;
}

pub enum ResidualCalculation {
    ExclusiveOR,
    Shifted,
}

impl ResidualTrait for ResidualCalculation {
    fn residual(&self, truth: &u32, prediction: &u32) -> u32 {
        match self {
            ResidualCalculation::ExclusiveOR => *truth ^ *prediction,
            ResidualCalculation::Shifted => unimplemented!(),
        }
    }
    fn truth(&self, residual: &u32, prediction: &u32) -> u32 {
        match self {
            ResidualCalculation::ExclusiveOR => *residual ^ *prediction,
            ResidualCalculation::Shifted => unimplemented!(),
        }
    }
}

use rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let pred: u32 = rng.gen();
    let truth: u32 = rng.gen();
    let xor = ResidualCalculation::ExclusiveOR.residual(&truth, &pred);
    let xor_rev = ResidualCalculation::ExclusiveOR.truth(&xor, &pred);
    println!("ExclusiveOR");
    println!("{:032b} {:032b}", pred, truth);
    println!("{:032b} {:032b} {}", xor, xor_rev, xor_rev == truth);


}
