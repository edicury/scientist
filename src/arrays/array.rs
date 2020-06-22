use std::num::Wrapping;

pub struct Shape {
    pub length: usize,
    pub shape: usize,
    pub values: Vec<Vec<usize>>
}

pub fn zeros(n: usize) -> Vec<usize> {
    let mut arr: Vec<usize> = Vec::new();
    for _i in 0..n {
        arr.push(0);
    }
    arr
}

pub fn zeros_from_pair(n: (usize, usize)) -> Shape {
    let mut arr: Vec<Vec<usize>> = Vec::new();
    for i in 0..n.0 {
        arr.push(Vec::new());
        for _j in 0..n.1 {
            arr[i].push(0);
        }
    }
    Shape { values:  arr, shape: n.1, length: n.0 }
}

pub fn sum_matrices(arrays: &Vec<&Vec<Vec<usize>>>, initial: Option<usize>) -> Vec<Vec<usize>> {
    operate_matrices(arrays, "+", initial)
}

pub fn subtract_matrices(arrays: &Vec<&Vec<Vec<usize>>>, initial: Option<usize>) -> Vec<Vec<usize>> {
    operate_matrices(arrays, "-", initial)
}

pub fn multiply_matrices(arrays: &Vec<&Vec<Vec<usize>>>, initial: Option<usize>) -> Vec<Vec<usize>> {
    operate_matrices(arrays, "*", initial)
}

pub fn divide_matrices(arrays: &Vec<&Vec<Vec<usize>>>, initial: Option<usize>) -> Vec<Vec<usize>> {
    operate_matrices(arrays, "/", initial)
}

pub fn reshape(array: Vec<usize>, shape: (usize, usize)) -> Vec<Vec<usize>> {
    if array.len() != (shape.0 * shape.1) {
        panic!("Shape is incompatible");
    }
    let mut pos: usize = 0;
    let mut matrix: Vec<Vec<usize>> = Vec::new();
    for i in 0..shape.0 {
        let row: Vec<usize> = Vec::new();
        matrix.push(row);
        for _j in 0..shape.1 {
            matrix[i].push(array[pos]);
            pos += 1;
        }
    }
    matrix
}

pub fn arange(n: usize) -> Vec<usize> {
    let mut arr : Vec<usize> = Vec::new();
    for i in 0..n {
        arr.push(i);
    }
    arr
}

pub fn get_sum(array: &Vec<Vec<usize>>) -> usize {
    let mut sum: usize = 0;
    for i in 0..array.len() {
        for j in 0..array[i].len() {
            sum += array[i][j];
        }
    }
    sum
}

pub fn pow_matrix(array: &Vec<Vec<usize>>, power: u32, initial: Option<usize>) -> Vec<Vec<usize>> {
    let rows: usize = array.len();
    let cols: usize = array[0].len();

    let mut powered = zeros_from_pair((rows, cols)).values;
    for i in 0..rows {
        for j in 0..cols {
            let mut val : Wrapping<usize> = Wrapping(initial.unwrap_or(1));
            let mut v : Wrapping<usize> = Wrapping(array[i][j]);
            for _p in 0..power {
                v *= v;
            }
            val += v;
            powered[i][j] = val.0;
        }
    }
    powered
}

fn operate_matrices<'a>(arrays: &'a Vec<&Vec<Vec<usize>>>, operator: &str, initial: Option<usize>) -> Vec<Vec<usize>> {
    let rows: usize = arrays[0].len();
    let cols: usize = arrays[0][0].len();

    let mut operated = zeros_from_pair((rows, cols)).values;
    for i in 0..rows {
        for j in 0..cols {
            let mut val : Wrapping<usize> = Wrapping(initial.unwrap_or(1));
            for array in 0.. arrays.len() {
                match operator {
                    "+" => { val += Wrapping(arrays[array][i][j]); },
                    "-" => { val -= Wrapping(arrays[array][i][j]); },
                    "*" => { val *= Wrapping(arrays[array][i][j]); },
                    "/" => { val /= Wrapping(arrays[array][i][j]); },
                    _ => panic!("invalid operator")
                }
            }
            operated[i][j] = val.0;
        }
    }
    operated
}