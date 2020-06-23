use super::model::{Model};
use crate::globals::constants::MSE;
use std::fmt::{Display, Formatter, Error};
use crate::arrays::array;
use crate::arrays::array::{sum_matrices, multiply_matrices, subtract_matrices, pow_matrix, get_sum, subtract_matrix, multiply_matrix};
use std::num::Wrapping;
use core::num::FpCategory::Infinite;

#[derive(Debug)]
pub struct LinearRegressor<'a> {
    pub random_state: Option<usize>,
    pub method: Option<&'a str>,
    pub a_0: Option<Vec<Vec<f64>>>,
    pub a_1: Option<Vec<Vec<f64>>>
}

impl<'a> Model for LinearRegressor<'a> {
    fn new(random_state: Option<usize>) -> Self {
        Self {
            random_state: Some(10),
            method: Some(MSE),
            a_0: None,
            a_1: None
        }
    }

    fn fit(&mut self, x_train: Vec<Vec<f64>>, y_train: Vec<f64>) {
        let n : f64 = x_train.len() as f64;
        let alpha : f64 = 0.0001;
        let mut a_0: Vec<Vec<f64>> = array::zeros_from_pair((n as usize, 1)).values;
        let mut a_1: Vec<Vec<f64>> = array::zeros_from_pair((n as usize, 1)).values;

        let mut y : Vec<Vec<f64>> = Vec::new();
        let mut epochs : usize = 0;

        while epochs < 2 {
            let summed : Vec<Vec<f64>> = sum_matrices(&[&a_0, &a_1].to_vec(), None);
            y = multiply_matrices(&[&x_train, &summed].to_vec(), None);
            let error_val = subtract_matrices(&[&y, &x_train].to_vec(), None);
            let sq_er_ar = pow_matrix(&error_val, 2, None);
            let mut mean_sq_er = get_sum(&sq_er_ar);
            mean_sq_er = mean_sq_er/n;
            let corrected_alpha_0 = Wrapping(alpha * 2 as f64 * (get_sum(&error_val) / n) as f64);
            let multiplied_error = multiply_matrices(&[&error_val, &x_train].to_vec(), None);
            let correct_alpha_1 = Wrapping(alpha * 2 as f64 * (get_sum(&multiplied_error)/n) as f64);
            a_0 = subtract_matrix(&a_0, corrected_alpha_0.0 as f64, None);
            a_1 = subtract_matrix(&a_1, correct_alpha_1.0 as f64, None);

            epochs += 1;
        }

        self.a_0 = Some(a_0.to_vec());
        self.a_1 = Some(a_1.to_vec());
    }

    fn predict(&self, x_axis: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let a_0 = self.a_0.as_ref().expect("Model was not trained yet");
        let a_1 = self.a_1.as_ref().expect("Model was not trained yet");

        let a_1_test = multiply_matrices(&[a_1, &x_axis].to_vec(), None);
        let prediction = sum_matrices(&[a_0, &a_1_test].to_vec(), None);
        prediction
    }
}

impl Display for LinearRegressor<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Cost Function Method: {:?}, Random State: {:?}", self.method, self.random_state)
    }
}