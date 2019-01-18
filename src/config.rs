use super::Shape;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum CodingMode {
    Encode,
    Decode,
}

#[derive(Debug, PartialEq)]
pub enum FileType {
    F32,
    F64,
}

#[derive(Debug, PartialEq)]
pub enum Predictor {
    LastValue,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config<'a> {
    pub input: &'a String,
    pub output: &'a String,
    pub coding: CodingMode,
    pub filetype: FileType,
    pub shape: Shape,
    pub predictor: Predictor,
}

pub fn parse_args<'a>(args: &'a Vec<String>) -> Config {
    let mut cli = HashMap::new();
    cli.insert("coding", 1);
    cli.insert("filetype", 2);
    cli.insert("input", 3);
    cli.insert("output", 4);
    cli.insert("z", 6);
    cli.insert("y", 7);
    cli.insert("x", 8);
    cli.insert("predictor", 10);

    let coding = if args[cli["coding"]] == "-c" {
        CodingMode::Encode
    } else if args[cli["coding"]] == "-d" {
        CodingMode::Decode
    } else {
        panic!("Wrong coding mode")
    };

    let filetype = if args[cli["filetype"]] == "-f32" {
        FileType::F32
    } else if args[cli["filetype"]] == "-f64" {
        FileType::F64
    } else {
        panic!("Wrong filetype")
    };

    let (input, output) = (&args[cli["input"]], &args[cli["output"]]);

    assert_eq!(args[5], "-s");

    let shape = Shape {
        z: args[cli["z"]].parse::<usize>().unwrap_or(1),
        y: args[cli["y"]].parse::<usize>().unwrap_or(1),
        x: args[cli["x"]].parse::<usize>().unwrap_or(1),
    };

    assert_eq!(args[9], "-p");

    let predictor = if args[cli["predictor"]] == "lv" {
        Predictor::LastValue
    } else {
        panic!("Wrong predictor, {}", args[cli["predictor"]])
    };

    Config {
        input,
        output,
        coding,
        filetype,
        shape,
        predictor,
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_to_config() {
        let t = vec![
            "pzip",
            "-c",
            "-f32",
            "inputfile.bin",
            "outputfile.bin",
            "-s",
            "321",
            "32",
            "12",
            "-p",
            "lv",
        ];
        let mut args: Vec<String> = Vec::new();
        for a in t {
            args.push(String::from(a));
        }
        let configuration = parse_args(&args);
        assert_eq!(configuration.coding, CodingMode::Encode);
        assert_eq!(configuration.filetype, FileType::F32);
        assert_eq!(
            configuration.shape,
            Shape {
                z: 321,
                y: 32,
                x: 12
            }
        );
        assert_eq!(configuration.predictor, Predictor::LastValue);
        assert_eq!(*configuration.input, args[3]);
        assert_eq!(*configuration.output, args[4]);
    }
}
