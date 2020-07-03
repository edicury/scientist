pub trait Model {
    fn new() -> Self;
    fn fit(&mut self, x_train: &Vec<Vec<f64>>, y_train: &Vec<f64>);
    fn predict(&self, x_validation: &Vec<Vec<f64>>) -> Vec<f64>;
}