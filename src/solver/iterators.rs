//! Iterator wrappers over the solver interface.
//! 
//! The solver implements two forms of `next`: next solution and next
//! solver step. As a result it does not implement Iterator and instead
//! has methods `.iter_solutions() and .iter_steps()` that create
//! these iterator wrappers to call next on. This is the same pattern
//! as used in e.g. standard library HashMap.

use super::input::ExactCoverProblem;
use super::solver::ExactCoverSolver;
use super::types::SolverStep;

/// An iterator over the remaining solutions from the state of
/// an `ExactCoverSolver`.
pub struct ExactCoverSolutionIter<'a, T> {
    pub(super) solver: &'a mut ExactCoverSolver<T>,
}

impl<'a, T: ExactCoverProblem> Iterator for ExactCoverSolutionIter<'a, T> {
    type Item = T::TSolution;

    fn next(&mut self) -> Option<Self::Item> {
        self.solver.next_solution()
    }
}
/// An iterator over the remaining solver steps from the state of
/// an `ExactCoverSolver`.
pub struct ExactCoverStepIter<'a, T> {
    pub(super) solver: &'a mut ExactCoverSolver<T>,
}

impl<'a, T: ExactCoverProblem> Iterator for ExactCoverStepIter<'a, T> {
    type Item = SolverStep<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.solver.next_step()
    }
}
