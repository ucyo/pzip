const ONE_ZERO_U32: u32 = 2863311530;
const ZERO_ONE_U32: u32 = 1431655765;
// TODO: Current implmentation only supports u32 values. Add to this u64.
// const ONE_ZERO_U64: u64 = 12297829382473034410;
// const ZERO_ONE_U64: u64 = 6148914691236517205;
use log::{debug, warn};

#[derive(Debug)]
pub struct RContext {
    cut: u32,
    truth: u32,
    prediction: u32,
}

impl RContext {
    pub fn new(cut: u32) -> Self {
        RContext { cut: cut , truth: 0, prediction: 0}
    }
}

pub trait ResidualTrait {
    fn residual(&self, truth: &u32, prediction: &u32, rctx: &mut RContext) -> u32;
    fn truth(&self, residual: &u32, prediction: &u32, rctx: &mut RContext) -> u32;
    fn update(&self, truth: &u32, prediction: &u32, rctx: &mut RContext);
}

#[derive(Debug)]
pub enum ResidualCalculation {
    ExclusiveOR,
    Shifted,
    ShiftedLZC,
    // TODO: Choose residual based on experience (past values) instead of LZC or given cut
}

impl ResidualTrait for ResidualCalculation {
    fn residual(&self, truth: &u32, prediction: &u32, rctx: &mut RContext) -> u32 {
        match self {
            ResidualCalculation::ExclusiveOR => *truth ^ *prediction,
            ResidualCalculation::Shifted => {
                let (add, shift) = shift_calculation(*prediction, rctx);
                let shifted_prediction = apply_shift(*prediction, &add, &shift);
                let shifted_truth = apply_shift(*truth, &add, &shift);
                let result = shifted_prediction ^ shifted_truth;
                debug!("Panic?\n T{:032b}\n P{:032b}\nST{:032b}\nSP{:032b}\n X{:032b}\nSX{:032b}\n", *truth, *prediction, shifted_truth, shifted_prediction, (truth ^ prediction), result);
                debug!("{} {}", result.leading_zeros(), (truth ^ prediction).leading_zeros());
                if result.leading_zeros() < (truth ^ prediction).leading_zeros() - 1 {
                    warn!("LZC worse using shift by {}", (truth ^ prediction).leading_zeros() - result.leading_zeros());
                }
                result
            }
            ResidualCalculation::ShiftedLZC => {
                let (add, shift) = shift_calculation(*prediction, rctx);
                let shifted_prediction = apply_shift(*prediction, &add, &shift);
                let shifted_truth = apply_shift(*truth, &add, &shift);
                let result = shifted_prediction ^ shifted_truth;
                debug!("Panic?\n T{:032b}\n P{:032b}\nST{:032b}\nSP{:032b}\n X{:032b}\nSX{:032b}\n", *truth, *prediction, shifted_truth, shifted_prediction, (truth ^ prediction), result);
                debug!("Panic?\n T{}\n P{}\nST{}\nSP{}\n X{}\nSX{}\n", *truth, *prediction, shifted_truth, shifted_prediction, (truth ^ prediction), result);
                debug!("{} {}", result.leading_zeros(), (truth ^ prediction).leading_zeros());
                if result.leading_zeros() < (truth ^ prediction).leading_zeros() - 1 {
                    warn!("LZC worse using shift by {}", (truth ^ prediction).leading_zeros() - result.leading_zeros());
                }
                result
            }
        }
    }
    fn truth(&self, residual: &u32, prediction: &u32, rctx: &mut RContext) -> u32 {
        match self {
            ResidualCalculation::ExclusiveOR => *residual ^ *prediction,
            ResidualCalculation::Shifted => {
                let (add, shift) = shift_calculation(*prediction, rctx);
                let shifted_prediction = apply_shift(*prediction, &add, &shift);
                let shifted_truth = *residual ^ shifted_prediction;
                let truth = apply_shift(shifted_truth, &!add, &shift);
                truth
            }
            ResidualCalculation::ShiftedLZC => {
                let (add, shift) = shift_calculation(*prediction, rctx);
                let shifted_prediction = apply_shift(*prediction, &add, &shift);
                let shifted_truth = *residual ^ shifted_prediction;
                let truth = apply_shift(shifted_truth, &!add, &shift);
                truth
            }
        }
    }
    fn update(&self, truth: &u32, prediction: &u32, rctx: &mut RContext) {
        match self {
            ResidualCalculation::ExclusiveOR => {
                rctx.prediction = *prediction;
                rctx.truth = *truth;}
            ResidualCalculation::Shifted => {
                rctx.prediction = *prediction;
                rctx.truth = *truth;}
            ResidualCalculation::ShiftedLZC => {
                rctx.prediction = *prediction;
                rctx.truth = *truth;
                let new_cut = 32 - (*truth ^ *prediction).leading_zeros();
                rctx.cut = new_cut.max(4); //TODO: Test influence of minimal cut
            }
        }
    }
}

