//! Module containing the exact cover solver and associated input/output types.
//! 
//! TODO: an example of calling the solver.

mod input;
mod iterators;
mod output;
mod solver;
mod tests;

pub use input::ExactCoverSpec;
pub use iterators::{ExactCoverSolutionIter, ExactCoverStepIter};
pub use output::{ExactCover, SolverStep};
pub use solver::ExactCoverSolver;
