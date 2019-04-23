//! Encoding of residual files

use bit_vec::BitVec;

pub struct EncodingContext {
    direction: BitVec,
}

trait EncoderTrait {
    fn encode(&self, residual: &Vec<u32>, ctx: EncodingContext) -> Vec<u8>;
    // fn create_header(data: Vec<u32>) -> Vec<u8>;
    // fn decode(data: Vec<u8>) -> Vec<u32>;
}

pub enum Encoder {
    Untouched,
    HuffmanLZC,
    AdjustedHuffmanLZC,
    HuffmanPowerLZC,  // Done by FPZIP
    // TODO: Canonical Huffman
    // TODO: ANS LZC
    // TODO: Adaptive Huffman
    // TODO: Huffman with memory loss (adaptive with reset of counter)
    // TODO: Range Encoder LZC
    // TODO: PPM fpzip
}
