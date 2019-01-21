extern crate pzip;

use pzip::config::FileType;
use pzip::Setup;
use std::env;
use pzip::config::{MapType, Predictor};
use pzip::mapping::{Raw, Ordered, Untouched};
use pzip::traversal::predictors;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = pzip::config::parse_args(&args);

    if configuration.predictor == Predictor::LastValue {
        let predictor = predictors::get_lastvalue();
        if configuration.filetype == FileType::F64 {
            let setup = Setup::<f64>::new(configuration.input, configuration.shape, predictor);  // todo: include choosing by predictor
            match configuration.mapping {
                MapType::Raw => setup.write::<Raw,Untouched>(configuration.output),  // todo: include choosing intra map
                MapType::Ordered => setup.write::<Ordered,Untouched>(configuration.output),
            }
        } else if configuration.filetype == FileType::F32 {
            let setup = Setup::<f32>::new(configuration.input, configuration.shape, predictor);  // todo: include choosing by predictor
            match configuration.mapping {
                MapType::Raw => setup.write::<Raw, Untouched>(configuration.output),
                MapType::Ordered => setup.write::<Ordered, Untouched>(configuration.output),  // todo: include choosing intra map
            }
        } else {
            panic!("Error")
        }
    }
}
