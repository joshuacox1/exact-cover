use crate::sparse_binary_matrix::SparseBinaryMatrix;

use super::input::ExactCoverProblem;

/// Raw specification of a generalised exact cover problem, consisting of
/// a binary matrix and a given split of primary and secondary columns.
/// Secondary columns, if any, are assumed to be placed after all the
/// primary columns.
pub struct ExactCoverSpec {
    matrix: SparseBinaryMatrix,
    secondary_columns: usize,
}

impl ExactCoverSpec {
    /// Creates a generalised exact cover problem specification from a sparse
    /// binary matrix and the number of secondary columns. All secondary
    /// columns are assumed to be placed after all primary columns.
    /// Returns an `Err` if and only if `secondary_columns` > `matrix.num_cols()`.
    pub fn new_general(
        matrix: SparseBinaryMatrix,
        secondary_columns: usize,
    ) -> Result<Self, ()> {
        if secondary_columns > matrix.num_cols() {
            Err(())
        } else {
            Ok(Self { matrix, secondary_columns })
        }
    }

    /// Creates a new non-generalised exact cover problem (i.e. no
    /// secondary columns) specification from a sparse binary matrix.
    pub fn new_standard(matrix: SparseBinaryMatrix) -> Self {
        Self::new_general(matrix, 0).unwrap() // infallible
    }

    /// The matrix for this exact cover problem specification.
    pub fn matrix(&self) -> &SparseBinaryMatrix { &self.matrix }

    /// The number of primary columns for this exact cover problem specification.
    pub fn primary_columns(&self) -> usize { self.matrix.num_cols() - self.secondary_columns }

    /// The number of secondary columns for this exact cover problem specification.
    pub fn secondary_columns(&self) -> usize { self.secondary_columns }

    /// Whether this is a generalised exact cover problem (i.e. has
    /// secondary columns as optional constraints).
    pub fn generalised(&self) -> bool { self.secondary_columns > 0 }
}

/// A solution of the solver. i.e. a list of unique row indices which form an
/// exact cover of the problem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactCover(
    /// The inner solution.
    pub Vec<usize>,
);

/// A partial solution of the solver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialCover(
    /// The inner solution.
    pub Vec<usize>,
);

/// A single step of the solver.
/// The solver logically holds a stack containing the row indices
/// making up its current provisional solution.
///
/// Both column and row operations independently form a logical stack
/// (though this is not stored in explicit form in the solver's internals).
/// Concretely, for columns, starting with an empty stack, `SelectColumn`
/// with `col` appends `col` to the stack and `DeselectColumn` pops
/// its value, which will be the final value in the stack. For rows, `PushRow`
/// and `PopRow` behave similarly, with `AdvanceRow` equivalent to a
/// pop of `before`, then a push of `after`. This row stack will be equivalent
/// at all times to a call to `current_partial_solution()`.
///
/// These invariants are tested in a comprehensive test suite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolverStep<T: ExactCoverProblem> {
    /// Choose a column (constraint) to enumerate over. The solver always
    /// chooses the lowest-indexed column with the fewest satisfying choices.
    SelectColumn {
        /// The index of the chosen column.
        col: usize,
        /// The number of rows with 1s for this column. This will
        /// be minimal among all columns.
        size: usize,
    },
    /// Finished enumerating over this column; bin it.
    DeselectColumn(usize),
    /// Pushes a row onto the list forming the current provisional solution.
    PushRow(usize),
    /// Advances the latest row being considered in the current provisional solution.
    AdvanceRow(usize, usize),
    /// Pops the last row from the list forming the current provisional solution.
    PopRow(usize),
    /// Reports a complete solution.
    ReportSolution(T::TSolution),
}
