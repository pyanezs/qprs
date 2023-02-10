use core::fmt;
use std::collections::HashMap;

use crate::{constraint::Constraint, objective::Objective, variables::Variable};

#[derive(Debug)]
pub struct Problem<'a> {
    pub name: String,
    variables: HashMap<String, &'a Variable>,
    constraints: HashMap<String, Constraint>,
    // objective: Objective,
}

impl<'a> Problem<'a> {
    pub fn new(name: &str) -> Self {
        Problem {
            name: String::from(name),
            variables: HashMap::new(),
            constraints: HashMap::new(),
            // objective: Objective::new(),
        }
    }

    pub fn add_variable(&mut self, variable: &'a Variable) {
        let cst = variable.to_constraint();
        self.constraints.insert(cst.name.clone(), cst);
        self.variables.insert(variable.name.clone(), variable);
    }
}

impl<'a> fmt::Display for Problem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut report = format!(
            "Problem Name: {}\nN Variables: {} | N Constraints {}",
            self.name,
            self.variables.len(),
            self.constraints.len()
        );

        for key, value in map:
            report += f"\n{key}: {value}"

        write!(f, "{}", report)
    }
}

// pub fn add_constraint(&mut self, constraint: Constraint) {
//     // TODO Check that constrains has valid variables
//     self.constraints.insert(constraint.name.clone(), constraint);
// }
//
// pub fn set_quadratic_obj_coeff(&mut self, var_name_1: &str, var_name_2: &str, coeff: f64) {
//     // TODO Check that constrains has valid variables
//     self.objective
//         .set_quadratic_coeff(var_name_1, var_name_2, coeff);
// }
//
// pub fn set_linear_obj_coeff(&mut self, var_name, )

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
        let var = Variable::new("x", 10.0, 20.0);
        problem.add_variable(&var);

        println!("{:?}", problem.variables);
        println!("{:?}", var);
    }
}
