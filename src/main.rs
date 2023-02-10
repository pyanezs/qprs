use std::collections::HashMap;
mod constraint;
mod objective;
mod problem;
mod variables;

fn main() {
    println!("Hello, world!");

    // let vars = vec![
    //     variables::Variable::new("X", 0.0, 20.0),
    //     variables::Variable::new("y", 0.0, 25.0),
    // ];
    //
    // let my_problem = problem::Problem::new("MyProb", vars);
    //
    //

    let obj = objective::Objective::new();

    println!("{:?}", obj);
}
