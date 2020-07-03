use crate::arrays::array::{absolute_vec, subtract_2d_vectors};
use crate::math::math::pow;

pub fn mean_absolute_error(y_true: &Vec<f64>, y_pred: &Vec<f64>) -> f64 {
    let size = y_true.len() as f64;

    let diff = subtract_2d_vectors(y_pred, y_true);

    let abs_diff = absolute_vec(diff);
    let total: f64 = abs_diff.iter().sum();
    let mae: f64 = total/size;

    mae
}

pub fn square_loss(y_true: &Vec<f64>, y_pred: &Vec<f64>) -> f64 {
    let diff = subtract_2d_vectors(y_pred, y_true);
    let squared: Vec<f64> = diff.iter().map(|x| pow(*x, 2)).collect();
    let total: f64 = squared.iter().sum();


    total/y_pred.len() as f64
}