fn shift_calculation(num: u32, rctx: &mut RContext) -> (bool, u32) {
    let bits = 32;
    let base = (num >> rctx.cut) << rctx.cut;
    let last_value = (num >> rctx.cut) & 1;
    if last_value == 1 {
        let delta = ZERO_ONE_U32 >> (bits - rctx.cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        debug!("Shift {0:032b} @ {1} by {2:032b} to {3:032b} f", num, rctx.cut, shift, goal);
        return (num <= goal, shift);
    } else {
        let delta = ONE_ZERO_U32 >> (bits - rctx.cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        debug!("Shift {0:032b} @ {1} by {2:032b} to {3:032b} t", num, rctx.cut, shift, goal);
        return (num < goal, shift);
    }
}

fn apply_shift(num: u32, sign: &bool, delta: &u32) -> u32 {
    if *sign {
        let result = num + *delta;
        debug!("Apply Shift {0:032b} + {2:032b} = {1:032b}", *delta, result, num);
        return result
    } else {
        let result = num - *delta;
        debug!("Apply Shift {0:032b} + {2:032b} = {1:032b}", *delta, result, num);
        return result
    }
}


#[allow(unused_imports)]
mod tests {
    use rand::{thread_rng, Rng};
    use super::*;

    #[test]
    fn test_exclusive_or() {
        let mut rctx = RContext::new(20);

        for _ in 0..3_000_000 {
            let mut rng = thread_rng();
            let pred: u32 = rng.gen();
            let truth: u32 = rng.gen();
            let xor = ResidualCalculation::ExclusiveOR.residual(&truth, &pred, &mut rctx);
            let xor_rev = ResidualCalculation::ExclusiveOR.truth(&xor, &pred, &mut rctx);
            ResidualCalculation::ExclusiveOR.update(&truth, &pred, &mut rctx);
            assert_eq!(xor_rev, truth);
        }

    }

    #[test]
    fn test_shifted_random_small_delta() {
        let position = 20;
        let mut rctx = RContext::new(position);

        for _ in 0..1_000_000 {
            let mut rng = thread_rng();
            let pred: u32 = rng.gen();
            let delta: u32 = rng.gen_range(0,100);
            let sign: bool = rng.gen();
            let truth = if sign {pred + delta} else {pred - delta};
            let shifted_xor = ResidualCalculation::Shifted.residual(&truth, &pred, &mut rctx);
            let shifted_xor_rev = ResidualCalculation::Shifted.truth(&shifted_xor, &pred, &mut rctx);
            ResidualCalculation::Shifted.update(&truth, &pred, &mut rctx);
            assert_eq!(shifted_xor_rev, truth);
        }

    }
    #[test]
    #[ignore]
    fn test_overflow_danger_shifted() {
        let mut rctx = RContext::new(20);

        // TODO: Fix overflow error.
        // This error occurs if the shift calculation does not take into account that
        // the predicted value is way off. It could be prevented if we take into account
        // the cut point (since this defines the size of the shift operation).
        let pred: u32 = 2183263055;
        let truth: u32 = 4294696180;
        let shifted_xor = ResidualCalculation::Shifted.residual(&truth, &pred, &mut rctx);
        let shifted_xor_rev = ResidualCalculation::Shifted.truth(&shifted_xor, &pred, &mut rctx);
        ResidualCalculation::Shifted.update(&truth, &pred, &mut rctx);
        assert_eq!(shifted_xor_rev, truth);

    }

    #[test]
    fn test_shifted_at_lzc_random_small_delta() {
        let position = 20;
        let mut rctx = RContext::new(position);

        for _ in 0..1_000_000 {
            let mut rng = thread_rng();
            let pred: u32 = rng.gen();
            let delta: u32 = rng.gen_range(0,100);
            let sign: bool = rng.gen();
            let truth = if sign {pred + delta} else {pred - delta};
            let shifted_xor = ResidualCalculation::ShiftedLZC.residual(&truth, &pred, &mut rctx);
            let shifted_xor_rev = ResidualCalculation::ShiftedLZC.truth(&shifted_xor, &pred, &mut rctx);
            ResidualCalculation::ShiftedLZC.update(&truth, &pred, &mut rctx);
            assert_eq!(shifted_xor_rev, truth);
        }

    }
}
