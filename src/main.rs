use std::collections::HashMap;
mod constraint;
mod objective;
mod problem;
mod variables;

fn main() {
    let x = variables::Variable::new("X", 0.0, 20.0);
    let y = variables::Variable::new("Y", 0.0, 25.0);

    let mut problem = problem::Problem::new("example");
    problem.add_variable(&x);
    problem.add_variable(&y);

    println!("{:?}", problem);

    // Problem Name: example
    // N Variables: 2 | N Constraints 2
    // CST NAME : 0.0 <= x <= 20.0 -> x
    // 0.0 <= 10 * y <= 25.0 -> y
    // 0.0 <= 10 * x + 1 * y <= 20 -> Fuse 1
    //
    //
    //
}
