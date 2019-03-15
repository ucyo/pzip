const ONE_ZERO_U32: u32 = 2863311530;
const ZERO_ONE_U32: u32 = 1431655765;
// TODO: Current implmentation only supports u32 values. Add to this u64.
// const ONE_ZERO_U64: u64 = 12297829382473034410;
// const ZERO_ONE_U64: u64 = 6148914691236517205;

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
}

impl ResidualTrait for ResidualCalculation {
    fn residual(&self, truth: &u32, prediction: &u32, rctx: &mut RContext) -> u32 {
        match self {
            ResidualCalculation::ExclusiveOR => *truth ^ *prediction,
            ResidualCalculation::Shifted => {
                let (add, shift) = shift_calculation(*prediction, rctx);
                let shifted_prediction = apply_shift(*prediction, &add, &shift);
                let shifted_truth = apply_shift(*truth, &add, &shift);
                shifted_prediction ^ shifted_truth
            }
            ResidualCalculation::ShiftedLZC => {
                let (add, shift) = shift_calculation(*prediction, rctx);
                let shifted_prediction = apply_shift(*prediction, &add, &shift);
                let shifted_truth = apply_shift(*truth, &add, &shift);
                shifted_prediction ^ shifted_truth
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
    // let max_value = u32::max_value();
    let base = (num >> rctx.cut) << rctx.cut;
    let last_value = (num >> rctx.cut) & 1;
    if last_value == 1 {
        let delta = ZERO_ONE_U32 >> (bits - rctx.cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        //info!("Cutting {0} @ {1} shift {2} goal {3} f", num, cut, shift, goal);
        return (false, shift);
    } else {
        let delta = ONE_ZERO_U32 >> (bits - rctx.cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        //info!("Cutting {0} @ {1} shift {2} goal {3} t", num, cut, shift, goal);
        return (true, shift);
    }
}

fn apply_shift(num: u32, sign: &bool, delta: &u32) -> u32 {
    //info!("Applying {0} with {1} to {2}", *delta, sign, num);
    if *sign {
        return num + *delta;
    } else {
        return num - *delta;
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
            // debug!("{:032b} {:032b} {:?} {0} {1} {3}", pred, truth, rctx, u32::max_value());
            let shifted_xor = ResidualCalculation::Shifted.residual(&truth, &pred, &mut rctx);
            let shifted_xor_rev = ResidualCalculation::Shifted.truth(&shifted_xor, &pred, &mut rctx);
            // debug!("{:032b} {:032b}", shifted_xor, shifted_xor_rev);
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
            println!("{:032b} {:032b} {:?}", pred, truth, rctx);
            // debug!("{:032b} {:032b} {:?} {0} {1} {3}", pred, truth, rctx, u32::max_value());
            let shifted_xor = ResidualCalculation::ShiftedLZC.residual(&truth, &pred, &mut rctx);
            let shifted_xor_rev = ResidualCalculation::ShiftedLZC.truth(&shifted_xor, &pred, &mut rctx);
            // debug!("{:032b} {:032b}", shifted_xor, shifted_xor_rev);
            ResidualCalculation::ShiftedLZC.update(&truth, &pred, &mut rctx);
            assert_eq!(shifted_xor_rev, truth);
        }

    }
}
