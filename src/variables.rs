use crate::constraint::Constraint;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl Variable {
    pub fn new(name: &str, lower_limit: f64, upper_limit: f64) -> Self {
        if lower_limit > upper_limit {
            panic!("lower_limit cannot be bigger than upper_limit")
        }

        Variable {
            name: String::from(name),
            lower_bound: lower_limit,
            upper_bound: upper_limit,
        }
    }

    pub fn to_constraint(&self) -> Constraint {
        let coeffs: HashMap<String, f64> = HashMap::from([(self.name.clone(), 1.0)]);
        Constraint::new(
            &self.name.clone(),
            coeffs,
            self.lower_bound,
            self.upper_bound,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let variable = Variable::new(&String::from("HELLO"), 5.0, 10.0);
        assert_eq!(variable.name, String::from("HELLO"));
        assert_eq!(variable.lower_bound, 5.0);
        assert_eq!(variable.upper_bound, 10.0);
    }

    #[test]
    fn test_to_constraint() {
        let cst = Variable::new(&String::from("x"), 5.0, 10.0).to_constraint();

        assert_eq!("x", cst.name);
        assert_eq!(5.0, cst.lower_bound);
        assert_eq!(10.0, cst.upper_bound);

        let mut coeffs: HashMap<String, f64> = HashMap::new();
        coeffs.insert(String::from("x"), 1.0);
        assert_eq!(coeffs, cst.coeffs);
    }
}
