
/// An error in the specification of the exact cover problem.
/// 
/// As the exact cover problem is well-defined on _all_ boolean
/// matrices, even those of zero length or width, the only errors
/// that can occur do so due to straightforward reasons like
/// inconsistent array bounds.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProblemError {
    /// A column index or count has been provided which is inconsistent
    /// with the stated column count.
    InconsistentColumnCount {
        /// The first row index at which this problem arose.
        bad_row: usize,
        /// The offending column index.
        bad_col: usize,
    },
}