use std::time::{Duration, Instant};
use bumpalo::Bump;

use std::thread::sleep;
use itertools::Itertools;

use exact_cover_solver::{
    ExactCoverSolver, SolverStep,
    SparseBinaryMatrix, ExactCoverProblem,
};
use exact_cover_solver::kaleidoscope;
use exact_cover_solver::n_queens;


fn main() {
    // let o = false; let x = true;
    // // let matrix = SparseBinaryMatrix::from_2d_array([
    // //     [o,o,x,o,x,x,o],
    // //     [x,o,o,x,o,o,x],
    // //     [o,x,x,o,o,x,o],
    // //     [x,o,o,x,o,o,o],
    // //     [o,x,o,o,o,o,x],
    // //     [o,o,o,x,x,o,x],
    // // ]);
    // // let problem = ExactCoverProblem::new_general(matrix, 0).unwrap();
    // // let problem = n_queens::create_problem(16);
    // // let problem = kaleidoscope

    // let mut solver = ExactCoverSolver::new(&problem);
    // let mut num_solutions = 0;
    // loop {
    //     solver.advance_step();
    //     let next_step = match solver.get_step() {
    //         None => break,
    //         Some(n) => n,
    //     };
    //     match next_step {
    //         // SolverStep::PushColumn { col, size } => println!("Pushing column {col}"),
    //         // SolverStep::PopColumn(c) => println!("Popping column {c}"),
    //         // SolverStep::PushRow(r) => println!("Pushing row {r}"),
    //         // SolverStep::AdvanceRow(r1, r2) => {
    //         //     println!("Popping row {r1}");
    //         //     println!("Pushing row {r2}");
    //         // },
    //         // SolverStep::PopRow(r) => println!("Popping row {r}"),
    //         SolverStep::ReportSolution => {
    //             // let (cover, is_exact) = solver.solution();
    //             num_solutions += 1;
    //             // println!("Found exact cover {cover:?}");
    //         },
    //         _ => (),
    //     }
    // }

    // println!("Total number of solutions: {num_solutions}");
    kaleidoscope::kaleidoscope_cover(&kaleidoscope::HOT_AIR_BALLOON);
}
