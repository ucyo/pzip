pub trait CorrectionTrait {
    fn calculate_offset(&mut self, truth: &u32, pred: &u32);
    fn apply_correction(&self, pred: &u32) -> u32;
}


/// # PreviousError correction
///
/// Correction of the error based on the previous run. The delta of the previous
/// error is added to the current one with certain parts.
///
/// $ corr_t = corr_{t-1} * beta\parts $
/// $ fpred_t = pred_t + F * corr_t
/// with F = if pred_{t-1} < truth_{t-1} -1 else +1 $
///
#[derive(Debug)]
pub struct PreviousError {
    overshot: bool,
    offset: u32,
    beta: u32,   // parts of parts
    parts: u32,  // absolute parts [default: 100]
}

impl PreviousError {
    pub fn new() -> Self {
        PreviousError {
            overshot: false,
            offset: 0,
            beta: 50,
            parts: 100,
        }
    }
    pub fn update_beta(&mut self, val: u32) {
        self.beta = val.min(self.parts)
    }
}

impl CorrectionTrait for PreviousError {
    fn calculate_offset(&mut self, truth: &u32, pred: &u32) {
        self.overshot = pred > truth;
        self.offset = truth.max(pred) - truth.min(pred);
    }
    fn apply_correction(&self, pred: &u32) -> u32 {
        if self.overshot {
            pred - (self.offset * self.beta) / self.parts
        } else {
            pred + (self.offset * self.beta) / self.parts
        }
    }
}

use std::fmt;
impl fmt::Display for PreviousError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PreviousError {{ overshot: {}, offset: {:b} }}", self.overshot, self.offset)
    }
}
#[derive(Debug)]
pub struct FirstFlipChange {
    overshot: bool,
    flipbit: u8,
    offset: u32,
}

impl FirstFlipChange {
    pub fn new() -> Self {
        FirstFlipChange { overshot: false, flipbit: 0, offset: 0 }
    }
}

impl CorrectionTrait for FirstFlipChange {
    fn calculate_offset(&mut self, truth: &u32, pred: &u32) {
        self.overshot = pred > truth;
        self.flipbit = 32 - (truth ^ pred).leading_zeros() as u8;
    }
    fn apply_correction(&self, pred: &u32) -> u32 {
        if self.overshot {
            pred - self.offset
        } else {
            pred + self.offset
        }
    }
}

fn calculate_msb_ones(num: &u32) -> usize {
    let next = num.next_power_of_two();
    let mut pos = 1;

    while ((next >> pos) & num) != 0 {
        pos += 1;
    }
    pos - 1
}

fn main() {
    let trth_0 = 2312.262f32.to_bits();
    let pred_0 = 2312.2787f32.to_bits();
    // let pred_0 = 2312.2587f32.to_bits();
    // let pred_0 = 2312.2487f32.to_bits();

    println!("TRT      : {:32b}", trth_0);
    println!("OLD      : {:32b}", pred_0);

    let mut method = PreviousError::new();
    method.calculate_offset(&trth_0, &pred_0);
    let result = method.apply_correction(&pred_0);
    print!("NEXT (PE): {:32b}", result);
    println!(" {:?}", method);

    let mut method = FirstFlipChange::new();
    method.calculate_offset(&trth_0, &pred_0);
    let result = method.apply_correction(&pred_0);

    print!("NEXT (FF): {:32b}", result);
    println!(" {:?}", method);

    // let (ten, off) = get_overshot(truth, pred);

    // let adjusted_prediction = adjust_prediction(pred, ten, off);
    // println!("NEW: {:32b}", adjusted_prediction.to_bits());
    // println!("ADJ: {:32b}", adjusted_prediction.to_bits() ^ truth.to_bits());
}
