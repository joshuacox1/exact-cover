use super::{
    ExactCover, ExactCoverSolver, ExactCoverProblem, PartialCover
};

/**
 * A trait for a problem representable as an exact cover problem.
 * This trait may be implemented by various common problems such as
 * N queens, sudoku, tiling problems and so on.
.*/
pub trait ExactCoverRepresentable {
    /**
     * A type representing the problem's solution.
     */
    type TSolution;

    /**
     * A type representing a partial solution to the problem.
     * Often this will be the same type as `TSolution`, but not
     * always. For example, a partial solution to a Sudoku problem
     * may have grid cell type `Option<u8>`, whereas a full solution
     * may have grid cell type `u8`.
     *
     * If a partial solution has no meaningful representation or
     * is not relevant for the application in question, set
     * `TPartialSolution` to `()`.
     */
    type TPartialSolution;

    /**
     * Converts the problem into an exact cover problem.
     */
    fn exact_cover_problem(&self) -> ExactCoverProblem;

    /**
     * Converts an exact cover into a domain-specific solution.
     */
    fn from_exact_cover(&self, s: &ExactCover) -> Self::TSolution;

    /**
     * Converts a partial cover into a domain-specific partial
     * solution.
     */
    fn from_partial_cover(&self, s: &PartialCover) -> Self::TPartialSolution;
}
