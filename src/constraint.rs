use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use std::fmt;

use crate::variables::Variable;

#[derive(Debug)]
pub struct Constraint<'a> {
    pub name: String,
    pub coeffs: HashMap<&'a Variable, f64>,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl<'a> Constraint<'a> {
    pub fn new(
        name: &str,
        coeffs: HashMap<&'a Variable, f64>,
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
}
//
// impl Hash for Constraint {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // This is a crappy solution, but ATM I dont have the
//         // knowledge (and the will) to make it better
//         self.name.hash(state);
//     }
// }
//
// impl PartialEq for Constraint {
//     fn eq(&self, other: &Self) -> bool {
//         // This is a crappy solution, but ATM I dont have the
//         // knowledge (and the will) to make it better
//         self.name == other.name
//     }
// }
//
// impl Eq for Constraint {}
//
impl<'a> fmt::Display for Constraint<'a> {
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

    #[test]
    fn test_new() {
        let x = Variable::new("x", 0.0, 10.0);
        let y = Variable::new("y", 0.0, 10.0);
        let z = Variable::new("z", 0.0, 10.0);

        let mut coeffs: HashMap<&Variable, f64> = HashMap::new();
        coeffs.insert(&x, 10.0);
        coeffs.insert(&y, 1.0);

        let cst = Constraint::new(&String::from("cst1"), coeffs, 5.0, 10.0);

        assert_eq!(cst.name, String::from("cst1"));
        assert_eq!(cst.lower_bound, 5.0);
        assert_eq!(cst.upper_bound, 10.0);

        assert!(!cst.coeffs.contains_key(&z));
        assert!(cst.coeffs.contains_key(&x));
        assert!(cst.coeffs.contains_key(&y));
    }
}
