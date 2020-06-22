use models::linear::{LinearRegressor};
use crate::models::model::Model;
use crate::arrays::array::{arange, reshape};

pub mod models;
pub mod globals;
pub mod arrays;


pub fn test () {
    println!("hello world");

    let model : LinearRegressor = LinearRegressor::new(None);

    println!("Model: {:?}", model);

    let mut X : Vec<Vec<usize>> = Vec::new();
    let y :  Vec<usize> = Vec::new();

    for i in 0..3 {
        let mut x = Vec::new();
        x.push(10);
        X.push(x);
    }

    model.fit(X, y);
}