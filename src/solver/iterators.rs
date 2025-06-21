use super::solver::ExactCoverSolver;
use super::output::{ExactCover, SolverStep};

/**
 * An iterator over the remaining solutions from an
 * [`ExactCoverSolver`].
 *
 * This `struct` is created by the
 * [`iter_solutions`](ExactCoverSolver::iter_solutions)
 * method on [`ExactCoverSolver`]. See its documentation for more
 * information.
 */
pub struct Solutions<'a> {
    pub(super) solver: &'a mut ExactCoverSolver,
}

impl<'a> Iterator for Solutions<'a> {
    type Item = ExactCover;

    fn next(&mut self) -> Option<Self::Item> {
        self.solver.next_solution()
    }
}
/**
 * An iterator over the remaining solver steps of an
 * [`ExactCoverSolver`].
 *
 * This `struct` is created by the
 * [`iter_steps`](ExactCoverSolver::iter_steps)
 * method on [`ExactCoverSolver`]. See its documentation for more
 * information.
 */
pub struct SolverSteps<'a> {
    pub(super) solver: &'a mut ExactCoverSolver,
}

impl<'a> Iterator for SolverSteps<'a> {
    type Item = SolverStep;

    fn next(&mut self) -> Option<Self::Item> {
        self.solver.next_step()
    }
}
