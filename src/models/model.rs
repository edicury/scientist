pub trait Model {
    fn new(random_state: Option<usize>) -> Self;
    fn fit(&mut self, x_train: Vec<Vec<f64>>, y_train: Vec<f64>);
    fn predict(&self, x_axis: Vec<Vec<f64>>) -> Vec<Vec<f64>>;
}