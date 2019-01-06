pub trait Mapping {
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

pub struct Raw {}

pub struct Raw {}
impl Mapping for Raw {}

pub struct Ordered {}
impl Mapping for Ordered {
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
}
