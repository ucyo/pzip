/// # Application of bias / correction on predictions
///
/// ## Experiment: Run 1
/// The influence of DeltaCorrection should be better. Analysis on why this is
/// not the case. Currently most of the predictions are untouched by the
/// correction.
///
/// ### Cause: The restricted area should be calculated via diff of truth values
/// ### Cause: The restricted area should consider number of 1s/0s in prediction
///
/// # Context
/// The context element does hold the data needed for the different correction
/// methods.
///
#[derive(Debug)]
pub struct CContext {
    pub overshot: bool,
    pub beta: u32,  // relative of parts
    pub parts: u32, // absolute parts

    // Data for PreviousError method
    pub offset: u32,
    pub truth: u32,      // last truth value
    pub prediction: u32, // last prediction value

    // Data for DeltaToPowerOf2 method
    pub restricted: u32,
}

impl CContext {
    pub fn new(beta: u32, parts: u32) -> Self {
        CContext {
            overshot: false,
            beta: beta,
            parts: parts,
            truth: 0,
            prediction: 0,
            offset: 0,
            restricted: 0,
        }
    }
}

pub trait CorrectionContextTrait {
    fn update(&self, ctx: &mut CContext);
    fn apply_correction(&self, num: &u32, ctx: &mut CContext) -> u32;
}


/// # PreviousError correction
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
/// # DeltaToPowerOf2
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
pub enum Correction {
    PreviousError,
    DeltaToPowerOf2,
}

impl CorrectionContextTrait for Correction {
    fn update(&self, ctx: &mut CContext) {
        match self {
            Correction::PreviousError => {
                let diff = ctx.truth as i64 - ctx.prediction as i64 + ctx.offset as i64;
                ctx.overshot = diff < 0;
                ctx.offset = diff.abs() as u32;
            }
            Correction::DeltaToPowerOf2 => {
                ctx.restricted = (ctx.truth ^ ctx.prediction).leading_zeros();
                ctx.overshot = ctx.prediction > ctx.truth;
            }
        }
    }
    fn apply_correction(&mut self, num: &u32, ctx: &mut CContext) -> u32 {
        match self {
            Correction::PreviousError => {
                let correction = (ctx.offset * ctx.beta) / ctx.parts;
                if ctx.overshot {
                    if correction > *num {
                        ctx.offset = 0;
                        return 0;
                    } else {
                        return *num - correction;
                    }
                } else {
                    return *num + correction;
                }
            }
            Correction::DeltaToPowerOf2 => {
                if ctx.restricted < 10 {
                    return *num;
                }
                if ctx.overshot {
                    let delta = delta_to_former_power_of_two(*num, ctx.restricted);
                    return *num - (delta * ctx.beta) / ctx.parts;
                } else {
                    let delta = delta_to_next_power_of_two(*num, ctx.restricted);
                    return *num + (delta * ctx.beta) / ctx.parts;
                }
            }
        }
    }
}

fn delta_to_next_power_of_two(val: u32, pos: u32) -> u32 {
    let pos = if pos > 0 { pos - 1 } else { 0 };
    let val = val << pos >> pos;
    val.next_power_of_two() - val
}

fn delta_to_former_power_of_two(val: u32, pos: u32) -> u32 {
    let pos = pos - 1;
    let val = val << pos >> pos;
    val - (val.next_power_of_two() >> 1)
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_to_next_pow_two() {
        let data = vec![2,3,4,5,6,7,8,9];
        for v in data {
            let val = 2u32.pow(v);
            let delta = 4;
            assert_eq!(val, val + delta - delta_to_former_power_of_two(val+delta, v));
            assert_eq!(2u32.pow(v+1), val + delta_to_next_power_of_two(val+1, v) + 1);
        }
    }

    // TODO: Add tests for actual corrections
}
