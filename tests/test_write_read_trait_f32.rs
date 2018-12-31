/// First walking skeleton for testing
///

use pzip::testing::CompressedFile;
use pzip::testing::FileToBeCompressed;


#[test]
fn trait_write_f32_to_file() {
    let filename = String::from("/tmp/output.raw");
    let v = 213.232_f32;

    let mut sink: pzip::testing::Sink<f32> = pzip::testing::Sink::new(&filename);
    sink.put(v).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::testing::Source<f32> = pzip::testing::Source::new(&filename);
    let value = source.get();

    std::fs::remove_file(&filename).expect("Error");
    assert_eq!(v, value)
}

#[test]
fn trait_write_f32s_to_file() {
    let filename = String::from("/tmp/output.raw");
    let values = [213.236_f32, 839.9482_f32, 94.32_f32];

    let mut sink: pzip::testing::Sink<f32> = pzip::testing::Sink::new(&filename);
    sink.put_all(&values).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::testing::Source<f32> = pzip::testing::Source::new(&filename);
    source.load().expect("Load unsuccessfull");

    for i in 0..3 {
        assert_eq!(source.ix(i), &values[i])
    }
    std::fs::remove_file(&filename).expect("Error");
}



#[test]
fn trait_read_first_f32_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::testing::Source<f32> = pzip::testing::Source::new(&filename);
    let first = source.get();
    assert_eq!(first, 160.57284545898_f32)
}


#[test]
fn trait_read_f32_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::testing::Source<f32> = pzip::testing::Source::new(&filename);
    source.load().expect("Error loading the data");

    let expected = [160.57284545898_f32,
                    160.47055053711_f32,
                    160.36930847168_f32];
    for i in 0..3 {
        assert_eq!(source.ix(i), &expected[i]);
    }
}
