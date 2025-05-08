use exact_cover_solver::solver::ExactCoverSolver;


fn main() {
    let o = false; let x = true;
    let problem = [
        [o,o,x,o,x,x,o],
        [x,o,o,x,o,o,x],
        [o,x,x,o,o,x,o],
        [x,o,o,x,o,o,o],
        [o,x,o,o,o,o,x],
        [o,o,o,x,x,o,x],
    ];

    let mut solver = ExactCoverSolver::from_array_2d(problem, 0).unwrap();

    // println!("{:?}", solver);

    solver.search(0);
}
