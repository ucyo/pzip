#![deprecated(since="0.1.0", note="Use transform module instead")]

pub trait Intermapping {
    fn to_u32(from: f32) -> u32;
    fn from_u32(from: u32) -> f32;
    fn to_u64(from: f64) -> u64;
    fn from_u64(from: u64) -> f64;
}

pub trait Intramapping {
    fn to_new_u32(num: u32) -> u32;
    fn from_new_u32(num: u32) -> u32;
    fn to_new_u64(num: u64) -> u64;
    fn from_new_u64(num: u64) -> u64;
}

pub struct ClassicGray {}
impl Intramapping for ClassicGray {
    fn to_new_u32(num: u32) -> u32 {
        num ^ (num >> 1)
    }
    fn from_new_u32(num: u32) -> u32 {
        let mut number = num;
        let mut mask = number >> 1;
        while mask != 0 {
            number = number ^ mask;
            mask = mask >> 1;
        }
        num
    }
    fn to_new_u64(num: u64) -> u64 {
        num ^ (num >> 1)
    }
    fn from_new_u64(num: u64) -> u64 {
        let mut number = num;
        let mut mask = number >> 1;
        while mask != 0 {
            number = number ^ mask;
            mask = mask >> 1;
        }
        num
    }
}

pub struct Untouched {}
impl Intramapping for Untouched {
    fn to_new_u32(num: u32) -> u32 {
        num
    }
    fn from_new_u32(num: u32) -> u32 {
        num
    }
    fn to_new_u64(num: u64) -> u64 {
        num
    }
    fn from_new_u64(num: u64) -> u64 {
        num
    }
}

pub struct Raw {}
impl Intermapping for Raw {
    fn to_u32(from: f32) -> u32 {
        unsafe { std::mem::transmute::<f32, u32>(from) }
    }
    fn from_u32(from: u32) -> f32 {
        unsafe { std::mem::transmute::<u32, f32>(from) }
    }
    fn to_u64(from: f64) -> u64 {
        unsafe { std::mem::transmute::<f64, u64>(from) }
    }
    fn from_u64(from: u64) -> f64 {
        unsafe { std::mem::transmute::<u64, f64>(from) }
    }
}

pub struct Ordered {}
impl Intermapping for Ordered {
    fn to_u32(from: f32) -> u32 {
        let mut result = unsafe { std::mem::transmute::<f32, u32>(from) };
        result = if from < 0f32 {
            !result
        } else {
            result | (1 << 31)
        };

        // If a value falls within this if-clause, it would
        // be mapped to a NaN value. To prevent this and to accept
        // arbitary u32 values, the result needs to be negated. Preliminary
        // results show that this does not have any side-effects (espacially
        // for transformations done from proper f32 source)
        if result > 0x7F80_0000 && from.is_nan() {
            return !result;
        }
        result
    }
    fn from_u32(from: u32) -> f32 {
        if from < (1 << 31) {
            return unsafe { std::mem::transmute::<u32, f32>(!from) };
        } else {
            return unsafe { std::mem::transmute::<u32, f32>(from - (1 << 31)) };
        }
    }
    fn to_u64(from: f64) -> u64 {
        let mut result = unsafe { std::mem::transmute::<f64, u64>(from) };
        result = if from < 0f64 {
            !result
        } else {
            result | (1 << 63)
        };

        // see comment @to_u32()
        if result > 0x7FC0_0000_0000_0000 && from.is_nan() {
            return !result;
        }
        result
    }
    fn from_u64(from: u64) -> f64 {
        if from < (1 << 63) {
            return unsafe { std::mem::transmute::<u64, f64>(!from) };
        } else {
            return unsafe { std::mem::transmute::<u64, f64>(from - (1 << 63)) };
        }
    }
}

pub trait ByteMapping {
    fn to_u8(num: u8) -> u8;
    fn from_u8(num: u8) -> u8;
}

