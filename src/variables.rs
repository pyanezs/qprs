use crate::constraint::Constraint;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

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
        let coeffs: HashMap<&Self, f64> = HashMap::from([(self, 1.0)]);
        Constraint::new(
            &self.name.clone(),
            coeffs,
            self.lower_bound,
            self.upper_bound,
        )
    }
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // This is a crappy solution, but ATM I dont have the
        // knowledge (and the will) to make it better
        self.name.hash(state);
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        // This is a crappy solution, but ATM I dont have the
        // knowledge (and the will) to make it better
        self.name == other.name
    }
}

impl Eq for Variable {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{hash_map::DefaultHasher, HashSet};

    #[test]
    fn test_new() {
        let variable = Variable::new(&String::from("HELLO"), 5.0, 10.0);
        assert_eq!(variable.name, String::from("HELLO"));
        assert_eq!(variable.lower_bound, 5.0);
        assert_eq!(variable.upper_bound, 10.0);
    }

    #[test]
    fn test_hash() {
        let name = String::from("Hello");
        let variable = Variable::new(&name, 5.0, 10.0);

        let mut hasher = DefaultHasher::new();
        variable.hash(&mut hasher);
        let variable_hash = hasher.finish();

        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let name_hash = hasher.finish();

        assert_eq!(name_hash, variable_hash);
    }

    #[test]
    fn test_hashset() {
        let mut set: HashSet<Variable> = HashSet::new();

        set.insert(Variable::new("x", 5.0, 10.0));
        set.insert(Variable::new("x", 1.0, 10.0));
        set.insert(Variable::new("y", 5.0, 10.0));

        assert_eq!(set.len(), 2);
        assert!(!set.contains(&Variable::new("z", 1.0, 2.0)));
        assert!(set.contains(&Variable::new("x", 1.0, 2.0)));
    }

    #[test]
    fn test_to_constraint() {
        let var = Variable::new(&String::from("x"), 5.0, 10.0);
        let cst = var.to_constraint();

        assert_eq!("x", cst.name);
        assert_eq!(5.0, cst.lower_bound);
        assert_eq!(10.0, cst.upper_bound);

        let mut coeffs: HashMap<&Variable, f64> = HashMap::new();
        coeffs.insert(&var, 1.0);
        assert_eq!(coeffs, cst.coeffs);

        println!("{:?}", cst.coeffs);
    }
}
