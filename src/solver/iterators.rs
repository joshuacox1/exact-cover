//! Iterator wrappers over the solver interface.
//! 
//! The solver implements two forms of `next`: next solution and next
//! solver step. As a result it does not implement Iterator and instead
//! has methods `.iter_solutions() and .iter_steps()` that create
//! these iterator wrappers to call next on. This is the same pattern
//! as used in e.g. standard library HashMap.

use super::solver::ExactCoverSolver;
use super::output::{ExactCover, SolverStep};

/// An iterator over the remaining solutions from the state of
/// an `ExactCoverSolver`.
pub struct ExactCoverSolutionIter<'a> {
    pub(super) solver: &'a mut ExactCoverSolver,
}

impl<'a> Iterator for ExactCoverSolutionIter<'a> {
    type Item = ExactCover;

    fn next(&mut self) -> Option<Self::Item> {
        self.solver.next_solution()
    }
}
/// An iterator over the remaining solver steps from the state of
/// an `ExactCoverSolver`.
pub struct ExactCoverStepIter<'a> {
    pub(super) solver: &'a mut ExactCoverSolver,
}

impl<'a> Iterator for ExactCoverStepIter<'a> {
    type Item = SolverStep;

    fn next(&mut self) -> Option<Self::Item> {
        self.solver.next_step()
    }
}
