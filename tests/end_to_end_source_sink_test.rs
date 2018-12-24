/// First walking skeleton for testing
///

extern crate pzip;

#[test]

fn read_from_stdin_to_memory_and_write_to_stdout() {
    let source = pzip::Source::new("-".to_string());
    let sink   = pzip::Sink::new("-".to_string());

    source.memory_read().unwrap();
    source.write(&sink).unwrap();
}
