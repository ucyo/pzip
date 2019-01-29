/// Example on how to use neighbours

use pzip::position::{Position};
use pzip::testing::{Source, FileToBeCompressed};
use pzip::traversal::{GeneratorIteratorAdapter, neighbours, Traversal};

fn main() {
    let input = String::from("/home/ucyo/rust/pzip/data/emac.ml.tm1.f32.little.5x90x160x320_0.raw");
    let shape = pzip::Shape {
        z: 90,
        y: 160,
        x: 320,
    };
    let information = vec![
        {Position { z: 0, y: 0, x: 1 }},
        {Position { z: 1, y: 0, x: 1 }},
        {Position { z: 0, y: 1, x: 1 }}
    ];


    let mut source: Source<f32> = Source::new(&input);
    let traversal = Traversal::new(shape.z, shape.y, shape.x);
    let nbytes = source.load().unwrap();

    let values = GeneratorIteratorAdapter(neighbours(traversal, &source.data, &information));
    for environ in values {
        println!("{:?}", environ)
    }
}
