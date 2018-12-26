/// First walking skeleton for testing
///

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
