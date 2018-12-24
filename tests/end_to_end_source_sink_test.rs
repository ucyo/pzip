/// First walking skeleton for testing
///

extern crate pzip;

#[test]

fn read_from_stdin_to_memory_and_write_to_stdout() {
    let source = pzip::Source::New('-');
    let sink   = pzip::Sink::New('-');

    source.memory_read();
    source.write(&sink);
}
