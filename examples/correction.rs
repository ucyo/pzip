pub trait CorrectionTrait {
    fn calculate_offset(&mut self, truth: &u32, pred: &u32);
    fn apply_correction(&mut self, pred: &u32) -> u32;
}


/// PreviousError correction
///
/// ## tl;dr
/// Correction of the prediction by adding the previous error by parts
/// based on the error of the previous run.
///
/// ## Description
/// The previous error is added to the current one with certain parts.
///
/// $ corr_t = corr_{t-1} * beta\parts $
/// $ fpred_t = pred_t + F * corr_t
/// with F = if pred_{t-1} < truth_{t-1} -1 else +1 $
///
#[derive(Debug)]
pub struct PreviousError {
    overshot: bool,
    offset: u32,
    beta: u32,   // relative of parts
    parts: u32,  // absolute parts [default: 100]
}

impl PreviousError {
    pub fn new() -> Self {
        PreviousError {
            overshot: false,
            offset: 0,
            beta: 1,
            parts: 1,
        }
    }
    pub fn update_beta(&mut self, val: u32) {
        self.beta = val.min(self.parts)
    }
    pub fn get_parts(&self) -> u32 {
        self.parts
    }
}

impl CorrectionTrait for PreviousError {
    fn calculate_offset(&mut self, truth: &u32, pred: &u32) {
        let diff = *truth as i64 - *pred as i64 + self.offset as i64;

        // TODO: Works for example data, but not for actual datasets
        self.overshot = diff < 0;
        self.offset = diff.abs() as u32;
    }
    fn apply_correction(&mut self, pred: &u32) -> u32 {
        let correction = (self.offset * self.beta) / self.parts;
        if self.overshot {
            if correction > *pred {
                self.offset = 0;
                return 0
            } else {
                return pred - correction
            }
        } else {
            pred + correction
        }
    }
}

use std::fmt;
impl fmt::Display for PreviousError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PreviousError {{ overshot: {}, offset: {:b} }}", self.overshot, self.offset)
    }
}


/// DeltaToPowerOf2
///
/// ## tl;dr
/// Correction of the prediction using the delta difference to the previous
/// or next power of 2.
///
/// ## Description
/// The prediction will be brought closer to the next/former power of two. Has
/// the prediction overshot the last time such that a bit flip occurred, the
/// amount of will be subtracted (gaining a bit for LZC) and further decreased
/// by beta/parts. The same is true for the other direction in case of a
/// shortcoming of the prediction.
///
#[derive(Debug)]
pub struct DeltaToPowerOf2 {
    overshot: bool,
    restricted: u32,
    beta: u32,
    parts: u32,
    delta: u32,
}

impl fmt::Display for DeltaToPowerOf2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeltaToPowerOf2 {{ overshot: {}, restricted: {}, delta: {} }}", self.overshot, self.restricted, self.delta)
    }
}

impl DeltaToPowerOf2 {
    pub fn new() -> Self {
        DeltaToPowerOf2 {
            overshot: false,
            restricted: 0,
            beta: 1,
            parts: 3,
            delta: 0,
        }
    }
    pub fn update_beta(&mut self, val: u32) {
        self.beta = val
    }
    pub fn get_parts(&self) -> u32 {
        self.parts
    }
}

impl CorrectionTrait for DeltaToPowerOf2 {
    fn calculate_offset(&mut self, truth: &u32, pred: &u32) {
        self.restricted = (truth ^ pred).leading_zeros();
        self.overshot = pred > truth;
    }
    #[allow(unused_assignments)]
    fn apply_correction(&mut self, pred: &u32) -> u32 {
        // println!("{:032b} before correction", pred);
        if self.restricted < 10 {
            return *pred;
        }
        let mut result = 0u32;
        if self.overshot {
            let delta = delta_to_former_power_of_two(*pred, self.restricted);
            result = pred - (delta * self.beta) / self.parts;
        } else {
            let delta = delta_to_next_power_of_two(*pred, self.restricted);
            result = pred + (delta * self.beta) / self.parts;
        }
        // println!("{:032b} after correction", result);
        result
    }
}

