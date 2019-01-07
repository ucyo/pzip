
use pzip::position::Position;


#[test]
fn compression_using_last_value_all_once() {
    let input = String::from("/home/ucyo/rust/pzip/data/pres.1-90-181-361.f64.bin");
    let output = String::from("/home/ucyo/rust/pzip/data/lastvalue.all.pred");
    let shape = pzip::Shape {
        z: 90,
        y: 181,
        x: 361,
    };
    let weights = vec![(1, Position { z: 0, y: 0, x: 1 }),];

    let prediction = pzip::Setup::new(&input, shape, weights);
    prediction.write(&output);//.unwrap();

    let origin = pzip::testing::read_first_k_f64(&input, 360);
    let outcome = pzip::testing::read_first_k_f64(&output, 360);

    for i in 1..360 {
        println!("{} {} {}", i, origin[i - 1], outcome[i]);
        assert_eq!(origin[i - 1], outcome[i]);
    }
    std::fs::remove_file(&output).expect("Error");
}
