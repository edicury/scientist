pub trait Model {
    fn new(random_state: Option<usize>) -> Self;
    fn fit(&self, x_train: Vec<Vec<usize>>, y_train: Vec<usize>);
    fn predict(&self, x_axis: Vec<Vec<usize>>) -> Vec<usize>;
}