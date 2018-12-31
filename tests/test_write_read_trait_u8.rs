/// First walking skeleton for testing
///

use pzip::testing::CompressedFile;
use pzip::testing::FileToBeCompressed;


#[test]
fn trait_write_byte_to_file() {

    let filename = String::from("/tmp/output.raw");
    let v = 213_u8;

    let mut sink: pzip::testing::Sink<u8> = pzip::testing::Sink::new(&filename);
    sink.put(v).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::testing::Source<u8> = pzip::testing::Source::new(&filename);
    let value = source.get();

    std::fs::remove_file(&filename).expect("Error");
    assert_eq!(v, value)
}

#[test]
fn trait_write_bytes_to_file() {
    let filename = String::from("/tmp/output.raw");
    let values = [123_u8, 193_u8, 201_u8];

    let mut sink: pzip::testing::Sink<u8> = pzip::testing::Sink::new(&filename);
    sink.put_all(&values).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::testing::Source<u8> = pzip::testing::Source::new(&filename);
    source.load().expect("Load unsuccessfull");

    for i in 0..3 {
        assert_eq!(source.ix(i), &values[i])
    }
    std::fs::remove_file(&filename).expect("Error");
}


#[test]
fn trait_read_first_byte_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::testing::Source<u8> = pzip::testing::Source::new(&filename);
    let first = source.get();
    assert_eq!(first, 166u8)
}

#[test]
fn trait_read_bytes_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::testing::Source<u8> = pzip::testing::Source::new(&filename);
    source.load().expect("Error loading the data");

    let expected = [166_u8, 146_u8, 32_u8];
    for i in 0..3 {
        assert_eq!(source.ix(i), &expected[i]);
    }
}
