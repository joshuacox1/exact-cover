//! Module containing the exact cover solver and associated input/output types.
//! 
//! TODO: an example of calling the solver.

mod input;
mod iterators;
mod output;
mod solver;
mod tests;

pub use input::ExactCoverSpec;
pub use iterators::{ExactCoverSolutionIter, ExactCoverStepIter};
pub use output::{ExactCover, SolverStep};
pub use solver::ExactCoverSolver;

#[cfg(test)]
mod test {
    use crate::sparse_binary_matrix::SparseBinaryMatrix;

    use super::*;
    // use crate::problems::

    

    // // What N to test the N queens on. The brute-force generator is relatively
    // // simple (generate all permutations) in order to be trustworthy to
    // // compare to the output of the solver. If trying larger N, may want to
    // // replace this with a (still pretty simple) backtracking algorithm.
    // const N_QUEENS: usize = 8;

    // fn valid_queens<'a>(queens: impl Iterator<Item = &'a (usize, usize)>, n: usize) -> bool {
    //     let mut rows = HashSet::new();
    //     let mut cols = HashSet::new();
    //     let mut diags1 = HashSet::new();
    //     let mut diags2 = HashSet::new();
    //     for (x,y) in queens {
    //         let row = y; let col = x; let diag1 = x+y; let diag2 = x+n-y;
    //         if rows.contains(&row) { return false; } else { rows.insert(row); }
    //         if cols.contains(&col) { return false; } else { cols.insert(col); }
    //         if diags1.contains(&diag1) { return false; } else { diags1.insert(diag1); }
    //         if diags2.contains(&diag2) { return false; } else { diags2.insert(diag2); }
    //     }
    //     true
    // }

    // fn all_n_queens_solutions(n: usize) -> Vec<Vec<(usize, usize)>> {
    //     let mut solutions = vec![];
    //     for xs in (0..n).permutations(n) {
    //         let queen_points = xs.into_iter().enumerate().collect::<Vec<_>>();
    //         let valid = valid_queens(queen_points.iter(), n);
    //         if valid {
    //             solutions.push(queen_points);
    //         }
    //     }
    //     solutions.sort_unstable();
    //     solutions
    // }

    // fn n_queens_exact_cover_problem(n: usize) -> (impl Iterator<Item = [usize; 4]>, usize, usize) {
    //     let ones = (0..n)
    //         .cartesian_product(0..n)
    //         .map(move |(x,y)| {
    //             let row_constraint = y;
    //             let col_constraint = n + x;
    //             let diag1_constraint = 2*n + x + y;
    //             let diag2_constraint = 4*n + x + n - y - 2;
    //             [row_constraint, col_constraint, diag1_constraint, diag2_constraint]
    //         });
    //     let num_primary_cols = 2*n;
    //     let num_secondary_cols = 4*n-2;
    //     (ones, num_primary_cols, num_secondary_cols)
    // }

    // #[test]
    // fn all_n_queens_by_solver() {
    //     let (ones, num_primary_cols, num_secondary_cols) = n_queens_exact_cover_problem(N_QUEENS);
    //     let mut solver = ExactCoverSolver::from_ones(
    //         ones.into_iter().map(|s| s.into_iter()),
    //         num_primary_cols,
    //         num_secondary_cols).unwrap();
    //     let results = solver.search();
    // }
}
