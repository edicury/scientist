extern crate koala;

use models::linear::{LinearRegressor};
use crate::models::model::Model;
use crate::arrays::array::{arange, reshape};

pub mod models;
pub mod globals;
pub mod arrays;

use koala::dataframe::df;
use std::ops::Deref;

pub fn test () {
    println!("hello world");

    let mut content = String::new();
    let mut csv = df::read_csv("/Users/edisoncury/Documents/Development/Rust/scientist/src/LF_dataset.csv", &mut content, Some(";"));

    let mut df = csv.get_df();

    let x_train : Vec<Vec<f64>> = df["x"].iter().map(|x| [x.deref().deref().parse().expect("Wrong type")].to_vec()).collect();
    let y_train: Vec<f64> = df["y"].iter().map(|x| x.deref().deref().trim().parse().unwrap()).collect();
    let mut model : LinearRegressor = LinearRegressor::new(None);

    println!("y_train {:?}", y_train);

//     model.fit(x_train, y_train);


//    println!("Model: {:?}", model);
}