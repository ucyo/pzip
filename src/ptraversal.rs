use super::position::Position as Coordinate;
use super::traversal::GeneratorIteratorAdapter;

pub fn neighbours(shape: &Coordinate, pos: &Vec<Coordinate>, data: &Vec<f32>, ring: bool) -> Vec<Vec<f32>>{
    if ring {
        get_values_with_default_at_nonexistent_neighbours_all_ring_by_pos(shape, pos, data)
    } else {
        get_values_with_default_at_nonexistent_neighbours_all_by_pos(shape, pos, data)
    }
}

pub fn get_values_with_default_at_nonexistent_neighbours_all_ring_by_pos(shape: &Coordinate, pos: &Vec<Coordinate>, data: &Vec<f32>)
-> Vec<Vec<f32>>
{

    let tmp = get_values_with_default_at_nonexistent_neighbours_all_ring(&shape, &pos, &data);

    let mut result = Vec::new();
    for i in 0..data.len() {
        let mut r = Vec::new();
        for vec in &tmp {
            r.push(vec[i])
        }
        result.push(r)
    }

    result
}


pub fn get_values_with_default_at_nonexistent_neighbours_all_ring(shape: &Coordinate, pos: &Vec<Coordinate>, data: &Vec<f32>,
) -> Vec<Vec<f32>> {
    let mut result: Vec<Vec<f32>> = Vec::new();
    for p in pos {
        result.push(get_values_with_default_at_nonexistent_neighbours_ring(&shape, &p, &data));
    }
    result
}


pub fn get_values_with_default_at_nonexistent_neighbours_ring(shape: &Coordinate, pos: &Coordinate, data: &Vec<f32>) -> Vec<f32>{
    let offset = calculate_offset(shape, pos) as isize;
    let ptr = data.as_ptr();
    let Coordinate{x:_, y:dy, z:dz} = calculate_dims(shape);
    let Coordinate{x:nx, y:ny, z:nz} = *shape;
    let mut result: Vec<f32> = Vec::new();
    let mut ix = 0i32;
    let default = 0.0f32;


    repeated_add(&mut result, default, pos.z * dz); ix += pos.z * dz;
    for _ in 0..nz-pos.z {
        repeated_add(&mut result, default, pos.y * dy); ix += pos.y * dy;
        for _ in 0..ny-pos.y {
            for _ in 0..nx {
                let off = (ix as isize - offset).max(0);
                let val = unsafe { *ptr.offset(off) };
                repeated_add(&mut result, val, 1);
                ix += 1;
            }
        }
    }
    result
}

pub fn calculate_offset(shape: &Coordinate, pos: &Coordinate) -> usize {
    let agg_dims = calculate_dims(shape);
    let result = agg_dims.z * pos.z + agg_dims.y * pos.y + agg_dims.x * pos.x;
    result as usize
}

fn calculate_dims(shape: &Coordinate) -> Coordinate {
    let dx = 1;
    let dy = dx * shape.x;
    let dz = dy * shape.y;
    Coordinate{x:dx, y:dy, z:dz}
}

fn repeated_add(data: &mut Vec<f32>, val: f32, times: i32) {
    for _ in 0..times {
        data.push(val)
    }
}


/// Non-ring implementation of the neighbours

pub fn get_values_with_default_at_nonexistent_neighbours_all_by_pos(shape: &Coordinate, pos: &Vec<Coordinate>, data: &Vec<f32>)
-> Vec<Vec<f32>>
{

    let tmp = get_values_with_default_at_nonexistent_neighbours_all(&shape, &pos, &data);

    let mut result = Vec::new();
    for i in 0..data.len() {
        let mut r = Vec::new();
        for vec in &tmp {
            r.push(vec[i])
        }
        result.push(r)
    }

    result
}

pub fn get_values_with_default_at_nonexistent_neighbours_all(shape: &Coordinate, pos: &Vec<Coordinate>, data: &Vec<f32>,
) -> Vec<Vec<f32>> {
    let mut result: Vec<Vec<f32>> = Vec::new();
    for p in pos {
        result.push(get_values_with_default_at_nonexistent_neighbours(&shape, &p, &data));
    }
    result
}

pub fn get_values_with_default_at_nonexistent_neighbours(shape: &Coordinate, pos: &Coordinate, data: &Vec<f32>) -> Vec<f32>{
    let offset = calculate_offset(shape, pos) as isize;
    let ptr = data.as_ptr();
    let Coordinate{x:dx, y:dy, z:dz} = calculate_dims(shape);
    let Coordinate{x:nx, y:ny, z:nz} = *shape;
    let mut result: Vec<f32> = Vec::new();
    let mut ix = 0i32;
    let default = 0.0f32;


    repeated_add(&mut result, default, pos.z * dz); ix += pos.z * dz;
    for _ in 0..nz-pos.z {
        repeated_add(&mut result, default, pos.y * dy); ix += pos.y * dy;
        for _ in 0..ny-pos.y {
            repeated_add(&mut result, default, pos.x * dx);  ix += pos.x * dx;
            for _ in 0..nx-pos.x {
                let val = unsafe { *ptr.offset(ix as isize - offset) };
                repeated_add(&mut result, val, 1);
                ix += 1;
            }
        }
    }
    result
}


