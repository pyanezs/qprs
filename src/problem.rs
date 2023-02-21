use crate::constraint::Constraint;
use crate::objective::Objective;
use crate::standard_form::StandardForm;
use crate::utils::array2_to_vec;
use crate::variables::Variable;
use osqp::CscMatrix;
use osqp::Problem;
use osqp::Settings;

use core::fmt;
use ndarray::Array1;
use ndarray::Array2;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug)]
pub struct QPProblem {
    pub name: String,
    variables: HashMap<Rc<Variable>, usize>,
    constraints: HashSet<Constraint>,
    objectives: HashSet<Objective>,
}

impl QPProblem {
    pub fn new(name: &str) -> Self {
        QPProblem {
            name: String::from(name),
            variables: HashMap::new(),
            constraints: HashSet::new(),
            objectives: HashSet::new(),
        }
    }

    pub fn add_variable(&mut self, variable: &Rc<Variable>) {
        // TODO Check that each variable is inserted only once!
        let idx = self.variables.len();
        self.variables.insert(Rc::clone(variable), idx);
        let constraint = Constraint::from_variable(&variable);
        self.add_constraint(constraint);
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.insert(constraint);
    }

    pub fn add_objective(&mut self, objective: Objective) {
        self.objectives.insert(objective);
    }

    pub fn to_standard_form(&self) -> StandardForm {
        let (obj_quad, obj_linear) = self.objective();
        let (constraint_coeffs, lower_bound, upper_bound) = self.constraints();

        StandardForm::new(
            obj_quad,
            obj_linear,
            constraint_coeffs,
            lower_bound,
            upper_bound,
        )
    }

    fn objective(&self) -> (Array2<f64>, Array1<f64>) {
        let n_vars = self.variables.len();
        // P Matrix and q vector
        let mut obj_quad = Array2::<f64>::zeros((n_vars, n_vars));
        let mut obj_linear = Array1::<f64>::zeros(n_vars);

        for obj in &self.objectives {
            let var_1_idx = self.get_variable_idx(&obj.variable_1);
            match &obj.variable_2 {
                // NOTE Do I really need to clone the index values?
                // Quadratic part of obj function
                Some(x) => {
                    if &obj.variable_1 == x {
                        // Diagonal element -> x * x
                        obj_quad[[var_1_idx, var_1_idx]] = obj.coeff;
                    } else {
                        // Mixed element -> x * y
                        let var_2_idx = self.get_variable_idx(&x);
                        obj_quad[[var_1_idx, var_2_idx]] = obj.coeff / 2.0;
                        obj_quad[[var_2_idx, var_1_idx]] = obj.coeff / 2.0;
                    }
                }
                // Linear part of objective function
                None => {
                    obj_linear[var_1_idx] = obj.coeff;
                }
            }
        }
        (obj_quad, obj_linear)
    }

    fn constraints(&self) -> (Array2<f64>, Array1<f64>, Array1<f64>) {
        let n_vars = self.variables.len();
        let n_constraints = self.constraints.len();
        let mut constraint_coeffs = Array2::<f64>::zeros((n_constraints, n_vars));
        let mut lower_bound = Array1::<f64>::zeros(n_constraints);
        let mut upper_bound = Array1::<f64>::zeros(n_constraints);

        for (cst_idx, constraint) in self.constraints.iter().enumerate() {
            for (variable, coeff) in &constraint.coeffs {
                let var_idx = self.get_variable_idx(variable);
                constraint_coeffs[[cst_idx, var_idx]] = *coeff;
            }

            lower_bound[cst_idx] = constraint.lower_bound;
            upper_bound[cst_idx] = constraint.upper_bound;
        }

        (constraint_coeffs, lower_bound, upper_bound)
    }
    fn get_variable_idx(&self, variable: &Rc<Variable>) -> usize {
        self.variables.get(variable).unwrap().clone()
    }

    pub fn solve(&self) -> HashMap<String, f64> {
        let standard_form = self.to_standard_form();

        let obj_quadratic_coeffs =
            CscMatrix::from(&array2_to_vec(standard_form.p_matrix)).into_upper_tri();
        let obj_linear_coeffs = standard_form.q_vector.as_slice().unwrap();

        let constraint_coeffs = &array2_to_vec(standard_form.a_matrix);
        let lower_bounds = standard_form.l_vector.as_slice().unwrap();
        let upper_bounds = standard_form.u_vector.as_slice().unwrap();

        let settings = Settings::default().verbose(false);

        // Create an OSQP problem
        let mut prob = Problem::new(
            obj_quadratic_coeffs,
            obj_linear_coeffs,
            constraint_coeffs,
            lower_bounds,
            upper_bounds,
            &settings,
        )
        .expect("failed to setup problem");

        // Solve problem
        let result = prob.solve().x().expect("Failed to solve problem");

        // Build return value
        let mut result_map: HashMap<String, f64> = HashMap::new();
        for (var, idx) in &self.variables {
            result_map.insert(var.name.clone(), result[*idx]);
        }

        return result_map;
    }
}

impl fmt::Display for QPProblem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = String::from("QP Problem\n----------\n");

        // TODO Add objective
        text.push_str("Objective:\n");
        let obj: Vec<String> = self
            .objectives
            .iter()
            .map(|obj| format!("{}", obj))
            .collect();
        text.push_str(&format!("{}\n", obj.join(" + ")));

        text.push_str("Constraints:\n");
        for constraint in &self.constraints {
            let cst = format!("{}\n", constraint);
            text.push_str(&cst);
        }

        write!(f, "{}", text)
    }
}