pub struct MonotonicGrayBytes {}
impl ByteMapping for MonotonicGrayBytes {
    fn to_u8(num: u8) -> u8 {
        hardcoded_map::MAP[&num]
    }
    fn from_u8(num: u8) -> u8 {
        hardcoded_map::REVMAP[&num]
    }
}

impl ByteMapping for Untouched {
    fn to_u8(num: u8) -> u8 {
        num
    }
    fn from_u8(num: u8) -> u8 {
        num
    }
}

pub trait CompactTrait {
    fn compact_u32(data: Vec<u32>) -> Vec<u32>;
}

pub struct NoLZCCompact {}
impl CompactTrait for NoLZCCompact {
    fn compact_u32(data: Vec<u32>) -> Vec<u32> {
        let bits = 32;
        let mut result: Vec<u32> = Vec::new();

        let mut remaining = bits;
        let mut tmp = 0u32;
        for val in data {
            let size = bits - val.leading_zeros();
            if size <= remaining {
                tmp = add(val, &remaining, &tmp);
                remaining -= size;
            } else {
                let (a, b, zeros) = split(val, &remaining);

                tmp = add(a, &remaining, &tmp);
                result.push(tmp);
                // println!("   pushed: {:032b}", tmp);
                tmp = if b != 0 {
                    b << b.leading_zeros() - zeros
                } else {
                    0u32
                };
                remaining = bits - (bits - b.leading_zeros()) - zeros;
            }
        }
        result.push(tmp);
        // println!("last push: {:032b}",tmp);
        result
    }
}

impl CompactTrait for Untouched {
    fn compact_u32(data: Vec<u32>) -> Vec<u32> {
        data
    }
}

fn add(value: u32, remaining: &u32, into: &u32) -> u32 {
    let shift = remaining - (32 - value.leading_zeros());
    let tmp = value << shift;
    into + tmp
}

fn split(val: u32, pos: &u32) -> (u32, u32, u32) {
    let valuelength = 32 - val.leading_zeros();
    let a = val >> (valuelength - pos);
    let b = val & 2u32.pow(32 - val.leading_zeros() - pos) - 1;
    let zeros = if b != 0 {
        32 - val.leading_zeros() - pos - (32 - b.leading_zeros())
    } else {
        32 - val.leading_zeros() - pos - (32 - b.leading_zeros())
    };

    (a, b, zeros)
}

mod hardcoded_map {
    use lazy_static::lazy_static;
    // use std::collections::HashMap;
    use hashbrown::HashMap;

