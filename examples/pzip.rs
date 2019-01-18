extern crate pzip;

use pzip::config::FileType;
use pzip::position::Position;
use pzip::{Setup, Weight};
use std::env;
use pzip::config::MapType;
use pzip::mapping::{Raw, Ordered, Untouched};

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = pzip::config::parse_args(&args);

    let lv_weights = vec![Weight {
        coeff: 1,
        pos: Position { z: 0, y: 0, x: 1 },
    }];

    if configuration.filetype == FileType::F64 {
        let setup = Setup::<f64>::new(configuration.input, configuration.shape, lv_weights);  // todo: include choosing by predictor
        match configuration.mapping {
            MapType::Raw => setup.write::<Raw,Untouched>(configuration.output),  // todo: include choosing intra map
            MapType::Ordered => setup.write::<Ordered,Untouched>(configuration.output),
        }
    } else if configuration.filetype == FileType::F32 {
        let setup = Setup::<f32>::new(configuration.input, configuration.shape, lv_weights);  // todo: include choosing by predictor
        match configuration.mapping {
            MapType::Raw => setup.write::<Raw, Untouched>(configuration.output),
            MapType::Ordered => setup.write::<Ordered, Untouched>(configuration.output),  // todo: include choosing intra map
        }
    } else {
        panic!("Error")
    }
}
