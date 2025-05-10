//! All solver test cases.
//! Each test case

use crate::solver::{ExactCover, ExactCoverSolver, ExactCoverSpec};

pub(super) trait TestCase {
    fn spec() -> ExactCoverSpec;
    fn expected_solutions() -> Vec<ExactCover>;
    fn assert_solution_match() {
        let exp = Self::expected_solutions();
        let spec = Self::spec();
        let actual_sols = ExactCoverSolver::all_solutions(&spec);
        // TODO: add solution sorting helper
        assert_eq!(exp, actual_sols);
    }
}