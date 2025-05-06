//! Module containing the exact cover solver and associated input/output types.
//! 
//! TODO: an example of calling the solver.

mod errors;
mod iterators;
mod output;
mod node;
mod solver;

pub use errors::ExactCoverError;
pub use iterators::{ExactCoverSolutionIter, ExactCoverStepIter};
pub use output::{Solution, SolverStep};
pub use solver::ExactCoverSolver;