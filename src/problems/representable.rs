use crate::solver::{ExactCover, ExactCoverSolver, ExactCoverProblem, PartialCover};

/// A trait for a problem representable as an exact cover problem.
/// This trait is implemented by various common problems.
pub trait ExactCoverRepresentable {
    type TSolution;
    type TPartialSolution;

    /// The associated problem as an exact cover problem spec.
    fn exact_cover_problem(&self) -> ExactCoverProblem;

    /// Transforms a solution from the exact cover solver into a
    /// domain-specific solution.
    fn from_exact_cover(&self, s: &ExactCover) -> Self::TSolution;

    /// Transforms a partial (possibly exact) solution from the
    /// exact cover solver into a domain-specific partial solution.
    /// This will often, but not always, look the same as the exact
    /// cover problem.
    fn from_partial_cover(&self, s: &PartialCover) -> Self::TPartialSolution;
}
