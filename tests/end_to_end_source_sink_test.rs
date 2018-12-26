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


#[test]
fn read_f64_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::Source<f64> = pzip::Source::new(filename).expect("Error");
    source.load().expect("Error loading the data");

    let expected = [2.318024477526355e+15_f64, 2.2897421178755255e+15_f64,
                    2.262535647532541e+15_f64];
    for i in 0..3 {
        assert_eq!(source.ix(i), &expected[i]);
    }
}

#[test]
fn read_f32_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::Source<f32> = pzip::Source::new(filename).expect("Error");
    source.load().expect("Error loading the data");

    let expected = [160.57284545898_f32,
                    160.47055053711_f32,
                    160.36930847168_f32];
    for i in 0..3 {
        assert_eq!(source.ix(i), &expected[i]);
    }
}

#[test]
fn read_bytes_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::Source<u8> = pzip::Source::new(filename).expect("Error");
    source.load().expect("Error loading the data");

    let expected = [166_u8, 146_u8, 32_u8];
    for i in 0..3 {
        assert_eq!(source.ix(i), &expected[i]);
    }
}

#[test]
fn write_byte_to_file() {
    let filename = String::from("/tmp/output.raw");
    let v = 213_u8;

    let mut sink: pzip::Sink<u8> = pzip::Sink::new(filename.clone()).expect("Error");
    sink.put(v).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::Source<u8> = pzip::Source::new(filename).expect("Error");
    let value = source.get();

    std::fs::remove_file(String::from("/tmp/output.raw")).expect("Error");
    assert_eq!(v, value)
}
