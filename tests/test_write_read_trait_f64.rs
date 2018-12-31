/// First walking skeleton for testing
///

use pzip::testing::CompressedFile;
use pzip::testing::FileToBeCompressed;


#[test]
fn trait_write_f64_to_file() {
    let filename = String::from("/tmp/output.raw");
    let v = 213.232_f64;

    let mut sink: pzip::testing::Sink<f64> = pzip::testing::Sink::new(&filename);
    sink.put(v).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::testing::Source<f64> = pzip::testing::Source::new(&filename);
    let value = source.get();

    std::fs::remove_file(&filename).expect("Error");
    assert_eq!(v, value)
}

#[test]
fn trait_write_f64s_to_file() {
    let filename = String::from("/tmp/output.raw");
    let values = [324234.423234_f64, 9291.822_f64, 1.23131_f64];

    let mut sink: pzip::testing::Sink<f64> = pzip::testing::Sink::new(&filename);
    sink.put_all(&values).expect("Writing unsuccessfull");
    sink.flush().expect("Writing unsuccessfull");

    let mut source: pzip::testing::Source<f64> = pzip::testing::Source::new(&filename);
    source.load().expect("Load unsuccessfull");

    for i in 0..3 {
        assert_eq!(source.ix(i), &values[i])
    }
    std::fs::remove_file(&filename).expect("Error");
}



#[test]
fn trait_read_first_f64_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::testing::Source<f64> = pzip::testing::Source::new(&filename);
    let first = source.get();
    assert_eq!(first, 2.318024477526355e+15_f64)
}


#[test]
fn trait_read_f64_from_file() {
    let filename = "/home/ucyo/Developments/big_files/subset.raw".to_string();

    let mut source: pzip::testing::Source<f64> = pzip::testing::Source::new(&filename);
    source.load().expect("Error loading the data");

    let expected = [2.318024477526355e+15_f64, 2.2897421178755255e+15_f64,
                    2.262535647532541e+15_f64];
    for i in 0..3 {
        assert_eq!(source.ix(i), &expected[i]);
    }
}
