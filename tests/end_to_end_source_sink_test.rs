/// First walking skeleton for testing
///

#[test]
fn read_first_byte_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::Source<u8> = pzip::Source::new(filename).expect("Error");
    let first = source.get();
    assert_eq!(first, 166u8)
}

#[test]
fn read_first_f32_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::Source<f32> = pzip::Source::new(filename).expect("Error");
    let first = source.get();
    assert_eq!(first, 160.57284545898_f32)
}

#[test]
fn read_first_f64_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::Source<f64> = pzip::Source::new(filename).expect("Error");
    let first = source.get();
    assert_eq!(first, 2.318024477526355e+15_f64)
}
