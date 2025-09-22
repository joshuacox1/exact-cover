//! Solver test cases.
//! Test cases for generic "problems" implementing `ExactCoverProblem`
//! live in the `problems` module. This is because their generation
//! TODO: have some simple test _step_ cases for fine-grained detail.

use crate::{
    problems::NQueens,
    solver::{ExactCover, ExactCoverSolver, ExactCoverRepresentable,
        ExactCoverProblem, SparseBinaryMatrix}};

pub trait TestCase {
    fn spec(&self) -> ExactCoverProblem;
    fn expected_solutions(&self) -> Vec<ExactCover>;
    fn assert_solution_match(&self) {
        let exp = self.expected_solutions();
        let spec = self.spec();
        let actual_sols = ExactCoverSolver::new(&spec)
            .iter_solutions()
            .map(|mut s| { s.0.sort_unstable(); s })
            .collect::<Vec<_>>();
        // TODO: add solution sorting helper. As it stands this will blow up
        assert_eq!(exp, actual_sols);
    }
}

pub struct KnuthSimple;

impl TestCase for KnuthSimple {
    fn spec(&self) -> ExactCoverProblem {
        let o = false; let x = true;
        let arrays = [
            [o,o,x,o,x,x,o],
            [x,o,o,x,o,o,x],
            [o,x,x,o,o,x,o],
            [x,o,o,x,o,o,o],
            [o,x,o,o,o,o,x],
            [o,o,o,x,x,o,x],
        ];
        let matrix = SparseBinaryMatrix::from_array_2d(arrays);
        ExactCoverProblem::new_general(matrix, 0).unwrap()
    }

    fn expected_solutions(&self) -> Vec<ExactCover> {
        // Only one.
        vec![ExactCover(vec![0,3,4])]
    }
}

pub struct ZeroByZero;

impl TestCase for ZeroByZero {
    fn spec(&self) -> ExactCoverProblem {
        let matrix = SparseBinaryMatrix::from_array_2d::<0, 0>([]);
        ExactCoverProblem::new_general(matrix, 0).unwrap()
    }

    // You might think no solutions, but there is a solution.
    fn expected_solutions(&self) -> Vec<ExactCover> {
        vec![ExactCover(vec![])]
    }
}

pub struct ZeroRowsThreeCols;

impl TestCase for ZeroRowsThreeCols {
    fn spec(&self) -> ExactCoverProblem {
        let matrix = SparseBinaryMatrix::from_array_2d::<0, 3>([]);
        // As long as there is at leat one primary column...
        ExactCoverProblem::new_general(matrix, 2).unwrap()
    }

    fn expected_solutions(&self) -> Vec<ExactCover> {
        vec![]  // None.
    }
}

pub struct ZeroRowsThreeColsAllSecondary;

impl TestCase for ZeroRowsThreeColsAllSecondary {
    fn spec(&self) -> ExactCoverProblem {
        let matrix = SparseBinaryMatrix::from_array_2d::<0, 3>([]);
        // As long as there is at leat one primary column...
        ExactCoverProblem::new_general(matrix, 3).unwrap()
    }

    fn expected_solutions(&self) -> Vec<ExactCover> {
        vec![ExactCover(vec![])]
    }
}

pub struct ThreeRowsZeroCols;

impl TestCase for ThreeRowsZeroCols {
    fn spec(&self) -> ExactCoverProblem {
        let matrix = SparseBinaryMatrix::from_array_2d::<3, 0>([[],[],[]]);
        ExactCoverProblem::new_general(matrix, 3).unwrap()
    }

    fn expected_solutions(&self) -> Vec<ExactCover> {
        // As they are empty...
        [vec![], vec![0], vec![1], vec![2],
            vec![0,1], vec![0,2], vec![1,2], vec![0,1,2]]
            .into_iter()
            .map(ExactCover)
            .collect::<Vec<_>>()
    }
}

// TODO: consider writing a proc attribute macro to generate these on the fly.
// This is especially interesting for when we want to product all tests
// with all invariants.

#[test] fn check_solutions_knuth_simple() { KnuthSimple.assert_solution_match(); }
#[test] fn check_solutions_zero_by_zero() { ZeroByZero.assert_solution_match(); }
#[test] fn check_solutions_zero_rows_three_cols() { ZeroRowsThreeCols.assert_solution_match(); }
#[test] fn check_solutions_zero_rows_three_cols_all_secondary() { ZeroRowsThreeColsAllSecondary.assert_solution_match(); }
#[test] fn check_solutions_three_rows_zero_cols()      { ThreeRowsZeroCols.assert_solution_match(); }
