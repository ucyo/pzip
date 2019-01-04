

pub trait Mapping {
    fn to_u32(from: f32) -> u32 {
        unsafe {std::mem::transmute::<f32, u32>(from)}
    }
    fn from_u32(from: u32) -> f32 {
        unsafe {std::mem::transmute::<u32, f32>(from)}
    }
    fn to_u64(from: f64) -> u64 {
        unsafe {std::mem::transmute::<f64, u64>(from)}
    }
    fn from_u64(from: u64) -> f64 {
        unsafe {std::mem::transmute::<u64, f64>(from)}
    }
}

pub struct Raw {}

impl Mapping for Raw {}


mod tests {
    use super::*;

    #[test]
    fn transform_f32_to_u32() {
        let tests = vec![32f32,85.2934f32,8393.42f32];
        for val in tests {
            assert_eq!(Raw::from_u32(Raw::to_u32(val)), val)
        }
    }

    #[test]
    fn transform_f64_to_u64() {
        let tests = vec![32.3352,85.2934,8393.42];
        for val in tests {
            assert_eq!(Raw::from_u64(Raw::to_u64(val)), val)
        }
    }

    #[test]
    fn transform_u32_to_f32() {
        let tests = vec![32u32,85u32,293u32,83u32,9342u32,1u32];
        for val in tests {
            assert_eq!(Raw::to_u32(Raw::from_u32(val)), val)
        }
    }

    #[test]
    fn transform_u64_to_f64() {
        let tests = vec![32u64,3352u64,8u64,529u64,34u64,83u64,93u64,42u64,];
        for val in tests {
            assert_eq!(Raw::to_u64(Raw::from_u64(val)), val)
        }
    }
}
