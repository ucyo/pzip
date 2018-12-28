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
fn write_bytes_to_file() {
    let filename = String::from("/tmp/output.raw");
    let values = [123_u8, 193_u8, 201_u8];

    let mut sink: pzip::Sink<u8> = pzip::Sink::new(&filename).expect("Error");
    sink.put_all(&values).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::Source<u8> = pzip::Source::new(&filename).expect("Error");
    source.load().expect("Load unsuccessfull");

    for i in 0..3 {
        assert_eq!(source.ix(i), &values[i])
    }
    std::fs::remove_file(&filename).expect("Error");
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

fn write_f32s_to_file() {
    let filename = String::from("/tmp/output.raw");
    let values = [213.241_f32, 8392.9482_f32, 94.32_f32];

    let mut sink: pzip::Sink<f32> = pzip::Sink::new(&filename).expect("Errpr");
    sink.put_all(&values).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::Source<f32> = pzip::Source::new(&filename).expect("Error");
    source.load().expect("Load unsuccessfull");

    for i in 0..3 {
        assert_eq!(source.ix(i), &values[i])
    }
    std::fs::remove_file(&filename).expect("Error");
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

// #[test]
// fn write_f64s_to_file() {
//     let filename = String::from("/tmp/output.raw");
//     let values = [324234.423234_f64, 9291.822_f64, 1.23131_f64];

//     let mut sink: pzip::Sink<f64> = pzip::Sink::new(&filename).expect("Error");
//     sink.put_all(&values).expect("Writing unsuccessfull");
//     sink.flush().expect("Writing unsuccessfull");

//     let mut source: pzip::Source<f64> = pzip::Source::new(&filename).expect("Error");
//     source.load().expect("Load unsuccessfull");

//     for i in 0..3 {
//         assert_eq!(source.ix(i), &values[i])
//     }
//     std::fs::remove_file(&filename).expect("Error");
// }
