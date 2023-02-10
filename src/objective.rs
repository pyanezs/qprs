use std::collections::HashMap;

#[derive(Debug)]
pub struct Objective {
    quadratic_coeffs: HashMap<(String, String), f64>,
    linear_coeffs: HashMap<String, f64>,
}

impl Objective {
    pub fn new() -> Self {
        Objective {
            quadratic_coeffs: HashMap::new(),
            linear_coeffs: HashMap::new(),
        }
    }

    pub fn set_linear_coeff(&mut self, var_name: &str, coeff: f64) {
        self.linear_coeffs.insert(String::from(var_name), coeff);
    }

    pub fn set_quadratic_coeff(&mut self, var_name_1: &str, var_name_2: &str, coeff: f64) {
        // NOTE: I think this could be prettier
        let mut vars = vec![var_name_1, var_name_2];
        vars.sort();
        let vars = (String::from(vars[0]), String::from(vars[1]));
        self.quadratic_coeffs.insert(vars, coeff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let objective = Objective::new();
        let linear_coeffs: HashMap<String, f64> = HashMap::new();
        let quadratic_coeffs: HashMap<(String, String), f64> = HashMap::new();
        assert_eq!(objective.linear_coeffs, linear_coeffs);
        assert_eq!(objective.quadratic_coeffs, quadratic_coeffs);
    }

    #[test]
    fn test_linear_coeff() {
        let mut objective = Objective::new();
        objective.set_linear_coeff("x", 10.0);

        let mut linear_coeffs: HashMap<String, f64> = HashMap::new();
        linear_coeffs.insert(String::from("x"), 10.0);

        assert_eq!(objective.linear_coeffs, linear_coeffs);
    }

    #[test]
    fn test_quadratic_coeff() {
        let mut objective = Objective::new();
        objective.set_quadratic_coeff("b", "a", 10.0);

        let coeffs: HashMap<(String, String), f64> =
            HashMap::from([((String::from("a"), String::from("b")), 10.0)]);
        assert_eq!(objective.quadratic_coeffs, coeffs);

        let coeffs: HashMap<(String, String), f64> =
            HashMap::from([((String::from("b"), String::from("a")), 10.0)]);
        assert_ne!(objective.quadratic_coeffs, coeffs);
    }
}
