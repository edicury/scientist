pub trait SingleModel {
    fn new(random_state: Option<usize>) -> Self;
    fn fit(&mut self, x_train: &Vec<f64>, y_train: &Vec<f64>);
    fn predict(&self, x_axis: &Vec<f64>) -> Vec<f64>;
}