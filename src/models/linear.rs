use crate::models::model::{SingleModel};
use crate::arrays::array::{get_2d_sum, get_2d_multiplied_sum};
use crate::utils::math::{solve_system};

#[allow(dead_code)]
pub struct LinearRegression {
    equation: Option<Vec<f64>>,
    random_state: Option<usize>
}

#[allow(dead_code)]
impl SingleModel for LinearRegression {
    fn new(random_state: Option<usize>) -> Self {
        Self { equation: None, random_state }
    }

    fn fit(&mut self, x_train: &Vec<f64>, y_train: &Vec<f64>) {
        let n = x_train.len() as f64;
        let x: f64 = get_2d_sum(x_train, 1);
        let y: f64 = get_2d_sum(y_train, 1);
        let squared_x: f64 = get_2d_sum(x_train, 2);
        let xy: f64 = get_2d_multiplied_sum(x_train, y_train);

        let system = [[x, n, y].to_vec(),
            [squared_x, x, xy].to_vec()].to_vec();

        let equation = solve_system(system);
        self.equation = equation;
    }

    fn predict(&self, x_axis: &Vec<f64>) -> Vec<f64> {
        let mut y_pred: Vec<f64> = Vec::new();
        let equation = self.equation.as_ref().expect("Model was not fitted");

        for i in x_axis.iter() {
            y_pred.push(equation[0] * *i + equation[1]);
        }
        y_pred
    }
}