use osqp::{CscMatrix, Problem, Settings};

fn main() {
    // Define problem data
    let p_matrix = &[[4.0, 1.0], [1.0, 2.0]];
    let q_vector = &[1.0, 1.0];
    let a_matrix = &[[1.0, 1.0], [1.0, 0.0], [0.0, 1.0]];
    let l_vector = &[1.0, 0.0, 0.0];
    let u_vector = &[1.0, 0.7, 0.7];

    // Extract the upper triangular elements of `P`
    let p_matrix = CscMatrix::from(p_matrix).into_upper_tri();

    // Disable verbose output
    let settings = Settings::default().verbose(false);

    // Create an OSQP problem
    let mut prob = Problem::new(p_matrix, q_vector, a_matrix, l_vector, u_vector, &settings)
        .expect("failed to setup problem");

    // Solve problem
    let result = prob.solve();

    // Print the solution
    println!("{:?}", result.x().expect("failed to solve problem"));
}
