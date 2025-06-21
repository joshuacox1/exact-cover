use crate::sparse_binary_matrix::SparseBinaryMatrix;

/// Specification of a generalised exact cover problem.
pub struct ExactCoverProblem {
    matrix: SparseBinaryMatrix,
    secondary_columns: usize,
}

impl ExactCoverProblem {
    /// Creates a generalised exact cover problem specification from a sparse
    /// binary matrix and the number of secondary columns.
    /// Returns an `Err` if and only if `secondary_columns` > `matrix.num_cols()`.
    pub fn new(
        matrix: SparseBinaryMatrix,
        secondary_columns: usize,
    ) -> Result<Self, ()> {
        if secondary_columns > matrix.num_cols() {
            Err(())
        } else {
            Ok(Self { matrix, secondary_columns })
        }
    }

    /// The matrix for this exact cover problem.
    pub fn matrix(&self) -> &SparseBinaryMatrix { &self.matrix }

    /// The number of primary columns for this exact cover problem.
    pub fn primary_columns(&self) -> usize {
        self.matrix.num_cols() - self.secondary_columns
    }

    /// The number of secondary columns for this exact cover problem.
    pub fn secondary_columns(&self) -> usize {
        self.secondary_columns
    }

    /// The number of columns in this exact cover problem.
    pub fn columns(&self) -> usize { self.matrix.num_cols() }

    /// The number of rows in this exact cover problem.
    pub fn rows(&self) -> usize { self.matrix.num_rows() }
}
