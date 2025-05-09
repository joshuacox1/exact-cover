use std::collections::HashSet;

/// A solution of the solver, consisting of a strictly increasing
/// (?! TODO check) list of row indices constituting an exact cover.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactCover(
    /// The inner solution.
    pub Vec<usize>,
);

/// A single step of the solver.
/// The solver logically holds a stack containing the row indices
/// making up its current provisional solution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolverStep {
    /// Choose a column (constraint) to enumerate over. The solver always
    /// chooses the lowest-indexed column with the fewest satisfying choices.
    ChooseColumn {
        /// The index of the chosen column.
        col: usize,
        /// The number of rows with 1s for this column. This will
        /// be minimal among all columns.
        size: usize,
        /// Other columns, not including `col`, which are of equal minimal
        /// size.
        other_cols: HashSet<usize>,
    },
    /// Pushes a row onto the list forming the current provisional solution.
    PushRow(usize),
    /// Advances the latest row being considered in the current provisional solution.
    AdvanceRow(usize),
    /// Pops the last row from the list forming the current provisional solution.
    PopRow(usize),
    /// Reports that the current stack state forms a solution.
    ReportSolution(ExactCover),
}
