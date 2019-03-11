const ONE_ZERO_U32: u32 = 2863311530;
const ZERO_ONE_U32: u32 = 1431655765;
// TODO: Current implmentation only supports u32 values. Add to this u64.
// const ONE_ZERO_U64: u64 = 12297829382473034410;
// const ZERO_ONE_U64: u64 = 6148914691236517205;

#[derive(Debug)]
pub struct RContext {
    cut: u32,
}

impl RContext {
    pub fn new(cut: u32) -> Self {
        RContext { cut: cut }
    }
}

pub trait ResidualTrait {
    fn residual(&self, truth: &u32, prediction: &u32, rctx: &mut RContext) -> u32;
    fn truth(&self, residual: &u32, prediction: &u32, rctx: &mut RContext) -> u32;
}

pub enum ResidualCalculation {
    ExclusiveOR,
    Shifted,
}

impl ResidualTrait for ResidualCalculation {
    fn residual(&self, truth: &u32, prediction: &u32, rctx: &mut RContext) -> u32 {
        match self {
            ResidualCalculation::ExclusiveOR => *truth ^ *prediction,
            ResidualCalculation::Shifted => {
                let (add, shift) = shift_calculation(*prediction, rctx.cut);
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
                let (add, shift) = shift_calculation(*prediction, rctx.cut);
                let shifted_prediction = apply_shift(*prediction, &add, &shift);
                let shifted_truth = *residual ^ shifted_prediction;
                let truth = apply_shift(shifted_truth, &!add, &shift);
                truth
            }
        }
    }
}

fn shift_calculation(num: u32, cut: u32) -> (bool, u32) {
    let bits = 32;
    let base = (num >> cut) << cut;
    let last_value = (num >> cut) & 1;
    if last_value == 1 {
        let delta = ZERO_ONE_U32 >> (bits - cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        return (false, shift);
    } else {
        let delta = ONE_ZERO_U32 >> (bits - cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        return (true, shift);
    }
}

fn apply_shift(num: u32, sign: &bool, delta: &u32) -> u32 {
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
    fn test_circular_structure() {
        let mut rctx = RContext::new(20);

        for _ in 0..1000 {
            let mut rng = thread_rng();
            let pred: u32 = rng.gen();
            let truth: u32 = rng.gen();
            let xor = ResidualCalculation::ExclusiveOR.residual(&truth, &pred, &mut rctx);
            let xor_rev = ResidualCalculation::ExclusiveOR.truth(&xor, &pred, &mut rctx);
            let shifted_xor = ResidualCalculation::Shifted.residual(&truth, &pred, &mut rctx);
            let shifted_xor_rev = ResidualCalculation::Shifted.truth(&shifted_xor, &pred, &mut rctx);

            assert_eq!(xor_rev, truth);
            assert_eq!(shifted_xor_rev, truth);
        }

    }
}