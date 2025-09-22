
mod solver;
mod problems;

pub use solver::{
    ExactCoverSolver, ExactCoverProblem,
    ExactCover, PartialCover, SolverStep,
    // Solutions, SolverSteps,
    ExactCoverRepresentable,
};

pub use problems::{NQueens, kaleidoscope};
