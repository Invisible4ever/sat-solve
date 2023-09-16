use satsolve;
use satsolve::solver::Solver;




#[test]
fn test_solver_sat() {
    let mut solver = Solver::new(vec![], 2);
    solver.add_clause(vec![1, 2]);
    solver.add_clause(vec![-1, 2]);
    solver.add_clause(vec![1, -2]);
    assert_eq!(solver.solve(),Ok(())); 
}

#[test]
fn test_solver_unsat() {
    let mut solver = Solver::new(vec![], 2);
    solver.add_clause(vec![1, 2]);
    solver.add_clause(vec![-1, 2]);
    solver.add_clause(vec![1, -2]);
    solver.add_clause(vec![-1, -2]);
    assert_eq!(solver.solve(),Err(())); 
}
