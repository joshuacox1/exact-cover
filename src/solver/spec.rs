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

    /// Creates a new non-generalised exact cover problem specification
    /// from a sparse binary matrix.
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