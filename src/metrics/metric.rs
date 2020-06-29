use crate::arrays::array::{absolute_vec, subtract_2d_vectors};

pub fn single_mae(y_true: &Vec<f64>, y_pred: &Vec<f64>) -> f64 {
    let size = y_true.len() as f64;

    let diff = subtract_2d_vectors(y_pred, y_true);

    let abs_diff = absolute_vec(diff);
    let total: f64 = abs_diff.iter().sum();
    let mae: f64 = total/size;

    mae
}
