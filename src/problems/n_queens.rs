use std::{collections::HashSet, fmt::{Debug, Formatter}, fmt};

use itertools::Itertools;

use crate::{
    solver::{
        ExactCoverSolver, ExactCoverProblem, SolverStep,
    },
};

/// Create an exact cover problem representing the N queens problem.
pub fn create_problem(n: usize) -> ExactCoverProblem {
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
        let problem = ExactCoverProblem::new(
            ones, num_cols, secondary_columns,
        ).unwrap();
        problem
    } else {
        let problem = ExactCoverProblem::new(
            std::iter::empty::<std::iter::Empty<_>>(), 0, 0).unwrap();
        problem
    }
}

/// Convert a row of the N queens exact cover problem to the
/// board square it represents.
pub fn row_to_square(n: usize, row: usize) -> (usize, usize) {
    (row.div_euclid(n), row.rem_euclid(n))
}

fn n_queens_brute_force(n: usize) -> Vec<Vec<(usize, usize)>> {
    let mut solutions = vec![];
    for xs in (0..n).permutations(n) {
        let queen_points = xs.into_iter()
            .enumerate()
            .collect::<Vec<_>>();
        let valid = valid_queens(queen_points.iter(), n);
        if valid {
            solutions.push(queen_points);
        }
    }
    solutions
}

fn valid_queens<'a>(queens: impl Iterator<Item = &'a (usize, usize)>, n: usize) -> bool {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let mut diags1 = HashSet::new();
    let mut diags2 = HashSet::new();
    for (x,y) in queens {
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
    fn sort_solutions(sol: &mut [Vec<(usize, usize)>]) {
        for b in sol.iter_mut() {
            b.sort_unstable();
        }
        sol.sort_unstable();
    }

    fn test_n_queens(n: usize) {
        let mut brute_force_solutions = n_queens_brute_force(n);
        sort_solutions(&mut brute_force_solutions);

        let problem = create_problem(n);
        let mut solver = ExactCoverSolver::new(&problem);
        let mut ec_solutions = vec![];
        loop {
            solver.advance_step();
            match solver.get_step() {
                None => break,
                Some(SolverStep::ReportSolution) => {
                    let (solution, _) = solver.solution();
                    let squares = solution.iter()
                        .map(|&row| row_to_square(n, row))
                        .collect::<Vec<_>>();
                    ec_solutions.push(squares);
                },
                _ => (),
            }
        }
        sort_solutions(&mut ec_solutions);
        assert_eq!(brute_force_solutions, ec_solutions);
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
