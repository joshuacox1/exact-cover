
mod solver;
mod problems;

pub use solver::{
    ExactCoverSolver, ExactCoverProblem, SparseBinaryMatrix,
    /*ExactCover, PartialCover,*/ SolverStep,
    // Solutions, SolverSteps,
    // ExactCoverRepresentable,
};

pub use problems::{kaleidoscope, n_queens};
