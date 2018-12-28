/// First walking skeleton for testing
///

#[test]
fn write_byte_to_file() {
    let filename = String::from("/tmp/output.raw");
    let v = 213_u8;

    let mut sink: pzip::Sink<u8> = pzip::Sink::new(&filename).expect("Error");
    sink.put(v).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::Source<u8> = pzip::Source::new(&filename).expect("Error");
    let value = source.get();

    std::fs::remove_file(&filename).expect("Error");
    assert_eq!(v, value)
}


#[test]
fn write_f32_to_file() {
    let filename = String::from("/tmp/output.raw");
    let v = 213.232_f32;

    let mut sink: pzip::Sink<f32> = pzip::Sink::new(&filename).expect("Error");
    sink.put(v).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::Source<f32> = pzip::Source::new(&filename).expect("Error");
    let value = source.get();

    std::fs::remove_file(&filename).expect("Error");
    assert_eq!(v, value)
}


#[test]
fn write_f64_to_file() {
    let filename = String::from("/tmp/output.raw");
    let v = 213.232_f64;

    let mut sink: pzip::Sink<f64> = pzip::Sink::new(&filename).expect("Error");
    sink.put(v).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::Source<f64> = pzip::Source::new(&filename).expect("Error");
    let value = source.get();

    std::fs::remove_file(&filename).expect("Error");
    assert_eq!(v, value)
}
