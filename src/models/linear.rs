use super::model::{Model};
use crate::globals::constants::MSE;
use std::fmt::{Display, Formatter, Error};
use crate::arrays::array;
use crate::arrays::array::{sum_matrices, multiply_matrices, subtract_matrices, pow_matrix, get_sum};

#[derive(Debug)]
pub struct LinearRegressor<'a> {
    pub random_state: Option<usize>,
    pub method: Option<&'a str>
}

impl<'a> Model for LinearRegressor<'a> {
    fn new(random_state: Option<usize>) -> Self {
        Self {
            random_state: Some(10),
            method: Some(MSE)
        }
    }

    fn fit(&self, x_train: Vec<Vec<usize>>, y_train: Vec<usize>) {
        let n : usize = x_train.len();
        let alpha : f64 = 0.0001;
        let a_0: Vec<Vec<usize>> = array::zeros_from_pair((n, 1)).values;
        let a_1: Vec<Vec<usize>> = array::zeros_from_pair((n, 1)).values;

        let mut y : Vec<Vec<usize>> = Vec::new();
        let mut epochs :usize = 0;

        while epochs < 1 {
            let summed : Vec<Vec<usize>> = sum_matrices(&[&a_0, &a_1].to_vec(), None);
            y = multiply_matrices(&[&x_train, &summed].to_vec(), None);
            let error_val = subtract_matrices(&[&y, &x_train].to_vec(), None);
//            let sq_er_ar = pow_matrix(&error_val, 2, None);
//            let mut mean_sq_er = get_sum(&sq_er_ar);
//            mean_sq_er = mean_sq_er/n;

            epochs += 1;
        }

//            y = a_0 + a_1 * x_train
//        error = y - y_train
//        mean_sq_er = np.sum(error**2)
//        mean_sq_er = mean_sq_er/n
//        a_0 = a_0 - alpha * 2 * np.sum(error)/n
//        a_1 = a_1 - alpha * 2 * np.sum(error * x_train)/n
//        epochs += 1
//        if(epochs%10 == 0):
//            print(mean_sq_er)
    }

    fn predict(&self, x_axis: Vec<Vec<usize>>) -> Vec<usize> {
        unimplemented!()
    }
}

impl Display for LinearRegressor<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Cost Function Method: {:?}, Random State: {:?}", self.method, self.random_state)
    }
}