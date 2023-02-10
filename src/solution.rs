use std::collections::HashMap;

#[derive(Debug)]
pub struct Solution {
    solution: HashMap<String, f64>,
}

impl Solution {
    pub fn new(solution: HashMap<String, f64>) -> Self {
        Solution { solution }
    }
}
