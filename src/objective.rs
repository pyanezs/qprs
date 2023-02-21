use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::variables::Variable;

#[derive(Debug)]
pub struct Objective {
    variable_1: Rc<Variable>,
    variable_2: Option<Rc<Variable>>,
    coeff: f64,
}

impl Objective {
    pub fn quadratic(var1: &Rc<Variable>, var2: &Rc<Variable>, coeff: f64) -> Self {
        let mut vars = vec![Rc::clone(var1), Rc::clone(var2)];
        vars.sort_by_key(|variable| (variable.name.clone()));

        Objective {
            variable_1: Rc::clone(&vars[0]),
            variable_2: Some(Rc::clone(&vars[1])),
            coeff,
        }
    }

    pub fn linear(variable: &Rc<Variable>, coeff: f64) -> Self {
        Objective {
            variable_1: Rc::clone(variable),
            variable_2: None,
            coeff,
        }
    }
}

impl fmt::Display for Objective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.variable_2 {
            Some(x) => write!(f, "{} * {} * {}", self.coeff, self.variable_1.name, x.name),
            None => write!(f, "{} * {}", self.coeff, self.variable_1.name),
        }
    }
}

impl Hash for Objective {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // NOTE This is a crappy solution, but ATM I dont have the
        // knowledge (and the will) to make it better
        match &self.variable_2 {
            Some(x) => {
                self.variable_1.name.hash(state);
                x.name.hash(state);
            }
            None => {
                self.variable_1.name.hash(state);
            }
        }
    }
}

impl PartialEq for Objective {
    fn eq(&self, other: &Self) -> bool {
        // NOTE This is a crappy solution, but ATM I dont have the
        // knowledge (and the will) to make it better
        match (&self.variable_2, &other.variable_2) {
            (None, None) => self.variable_1 == other.variable_1,
            (Some(x), Some(y)) => (&self.variable_1 == &other.variable_1) && (x == y),
            _ => false,
        }
    }
}

impl Eq for Objective {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic_coeff() {
        let x = Variable::new("x", 1.0, 2.0);
        let obj = Objective::quadratic(&x, &x, 10.0);

        assert_eq!(obj.variable_1, x);
        assert_eq!(obj.variable_2, Some(x));
        assert_eq!(obj.coeff, 10.0);
    }

    #[test]
    fn test_quadratic_coeff_mixed_1() {
        let x = Variable::new("x", 1.0, 2.0);
        let y = Variable::new("y", 1.0, 2.0);
        let obj = Objective::quadratic(&x, &y, 10.0);

        assert_eq!(obj.variable_1, x);
        assert_eq!(obj.variable_2, Some(y));
        assert_eq!(obj.coeff, 10.0);
    }

    #[test]
    fn test_quadratic_coeff_mixed_2() {
        let x = Variable::new("x", 1.0, 2.0);
        let y = Variable::new("y", 1.0, 2.0);
        let obj = Objective::quadratic(&y, &x, 14.0);

        assert_eq!(obj.variable_1, x);
        assert_eq!(obj.variable_2, Some(y));
        assert_eq!(obj.coeff, 14.0);
    }

    #[test]
    fn test_linear_coeff() {
        let x = Variable::new("x", 1.0, 2.0);
        let obj = Objective::linear(&x, 10.0);

        assert_eq!(obj.variable_1, x);
        assert_eq!(obj.variable_2, None);
        assert_eq!(obj.coeff, 10.0);
    }

    #[test]
    fn test_eq_linear() {
        let x = Variable::new("x", 1.0, 2.0);
        let y = Variable::new("y", 1.0, 2.0);
        let obj_x = Objective::linear(&x, 1.0);
        let obj_y = Objective::linear(&y, 2.0);

        assert_eq!(obj_x, obj_x);
        assert_eq!(obj_y, obj_y);
        assert_ne!(obj_x, obj_y);
        assert_ne!(obj_y, obj_x);
    }

    #[test]
    fn test_eq_quadratic() {
        let x = Variable::new("x", 1.0, 2.0);
        let y = Variable::new("y", 1.0, 2.0);
        let obj_x = Objective::quadratic(&x, &x, 1.0);
        let obj_y = Objective::quadratic(&y, &y, 2.0);
        let obj_mixed = Objective::quadratic(&x, &y, 3.0);

        assert_eq!(obj_x, obj_x);
        assert_eq!(obj_y, obj_y);
        assert_eq!(obj_mixed, obj_mixed);
        assert_ne!(obj_x, obj_y);
        assert_ne!(obj_x, obj_mixed);
        assert_ne!(obj_y, obj_mixed);
    }

    #[test]
    fn test_eq_quadratic_and_linear() {
        let x = Variable::new("x", 1.0, 2.0);
        let obj_x = Objective::linear(&x, 1.0);
        let obj_x2 = Objective::quadratic(&x, &x, 1.0);

        assert_eq!(obj_x, obj_x);
        assert_eq!(obj_x2, obj_x2);
        assert_ne!(obj_x, obj_x2);
    }
}
