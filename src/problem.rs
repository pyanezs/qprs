use core::fmt;
use std::collections::{HashMap, HashSet};

use crate::{constraint::Constraint, variables::Variable};

#[derive(Debug)]
pub struct Problem {
    pub name: String,
    variables: Vec<Variable>,
}

impl Problem {
    pub fn new(name: &str) -> Self {
        Problem {
            name: String::from(name),
            variables: Vec::new(),
            // constraints: HashSet::new(),
            // objective: Objective::new(),
        }
    }

    pub fn add_variable(&mut self, name: &str, lower_limit: f64, upper_limit: f64) -> &Variable {
        let var = Variable::new(name, lower_limit, upper_limit);
        self.variables.push(var);
        self.variables.last().unwrap()
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        // self.constraints.insert(constraint);
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
    use crate::variables::Variable;

    #[test]
    fn test_new() {
        let name = "my-problem";
        let problem = Problem::new(name);

        assert_eq!(problem.name, String::from(name));
        assert_eq!(problem.variables.len(), 0);
        // assert_eq!(problem.constraints.len(), 0);
    }

    #[test]
    fn test_add_variable() {
        let mut problem = Problem::new("my-problem");
        assert_eq!(problem.variables.len(), 0);
        problem.add_variable("x", 10.0, 20.0);
        assert_eq!(problem.variables.len(), 1);
        println!("{:?}", problem.variables);
    }
}
