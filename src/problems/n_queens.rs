use std::{collections::HashSet, fmt::{Debug, Formatter}, fmt};

use itertools::Itertools;

use crate::{
    solver::{
        ExactCover, ExactCoverSolver, ExactCoverProblem, PartialCover,
        ExactCoverRepresentable,
    },
};

/// Create a new representation of an n-queens problem.
#[derive(Debug, Copy, Clone)]
pub struct NQueens(usize);

/// A chessboard square.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoardSquare { pub row: usize, pub column: usize }

impl Debug for BoardSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({},{})", self.row, self.column)
    }
}

impl ExactCoverRepresentable for NQueens {
    type TSolution = Vec<BoardSquare>;
    type TPartialSolution = Self::TSolution;

    fn exact_cover_problem(&self) -> ExactCoverProblem {
        let n = self.0;
        if n > 0 {
            let primary_columns = 2*n;
            let secondary_columns = 4*n-2;
            let num_cols = primary_columns + secondary_columns;
            let ones = (0..n)
                .cartesian_product(0..n)
                .map(move |(x,y)| {
                    let row_constraint = y;
                    let col_constraint = n + x;
                    let diag1_constraint = 2*n + x + y;
                    let diag2_constraint = 4*n + x + n - y - 2;
                    [row_constraint, col_constraint, diag1_constraint, diag2_constraint].into_iter()
                });
            // These are both infallible as constructed above.
            // let matrix = SparseBinaryMatrix::from_sparse_rows(ones, num_cols).unwrap();
            let problem = ExactCoverProblem::new(
                ones, num_cols, secondary_columns,
            ).unwrap();
            problem
        } else {
            // let matrix = SparseBinaryMatrix::from_array_2d::<0, 0>([]);
            let problem = ExactCoverProblem::new(
                std::iter::empty::<std::iter::Empty<_>>(), 0, 0).unwrap();
            problem
        }
    }

    fn from_exact_cover(&self, solution: &ExactCover) -> Self::TSolution {
        self.from_vec(&solution.0)
    }

    fn from_partial_cover(&self, solution: &PartialCover) -> Self::TPartialSolution {
        self.from_vec(&solution.0)
    }
}



fn n_queens_row_to_square(row: usize, n: usize) -> BoardSquare {
    BoardSquare { row: row.div_euclid(n), column: row.rem_euclid(n) }
}

impl NQueens {
    /// Create a new `n`-queens problem for the provided `n`.
    pub fn new(n: usize) -> Self { Self(n) }

    /// The `n` for this `n`-queens problem.
    pub fn n(&self) -> usize { self.0 }

    fn from_vec(&self, v: &[usize]) -> Vec<BoardSquare> {
        let result = v.iter()
            .map(|&r| n_queens_row_to_square(r, self.0))
            .collect::<Vec<_>>();
        result
    }

    /// Relatively brute-force method to generate all solutions (generate all
    /// permutations of N and check for diagonal attacks). It is simple in
    /// order to be trustworthy to compare to the output of the solver.
    /// If trying larger N, may want to  replace this with a (still
    /// pretty simple) backtracking algorithm.
    pub fn brute_force(&self) -> Vec<Vec<BoardSquare>> {
        let n = self.0;
        let mut solutions = vec![];
        for xs in (0..n).permutations(n) {
            let queen_points = xs.into_iter()
                .enumerate()
                .map(|(row,column)| BoardSquare { row, column })
                .collect::<Vec<_>>();
            let valid = valid_queens(queen_points.iter(), n);
            if valid {
                solutions.push(queen_points);
            }
        }
        solutions
    }

    // fn doeverything() {
    //     let problem = NQueens::new(8);
    //     let mut solver = ExactCoverSolver::new(&problem.exact_cover_problem());
    //     let solutions = vec![];
    //     while let Some(soln) = solver.next_solution() {
    //         solutions.insert(problem.from_exact_cover(soln));
    //     }
    // }
}

fn valid_queens<'a>(queens: impl Iterator<Item = &'a BoardSquare>, n: usize) -> bool {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let mut diags1 = HashSet::new();
    let mut diags2 = HashSet::new();
    for BoardSquare { row: x, column: y } in queens {
        let row = y; let col = x; let diag1 = x+y; let diag2 = x+n-y;
        if rows.contains(&row) { return false; } else { rows.insert(row); }
        if cols.contains(&col) { return false; } else { cols.insert(col); }
        if diags1.contains(&diag1) { return false; } else { diags1.insert(diag1); }
        if diags2.contains(&diag2) { return false; } else { diags2.insert(diag2); }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    // Sorts all solutions individually, then sorts the list of solutions itself.
    fn sort_solutions(sol: &mut [Vec<BoardSquare>]) {
        for b in sol.iter_mut() {
            b.sort_unstable();
        }
        sol.sort_unstable();
    }

    fn test_n_queens(n: usize) {
        let q = NQueens::new(n);

        let mut brute = q.brute_force();
        sort_solutions(&mut brute);

        let mut ec = ExactCoverSolver::new(&q.exact_cover_problem())
            .iter_solutions()
            .map(|s| q.from_exact_cover(&s))
            .collect::<Vec<_>>();
        sort_solutions(&mut ec);
        assert_eq!(brute, ec);
    }

    #[test] fn n_queens_0_equal_to_brute_force() { test_n_queens(0); }
    #[test] fn n_queens_1_equal_to_brute_force() { test_n_queens(1); }
    #[test] fn n_queens_2_equal_to_brute_force() { test_n_queens(2); }
    #[test] fn n_queens_3_equal_to_brute_force() { test_n_queens(3); }
    #[test] fn n_queens_4_equal_to_brute_force() { test_n_queens(4); }
    #[test] fn n_queens_5_equal_to_brute_force() { test_n_queens(5); }
    #[test] fn n_queens_6_equal_to_brute_force() { test_n_queens(6); }
    #[test] fn n_queens_7_equal_to_brute_force() { test_n_queens(7); }
    #[test] fn n_queens_8_equal_to_brute_force() { test_n_queens(8); }
}
