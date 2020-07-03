use crate::models::model::{Model};
use crate::arrays::array::{get_2d_sum, get_3d_sum, get_3d_into_2d_sum};
use crate::math::math::{solve_system};

#[allow(dead_code)]
pub struct LinearRegression {
    equation: Option<Vec<f64>>
}

impl Model for LinearRegression {
    fn new() -> Self {
        Self { equation: None }
    }

    fn fit(&mut self, x_train: &Vec<Vec<f64>>, y_train: &Vec<f64>) {
        let n = x_train.len() as f64;
        let x: f64 = get_3d_sum(x_train, 1);
        let y: f64 = get_2d_sum(y_train, 1);
        let squared_x: f64 = get_3d_sum(x_train, 2);
        let xy: f64 = get_3d_into_2d_sum(x_train, y_train);

        let system = [[x, n, y].to_vec(),
                                     [squared_x, x, xy].to_vec()].to_vec();

        let equation = solve_system(system);
        self.equation = equation;
    }

    fn predict(&self, x_validation: &Vec<Vec<f64>>) -> Vec<f64> {
        let mut y_pred: Vec<f64> = Vec::new();
        let equation = self.equation.as_ref().expect("Model was not fitted");

        for i in x_validation.iter() {
            let mut val: f64 = 0.;
            for j in i.iter() {
                val += equation[0] * *j + equation[1];
            }
            y_pred.push(val);
        }
        y_pred
    }
}