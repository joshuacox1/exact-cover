use crate::sparse_binary_matrix::SparseBinaryMatrix;

use super::{types::ExactCoverSpec, ExactCover, ExactCoverSolver, PartialCover};

/// A generic trait for a problem representable by an exact cover problem.
/// Includes ways to translate it to and from exact cover problems.
pub trait ExactCoverProblem {
    type TSolution;
    type TPartialSolution;

    /// The associated problem as an exact cover problem spec.
    fn exact_cover_spec(&self) -> ExactCoverSpec;

    /// Transforms a solution from the exact cover solver into a
    /// domain-specific solution.
    fn from_exact_cover_solution(&self, s: &ExactCover) -> Self::TSolution;

    /// Transforms a partial solution from the exact cover solver into a
    /// domain-specific partial solution.
    fn from_partial_cover_solution(&self, s: &PartialCover) -> Self::TPartialSolution;

    // /// Convenience method to chain creation of the problem spec
    // /// and creation of a ready solver.
    // fn exact_cover_solverc<T: AsRef<Self>>(this: T) -> ExactCoverSolver<Self> {
    //     ExactCoverSolver::new(this)
    // }

    // /// Convenience method to immediately return all solutions
    // /// to the problem.
    // fn exact_cover_all_solutions(self) -> Vec<Self::TSolution> {
    //     let mut solver = self.exact_cover_solver();
    //     solver.iter_solutions().map(|s| self.from_exact_cover_solution(s)).collect::<Vec<_>>()
    // }
}

impl AsRef<ExactCoverSpec> for ExactCoverSpec {
    fn as_ref(&self) -> &Self { self }
}

impl AsRef<ExactCover> for ExactCover {
    fn as_ref(&self) -> &Self { self }
}

impl AsRef<PartialCover> for PartialCover {
    fn as_ref(&self) -> &Self { self }
}

impl ExactCoverProblem for ExactCoverSpec {
    type TSolution = ExactCover;
    type TPartialSolution = PartialCover;

    fn exact_cover_spec(self) -> ExactCoverSpec {
        self
    }

    fn from_exact_cover_solution(self, s: &ExactCover) -> Self::TSolution {
        s
    }

    fn from_partial_cover_solution(self, s: &PartialCover) -> Self::TPartialSolution {

    }
}