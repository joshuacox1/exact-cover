
/// An error in the specification of the exact cover problem.
/// 
/// The solver checks that the problem is well-formed before attempting
/// to solve it.
pub enum ExactCoverError {
    /// A Vec has been provided whose length was inconsistent with
    /// the number of columns claimed.
    IncorrectRowLength {
        row_index: usize,
        bad_length: usize,
    },
    /// The number of primary columns stated was greater than the number
    /// of columns.
    BadPrimaryColumnCount,
    /// An index for a 1 was given which lies outside of [0, ..., num_cols-1].
    OutOfRangeColumn {
        row_index: usize,
        bad_col_index: usize,
    },
}