fn delta_to_next_power_of_two(val: u32, pos: u32) -> u32 {
    let pos = if pos > 0 {pos - 1} else {0};
    let val = val << pos >> pos;
    val.next_power_of_two() - val
}

fn delta_to_former_power_of_two(val: u32, pos: u32) -> u32 {
    let pos = pos - 1;
    let val = val << pos >> pos;
    val - (val.next_power_of_two() >> 1)
}

#[allow(unused_variables)]
use std::env::args;
use pzip::testing::{Source, FileToBeCompressed};
fn main() {
    let arguments: Vec<_> = args().collect();
    let filename = &arguments[1];
    let mut source : Source<f32> = Source::new(filename);
    source.load().unwrap();

    let data: Vec<u32> = source.data.iter().map(|x| x.to_bits()).collect();
    // let data = data[303568..303568+2].to_vec();
    // let (start, size) = (90300, 9000);
    // let data = data[start..start+size].to_vec();
    // let data: Vec<u32> = vec![4,5,6,8,10,9,0];
    let mut uncorrected_last_value_prediction = vec![0u32; data.len()];
    for v in 1..data.len() {
        uncorrected_last_value_prediction[v] = data[v-1];
    }

    let mut corrected_last_value_prediction : Vec<u32> = Vec::new();
    let mut pred = 0u32;
    let mut method = DeltaToPowerOf2::new();

    for value in data.iter() {
        // println!("{:032b}\n{:032b} {}", value, pred, method);
        pred = method.apply_correction(&pred);
        // println!("{:032b} new prediction", pred);
        corrected_last_value_prediction.push(pred);
        method.calculate_offset(value, &pred);  // call calculate_correction
        // println!("\n{:032b}\n{:032b}\n{:032b} Offset {}", value, pred, method.offset, method.overshot);
        pred = *value;
    }

    let lzc_data : u32 = data.iter().map(|x| x.leading_zeros()).sum();
    let lzc_uncorrected_pred: Vec<u32> = data.iter().zip(uncorrected_last_value_prediction.iter()).map(|(t,p)| (t^p).leading_zeros()).collect();
    let lzc_uncorrected_pred_sum : u32 = lzc_uncorrected_pred.iter().sum();
    let lzc_corrected_pred: Vec<u32> = data.iter().zip(corrected_last_value_prediction.iter()).map(|(t,p)| (t^p).leading_zeros()).collect();
    let lzc_corrected_pred_sum : u32 = lzc_corrected_pred.iter().sum();

    let same : u32 = lzc_corrected_pred.iter().zip(lzc_uncorrected_pred.iter()).filter(|(c,u)| c == u).fold(0, |sum, _| sum + 1 );
    let better : u32 = lzc_corrected_pred.iter().zip(lzc_uncorrected_pred.iter()).filter(|(c,u)| c > u).fold(0, |sum, _| sum + 1 );
    let worse = data.len() as u32 - same - better;

    let sum_bits = data.len() * 32;
    println!("METD: {}", method);
    println!("FILE: {}", filename);
    println!("====");
    println!("TOTL: {}", sum_bits);
    println!("ORIG: {} ({:.4}%)", lzc_data, lzc_data as f32/sum_bits as f32 * 100.0);
    println!("UNCO: {} ({:.4}%)", lzc_uncorrected_pred_sum, lzc_uncorrected_pred_sum as f32/sum_bits as f32 * 100.0);
    println!("CORR: {} ({:.4}%)", lzc_corrected_pred_sum, lzc_corrected_pred_sum as f32/sum_bits as f32 * 100.0);
    println!("====");
    println!("SAME: {} ({:.4}%)", same, same as f32 / data.len() as f32 * 100.0);
    println!("BETT: {} ({:.4}%)", better, better as f32 / data.len() as f32 * 100.0);
    println!("WORS: {} ({:.4}%)", worse, worse as f32 / data.len() as f32 * 100.0);
    // println!("{:?}\t data", data);
    // println!("{:?}\t pred", uncorrected_last_value_prediction);
    // println!("{:?}\t corr pred", corrected_last_value_prediction);

}
