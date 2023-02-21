# QP Model

Package to define a QP Problem by using something close to a model language.
Currently `main.rs` implements the same example problem presented in the OSQP crate
docs.

## Run examples

Execute problem defined using library:

```
cargo run --bin problem
```

Execute example from OSQP docs:
```
cargo run --bin osqp-sample
```


## Implemented
- Data structures for problem, variables, constraints, and objectives
- Convert problem to standard form
- Solve problem using OSQP.


## Pendig
- Solution Handler
- Solution evaluator
