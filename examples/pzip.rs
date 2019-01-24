#![allow(unused_imports)]
extern crate pzip;

use pzip::config::FileType;
use pzip::config::{ByteMappingType, CompactType, IntramappingType, MapType, Predictor};
use pzip::Setup;
use std::env;

use pzip::mapping::MonotonicGrayBytes;
use pzip::mapping::NoLZCCompact;
use pzip::mapping::{ClassicGray, Untouched};
use pzip::mapping::{Ordered, Raw};

use pzip::traversal::predictors;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = pzip::config::parse_args(&args);

    // let predictor = predictors::get_lastvalue();
    let predictor = predictors::get_lorenz();

    if configuration.filetype == FileType::F64 {
        let setup = Setup::<f64>::new(configuration.input, configuration.shape, predictor);

        // TODO: Test bitpacking for u64
        setup.write::<

                // Raw,          // intermapping from       f32 to u32
                Ordered,         // intermapping from       f32 to u32

                // Untouched,    // intramapping from       u32 to u32
                ClassicGray,     // intramapping from       u32 to u32

                // Untouched,       //  bytemapping from        u8 to u8
                MonotonicGrayBytes, //  bytemapping from        u8 to u8

            >(configuration.output)
    } else if configuration.filetype == FileType::F32 {
        let setup = Setup::<f32>::new(configuration.input, configuration.shape, predictor);
        setup.write::<

                // Raw,          // intermapping from       f32 to u32
                Ordered,         // intermapping from       f32 to u32

                // Untouched,    // intramapping from       u32 to u32
                ClassicGray,     // intramapping from       u32 to u32

                // Untouched,       //  bytemapping from        u8 to u8
                MonotonicGrayBytes, //  bytemapping from        u8 to u8

                Untouched,           //   bitpacking from  Vec<u32> to Vec<u32>
                // NoLZCCompact,     //   bitpacking from  Vec<u32> to Vec<u32>

            >(configuration.output)
    } else {
        panic!("Error!!")
    }

    // // check for predictor
    // if configuration.predictor == Predictor::LastValue {
    //     let predictor = predictors::get_lastvalue();

    //     //check for filetype
    //     if configuration.filetype == FileType::F64 {
    //         let setup = Setup::<f64>::new(configuration.input, configuration.shape, predictor);

    //         // match for mapping styles
    //         match (configuration.mapping, configuration.intramapping, configuration.bytemapping, configuration.compact) {
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::Untouched, CompactType::Untouched) => setup.write::<Raw, Untouched, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::Untouched, CompactType::Untouched) => setup.write::<Ordered, Untouched, Untouched>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes, CompactType::Untouched) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes, CompactType::Untouched) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched, CompactType::Untouched) => setup.write::<Raw, ClassicGray, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched, CompactType::Untouched) => setup.write::<Ordered, ClassicGray, Untouched>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes, CompactType::Untouched) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes, CompactType::Untouched) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::Untouched, CompactType::NoLZC) => setup.write::<Raw, Untouched, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::Untouched, CompactType::NoLZC) => setup.write::<Ordered, Untouched, Untouched>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes, CompactType::NoLZC) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes, CompactType::NoLZC) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched, CompactType::NoLZC) => setup.write::<Raw, ClassicGray, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched, CompactType::NoLZC) => setup.write::<Ordered, ClassicGray, Untouched>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes, CompactType::NoLZC) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes, CompactType::NoLZC) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
    //         }
    //     } else if configuration.filetype == FileType::F32 {
    //         let setup = Setup::<f32>::new(configuration.input, configuration.shape, predictor);

    //         match (configuration.mapping, configuration.intramapping, configuration.bytemapping, configuration.compact) {
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::Untouched, CompactType::Untouched) => setup.write::<Raw, Untouched, Untouched, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::Untouched, CompactType::Untouched) => setup.write::<Ordered, Untouched, Untouched, Untouched>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes, CompactType::Untouched) => setup.write::<Raw, Untouched, MonotonicGrayBytes, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes, CompactType::Untouched) => setup.write::<Raw, Untouched, MonotonicGrayBytes, Untouched>(configuration.output),
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched, CompactType::Untouched) => setup.write::<Raw, ClassicGray, Untouched, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched, CompactType::Untouched) => setup.write::<Ordered, ClassicGray, Untouched, Untouched>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes, CompactType::Untouched) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes, CompactType::Untouched) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes, Untouched>(configuration.output),
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::Untouched, CompactType::NoLZC) => setup.write::<Raw, Untouched, Untouched, NoLZCCompact>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::Untouched, CompactType::NoLZC) => setup.write::<Ordered, Untouched, Untouched, NoLZCCompact>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes, CompactType::NoLZC) => setup.write::<Raw, Untouched, MonotonicGrayBytes, NoLZCCompact>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes, CompactType::NoLZC) => setup.write::<Raw, Untouched, MonotonicGrayBytes, NoLZCCompact>(configuration.output),
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched, CompactType::NoLZC) => setup.write::<Raw, ClassicGray, Untouched, NoLZCCompact>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched, CompactType::NoLZC) => setup.write::<Ordered, ClassicGray, Untouched, NoLZCCompact>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes, CompactType::NoLZC) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes, NoLZCCompact>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes, CompactType::NoLZC) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes, NoLZCCompact>(configuration.output),
    //         }
    //     } else {
    //         panic!("Error")
    //     }
    // } else if configuration.predictor == Predictor::Lorenz {
    //     let predictor = predictors::get_lorenz();

    //     //check for filetype
    //     if configuration.filetype == FileType::F64 {
    //         let setup = Setup::<f64>::new(configuration.input, configuration.shape, predictor);

    //         // match for mapping styles
    //         match (configuration.mapping, configuration.intramapping, configuration.bytemapping) {
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::Untouched) => setup.write::<Raw, Untouched, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::Untouched) => setup.write::<Ordered, Untouched, Untouched>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched) => setup.write::<Raw, ClassicGray, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched) => setup.write::<Ordered, ClassicGray, Untouched>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
    //         }
    //     } else if configuration.filetype == FileType::F32 {
    //         let setup = Setup::<f32>::new(configuration.input, configuration.shape, predictor);

    //         match (configuration.mapping, configuration.intramapping, configuration.bytemapping) {
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::Untouched) => setup.write::<Raw, Untouched, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::Untouched) => setup.write::<Ordered, Untouched, Untouched>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Ordered, IntramappingType::Untouched, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, Untouched, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched) => setup.write::<Raw, ClassicGray, Untouched>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::Untouched) => setup.write::<Ordered, ClassicGray, Untouched>(configuration.output),  // todo: include choosing intra map
    //             (MapType::Raw, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
    //             (MapType::Ordered, IntramappingType::ClassicGrayCodes, ByteMappingType::MonotonicGrayCodes) => setup.write::<Raw, ClassicGray, MonotonicGrayBytes>(configuration.output),
    //         }
    //     } else {
    //         panic!("Error")
    //     }
    // }
}