    lazy_static! {
        pub static ref MAP: HashMap<u8, u8> = {
            let map: HashMap<_, _> = vec![
                (0_u8, 0_u8),(21_u8, 162_u8),(42_u8, 34_u8),(63_u8, 25_u8),
                (1_u8, 1_u8),(22_u8, 130_u8),(43_u8, 35_u8),(64_u8, 27_u8),
                (2_u8, 3_u8),(23_u8, 131_u8),(44_u8, 33_u8),(65_u8, 19_u8),
                (3_u8, 2_u8),(24_u8, 129_u8),(45_u8, 49_u8),(66_u8, 51_u8),
                (4_u8, 6_u8),(25_u8, 133_u8),(46_u8, 17_u8),(67_u8, 50_u8),
                (5_u8, 4_u8),(26_u8, 132_u8),(47_u8, 21_u8),(68_u8, 54_u8),
                (6_u8, 12_u8),(27_u8, 196_u8),(48_u8, 20_u8),(69_u8, 52_u8),
                (7_u8, 8_u8),(28_u8, 68_u8),(49_u8, 22_u8),(70_u8, 53_u8),
                (8_u8, 24_u8),(29_u8, 70_u8),(50_u8, 18_u8),(71_u8, 37_u8),
                (9_u8, 16_u8),(30_u8, 66_u8),(51_u8, 26_u8),(72_u8, 45_u8),
                (10_u8, 48_u8),(31_u8, 67_u8),(52_u8, 10_u8),(73_u8, 41_u8),
                (11_u8, 32_u8),(32_u8, 65_u8),(53_u8, 11_u8),(74_u8, 43_u8),
                (12_u8, 96_u8),(33_u8, 81_u8),(54_u8, 9_u8),(75_u8, 42_u8),
                (13_u8, 64_u8),(34_u8, 80_u8),(55_u8, 13_u8),(76_u8, 58_u8),
                (14_u8, 192_u8),(35_u8, 88_u8),(56_u8, 5_u8),(77_u8, 56_u8),
                (15_u8, 128_u8),(36_u8, 72_u8),(57_u8, 7_u8),(78_u8, 120_u8),
                (16_u8, 144_u8),(37_u8, 104_u8),(58_u8, 15_u8),(79_u8, 112_u8),
                (17_u8, 152_u8),(38_u8, 40_u8),(59_u8, 14_u8),(80_u8, 113_u8),
                (18_u8, 136_u8),(39_u8, 44_u8),(60_u8, 30_u8),(81_u8, 97_u8),
                (19_u8, 168_u8),(40_u8, 36_u8),(61_u8, 28_u8),(82_u8, 105_u8),
                (20_u8, 160_u8),(41_u8, 38_u8),(62_u8, 29_u8),(83_u8, 73_u8),

                (84_u8, 75_u8),(110_u8, 232_u8),(136_u8, 156_u8),(162_u8, 226_u8),
                (85_u8, 74_u8),(111_u8, 200_u8),(137_u8, 188_u8),(163_u8, 227_u8),
                (86_u8, 90_u8),(112_u8, 204_u8),(138_u8, 180_u8),(164_u8, 163_u8),
                (87_u8, 82_u8),(113_u8, 140_u8),(139_u8, 182_u8),(165_u8, 167_u8),
                (88_u8, 114_u8),(114_u8, 141_u8),(140_u8, 150_u8),(166_u8, 135_u8),
                (89_u8, 98_u8),(115_u8, 137_u8),(141_u8, 214_u8),(167_u8, 199_u8),
                (90_u8, 102_u8),(116_u8, 139_u8),(142_u8, 210_u8),(168_u8, 71_u8),
                (91_u8, 100_u8),(117_u8, 138_u8),(143_u8, 218_u8),(169_u8, 79_u8),
                (92_u8, 108_u8),(118_u8, 154_u8),(144_u8, 216_u8),(170_u8, 77_u8),
                (93_u8, 76_u8),(119_u8, 146_u8),(145_u8, 248_u8),(171_u8, 109_u8),
                (94_u8, 92_u8),(120_u8, 178_u8),(146_u8, 184_u8),(172_u8, 101_u8),
                (95_u8, 84_u8),(121_u8, 176_u8),(147_u8, 186_u8),(173_u8, 117_u8),
                (96_u8, 85_u8),(122_u8, 240_u8),(148_u8, 170_u8),(174_u8, 116_u8),
                (97_u8, 69_u8),(123_u8, 208_u8),(149_u8, 174_u8),(175_u8, 118_u8),
                (98_u8, 197_u8),(124_u8, 212_u8),(150_u8, 142_u8),(176_u8, 86_u8),
                (99_u8, 193_u8),(125_u8, 148_u8),(151_u8, 206_u8),(177_u8, 94_u8),
                (100_u8, 195_u8),(126_u8, 149_u8),(152_u8, 202_u8),(178_u8, 78_u8),
                (101_u8, 194_u8),(127_u8, 145_u8),(153_u8, 203_u8),(179_u8, 110_u8),
                (102_u8, 198_u8),(128_u8, 209_u8),(154_u8, 201_u8),(180_u8, 106_u8),
                (103_u8, 134_u8),(129_u8, 241_u8),(155_u8, 233_u8),(181_u8, 107_u8),
                (104_u8, 166_u8),(130_u8, 177_u8),(156_u8, 169_u8),(182_u8, 99_u8),
                (105_u8, 164_u8),(131_u8, 179_u8),(157_u8, 173_u8),(183_u8, 115_u8),
                (106_u8, 165_u8),(132_u8, 147_u8),(158_u8, 172_u8),(184_u8, 83_u8),
                (107_u8, 161_u8),(133_u8, 155_u8),(159_u8, 236_u8),(185_u8, 91_u8),
                (108_u8, 225_u8),(134_u8, 153_u8),(160_u8, 228_u8),(186_u8, 89_u8),
                (109_u8, 224_u8),(135_u8, 157_u8),(161_u8, 230_u8),(187_u8, 121_u8),

                (188_u8, 57_u8),(222_u8, 246_u8),(205_u8, 93_u8),(239_u8, 213_u8),
                (189_u8, 61_u8),(223_u8, 244_u8),(206_u8, 95_u8),(240_u8, 221_u8),
                (190_u8, 60_u8),(224_u8, 252_u8),(207_u8, 87_u8),(241_u8, 253_u8),
                (191_u8, 62_u8),(225_u8, 220_u8),(208_u8, 119_u8),(242_u8, 245_u8),
                (192_u8, 46_u8),(226_u8, 222_u8),(209_u8, 103_u8),(243_u8, 247_u8),
                (193_u8, 47_u8),(227_u8, 158_u8),(210_u8, 231_u8),(244_u8, 243_u8),
                (194_u8, 39_u8),(228_u8, 159_u8),(211_u8, 229_u8),(245_u8, 251_u8),
                (195_u8, 55_u8),(229_u8, 151_u8),(212_u8, 237_u8),(246_u8, 187_u8),
                (196_u8, 23_u8),(230_u8, 183_u8),(213_u8, 205_u8),(247_u8, 191_u8),
                (197_u8, 31_u8),(231_u8, 181_u8),(214_u8, 207_u8),(248_u8, 190_u8),
                (198_u8, 63_u8),(232_u8, 189_u8),(215_u8, 143_u8),(249_u8, 254_u8),
                (199_u8, 59_u8),(233_u8, 185_u8),(216_u8, 175_u8),(250_u8, 238_u8),
                (200_u8, 123_u8),(234_u8, 249_u8),(217_u8, 171_u8),(251_u8, 239_u8),
                (201_u8, 122_u8),(235_u8, 217_u8),(218_u8, 235_u8),(252_u8, 111_u8),
                (202_u8, 126_u8),(236_u8, 219_u8),(219_u8, 234_u8),(253_u8, 127_u8),
                (203_u8, 124_u8),(237_u8, 211_u8),(220_u8, 250_u8),(254_u8, 255_u8),
                (204_u8, 125_u8),(238_u8, 215_u8),(221_u8, 242_u8),(255_u8, 223_u8),
            ]
            .into_iter()
            .collect();
            map
        };
    }

