use std::{collections::HashMap, rc::Rc};

use crate::{
    constraint::Constraint, objective::Objective, problem::QPProblem, variables::Variable,
};

mod constraint;
mod objective;
mod problem;
mod solution;
mod standard_form;
mod utils;
mod variables;

fn main() {
    // Create a problem
    let mut problem = QPProblem::new("example");

    // Add variables
    let x = Variable::new("x", 0.0, 0.7);
    let y = Variable::new("y", 0.0, 0.7);
    problem.add_variable(&x);
    problem.add_variable(&y);

    // Add Constraints to the problem
    let coeffs = HashMap::from([(Rc::clone(&x), 1.0), (Rc::clone(&y), 1.0)]);
    let cst_1 = Constraint::new("Cst1", coeffs, 1.0, 1.0);
    problem.add_constraint(cst_1);

    // objective
    let obj = Objective::quadratic(&x, &x, 4.0);
    problem.add_objective(obj);

    let obj = Objective::quadratic(&x, &y, 2.0);
    problem.add_objective(obj);

    let obj = Objective::quadratic(&y, &y, 2.0);
    problem.add_objective(obj);

    let obj = Objective::linear(&x, 1.0);
    problem.add_objective(obj);

    let obj = Objective::linear(&y, 1.0);
    problem.add_objective(obj);

    println!("{}", problem);
    let solution = problem.solve();

    println!("Solution:");
    println!("{:?}", solution);
}
