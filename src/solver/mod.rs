//! Module containing the exact cover solver and associated input/output types.
//! 
//! TODO: an example of calling the solver.

mod spec;
mod iterators;
mod output;
mod solver;
mod tests;

pub use spec::ExactCoverProblem;
pub use iterators::{ExactCoverSolutionIter, ExactCoverStepIter};
pub use output::{ExactCover, PartialCover, SolverStep};
pub use solver::ExactCoverSolver;
