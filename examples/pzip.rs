extern crate pzip;

use pzip::config::FileType;
use pzip::Setup;
use std::env;
use pzip::config::{MapType, Predictor, ByteMappingType, IntramappingType};
use pzip::mapping::{Raw, Ordered, Untouched, MonotonicGrayBytes, ClassicGray};
use pzip::traversal::predictors;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = pzip::config::parse_args(&args);

    // check for predictor
    if configuration.predictor == Predictor::LastValue {
        let predictor = predictors::get_lastvalue();

        //check for filetype
        if configuration.filetype == FileType::F64 {
            let setup = Setup::<f64>::new(configuration.input, configuration.shape, predictor);

            // match for mapping styles
            match (configuration.mapping, configuration.intramapping, configuration.bytemapping) {
                (MapType::Raw, IntramappingType::Untouched, ByteMappingType::Untouched) => setup.write::<Raw, Untouched, Untouched>(configuration.output),
                (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::Untouched) => setup.write::<Ordered, Untouched, Untouched>(configuration.output),  // todo: include choosing intra map
                (MapType::Raw, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
                (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
                (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched) => setup.write::<Raw, ClassicGray, Untouched>(configuration.output),
                (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched) => setup.write::<Ordered, ClassicGray, Untouched>(configuration.output),  // todo: include choosing intra map
                (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
                (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
            }
        } else if configuration.filetype == FileType::F32 {
            let setup = Setup::<f32>::new(configuration.input, configuration.shape, predictor);

            match (configuration.mapping, configuration.intramapping, configuration.bytemapping) {
                (MapType::Raw, IntramappingType::Untouched, ByteMappingType::Untouched) => setup.write::<Raw, Untouched, Untouched>(configuration.output),
                (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::Untouched) => setup.write::<Ordered, Untouched, Untouched>(configuration.output),  // todo: include choosing intra map
                (MapType::Raw, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
                (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
                (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched) => setup.write::<Raw, ClassicGray, Untouched>(configuration.output),
                (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched) => setup.write::<Ordered, ClassicGray, Untouched>(configuration.output),  // todo: include choosing intra map
                (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
                (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
            }
        } else {
            panic!("Error")
        }
    }
}
