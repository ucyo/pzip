/// Enum implementations of the following structs:
///
/// - Untouched (Inter, Intra, Byte, Compact)
/// - Ordered (Inter)
/// - Gray (Intra)
/// - MonoGray (Byte)
pub mod arrays;

pub trait InterMapping {
    fn to_u32(&self, from: f32) -> u32;
    fn from_u32(&self, from: u32) -> f32;
    fn to_u64(&self, from: f64) -> u64;
    fn from_u64(&self, from: u64) -> f64;
}

pub trait IntraMapping {
    fn to_new_u32(&self, num: u32) -> u32;
    fn from_new_u32(&self, num: u32) -> u32;
    fn to_new_u64(&self, num: u64) -> u64;
    fn from_new_u64(&self, num: u64) -> u64;
}

pub trait CompactMapping {
    fn compact_u32(&self, data: Vec<u32>) -> Vec<u32>;
}

pub trait ByteMapping {
    fn to_u8(&self, num: u8) -> u8;
    fn from_u8(&self, num: u8) -> u8;
}

pub enum Inter {
    Untouched,
    Ordered,
}

pub enum Intra {
    Untouched,
    Gray,
}

pub enum Byte {
    Untouched,
    MonoGray,
}

pub enum Compact {
    Untouched,
    NoLZC,
}

impl IntraMapping for Intra {
    fn to_new_u32(&self, num: u32) -> u32 {
        match self {
            Intra::Untouched => num,
            Intra::Gray => num ^ (num >> 1),
        }
    }
    fn from_new_u32(&self, num: u32) -> u32 {
        match self {
            Intra::Untouched => num,
            Intra::Gray => {
                let mut number = num;
                let mut mask = number >> 1;
                while mask != 0 {
                    number = number ^ mask;
                    mask = mask >> 1;
                }
                num
            }
        }
    }
    fn to_new_u64(&self, num: u64) -> u64 {
        match self {
            Intra::Untouched => num,
            Intra::Gray => num ^ (num >> 1),
        }
    }
    fn from_new_u64(&self, num: u64) -> u64 {
        match self {
            Intra::Untouched => num,
            Intra::Gray => {
                let mut number = num;
                let mut mask = number >> 1;
                while mask != 0 {
                    number = number ^ mask;
                    mask = mask >> 1;
                }
                num
            }
        }
    }
}

