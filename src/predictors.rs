use super::position::Position;
use super::traversal::{GeneratorIteratorAdapter};
use super::ptraversal::{single_neighbours_grouped_no_ring};

pub trait PredictorTrait<T> {
    fn predict(&self, infospace: &Vec<T>) -> T;
    fn update(&mut self, information: T);
    fn consume(&mut self, data: &Vec<T>, shape: &Position, ring: bool) -> Vec<T>;
}

pub struct Ignorant<T> {
    coeff: Vec<T>,
    cells: Vec<Position>,
}

use std::ops::{Mul, AddAssign};
use std::iter::{Sum};
impl<T: AddAssign<<T as Mul>::Output>+Default+Copy+Mul + Sum<<T as Mul>::Output>> PredictorTrait<T> for Ignorant<T> {
    fn update(&mut self, _information: T) {}
    fn predict(&self, infospace: &Vec<T>) -> T {
        infospace.iter().zip(self.coeff.iter()).map(|(v,c)| *v * *c).sum()
    }
    fn consume(&mut self, data: &Vec<T>, shape: &Position, _ring:bool) -> Vec<T> {
        let spaces: Vec<Vec<T>> = GeneratorIteratorAdapter(single_neighbours_grouped_no_ring(shape, &self.cells, data)).collect();
        let mut result = Vec::new();
        for (i, space) in spaces.iter().enumerate() {
            result.push(self.predict(space));
            self.update(data[i]);
        }
        result
    }
}

pub mod predictors {
    use super::*;
    pub fn get_last_value_f32() -> Ignorant<f32> {
        Ignorant::<f32> { coeff:vec![1f32], cells: vec![Position{x:1,y:0,z:0}] }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_last_values() {
        let data: Vec<f32> = vec![
            0.0, 1.0, 2.0,
            3.0, 4.0, 5.0,
            6.0, 7.0, 8.0,

            9.0, 10.0, 11.0,
            12.0, 13.0, 14.0,
            15.0, 16.0, 17.0,

            18.0, 19.0, 20.0,
            21.0, 22.0, 23.0,
            24.0, 25.0, 26.0,
        ];

        let shape = Position{x:3, y:3, z:3};
        let mut p = predictors::get_last_value_f32();
        let result = p.consume(&data, &shape, false);
        let expected: Vec<f32> = vec![
            0.0,0.0,1.0,
            0.0,3.0,4.0,
            0.0,6.0,7.0,
            0.0,9.0,10.0,
            0.0,12.0,13.0,
            0.0,15.0,16.0,0.0,18.0,19.0,0.0,21.0,22.0,0.0,24.0,25.0];
        assert_eq!(result, expected)
    }
}
