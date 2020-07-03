# scientist

##### Machine Learning library for Rust

## Objective
Create Machine Learning abstractions to facilitate `ML` pipelines.

This package does not use interop with `C` or `Python`, it is meant to be written 100% in `Rust`.
## crate - latest: 0.1.2
```rust
[dependencies]
scientist = "0.1.2"
```


## Avaiable on this package

Linear Models
    
    - Linear Regression ( Single dependant variable )
    - Linear Classification ( Single dependant variable )
    
## Usage

LinearRegressor
```rust
    extern crate scientist;
    use scientist::models::linear::{LinearRegression};

       let x_train : Vec<Vec<f64>> = [[1.1], [1.3], [1.5], [2.0], [2.2], [2.9], [3.0]].to_vec().iter().map(|x| x.to_vec()).collect();
       let y_train : Vec<f64> = [39343., 46205., 37731., 43525., 39891., 56642., 60150.].to_vec();
   
       let mut model : LinearRegression = LinearRegression::new();
   
       model.fit(&x_train, &y_train);
   
       let preds = model.predict(&[[1.5].to_vec()].to_vec());
   
       println!("Prediction {:?}", preds); // Prediction: $41434.737394958
```    
    


## TBD
RandomForest
    
    - RandomForest Regression
    - RandomForest Classification
    
Reinforcement Algorithms
    
    - UCB
    - Thompson Sampling

XGBoost

    - XGBoost Regression
    - XGBoost Classification