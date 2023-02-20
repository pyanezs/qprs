use std::fmt;
use std::rc::Rc;

use crate::variables::Variable;

#[derive(Debug)]
pub struct Objective {
    var_1: Rc<Variable>,
    var_2: Option<Rc<Variable>>,
    coeff: f64,
}

impl Objective {
    pub fn quadratic(var1: &Rc<Variable>, var2: &Rc<Variable>, coeff: f64) -> Self {
        let mut vars = vec![Rc::clone(var1), Rc::clone(var2)];
        vars.sort_by_key(|variable| (variable.name.clone()));

        Objective {
            var_1: Rc::clone(&vars[0]),
            var_2: Some(Rc::clone(&vars[1])),
            coeff,
        }
    }

    pub fn linear(variable: &Rc<Variable>, coeff: f64) -> Self {
        Objective {
            var_1: Rc::clone(variable),
            var_2: None,
            coeff,
        }
    }
}

impl fmt::Display for Objective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.var_2 {
            Some(x) => write!(f, "{} * {} * {}", self.coeff, self.var_1.name, x.name),
            None => write!(f, "{} * {}", self.coeff, self.var_1.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic_coeff() {
        let x = Variable::new("x", 1.0, 2.0);
        let obj = Objective::quadratic(&x, &x, 10.0);

        assert_eq!(obj.var_1, x);
        assert_eq!(obj.var_2, Some(x));
        assert_eq!(obj.coeff, 10.0);
    }

    #[test]
    fn test_quadratic_coeff_mixed_1() {
        let x = Variable::new("x", 1.0, 2.0);
        let y = Variable::new("y", 1.0, 2.0);
        let obj = Objective::quadratic(&x, &y, 10.0);

        assert_eq!(obj.var_1, x);
        assert_eq!(obj.var_2, Some(y));
        assert_eq!(obj.coeff, 10.0);
    }

    #[test]
    fn test_quadratic_coeff_mixed_2() {
        let x = Variable::new("x", 1.0, 2.0);
        let y = Variable::new("y", 1.0, 2.0);
        let obj = Objective::quadratic(&y, &x, 14.0);

        assert_eq!(obj.var_1, x);
        assert_eq!(obj.var_2, Some(y));
        assert_eq!(obj.coeff, 14.0);
    }

    #[test]
    fn test_linear_coeff() {
        let x = Variable::new("x", 1.0, 2.0);
        let obj = Objective::linear(&x, 10.0);

        assert_eq!(obj.var_1, x);
        assert_eq!(obj.var_2, None);
        assert_eq!(obj.coeff, 10.0);
    }
}