use std::ops::{AddAssign, Mul};
use std::ops::{Generator};
pub fn single_neighbours_no_ring<'a, T: AddAssign<<T as Mul>::Output> + Copy + Default + Mul>(
    shape: &'a Coordinate, pos: &'a Coordinate, data: &'a Vec<T>) -> impl Generator<Yield = T, Return = ()> + 'a {
    move || {
        let offset = calculate_offset(shape, pos) as isize;
        let ptr = data.as_ptr();
        let Coordinate{x:dx, y:dy, z:dz} = calculate_dims(shape);
        let Coordinate{x:nx, y:ny, z:nz} = *shape;
        let mut ix = 0i32;

        for _ in 0..pos.z * dz {yield T::default();ix += 1;}
        for _ in 0..nz-pos.z {
            for _ in 0..pos.y * dy { yield T::default(); ix += 1;}
            for _ in 0..ny-pos.y {
                for _ in 0..pos.x * dx { yield T::default(); ix+=1;}
                for _ in 0..nx-pos.x {
                    yield unsafe { *ptr.offset(ix as isize - offset) };
                    ix+=1;
                }
            }
        }
    }
}

pub fn single_neighbours_grouped_no_ring<'a, T: AddAssign<<T as Mul>::Output> + Copy + Default + Mul>(
    shape: &'a Coordinate, pos: &'a Vec<Coordinate>, data: &'a Vec<T>) -> impl Generator<Yield = Vec<T>, Return = ()> + 'a {
    move || {
        let results: Vec<Vec<T>> = pos.iter().map(|p| GeneratorIteratorAdapter(single_neighbours_no_ring(shape, p, data)).collect()).collect();
        for i in 0..data.len(){
            let mut r = Vec::new();
            for k in 0..results.len() {
                r.push(results[k][i])
            }
            yield r;
        }
    }
}

pub fn single_neighbours_with_ring<'a, T: AddAssign<<T as Mul>::Output> + Copy + Default + Mul>(
    shape: &'a Coordinate, pos: &'a Coordinate, data: &'a Vec<T>) -> impl Generator<Yield = T, Return = ()> + 'a {
    move || {
        let offset = calculate_offset(shape, pos) as isize;
        let ptr = data.as_ptr();
        let Coordinate{x:_, y:dy, z:dz} = calculate_dims(shape);
        let Coordinate{x:nx, y:ny, z:nz} = *shape;
        let mut ix = 0i32;

        for _ in 0..pos.z * dz {yield T::default();ix += 1;}
        for _ in 0..nz-pos.z {
            for _ in 0..pos.y * dy { yield T::default(); ix += 1;}
            for _ in 0..ny-pos.y {
                for _ in 0..nx {
                    let off = (ix as isize - offset).max(0);
                    yield unsafe { *ptr.offset(off) };
                    ix += 1;
                }
            }
        }
    }
}


#[allow(unused_imports)]
mod tests {
    use super::super::{Position};
    use super::*;
    #[test]
    fn test_artificial_diff() {

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


        {
            let tr = Position{x:3, y:3, z:3};
            let mut weights: Vec<Position> = Vec::new();
            weights.push(Position { x: 2, y: 1, z: 1 });

            let result: Vec<Vec<f32>> = neighbours(&tr, &weights, &data, false);
            assert_eq!(result[24], vec![0f32]);
            assert_eq!(result[26], vec![12f32]);
        }

        {
            let tr = Position{x:3, y:3, z:3};
            let mut weights: Vec<Position> = Vec::new();
            weights.push(Position { x: 2, y: 1, z: 1 });

            let result: Vec<f32> = GeneratorIteratorAdapter(single_neighbours_no_ring(&tr, &weights[0], &data)).collect();
            assert_eq!(result[24], 0f32);
            assert_eq!(result[26], 12f32);
        }
    }
    #[test]
    fn test_generic_neighbours_no_ring() {

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

        {
            let tr = Position{x:3, y:3, z:3};
            let mut weights: Vec<Position> = Vec::new();
            weights.push(Position { x: 2, y: 1, z: 1 });

            let result: Vec<f32> = GeneratorIteratorAdapter(single_neighbours_no_ring(&tr, &weights[0], &data)).collect();
            assert_eq!(result[24], 0f32);
            assert_eq!(result[26], 12f32);
        }
    }


    #[test]
    fn test_generic_neighbours_with_ring() {

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

        {
            let tr = Position{x:3, y:3, z:3};
            let mut weights: Vec<Position> = Vec::new();
            weights.push(Position { x: 2, y: 1, z: 1 });

            let result: Vec<f32> = GeneratorIteratorAdapter(single_neighbours_with_ring(&tr, &weights[0], &data)).collect();
            assert_eq!(result[24], 10f32);
            assert_eq!(result[26], 12f32);
        }
    }

    #[test]
    fn test_generic_neighbours_grouped_no_ring() {

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

        {
            let tr = Position{x:3, y:3, z:3};
            let mut weights: Vec<Position> = Vec::new();
            weights.push(Position { x: 2, y: 1, z: 1 });
            weights.push(Position { x: 1, y: 0, z: 0 });

            let result: Vec<Vec<f32>> = GeneratorIteratorAdapter(single_neighbours_grouped_no_ring(&tr, &weights, &data)).collect();
            assert_eq!(result[24], vec![0f32, 0f32]);
            assert_eq!(result[25], vec![0f32, 24f32]);
            assert_eq!(result[26], vec![12f32, 25f32]);
        }
    }
}
