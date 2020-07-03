use crate::math::math::{pow, absolute};

pub struct Shape {
    pub length: usize,
    pub shape: usize,
    pub values: Vec<Vec<f64>>
}

pub fn zeros(n: usize) -> Vec<f64> {
    fill(n, 0.)
}

pub fn zeros_from_pair(n: (usize, usize)) -> Shape {
    fill_from_pair(n, 0.)
}

pub fn ones(n: usize) -> Vec<f64> {
    fill(n, 1.)
}


pub fn ones_from_pair(n: (usize, usize)) -> Shape {
    fill_from_pair(n, 1.)
}


pub fn fill(n: usize, value: f64) -> Vec<f64> {
    let mut arr: Vec<f64> = Vec::new();
    for _i in 0..n {
        arr.push(value);
    }
    arr
}


fn fill_from_pair(n: (usize, usize), value: f64) -> Shape {
    let mut arr: Vec<Vec<f64>> = Vec::new();
    for i in 0..n.0 {
        arr.push(Vec::new());
        for _j in 0..n.1 {
            arr[i].push(value);
        }
    }
    Shape { values:  arr, shape: n.1, length: n.0 }
}

pub fn sum_matrices(arrays: &Vec<&Vec<Vec<f64>>>, initial: Option<f64>) -> Vec<Vec<f64>> {
    operate_matrices(arrays, "+", initial)
}

pub fn subtract_matrices(arrays: &Vec<&Vec<Vec<f64>>>, initial: Option<f64>) -> Vec<Vec<f64>> {
    operate_matrices(arrays, "-", initial)
}

pub fn multiply_matrices(arrays: &Vec<&Vec<Vec<f64>>>, initial: Option<f64>) -> Vec<Vec<f64>> {
    operate_matrices(arrays, "*", Some(initial.unwrap_or(1 as f64)))
}

pub fn divide_matrices(arrays: &Vec<&Vec<Vec<f64>>>, initial: Option<f64>) -> Vec<Vec<f64>> {
    operate_matrices(arrays, "/", Some(initial.unwrap_or(1 as f64)))
}

pub fn reshape(array: Vec<f64>, shape: (usize, usize)) -> Vec<Vec<f64>> {
    if array.len() != (shape.0 * shape.1) {
        panic!("Shape is incompatible");
    }
    let mut pos: usize = 0;
    let mut matrix: Vec<Vec<f64>> = Vec::new();
    for i in 0..shape.0 {
        let row: Vec<f64> = Vec::new();
        matrix.push(row);
        for _j in 0..shape.1 {
            matrix[i].push(array[pos]);
            pos += 1;
        }
    }
    matrix
}

pub fn arange(n: usize) -> Vec<f64> {
    let mut arr : Vec<f64> = Vec::new();
    for i in 0..n {
        arr.push(i as f64);
    }
    arr
}

pub fn get_sum(array: &Vec<Vec<f64>>) -> f64 {
    let mut sum:f64 = 0 as f64;
    for i in 0..array.len() {
        for j in 0..array[i].len() {
            sum += array[i][j];
        }
    }
    sum
}


pub fn get_2d_sum(array: &Vec<f64>, power: usize) -> f64 {
    let mut sum: f64 = 0.;
    for i in array.iter() {
        sum += pow(*i, power);
    }
    sum
}

pub fn get_3d_sum(array: &Vec<Vec<f64>>, power: usize) -> f64 {
    let mut sum: f64 = 0.;

    for i in array.iter() {
        for j in 0..i.len() {
            sum += pow(i[j], power);
        }
    }
    sum
}

pub fn get_2d_multiplied_sum(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.;

    for i in 0..x.len() {
        sum += x[i]*y[i];
    }
    sum
}

pub fn get_3d_into_2d_sum(x: &Vec<Vec<f64>>, y: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.;

    for i in 0..x.len() {
        for j in 0..x[i].len() {
            sum += x[i][j]*y[i];
        }
    }
    sum
}


pub fn subtract_2d_vectors(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    let mut subtracted : Vec<f64> = Vec::new();

    for i in 0..vec1.len() {
        subtracted.push(vec1[i] - vec2[i]);
    }

    subtracted
}

pub fn pow_matrix(array: &Vec<Vec<f64>>, power: u32, initial: Option<f64>) -> Vec<Vec<f64>> {
    let rows: usize = array.len();
    let cols: usize = array[0].len();

    let mut powered = zeros_from_pair((rows, cols)).values;
    for i in 0..rows {
        for j in 0..cols {
            let mut val : f64 = initial.unwrap_or(1 as f64);
            let mut v : f64 = array[i][j];
            for _p in 0..power {
                v *= v;
            }
            val += v;
            powered[i][j] = val;
        }
    }
    powered
}

fn operate_matrices<'a>(arrays: &'a Vec<&Vec<Vec<f64>>>, operator: &str, initial: Option<f64>) -> Vec<Vec<f64>> {
    let rows: usize = arrays[0].len();
    let cols: usize = arrays[0][0].len();

    let mut operated = zeros_from_pair((rows, cols)).values;
    for i in 0..rows {
        for j in 0..cols {
            let mut val : f64 = initial.unwrap_or(0 as f64);
            for array in 0.. arrays.len() {
                match operator {
                    "+" => { val += arrays[array][i][j]; },
                    "-" => { val -= arrays[array][i][j]; },
                    "*" => { val *= arrays[array][i][j]; },
                    "/" => { val /= arrays[array][i][j]; },
                    _ => panic!("invalid operator")
                }
            }
            operated[i][j] = val;
        }
    }
    operated
}

fn operate_matrix(matrix: &Vec<Vec<f64>>, value: f64, operator: &str, initial: Option<f64>) -> Vec<Vec<f64>> {
    let rows: usize = matrix.len();
    let cols: usize = matrix[0].len();
    let mut operated = zeros_from_pair((rows, cols)).values;
    for i in 0..rows {
        for j in 0..cols {
            let mut val :f64 = initial.unwrap_or(0 as f64);
            match operator {
                "+" => { val += value; }
                "-" => { val -= value; }
                "*" => { val *= value; }
                "/" => { val /= value; }
                _ => panic!("invalid operator")
            }
            operated[i][j] = val;
        }
    }
    operated
}

pub fn sum_matrix(matrix: &Vec<Vec<f64>>, value: f64, initial: Option<f64>) -> Vec<Vec<f64>> {
    operate_matrix(matrix, value, "+", initial)
}

pub fn subtract_matrix(matrix: &Vec<Vec<f64>>, value: f64, initial: Option<f64>) -> Vec<Vec<f64>> {
    operate_matrix(matrix, value, "-", initial)
}
pub fn multiply_matrix(matrix: &Vec<Vec<f64>>, value: f64, initial: Option<f64>) -> Vec<Vec<f64>> {
    operate_matrix(matrix, value, "*", Some(initial.unwrap_or(1 as f64)))
}
pub fn divide_matrix(matrix: &Vec<Vec<f64>>, value: f64, initial: Option<f64>) -> Vec<Vec<f64>> {
    operate_matrix(matrix, value, "/", Some(initial.unwrap_or(1 as f64)))
}

pub fn absolute_vec(vec: Vec<f64>) -> Vec<f64> {  vec.iter().map(|x| absolute(*x)).collect() }