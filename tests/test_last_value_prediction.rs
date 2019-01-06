#[test]
fn compression_using_last_value_asbytes() {
    let input = String::from("/home/ucyo/rust/pzip/data/pres.1-90-181-361.f32.bin");
    let output = String::from("/home/ucyo/rust/pzip/data/pres.1-90-181-361.f32.bin.lastvalue.pred");
    let shape = pzip::Shape {
        z: 90,
        y: 181,
        x: 361,
    };
    let weights = [(1, pzip::Position { z: 0, y: 0, x: 1 })];

    let prediction = pzip::Setup::new(input, shape, weights);
    prediction.write_bytes(output);
}

#[test]
fn compression_using_last_value_all_once() {
    let input = String::from("/home/ucyo/rust/pzip/data/pres.1-90-181-361.f32.bin");
    let output = String::from("/home/ucyo/rust/pzip/data/pres.1-90-181-361.f32.bin.lastvalue.pred");
    let shape = pzip::Shape {
        z: 90,
        y: 181,
        x: 361,
    };
    let weights = [(1, pzip::Position { z: 0, y: 0, x: 1 })];

    let prediction = pzip::Setup::new(input, shape, weights);
    prediction.write(output);
}
