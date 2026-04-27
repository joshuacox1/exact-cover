use std::time::{Duration, Instant};
use bumpalo::Bump;

use std::thread::sleep;
use itertools::Itertools;

use exact_cover_solver::{
    ExactCoverSolver, ExactCoverRepresentable, SolverStep
};
use exact_cover_solver::NQueens;
use exact_cover_solver::kaleidoscope;


fn main() {
    let o = false; let x = true;
    let matrix = SparseBinaryMatrix::from_arrays([
        [o,o,x,o,x,x,o],
        [x,o,o,x,o,o,x],
        [o,x,x,o,o,x,o],
        [x,o,o,x,o,o,o],
        [o,x,o,o,o,o,x],
        [o,o,o,x,x,o,x],
    ]);
    let problem = ExactCoverProblem::new(matrix, 0).unwrap();

    let mut solver = ExactCoverSolver::new(&problem);
    let mut num_solutions = 0;
    while let Some(next_step) = solver.advance() {
        match next_step {
            PushColumn(c) => println!("Pushing column {c}"),
            PopColumn(c) => println!("Popping column {c}"),
            PushRow(r) => println!("Pushing row {r}"),
            AdvanecRow(r1, r2) => {
                println!("Popping row {r1}");
                println!("Pushing row {r2}");
            },
            PopRow(r) => println!("Popping row {r}"),
            ReportSolution => {
                let (cover, is_exact) = solver.get();
                assert!(is_exact);
                num_solutions += 1;
                println!("Found exact cover: {cover:?}");
            }
        }
    }

    println!("Total number of solutions: {num_solutions}");
    // kaleidoscope::kaleidoscope_cover(&kaleidoscope::HOT_AIR_BALLOON);
}
