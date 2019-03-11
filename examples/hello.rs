use pzip::gen::GeneratorIteratorAdapter;
/// Example on how to use neighbours
use pzip::position::Position;
use pzip::ptraversal::single_neighbours_grouped_no_ring;
use pzip::testing::{FileToBeCompressed, Source};

fn main() {
    let input = String::from("/home/ucyo/rust/pzip/data/emac.ml.tm1.f32.little.5x90x160x320_0.raw");
    let shape = pzip::position::Position {
        z: 90,
        y: 160,
        x: 320,
    };
    let information = vec![
        { Position { z: 0, y: 0, x: 1 } },
        { Position { z: 1, y: 0, x: 1 } },
        { Position { z: 0, y: 1, x: 1 } },
    ];

    let mut source: Source<f32> = Source::new(&input);
    let _nbytes = source.load().unwrap();

    let values = GeneratorIteratorAdapter(single_neighbours_grouped_no_ring(
        &shape,
        &information,
        &source.data,
    ));
    for environ in values {
        println!("{:?}", environ)
    }
}
