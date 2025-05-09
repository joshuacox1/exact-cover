use crate::solver::{ExactCoverProblemSpec, ExactCoverSolver, Solution};

/// A trait for a problem representable as an exact cover problem.
/// This trait is implemented by various common problems.
pub trait ExactCoverProblem : Sized {
    type TSolution;

    /// Fallibly transforms an input into an exact cover problem specification.
    fn exact_cover_spec(&self) -> ExactCoverProblemSpec;

    /// Transforms a solution from the exact cover solver into a
    /// domain-specific solution.
    fn from_solution(&self, s: &Solution) -> Self::TSolution;

    /// Convenience method to chain creation of a problem into a ready solver.
    fn solver(&self) -> ExactCoverSolver {
        ExactCoverSolver::new(&self.exact_cover_spec())
    }

    /// Convenience method to immediately return the solutions to an exact
    /// cover problem.
    fn solve(&self) -> Vec<Self::TSolution> {
        let mut solver = self.solver();
        solver.iter_solutions().map(|s| self.from_solution(&s)).collect::<Vec<_>>()
    }
}
