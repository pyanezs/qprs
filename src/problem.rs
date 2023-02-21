use core::fmt;
use std::collections::HashSet;
use std::rc::Rc;

use crate::constraint::Constraint;
use crate::objective::Objective;
use crate::variables::Variable;

#[derive(Debug)]
pub struct Problem {
    pub name: String,
    variables: HashSet<Rc<Variable>>,
    var_constraints: HashSet<Constraint>,
    constraints: HashSet<Constraint>,
    objectives: HashSet<Objective>,
}

impl Problem {
    pub fn new(name: &str) -> Self {
        Problem {
            name: String::from(name),
            variables: HashSet::new(),
            var_constraints: HashSet::new(),
            constraints: HashSet::new(),
            objectives: HashSet::new(),
        }
    }

    pub fn add_variable(&mut self, variable: &Rc<Variable>) {
        self.variables.insert(Rc::clone(variable));
        let constraint = Constraint::from_variable(&variable);
        self.var_constraints.insert(constraint);
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.insert(constraint);
    }

    pub fn add_objective(&mut self, objective: Objective) {
        self.objectives.insert(objective);
    }

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
        let mut text = String::from("QP Problem\n");

        // TODO Add objective
        text.push_str("Objective:\n");
        let obj: Vec<String> = self
            .objectives
            .iter()
            .map(|obj| format!("{}", obj))
            .collect();
        text.push_str(&format!("{}\n", obj.join(" + ")));

        text.push_str("Constraints:\n");
        for constraint in &self.constraints {
            let cst = format!("{}\n", constraint);
            text.push_str(&cst);
        }

        for constraint in &self.var_constraints {
            let cst = format!("{}\n", constraint);
            text.push_str(&cst);
        }

        write!(f, "{}", text)
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
