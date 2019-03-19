
fn from_bitplanes_for_u8(data: &Vec<u8>) -> Vec<u8> {
    let mut results: Vec<u8> = vec![0; data.len()];
    for i in 0..(data.len() * 8) {
        let (elem, pow) = (i % 5, 8 - i / 5 - 1);
        let (elem_o, pow_o) = (i / 8, 8 - i % 8 - 1);
        if data[elem_o] & (1 << pow_o) > 0 {
            results[elem] += 1 << pow;
        }
    }
    results
}

fn to_bitplanes_for_u8(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut scanned = 0;
    let mut pos = 7;
    let mut value = 0;
    while pos >= 0 {
        let check_value = 1 << pos;
        for i in 0..data.len() {
            value <<= 1;
            value += (check_value & data[i] > 0) as u8;
            scanned += 1;
            if scanned == 8 {
                result.push(value);
                value = 0;
                scanned = 0;
            }
        }
        pos -= 1;
    }
    result
}

fn from_bitplanes_irregular(sizes: &Vec<i32>, bits: &Vec<u8>, push: &u8) -> Vec<u8> {
    let mut bitstream: Vec<bool> = vec![false; bits.len() * 8 as usize];
    let mut bitstream_ix = 0;
    for s in 0..bits.len() {
        for pow in 0..8 {
            bitstream[bitstream_ix] = bits[s] & (1 << (8 - pow - 1)) > 0;
            bitstream_ix += 1;
        }
    }
    for _ in 0..*push {
        bitstream.pop();
    }
    let mut result: Vec<u8> = vec![0; sizes.len()];

    let mut options = sizes.clone();
    bitstream_ix = 0;
    while options.iter().any(|&a| a > 0) {
        for i in 0..options.len() {
            if options[i] > 0 {
                result[i] <<= 1;
                result[i] += bitstream[bitstream_ix] as u8;
                options[i] -= 1;
                bitstream_ix += 1;
            }
        }
    }

    result
}

fn to_bitplanes_irregular(data: &Vec<u8>) -> (Vec<i32>, Vec<u8>, u8) {
    let mut block_size: Vec<i32> = data.iter().map(|a| 8 - a.leading_zeros() as i32).collect();
    let sizes = block_size.clone();
    let mut results: Vec<u8> = Vec::with_capacity(data.len()); // TODO: Can be optimised by calculating size of vector using `sizes`
    let mut ix = 0;
    let mut val = 0;
    while block_size.iter().any(|&a| a > 0) {
        block_size = block_size.iter().map(|a| a - 1).collect();
        for i in 0..block_size.len() {
            if block_size[i] >= 0 {
                val <<= 1;
                ix += 1;
                let bit = data[i] & (1 << block_size[i]) > 0;
                val += bit as u8;
                if ix == 8 {
                    results.push(val);
                    val = 0;
                    ix = 0;
                }
            }
        }
    }
    let push = val.leading_zeros() as u8;
    results.push(val << push);

    return (sizes, results, push);
}

fn from_bitplanes_irregular_u32(sizes: &Vec<i32>, bits: &Vec<u32>, push: &u8) -> Vec<u32> {
    let mut bitstream: Vec<bool> = vec![false; bits.len() * 32 as usize];
    let mut bitstream_ix = 0;
    for s in 0..bits.len() {
        for pow in 0..32 {
            bitstream[bitstream_ix] = bits[s] & (1 << (32 - pow - 1)) > 0;
            bitstream_ix += 1;
        }
    }
    for _ in 0..*push {
        bitstream.pop();
    }
    let mut result: Vec<u32> = vec![0; sizes.len()];

    let mut options = sizes.clone();
    bitstream_ix = 0;
    while options.iter().any(|&a| a > 0) {
        for i in 0..options.len() {
            if options[i] > 0 {
                result[i] <<= 1;
                result[i] += bitstream[bitstream_ix] as u32;
                options[i] -= 1;
                bitstream_ix += 1;
            }
        }
    }

    result
}

fn to_bitplanes_irregular_u32(data: &Vec<u32>) -> (Vec<i32>, Vec<u32>, u8) {
    let mut block_size: Vec<i32> = data.iter().map(|a| 32 - a.leading_zeros() as i32).collect();
    let sizes = block_size.clone();
    let mut results: Vec<u32> = Vec::with_capacity(data.len()); // TODO: Can be optimised by calculating size of vector using `sizes`
    let mut ix = 0;
    let mut val = 0;
    while block_size.iter().any(|&a| a > 0) {
        block_size = block_size.iter().map(|a| a - 1).collect();
        for i in 0..block_size.len() {
            if block_size[i] >= 0 {
                val <<= 1;
                ix += 1;
                let bit = data[i] & (1 << block_size[i]) > 0;
                val += bit as u32;
                if ix == 32 {
                    results.push(val);
                    val = 0;
                    ix = 0;
                }
            }
        }
    }
    let push = val.leading_zeros() as u8;
    results.push(val << push);

    return (sizes, results, push);
}

mod tests {
    use super::*;

    #[test]
    fn test_irregular_u8() {
        let data: Vec<u8> = vec![41, 213, 5, 234, 165];
        let val = to_bitplanes_irregular(&data);
        assert_eq!(val.0, vec![6, 8, 3, 8, 8]);
        assert_eq!(val.1, vec![250, 174, 133, 170, 128]);
        assert_eq!(val.2, 7);
        let result = from_bitplanes_irregular(&val.0, &val.1, &val.2);
        assert_eq!(data, result);
    }

    #[test]
    fn test_regular_u8() {
        let data: Vec<u8> = vec![41, 213, 5, 234, 165];
        let val = to_bitplanes_for_u8(&data);
        let result = from_bitplanes_for_u8(&val);
        assert_eq!(data, result);
    }

    #[test]
    fn test_irregular_u32() {
        let data: Vec<u32> = vec![41, 213, 5, 234, 165];
        let val = to_bitplanes_irregular_u32(&data);
        assert_eq!(val.0, vec![6, 8, 3, 8, 8]);
        assert_eq!(val.1, vec![4205741482, 2147483648]);
        assert_eq!(val.2, 31);
        let result = from_bitplanes_irregular_u32(&val.0, &val.1, &val.2);
        assert_eq!(data, result);
    }
}
