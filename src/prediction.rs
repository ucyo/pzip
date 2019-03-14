#![allow(dead_code)]

/// A new implementation of the predictor module.
/// This implementation is using ENUMs instead of structs.
/// There is a strict split between data and logic.

use super::position::Position;

/// # Data
/// Information/Setup to be used by the prediction logic.
///
/// TODO: Implementation of f64

type CCoefficients = Vec<f32>;
type Context = Vec<Position>;

#[derive(Debug)]
pub struct IContext {
    coeff: Vec<CCoefficients>,
    cells: Vec<Context>,
    best: usize,
    consolidation_coeffs: CCoefficients,
}

impl IContext {
    fn new(
        coeff: Vec<Vec<f32>>,
        cells: Vec<Vec<Position>>,
        best: usize,
        consolidation_coeffs: Vec<f32>,
    ) -> Self {
        IContext {
            coeff: coeff,
            cells: cells,
            best: best,
            consolidation_coeffs: consolidation_coeffs,
        }
    }
}

/// TODO: Implementation of f64
pub trait PredictorTrait {
    fn predict(&self, infospace: &Vec<Vec<f32>>, ictx: &mut IContext) -> f32;
    fn update(&self, information: &f32, ictx: &mut IContext);
    // fn consume(&mut self, data: &Vec<T>, shape: &Position, ring: bool) -> Vec<T>;
}


/// # Logic
/// The handling logic is done using an ENUM.
///
pub enum Predictor {
    Classic,
    // InfoSpaceMixed,
    // InfoSpaceRanked,
}

#[allow(unused_variables)]
impl PredictorTrait for Predictor {
    fn predict(&self, infospace: &Vec<Vec<f32>>, ictx: &mut IContext) -> f32 {
        match self {
            Predictor::Classic => {
                let information = infospace.get(0).expect("Empty!");
                let coeff = ictx.coeff.get(0).expect("Empty!");
                let result: f32 = information.iter().zip(coeff.iter()).map(|(a,b)| b*a).sum();
                result
            }
        }
    }
    fn update(&self, information: &f32, ictx: &mut IContext) {
        match self {
            Predictor::Classic => {}
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_approach() {
        let cells = vec![
            vec![Position{x:1,y:0,z:0},Position{x:1,y:1,z:0},Position{x:0,y:1,z:0}],
        ];
        let coeff = vec![
            vec![1f32,-1f32,1f32],
        ];
        let mut ictx = IContext::new(coeff, cells, 0usize, vec![0f32]);

        let prepared_spaces = vec![
            vec![1f32;3],
            vec![0f32;3],
            vec![2f32,4f32,1f32],
            vec![21f32,33f32,12f32],
        ];

        let data = vec![1f32, 0f32, -1f32, 0f32];
        let mut result = Vec::new();
        for (i,space) in prepared_spaces.into_iter().enumerate() {
            let input = vec![space];
            let prediction = Predictor::Classic.predict(&input, &mut ictx);
            Predictor::Classic.update(&data[i], &mut ictx);
            result.push(prediction);
        }
        assert_eq!(result, vec![1f32, 0f32, -1f32, 0f32]);
    }

    // #[test]
    // fn test_infospace_last_best() {
    //     let cells = vec![
    //         vec![Position{x:1,y:0,z:0},Position{x:1,y:1,z:0},Position{x:0,y:1,z:0}],
    //         vec![Position{x:1,y:1,z:2},Position{x:1,y:1,z:2},Position{x:2,y:1,z:0}],
    //         vec![Position{x:1,y:0,z:0},Position{x:1,y:1,z:0},Position{x:2,y:1,z:2}],
    //     ];
    //     let coeff = vec![
    //         vec![1f32,-1f32,1f32],
    //         vec![2f32,-3f32,11f32],
    //         vec![-2f32,1f32,3f32],
    //     ];
    //     let mut ictx = IContext::new(coeff, cells, 0usize, vec![1f32,1f32,-1f32]);


    //     let data = vec![1f32, 0f32, -1f32, 0f32];

    //     assert_eq!(true, false);

    // }
}
