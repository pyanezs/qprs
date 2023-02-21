use ndarray::Array1;
use ndarray::Array2;

#[derive(Debug)]
pub struct StandardForm {
    pub p_matrix: Array2<f64>,
    pub q_vector: Array1<f64>,
    pub a_matrix: Array2<f64>,
    pub l_vector: Array1<f64>,
    pub u_vector: Array1<f64>,
}

impl StandardForm {
    pub fn new(
        p_matrix: Array2<f64>,
        q_vecotr: Array1<f64>,
        a_matrix: Array2<f64>,
        l_vector: Array1<f64>,
        u_vector: Array1<f64>,
    ) -> Self {
        StandardForm {
            p_matrix,
            q_vector: q_vecotr,
            a_matrix,
            u_vector,
            l_vector,
        }
    }
}