impl InterMapping for Inter {
    fn to_u32(&self, from: f32) -> u32 {
        match self {
            Inter::Untouched => unsafe { std::mem::transmute::<f32, u32>(from) },
            Inter::Ordered => {
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
        }
    }
    fn from_u32(&self, from: u32) -> f32 {
        match self {
            Inter::Untouched => unsafe { std::mem::transmute::<u32, f32>(from) },
            Inter::Ordered => {
                if from < (1 << 31) {
                    return unsafe { std::mem::transmute::<u32, f32>(!from) };
                } else {
                    return unsafe { std::mem::transmute::<u32, f32>(from - (1 << 31)) };
                }
            }
        }
    }
    fn to_u64(&self, from: f64) -> u64 {
        match self {
            Inter::Untouched => unsafe { std::mem::transmute::<f64, u64>(from) },
            Inter::Ordered => {
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
                return result;
            }
        }
    }
    fn from_u64(&self, from: u64) -> f64 {
        match self {
            Inter::Untouched => unsafe { std::mem::transmute::<u64, f64>(from) },
            Inter::Ordered => {
                if from < (1 << 63) {
                    return unsafe { std::mem::transmute::<u64, f64>(!from) };
                } else {
                    return unsafe { std::mem::transmute::<u64, f64>(from - (1 << 63)) };
                }
            }
        }
    }
}

impl ByteMapping for Byte {
    fn to_u8(&self, num: u8) -> u8 {
        match self {
            Byte::Untouched => num,
            Byte::MonoGray => arrays::IX_MONO[num as usize],
        }
    }
    fn from_u8(&self, num: u8) -> u8 {
        match self {
            Byte::Untouched => num,
            Byte::MonoGray => arrays::MONO_IX[num as usize],
        }
    }
}

impl CompactMapping for Compact {
    fn compact_u32(&self, data: Vec<u32>) -> Vec<u32> {
        match self {
            Compact::Untouched => data,
            Compact::NoLZC => {
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
                        tmp = if b != 0 {
                            b << b.leading_zeros() - zeros
                        } else {
                            0u32
                        };
                        remaining = bits - (bits - b.leading_zeros()) - zeros;
                    }
                }
                result.push(tmp);
                result
            }
        }
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

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn correct_mapping() {
        let values = vec![33832.8f32, -33832.8f32, 1f32, -1f32, 0f32];

        let expected_ordered = vec![0xc70428cd, !0xc70428cd, 0xbf800000, !0xbf800000, 0x80000000];

        for (v, e) in values.iter().zip(expected_ordered) {
            assert_eq!(Inter::Ordered.to_u32(*v), e);
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
                assert!(Inter::Untouched
                    .from_u32(Inter::Untouched.to_u32(val))
                    .is_nan());
                assert!(Inter::Ordered.from_u32(Inter::Ordered.to_u32(val)).is_nan());
                continue;
            }
            assert_eq!(Inter::Untouched.from_u32(Inter::Untouched.to_u32(val)), val);
            assert_eq!(Inter::Ordered.from_u32(Inter::Ordered.to_u32(val)), val);
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
                assert!(Inter::Untouched
                    .from_u64(Inter::Untouched.to_u64(val))
                    .is_nan());
                assert!(Inter::Ordered.from_u64(Inter::Ordered.to_u64(val)).is_nan());
                continue;
            }
            assert_eq!(Inter::Untouched.from_u64(Inter::Untouched.to_u64(val)), val);
            assert_eq!(Inter::Ordered.from_u64(Inter::Ordered.to_u64(val)), val);
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
            assert_eq!(Inter::Untouched.to_u32(Inter::Untouched.from_u32(val)), val);
            assert_eq!(Inter::Ordered.to_u32(Inter::Ordered.from_u32(val)), val);
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
            assert_eq!(Inter::Untouched.to_u64(Inter::Untouched.from_u64(val)), val);
            assert_eq!(Inter::Ordered.to_u64(Inter::Ordered.from_u64(val)), val);
        }
    }

    #[test]
    fn classic_gray_codes() {
        let input: Vec<u32> = vec![15, 5, 6, 3, 1];
        let expected: Vec<u32> = vec![8, 7, 5, 2, 1];
        let result: Vec<u32> = input.iter().map(|x| Intra::Gray.to_new_u32(*x)).collect();

        for (e, r) in expected.iter().zip(result.iter()) {
            assert_eq!(r, e)
        }
    }

    #[test]
    fn untouched_intramapping() {
        let input: Vec<u32> = vec![15, 5, 6, 3, 1];
        let result: Vec<u32> = input
            .iter()
            .map(|x| Intra::Untouched.to_new_u32(*x))
            .collect();

        for i in 0..input.len() {
            assert_eq!(input[i], result[i])
        }
    }

    #[test]
    fn test_monotonic_gray_to() {
        let input = vec![34u8];
        let output = vec![80_u8];

        for (a, b) in input.iter().zip(output.iter()) {
            assert_eq!(Byte::MonoGray.to_u8(*a), *b)
        }
    }
    #[test]
    fn test_monotonic_gray_from() {
        let input = vec![34u8];
        let output = vec![80_u8];

        for (a, b) in input.iter().zip(output.iter()) {
            assert_eq!(Byte::MonoGray.from_u8(*b), *a)
        }
    }
    #[test]
    fn test_monotonic_gray_consistent() {
        let input = vec![34u8];

        for a in input.iter() {
            assert_eq!(Byte::MonoGray.from_u8(Byte::MonoGray.to_u8(*a)), *a)
        }
    }
    #[test]
    fn test_monotonic_gray_consistent_rev() {
        let input = vec![34u8];

        for a in input.iter() {
            assert_eq!(Byte::MonoGray.to_u8(Byte::MonoGray.from_u8(*a)), *a)
        }
    }

    #[test]
    fn test_hashmap_size() {
        assert_eq!(arrays::IX_MONO.len(), 256);
        assert_eq!(arrays::MONO_IX.len(), 256);
    }

}
