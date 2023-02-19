use std::collections::HashMap;

use crate::constraint::Constraint;
mod constraint;
mod objective;
mod problem;
mod variables;

fn main() {
    // Create a problem and add variables to it
    let mut problem = problem::Problem::new("example");

    // Add variables
    let x = problem.add_variable("X", 0.0, 20.0);
    let y = problem.add_variable("Y", 0.0, 25.0);

    // problem.add_variable(&x);
    // problem.add_variable(&y);
    //
    // // TODO Add Constraints to the problem
    // let mut coeffs: HashMap<&variables::Variable, f64> = HashMap::new();
    // coeffs.insert(&x, 1.0);
    // coeffs.insert(&y, 1.0);
    // let cst = Constraint::new("Cst1", coeffs, 5.0, 15.0);
    //
    // println!("{:?}", problem);
    println!("{:?}", x);
    // println!("{:?}", y);

    // TODO Solve the problem
    // TODO Evaluate solution
}
