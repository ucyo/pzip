// use pzip::mapping::{Intermapping, Raw, Untouched};
use pzip::position::Position as Coordinate;
use pzip::predictors::predictors;
use pzip::transform::InterMapping;
use pzip::transform::{Byte, Compact, Inter, Intra};
use pzip::Setup;

#[test]
#[ignore]
fn compression_using_last_value_all_once_f64_raw() {
    let input = String::from("/home/ucyo/rust/pzip/data/icon.ml.qv.f64.little.4x90x351x901_0.raw");
    let output = String::from("/tmp/testing64.pzip");
    let shape = Coordinate {
        z: 90,
        y: 351,
        x: 901,
    };
    let predictor = predictors::get_last_value_f64();

    let mut prediction = Setup::<f64>::new(&input, shape, predictor);
    prediction.write(Inter::Untouched, Intra::Untouched, Byte::Untouched, &output);

    let origin = pzip::testing::read_first_k_f64(&input, 760);
    let outcome = pzip::testing::read_first_k_f64(&output, 760);

    for i in 362..623 {
        println!("{} {} {}", i, origin[i - 1], outcome[i]);
        assert_eq!(
            Inter::Untouched.from_u64(
                Inter::Untouched.to_u64(origin[i - 1]) ^ Inter::Untouched.to_u64(origin[i])
            ),
            outcome[i]
        );
    }
    std::fs::remove_file(&output).expect("Error");
}

#[test]
#[ignore]
fn compression_using_last_value_all_once_f32_raw() {
    let input = String::from("/home/ucyo/rust/pzip/data/icon.ml.qv.f32.little.4x90x351x901_0.raw");
    let output = String::from("/tmp/testing32.pzip");
    let shape = Coordinate {
        z: 90,
        y: 351,
        x: 901,
    };
    let predictor = predictors::get_last_value_f32();

    let mut prediction = Setup::<f32>::new(&input, shape, predictor);
    prediction.write(
        Inter::Untouched,
        Intra::Untouched,
        Byte::Untouched,
        Compact::Untouched,
        false,
        &output,
    );

    let origin = pzip::testing::read_first_k_f32(&input, 760);
    let outcome = pzip::testing::read_first_k_f32(&output, 760);

    for i in 362..623 {
        println!("{} {} {}", i, origin[i - 1], outcome[i]);
        assert_eq!(
            Inter::Untouched.from_u32(
                Inter::Untouched.to_u32(origin[i - 1]) ^ Inter::Untouched.to_u32(origin[i])
            ),
            outcome[i]
        );
    }
    std::fs::remove_file(&output).expect("Error");
}
