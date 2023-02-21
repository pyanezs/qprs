use std::{collections::HashMap, rc::Rc};

use crate::{constraint::Constraint, objective::Objective, variables::Variable};

mod constraint;
mod objective;
mod problem;
mod variables;

fn main() {
    // Create a problem
    let mut problem = problem::Problem::new("example");

    // Add variables
    let x = Variable::new("x", 0.0, 20.0);
    let y = Variable::new("y", 0.0, 20.0);
    problem.add_variable(&x);
    problem.add_variable(&y);

    // Add Constraints to the problem
    let coeffs = HashMap::from([(Rc::clone(&x), 1.0), (Rc::clone(&y), 2.0)]);
    let cst_1 = Constraint::new("Cst1", coeffs, 0.0, 25.0);
    problem.add_constraint(cst_1);

    // objective
    let obj1 = Objective::linear(&x, 1.0);
    let obj2 = Objective::linear(&y, 1.0);
    let obj3 = Objective::quadratic(&x, &x, 1.0);
    let obj4 = Objective::quadratic(&x, &x, 1.0);
    problem.add_objective(obj1);
    problem.add_objective(obj2);
    problem.add_objective(obj3);
    problem.add_objective(obj4);

    println!("{:?}", problem);
}
