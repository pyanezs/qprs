use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::variable::Variable;
use std::fmt;

#[derive(Debug)]
pub struct Constraint {
    pub name: String,
    pub coeffs: HashMap<Rc<Variable>, f64>,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl Constraint {
    pub fn new(
        name: &str,
        coeffs: HashMap<Rc<Variable>, f64>,
        lower_bound: f64,
        upper_bound: f64,
    ) -> Self {
        if lower_bound > upper_bound {
            panic!("lower_limit cannot be bigger than upper_limit")
        }

        Constraint {
            name: String::from(name),
            coeffs,
            lower_bound,
            upper_bound,
        }
    }

    pub fn from_variable(variable: &Rc<Variable>) -> Self {
        let coeffs = HashMap::from([(Rc::clone(variable), 1.0)]);
        Constraint {
            name: variable.name.clone(),
            coeffs,
            lower_bound: variable.lower_bound,
            upper_bound: variable.upper_bound,
        }
    }
}

impl Hash for Constraint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // NOTE This is a crappy solution, but ATM I dont have the
        // knowledge (and the will) to make it better
        self.name.hash(state);
    }
}

impl PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        // NOTE This is a crappy solution, but ATM I dont have the
        // knowledge (and the will) to make it better
        self.name == other.name
    }
}

impl Eq for Constraint {}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO Sort variables!
        let expresion: Vec<String> = self
            .coeffs
            .iter()
            .map(|(variable, coeff)| format!("{} * {}", coeff, variable.name))
            .collect();

        write!(
            f,
            "{} <= {} <= {}",
            self.lower_bound,
            expresion.join(" + "),
            self.upper_bound
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn test_new() {
        let x = Variable::new("x", 0.0, 10.0);
        let mut coeffs: HashMap<Rc<Variable>, f64> = HashMap::new();
        coeffs.insert(Rc::clone(&x), 5.0);

        let cst = Constraint::new(&String::from("cst1"), coeffs, 5.0, 10.0);

        assert_eq!(cst.name, String::from("cst1"));
        assert_eq!(cst.lower_bound, 5.0);
        assert_eq!(cst.upper_bound, 10.0);
        assert!(cst.coeffs.contains_key(&x));
    }

    #[test]
    fn test_from_variable() {
        let x = Variable::new("x", 0.0, 10.0);

        let cst = Constraint::from_variable(&x);
        assert_eq!(cst.name, String::from("x"));
        assert_eq!(cst.lower_bound, 0.0);
        assert_eq!(cst.upper_bound, 10.0);

        let coeffs = HashMap::from([(Rc::clone(&x), 1.0)]);
        assert_eq!(cst.coeffs, coeffs);
    }

    #[test]
    fn test_hash() {
        let x = Variable::new("x", 0.0, 10.0);
        let cst = Constraint::from_variable(&x);

        let mut hasher = DefaultHasher::new();
        cst.hash(&mut hasher);
        let variable_hash = hasher.finish();

        let mut hasher = DefaultHasher::new();
        String::from("x").hash(&mut hasher);
        let name_hash = hasher.finish();

        assert_eq!(name_hash, variable_hash);
    }
}
