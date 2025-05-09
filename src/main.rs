use std::{collections::HashSet, ops::Rem};

use exact_cover_solver::solver::ExactCoverSolver;
use itertools::Itertools;

use exact_cover_solver::problems::*;

fn main() {
    // let (ones, num_primary_cols, num_secondary_cols) = n_queens_exact_cover_problem(8);
    // let mut solver = ExactCoverSolver::from_traitversion(
    //     ones,
    //     num_primary_cols,
    //     num_secondary_cols).unwrap();
    // let results = solver.search();

    // println!("{}", results.len());
    // for result in results {
    //     let c = result.0.iter()
    //         .map(|&row| n_queens_exact_cover_row_undo(row, 8))
    //         .collect::<Vec<_>>();
    //     println!("{c:?}");
    // }

    // let mut qwer = (0..8).cartesian_product(0..8).collect::<Vec<_>>();
    // qwer.sort_unstable();
    // println!("{:?}", qwer);

    // let o = false; let x = true;
    // let problem = [
    //     [o,o,x,o,x,x,o],
    //     [x,o,o,x,o,o,x],
    //     [o,x,x,o,o,x,o],
    //     [x,o,o,x,o,o,o],
    //     [o,x,o,o,o,o,x],
    //     [o,o,o,x,x,o,x],
    // ];

    // let mut solver = ExactCoverSolver::from_traitversion(problem, 7, 0).unwrap();

    // // println!("{:?}", solver);

    // // solver.search(0);

    // let mut k = all_n_queens_solutions(8);
    // k.sort();
    // for sol in k {
    //     println!("{sol:?}");
    // }
}


