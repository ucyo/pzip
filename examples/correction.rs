pub trait CorrectionTrait {
    fn calculate_offset(&mut self, truth: &u32, pred: &u32);
    fn apply_correction(&self, pred: &u32) -> u32;
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

        // TODO: Wrong update if prediction is too high
        // self.overshot = *pred > *truth;
        self.offset = diff.max(0) as u32;
    }
    fn apply_correction(&self, pred: &u32) -> u32 {
        pred + (self.offset * self.beta) / self.parts
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
}

impl DeltaToPowerOf2 {
    pub fn new() -> Self {
        DeltaToPowerOf2 {
            overshot: false,
            restricted: 32,
            beta: 100,
            parts: 100,
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
    fn apply_correction(&self, pred: &u32) -> u32 {
        if self.overshot {
            let delta = delta_to_former_power_of_two(*pred, self.restricted);
            pred - (delta * self.beta) / self.parts
        } else {
            let delta = delta_to_next_power_of_two(*pred, self.restricted);
            pred + (delta * self.beta) / self.parts
        }
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
    // let data = data[..].to_vec();
    let data: Vec<u32> = vec![4,5,6,8,10,9,0];
    let mut uncorrected_last_value_prediction = vec![0u32; data.len()];
    for v in 1..data.len() {
        uncorrected_last_value_prediction[v] = data[v-1];
    }

    let mut corrected_last_value_prediction : Vec<u32> = Vec::new();
    let mut pred = 0u32;
    let mut method = PreviousError::new();

    for value in data.iter() {
        // println!("{} {} {}", value, pred, method);
        pred = method.apply_correction(&pred);
        corrected_last_value_prediction.push(pred);
        method.calculate_offset(value, &pred);  // call calculate_correction
        // println!("\n{:032b}\n{:032b}\n{:032b} Offset {}", value, pred, method.offset, method.overshot);
        pred = *value;
    }

    let lzc_data : u32 = data.iter().map(|x| x.leading_zeros()).sum();
    let lzc_uncorrected_pred : u32 = data.iter().zip(uncorrected_last_value_prediction.iter()).map(|(t,p)| (t^p).leading_zeros()).sum();
    let lzc_corrected_pred : u32 = data.iter().zip(corrected_last_value_prediction.iter()).map(|(t,p)| (t^p).leading_zeros()).sum();

    let sum_bits = data.len() * 32;
    println!("TOTL: {}", sum_bits);
    println!("ORIG: {} ({:.4}%)", lzc_data, lzc_data as f32/sum_bits as f32 * 100.0);
    println!("UNCO: {} ({:.4}%)", lzc_uncorrected_pred, lzc_uncorrected_pred as f32/sum_bits as f32 * 100.0);
    println!("CORR: {} ({:.4}%)", lzc_corrected_pred, lzc_corrected_pred as f32/sum_bits as f32 * 100.0);

    println!("{:?}\t data", data);
    println!("{:?}\t pred", uncorrected_last_value_prediction);
    println!("{:?}\t corr pred", corrected_last_value_prediction);

}
