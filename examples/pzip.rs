extern crate pzip;

use pzip::config::FileType;
use pzip::position::Position;
use pzip::{Setup, Weight};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = pzip::config::parse_args(&args);

    let lv_weights = vec![Weight {
        coeff: 1,
        pos: Position { z: 0, y: 0, x: 1 },
    }];

    if configuration.filetype == FileType::F64 {
        let setup = Setup::<f64>::new(configuration.input, configuration.shape, lv_weights);
        setup.write(configuration.output);
    } else if configuration.filetype == FileType::F32 {
        let setup = Setup::<f32>::new(configuration.input, configuration.shape, lv_weights);
        setup.write(configuration.output);
    } else {
        panic!("Error")
    }
}
