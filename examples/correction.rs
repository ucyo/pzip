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

#[derive(Debug)]
pub struct Context {
    overshot: bool,
    beta: u32,  // relative of parts
    parts: u32, // absolute parts

    // Data for PreviousError method
    offset: u32,
    truth: u32,      // last truth value
    prediction: u32, // last prediction value

    // Data for DeltaToPowerOf2 method
    restricted: u32,
}

impl Context {
    pub fn new(beta: u32, parts: u32) -> Self {
        Context { overshot: false, beta: beta, parts: parts,
                  truth: 0, prediction: 0, offset: 0,
                  restricted: 0,
        }
    }
}

pub trait CorrectionContextTrait {
    fn update(&self, ctx: &mut Context);
    fn apply_correction(&mut self, num: &u32, ctx: &mut Context) -> u32;
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
    fn update(&self, ctx: &mut Context){
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
    fn apply_correction(&mut self, num: &u32, ctx: &mut Context) -> u32 {
        match self {
            Correction::PreviousError => {
                let correction = (ctx.offset * ctx.beta) / ctx.parts;
                if ctx.overshot {
                    if correction > *num {
                        ctx.offset = 0;
                        return 0;
                    } else {
                        return num - correction;
                    }
                } else {
                    return num + correction;
                }
            }
            Correction::DeltaToPowerOf2 => {
                if ctx.restricted < 10 {
                    return *num;
                }
                if ctx.overshot {
                    let delta = delta_to_former_power_of_two(*num, ctx.restricted);
                    return num - (delta * ctx.beta) / ctx.parts;
                } else {
                    let delta = delta_to_next_power_of_two(*num, ctx.restricted);
                    return num + (delta * ctx.beta) / ctx.parts;
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

#[allow(unused_variables, unused_imports)]
fn main() {
use log::{debug, error, info, trace, warn};
use pzip::testing::{FileToBeCompressed, Source};
use std::env::args;

    // Setup of environment
    env_logger::init();
    let arguments: Vec<_> = args().collect();
    let filename = &arguments[1];
    let mut source: Source<f32> = Source::new(filename);
    source.load().unwrap();

    // Subsetting of data
    let data: Vec<u32> = source.data.iter().map(|x| x.to_bits()).collect();
    let (start, size) = (30356, 1000);
    let data = data[start..start + size].to_vec(); // let data: Vec<u32> = vec![4,5,6,8,10,9,0];

    // Get results of uncorrected last value prediction
    let mut uncorrected_last_value_prediction = vec![0u32; data.len()];
    for v in 1..data.len() {
        uncorrected_last_value_prediction[v] = data[v - 1];
    }

    // Get results of corrected last value prediction
    let mut corrected_last_value_prediction: Vec<u32> = Vec::new();
    let mut pred = 0u32;
    let mut ctx = Context::new(1, 3);

    // Decision about method
    let mut method = Correction::DeltaToPowerOf2;

    // Calculate correction and iterate the vector
    info!(",index,uncorrected,pred,truth,m_overshot,m_offset,_");
    for (i, value) in data.iter().enumerate() {
        let uncorrected = pred;
        pred = method.apply_correction(&pred, &mut ctx);
        debug!("IX: {:08} Uncorrected: {:032b}", i, uncorrected);
        debug!("IX: {:08}   Corrected: {:032b} by {:?}", i, pred, ctx);
        debug!("IX: {:08}       Truth: {:032b}", i, value);
        let (before, after) = (
            (uncorrected ^ value).leading_zeros(),
            (pred ^ value).leading_zeros(),
        );
        if before > after {
            warn!("              Degradation: {:02}", before - after);
        } else if before == after {
            debug!("               No change: {:02}", after);
        } else {
            debug!("             Improvement: {:02}", after - before);
        }
        info!(",{},{},{},{},{},{},{}", i, uncorrected, pred, value, ctx.overshot, ctx.offset, "");
        corrected_last_value_prediction.push(pred);
        ctx.prediction = pred; ctx.truth = *value;
        method.update(&mut ctx);
        pred = *value;
    }

    // Analysis
    let lzc_data: u32 = data.iter().map(|x| x.leading_zeros()).sum();
    let lzc_uncorrected_pred: Vec<u32> = data
        .iter()
        .zip(uncorrected_last_value_prediction.iter())
        .map(|(t, p)| (t ^ p).leading_zeros())
        .collect();
    let lzc_uncorrected_pred_sum: u32 = lzc_uncorrected_pred.iter().sum();
    let lzc_corrected_pred: Vec<u32> = data
        .iter()
        .zip(corrected_last_value_prediction.iter())
        .map(|(t, p)| (t ^ p).leading_zeros())
        .collect();
    let lzc_corrected_pred_sum: u32 = lzc_corrected_pred.iter().sum();

    let same: u32 = lzc_corrected_pred
        .iter()
        .zip(lzc_uncorrected_pred.iter())
        .filter(|(c, u)| c == u)
        .fold(0, |sum, _| sum + 1);
    let better: u32 = lzc_corrected_pred
        .iter()
        .zip(lzc_uncorrected_pred.iter())
        .filter(|(c, u)| c > u)
        .fold(0, |sum, _| sum + 1);
    let worse = data.len() as u32 - same - better;

    let sum_bits = data.len() * 32;
    info!("METD: {:?}", ctx);
    info!("FILE: {}", filename);
    info!("====");
    info!("TOTL: {}", sum_bits);
    info!(
        "ORIG: {} ({:.4}%)",
        lzc_data,
        lzc_data as f32 / sum_bits as f32 * 100.0
    );
    info!(
        "UNCO: {} ({:.4}%)",
        lzc_uncorrected_pred_sum,
        lzc_uncorrected_pred_sum as f32 / sum_bits as f32 * 100.0
    );
    info!(
        "CORR: {} ({:.4}%)",
        lzc_corrected_pred_sum,
        lzc_corrected_pred_sum as f32 / sum_bits as f32 * 100.0
    );
    info!("====");
    info!(
        "SAME: {} ({:.4}%)",
        same,
        same as f32 / data.len() as f32 * 100.0
    );
    info!(
        "BETT: {} ({:.4}%)",
        better,
        better as f32 / data.len() as f32 * 100.0
    );
    info!(
        "WORS: {} ({:.4}%)",
        worse,
        worse as f32 / data.len() as f32 * 100.0
    );
}
