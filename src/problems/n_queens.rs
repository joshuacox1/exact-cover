use std::collections::HashSet;

use itertools::Itertools;

use crate::solver::Solution;



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

/// Relatively brute-force solution to the n queens problem (generate all
/// permutations of N and check for diagonal attacks). It is simple in
/// order to be trustworthy to compare to the output of the solver.
/// If trying larger N, may want to  replace this with a (still
/// pretty simple) backtracking algorithm.
fn n_queens_brute_force(n: usize) -> Vec<Vec<(usize, usize)>> {
    let mut solutions = vec![];
    for xs in (0..n).permutations(n) {
        let queen_points = xs.into_iter().enumerate().collect::<Vec<_>>();
        let valid = valid_queens(queen_points.iter(), n);
        if valid {
            solutions.push(queen_points);
        }
    }
    solutions
}

/// Returns an exact cover problem equivalent to the `n` queens problem.
pub fn as_exact_cover(n: usize)
-> (impl Iterator<Item = impl Iterator<Item = usize>>, usize, usize)
{
    // cartesian product moves the second value first, so in lex order.
    let ones = (0..n)
        .cartesian_product(0..n)
        .map(move |(x,y)| {
            let row_constraint = y;
            let col_constraint = n + x;
            let diag1_constraint = 2*n + x + y;
            let diag2_constraint = 4*n + x + n - y - 2;
            [row_constraint, col_constraint, diag1_constraint, diag2_constraint]
                .into_iter()
        });
    let num_primary_cols = 2*n;
    let num_secondary_cols = 4*n-2;
    (ones, num_primary_cols, num_secondary_cols)
}

/// Given an exact cover problem and size `n`
pub fn from_exact_cover(solution: &Solution, n: usize) -> Vec<(usize, usize)> {
    solution.0.iter()
        .map(|&r| n_queens_row_to_square(r, n))
        .collect::<Vec<_>>()
}

fn n_queens_row_to_square(row: usize, n: usize) -> (usize, usize) {
    (row.div_euclid(n), row.rem_euclid(n))
}

pub struct NQueens;

// impl ExactCoverable<usize, (), Vec<(usize, usize)>> for NQueens {
//     fn to_exact_cover(n: usize)
//         -> Result<(impl Iterator<Item = impl Iterator<Item = usize>>, usize, usize), ()> {

//     }
// }

/// A problem which is .
pub trait ExactCoverable<TArgs, TFail, TSolution> {
    /// Fallibly transforms args into an exact cover problem.
    fn to_exact_cover(t: TArgs) -> Result<(impl Iterator<Item = impl Iterator<Item = usize>>, usize, usize), TFail>;

    /// s.
    fn from_solution(s: &Solution) -> TSolution;
}

#[cfg(test)]
mod test {
    use super::*;

    // What N to test the N queens on. The brute-force generator is relatively
    // simple (generate all permutations) in order to be trustworthy to
    // compare to the output of the solver. If trying larger N, may want to
    // replace this with a (still pretty simple) backtracking algorithm.
    const N_QUEENS: usize = 8;


}

