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
            beta: 100,
            parts: 100,
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
            restricted: 0,
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
            pred - delta - (delta * self.beta) / self.parts
        } else {
            let delta = delta_to_next_power_of_two(*pred, self.restricted);
            pred + delta + (delta * self.beta) / self.parts
        }
    }
}

fn delta_to_next_power_of_two(val: u32, pos: u32) -> u32 {
    let pos = pos - 1;
    let val = val << pos >> pos;
    val.next_power_of_two() - val
}

fn delta_to_former_power_of_two(val: u32, pos: u32) -> u32 {
    let pos = pos - 1;
    let val = val << pos >> pos;
    val - (val.next_power_of_two() >> 1)
}

fn main() {
    let trth_0 = 2312.262f32.to_bits();
    let pred_0 = 2312.2787f32.to_bits();  // okayish Delta
    let pred_0 = 2312.2587f32.to_bits();  // crapy Delta (I did not undershoot that much, apparant because of the 00s)
    // let pred_0 = 2312.2487f32.to_bits();  // glorious Delta

    println!("TRT      : {:32b}", trth_0);
    println!("OLD      : {:32b} ({})", pred_0, (pred_0^trth_0).leading_zeros());

    let mut method = PreviousError::new();
    method.calculate_offset(&trth_0, &pred_0);
    let result = method.apply_correction(&pred_0);
    print!("NEXT (FF): {:32b} ({})", result, (result^trth_0).leading_zeros());
    println!(" {:?}", method);

    let mut method = DeltaToPowerOf2::new();
    method.calculate_offset(&trth_0, &pred_0);
    let result = method.apply_correction(&pred_0);
    print!("NEXT (FF): {:32b} ({})", result, (result^trth_0).leading_zeros());
    println!(" {:?}", method);

}
