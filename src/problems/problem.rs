use crate::solver::{ExactCover, ExactCoverSolver, ExactCoverSpec};

/// A trait for a problem representable as an exact cover problem.
/// This trait is implemented by various common problems.
pub trait ExactCoverProblem {
    type TSolution;

    /// The associated problem as an exact cover problem spec.
    fn exact_cover_spec(&self) -> ExactCoverSpec;

    /// Transforms a solution from the exact cover solver into a
    /// domain-specific solution.
    fn from_exact_cover_solution(&self, s: &ExactCover) -> Self::TSolution;

    /// Convenience method to chain creation of the problem spec
    /// and creation of a ready solver.
    fn exact_cover_solver(&self) -> ExactCoverSolver {
        ExactCoverSolver::new(&self.exact_cover_spec())
    }

    /// Convenience method to immediately return all solutions
    /// to the problem.
    fn exact_cover_all_solutions(&self) -> Vec<Self::TSolution> {
        let mut solver = self.exact_cover_solver();
        solver.iter_solutions().map(|s| self.from_exact_cover_solution(&s)).collect::<Vec<_>>()
    }
}
