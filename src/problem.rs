use core::fmt;
use std::collections::HashSet;
use std::rc::Rc;

use crate::constraint::Constraint;
// use crate::objective::Objective;
use crate::variables::Variable;

#[derive(Debug)]
pub struct Problem {
    pub name: String,
    variables: HashSet<Rc<Variable>>,
    constraints: HashSet<Constraint>,
    // objective: Objective,
}

impl Problem {
    pub fn new(name: &str) -> Self {
        Problem {
            name: String::from(name),
            variables: HashSet::new(),
            constraints: HashSet::new(),
            // objective: Objective::new(),
        }
    }

    pub fn add_variable(&mut self, variable: &Rc<Variable>) {
        self.variables.insert(Rc::clone(variable));
        let constraint = Constraint::from_variable(&variable);
        self.add_constraint(constraint);
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.insert(constraint);
    }

    // pub fn add_obj_quadratic_coeff(
    //     &mut self,
    //     variable_1: &'a Variable,
    //     variable_2: &'a Variable,
    //     coeff: f64,
    // ) {
    //     // TODO Implement!
    // }
    //
    // // pub fn add_obj_linear_coeff(&mut self, variable: &'a Variable, coeff: f64) {
    //     // TODO Implement!
    // }
    //
    // pub fn solve(self) -> Solution {
    //     // TODO Implement!
    // }
    //
    // pub fn evaluate(self, Solution) {
    //     // TODO Implement!
    // }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QP Problem\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = "my-problem";
        let problem = Problem::new(name);

        assert_eq!(problem.name, String::from(name));
        // assert_eq!(problem.variables.len(), 0);
        // // assert_eq!(problem.constraints.len(), 0);
    }

    // #[test]
    // fn test_add_variable() {
    //     let mut problem = Problem::new("my-problem");
    //     assert_eq!(problem.variables.len(), 0);
    //     problem.add_variable("x", 10.0, 20.0);
    //     assert_eq!(problem.variables.len(), 1);
    //     println!("{:?}", problem.variables);
    // }
}
