use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Constraint {
    pub name: String,
    pub coeffs: HashMap<String, f64>,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl Constraint {
    pub fn new(
        name: &str,
        coeffs: HashMap<String, f64>,
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

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expresion: Vec<String> = self
            .coeffs
            .iter()
            .map(|(var_name, coeff)| format!("{} * {}", coeff, var_name))
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
        let mut coeffs: HashMap<String, f64> = HashMap::new();
        coeffs.insert(String::from("var1"), 10.0);
        coeffs.insert(String::from("var2"), 1.0);

        let cst = Constraint::new(&String::from("cst1"), coeffs, 5.0, 10.0);

        assert_eq!(cst.name, String::from("cst1"));
        assert_eq!(cst.lower_bound, 5.0);
        assert_eq!(cst.upper_bound, 10.0);

        println!("{}", cst);
    }
}