    lazy_static! {
        pub static ref REVMAP: HashMap<u8, u8> = {
            let revmap: HashMap<_, _> = vec![
                (0_u8, 0_u8),(21_u8, 47_u8),(92_u8, 94_u8),(214_u8, 141_u8),
                (1_u8, 1_u8),(20_u8, 48_u8),(84_u8, 95_u8),(210_u8, 142_u8),
                (3_u8, 2_u8),(22_u8, 49_u8),(85_u8, 96_u8),(218_u8, 143_u8),
                (2_u8, 3_u8),(18_u8, 50_u8),(69_u8, 97_u8),(216_u8, 144_u8),
                (6_u8, 4_u8),(26_u8, 51_u8),(197_u8, 98_u8),(248_u8, 145_u8),
                (4_u8, 5_u8),(10_u8, 52_u8),(193_u8, 99_u8),(184_u8, 146_u8),
                (12_u8, 6_u8),(11_u8, 53_u8),(195_u8, 100_u8),(186_u8, 147_u8),
                (8_u8, 7_u8),(9_u8, 54_u8),(194_u8, 101_u8),(170_u8, 148_u8),
                (24_u8, 8_u8),(13_u8, 55_u8),(198_u8, 102_u8),(174_u8, 149_u8),
                (16_u8, 9_u8),(5_u8, 56_u8),(134_u8, 103_u8),(142_u8, 150_u8),
                (48_u8, 10_u8),(7_u8, 57_u8),(166_u8, 104_u8),(206_u8, 151_u8),
                (32_u8, 11_u8),(15_u8, 58_u8),(164_u8, 105_u8),(202_u8, 152_u8),
                (96_u8, 12_u8),(14_u8, 59_u8),(165_u8, 106_u8),(203_u8, 153_u8),
                (64_u8, 13_u8),(30_u8, 60_u8),(161_u8, 107_u8),(201_u8, 154_u8),
                (192_u8, 14_u8),(28_u8, 61_u8),(225_u8, 108_u8),(233_u8, 155_u8),
                (128_u8, 15_u8),(29_u8, 62_u8),(224_u8, 109_u8),(169_u8, 156_u8),
                (144_u8, 16_u8),(25_u8, 63_u8),(232_u8, 110_u8),(173_u8, 157_u8),
                (152_u8, 17_u8),(27_u8, 64_u8),(200_u8, 111_u8),(172_u8, 158_u8),
                (136_u8, 18_u8),(19_u8, 65_u8),(204_u8, 112_u8),(236_u8, 159_u8),
                (168_u8, 19_u8),(51_u8, 66_u8),(140_u8, 113_u8),(228_u8, 160_u8),
                (160_u8, 20_u8),(50_u8, 67_u8),(141_u8, 114_u8),(230_u8, 161_u8),
                (162_u8, 21_u8),(54_u8, 68_u8),(137_u8, 115_u8),(226_u8, 162_u8),
                (130_u8, 22_u8),(52_u8, 69_u8),(139_u8, 116_u8),(227_u8, 163_u8),
                (131_u8, 23_u8),(53_u8, 70_u8),(138_u8, 117_u8),(163_u8, 164_u8),
                (129_u8, 24_u8),(37_u8, 71_u8),(154_u8, 118_u8),(167_u8, 165_u8),
                (133_u8, 25_u8),(45_u8, 72_u8),(146_u8, 119_u8),(135_u8, 166_u8),
                (132_u8, 26_u8),(41_u8, 73_u8),(178_u8, 120_u8),(199_u8, 167_u8),
                (196_u8, 27_u8),(43_u8, 74_u8),(176_u8, 121_u8),(71_u8, 168_u8),
                (68_u8, 28_u8),(42_u8, 75_u8),(240_u8, 122_u8),(79_u8, 169_u8),
                (70_u8, 29_u8),(58_u8, 76_u8),(208_u8, 123_u8),(77_u8, 170_u8),
                (66_u8, 30_u8),(56_u8, 77_u8),(212_u8, 124_u8),(109_u8, 171_u8),
                (67_u8, 31_u8),(120_u8, 78_u8),(148_u8, 125_u8),(101_u8, 172_u8),
                (65_u8, 32_u8),(112_u8, 79_u8),(149_u8, 126_u8),(117_u8, 173_u8),
                (81_u8, 33_u8),(113_u8, 80_u8),(145_u8, 127_u8),(116_u8, 174_u8),
                (80_u8, 34_u8),(97_u8, 81_u8),(209_u8, 128_u8),(118_u8, 175_u8),
                (88_u8, 35_u8),(105_u8, 82_u8),(241_u8, 129_u8),(86_u8, 176_u8),
                (72_u8, 36_u8),(73_u8, 83_u8),(177_u8, 130_u8),(94_u8, 177_u8),
                (104_u8, 37_u8),(75_u8, 84_u8),(179_u8, 131_u8),(78_u8, 178_u8),
                (40_u8, 38_u8),(74_u8, 85_u8),(147_u8, 132_u8),(110_u8, 179_u8),
                (44_u8, 39_u8),(90_u8, 86_u8),(155_u8, 133_u8),(106_u8, 180_u8),
                (36_u8, 40_u8),(82_u8, 87_u8),(153_u8, 134_u8),(107_u8, 181_u8),
                (38_u8, 41_u8),(114_u8, 88_u8),(157_u8, 135_u8),(99_u8, 182_u8),
                (34_u8, 42_u8),(98_u8, 89_u8),(156_u8, 136_u8),(115_u8, 183_u8),
                (35_u8, 43_u8),(102_u8, 90_u8),(188_u8, 137_u8),(83_u8, 184_u8),
                (33_u8, 44_u8),(100_u8, 91_u8),(180_u8, 138_u8),(91_u8, 185_u8),
                (49_u8, 45_u8),(108_u8, 92_u8),(182_u8, 139_u8),(89_u8, 186_u8),
                (17_u8, 46_u8),(76_u8, 93_u8),(150_u8, 140_u8),(121_u8, 187_u8),

                (57_u8, 188_u8),(103_u8, 209_u8),(183_u8, 230_u8),
                (61_u8, 189_u8),(231_u8, 210_u8),(181_u8, 231_u8),
                (60_u8, 190_u8),(229_u8, 211_u8),(189_u8, 232_u8),
                (62_u8, 191_u8),(237_u8, 212_u8),(185_u8, 233_u8),
                (46_u8, 192_u8),(205_u8, 213_u8),(249_u8, 234_u8),
                (47_u8, 193_u8),(207_u8, 214_u8),(217_u8, 235_u8),
                (39_u8, 194_u8),(143_u8, 215_u8),(219_u8, 236_u8),
                (55_u8, 195_u8),(175_u8, 216_u8),(211_u8, 237_u8),
                (23_u8, 196_u8),(171_u8, 217_u8),(215_u8, 238_u8),
                (31_u8, 197_u8),(235_u8, 218_u8),(213_u8, 239_u8),
                (63_u8, 198_u8),(234_u8, 219_u8),(221_u8, 240_u8),
                (59_u8, 199_u8),(250_u8, 220_u8),(253_u8, 241_u8),
                (123_u8, 200_u8),(242_u8, 221_u8),(245_u8, 242_u8),
                (122_u8, 201_u8),(246_u8, 222_u8),(247_u8, 243_u8),
                (126_u8, 202_u8),(244_u8, 223_u8),(243_u8, 244_u8),
                (124_u8, 203_u8),(252_u8, 224_u8),(251_u8, 245_u8),
                (125_u8, 204_u8),(220_u8, 225_u8),(187_u8, 246_u8),
                (93_u8, 205_u8),(222_u8, 226_u8),(191_u8, 247_u8),
                (95_u8, 206_u8),(158_u8, 227_u8),(190_u8, 248_u8),
                (87_u8, 207_u8),(159_u8, 228_u8),(254_u8, 249_u8),
                (119_u8, 208_u8),(151_u8, 229_u8),(238_u8, 250_u8),
                (239_u8, 251_u8),(111_u8, 252_u8),(127_u8, 253_u8),
                (255_u8, 254_u8),(223_u8, 255_u8),
            ]
            .into_iter()
            .collect();
            revmap
        };
    }

}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn correct_mapping() {
        let values = vec![33832.8f32, -33832.8f32, 1f32, -1f32, 0f32];

        let expected_ordered = vec![0xc70428cd, !0xc70428cd, 0xbf800000, !0xbf800000, 0x80000000];

        for (v, e) in values.iter().zip(expected_ordered) {
            assert_eq!(Ordered::to_u32(*v), e);
        }
    }

    #[test]
    fn transform_f32_to_u32() {
        let tests = vec![
            32f32,
            85.2934f32,
            8393.42f32,
            -53f32,
            -0f32,
            0f32,
            std::f32::INFINITY,
            std::f32::NAN,
        ];
        for val in tests {
            if val.is_nan() {
                assert!(Raw::from_u32(Raw::to_u32(val)).is_nan());
                assert!(Ordered::from_u32(Ordered::to_u32(val)).is_nan());
                continue;
            }
            assert_eq!(Raw::from_u32(Raw::to_u32(val)), val);
            assert_eq!(Ordered::from_u32(Ordered::to_u32(val)), val);
        }
    }

    #[test]
    fn transform_f64_to_u64() {
        let tests = vec![
            32.3352,
            85.2934,
            8393.42,
            0.0,
            -0.0,
            std::f64::INFINITY,
            std::f64::NAN,
        ];
        for val in tests {
            if val.is_nan() {
                assert!(Raw::from_u64(Raw::to_u64(val)).is_nan());
                assert!(Ordered::from_u64(Ordered::to_u64(val)).is_nan());
                continue;
            }
            assert_eq!(Raw::from_u64(Raw::to_u64(val)), val);
            assert_eq!(Ordered::from_u64(Ordered::to_u64(val)), val);
        }
    }

    #[test]
    fn transform_u32_to_f32() {
        let tests = vec![
            32u32,
            85u32,
            293u32,
            83u32,
            9342u32,
            1u32,
            0u32,
            984744474u32,
        ];
        for val in tests {
            assert_eq!(Raw::to_u32(Raw::from_u32(val)), val);
            assert_eq!(Ordered::to_u32(Ordered::from_u32(val)), val);
        }
    }

    #[test]
    fn transform_u64_to_f64() {
        let tests = vec![
            32u64,
            3352u64,
            8u64,
            529u64,
            34u64,
            83u64,
            93u64,
            42u64,
            984742123298754474u64,
        ];
        for val in tests {
            assert_eq!(Raw::to_u64(Raw::from_u64(val)), val);
            assert_eq!(Ordered::to_u64(Ordered::from_u64(val)), val);
        }
    }

    #[test]
    fn classic_gray_codes() {
        let input: Vec<u32> = vec![15, 5, 6, 3, 1];
        let expected: Vec<u32> = vec![8, 7, 5, 2, 1];
        let result: Vec<u32> = input.iter().map(|x| ClassicGray::to_new_u32(*x)).collect();

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_eq!(r, e)
        }
    }

    #[test]
    fn untouched_intramapping() {
        let input: Vec<u32> = vec![15, 5, 6, 3, 1];
        let result: Vec<u32> = input.iter().map(|x| Untouched::to_new_u32(*x)).collect();

        for i in 0..input.len() {
            assert_eq!(input[i], result[i])
        }
    }

    #[test]
    fn test_monotonic_gray_to() {
        let input = vec![34u8];
        let output = vec![80_u8];

        for (a, b) in input.iter().zip(output.iter()) {
            assert_eq!(MonotonicGrayBytes::to_u8(*a), *b)
        }
    }
    #[test]
    fn test_monotonic_gray_from() {
        let input = vec![34u8];
        let output = vec![80_u8];

        for (a, b) in input.iter().zip(output.iter()) {
            assert_eq!(MonotonicGrayBytes::from_u8(*b), *a)
        }
    }
    #[test]
    fn test_monotonic_gray_consistent() {
        let input = vec![34u8];

        for a in input.iter() {
            assert_eq!(
                MonotonicGrayBytes::from_u8(MonotonicGrayBytes::to_u8(*a)),
                *a
            )
        }
    }
    #[test]
    fn test_monotonic_gray_consistent_rev() {
        let input = vec![34u8];

        for a in input.iter() {
            assert_eq!(
                MonotonicGrayBytes::to_u8(MonotonicGrayBytes::from_u8(*a)),
                *a
            )
        }
    }

    #[test]
    fn test_hashmap_size() {
        assert_eq!(hardcoded_map::MAP.len(), 256);
        assert_eq!(hardcoded_map::REVMAP.len(), 256);
    }

}
