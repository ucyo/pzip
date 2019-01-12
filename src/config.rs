
use super::Shape;

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
pub struct Config {
    input:     String,
    output:    String,
    cmd:       CodingMode,
    filetype:  FileType,
    shape:     Shape,
    predictor: Predictor,
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_to_config(){
        let t = vec![
            "pzip", "-c", "-f32",
            "inputfile.bin", "outputfile.bin",
            "-s", "321","32","12",
            "-p", "lv",
        ];
        let mut args : Vec<String> = Vec::new();
        for a in t {
            args.push(String::from(a));
        }
        let configuration = parse_args(&args);
        assert_eq!(configuration.cmd, CodingMode::Encode);
        assert_eq!(configuration.filetype, FileType::F32);
        assert_eq!(configuration.shape, Shape{z:321, y:32, x:12});
        assert_eq!(configuration.predictor, Predictor::LastValue);
        assert_eq!(configuration.input, args[3]);
        assert_eq!(configuration.output, args[4]);
    }
}

pub fn parse_args(args: &Vec<String>) -> Config {
    println!("Reading in: {:?}", args);
    unimplemented!()
}
