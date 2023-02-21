use std::collections::HashMap;
use std::rc::Rc;

use crate::variables::Variable;

#[derive(Debug)]
pub struct Solution {
    solution: HashMap<String, f64>,
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            solution: HashMap::new(),
        }
    }

    pub fn add_solution_value(&mut self, variable: &Rc<Variable>, value: f64) {}
}
