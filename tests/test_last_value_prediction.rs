
use pzip::position::Position;
use pzip::{Setup, Weight};

#[test]
fn compression_using_last_value_all_once_f64() {
    let input = String::from("/home/ucyo/rust/pzip/data/pres.1-90-181-361.f64.bin");
    let output = String::from("/home/ucyo/rust/pzip/data/lastvalue.all.f64.pred");
    let shape = pzip::Shape {
        z: 90,
        y: 181,
        x: 361,
    };
    let weights = vec![Weight{coeff: 1, pos: Position { z: 0, y: 0, x: 1 }}];

    let prediction = Setup::<f64>::new(&input, shape, weights);
    prediction.write(&output);

    let origin = pzip::testing::read_first_k_f64(&input, 760);
    let outcome = pzip::testing::read_first_k_f64(&output, 760);

    for i in 362..623 {
        println!("{} {} {}", i, origin[i - 1], outcome[i]);
        assert_eq!(origin[i - 1], outcome[i]);
    }
    std::fs::remove_file(&output).expect("Error");
}


#[test]
fn compression_using_last_value_all_once_f32() {
    let input = String::from("/home/ucyo/rust/pzip/data/pres.1-90-181-361.f32.bin");
    let output = String::from("/home/ucyo/rust/pzip/data/lastvalue.all.f32.pred");
    let shape = pzip::Shape {
        z: 90,
        y: 181,
        x: 361,
    };
    let weights = vec![Weight{coeff: 1, pos: Position { z: 0, y: 0, x: 1 }},];

    let prediction = Setup::<f32>::new(&input, shape, weights);
    prediction.write(&output);

    let origin = pzip::testing::read_first_k_f32(&input, 760);
    let outcome = pzip::testing::read_first_k_f32(&output, 760);

    for i in 362..623 {
        println!("{} {} {}", i, origin[i - 1], outcome[i]);
        assert_eq!(origin[i-1], outcome[i]);
    }
    std::fs::remove_file(&output).expect("Error");
}
