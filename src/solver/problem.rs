/// Specification of a generalised exact cover problem.
pub struct ExactCoverProblem {
    pub matrix: SparseBinaryMatrix,
    num_secondary_columns: usize,
}

/// Something that can go wrong while constructing an
/// exact cover problem.
#[derive(Debug)]
pub enum ExactCoverProblemError {
    /// Column indices of 1s within a row were out of order.
    ColumnIndicesOutOfOrder { row_idx: usize },
    /// The column index of a 1 within a row was out of bounds
    /// given the number of columns passed in.
    ColumnIndexOutOfBounds { row_idx: usize, col_idx: usize },
    /// The number of secondary columns passed in was greater than
    /// the number of columns passed in.
    TooManySecondaryColumns,
}

impl ExactCoverProblem {
    /// Creates a generalised exact cover problem from a sparse
    /// binary matrix and the number of columns and secondary columns.
    /// Returns an Ok under
    pub fn new(
        row_indices: impl Iterator<Item = impl Iterator<Item = usize>>,
        num_columns: usize,
        num_secondary_columns: usize,
    ) -> Result<Self, ExactCoverProblemError> {
        // TODO: Don't unwrap?!
        let matrix = SparseBinaryMatrix::from_sparse_rows(
            row_indices, num_columns).unwrap();
        if num_secondary_columns > matrix.num_cols {
            Err(ExactCoverProblemError::TooManySecondaryColumns)
        } else {
            Ok(Self { matrix, num_secondary_columns })
        }
    }

    // TODO: delete or replace the other one with this
    pub fn new_general(matrix: SparseBinaryMatrix,
        num_secondary_columns: usize) -> Result<Self, ()> {
        // todo check validity
        Ok(Self { matrix, num_secondary_columns })
    }


    /// Returns an iterator of rows, which are themselves iterators
    /// over the 1s of the array.
    pub fn ordered_points_rows(&self)
        -> impl Iterator<Item = impl Iterator<Item = usize>>
    {
        // TODO: make this more efficient?
        (0..(self.matrix.row_starts.len()-1))
            .map(|r| {
                let start = self.matrix.row_starts[r];
                let end = self.matrix.row_starts[r+1];
                (start..end).map(|i| self.matrix.cols[i])
            })
    }

    // /// Gets the state with the given row and column index.
    // /// Returns None if the indices are out of bounds.
    // pub fn get(&self, row_idx: usize, col_idx: usize) -> Option<bool> {

    // }

    /// The number of rows in this exact cover problem.
    #[inline]
    pub fn rows(&self) -> usize { self.matrix.num_rows() }

    /// The number of columns in this exact cover problem.
    #[inline]
    pub fn columns(&self) -> usize { self.matrix.num_cols }

    /// The number of secondary columns for this exact cover problem.
    #[inline]
    pub fn primary_columns(&self) -> usize {
        self.columns() - self.num_secondary_columns
    }

    /// The number of secondary columns for this exact cover problem.
    #[inline]
    pub fn secondary_columns(&self) -> usize {
        self.num_secondary_columns
    }

    #[inline]
    pub fn num_ones(&self) -> usize {
        self.matrix.cols.len()
    }
}

/// A sparse binary matrix type. Stored in compressed sparse row
/// (CSR) format.
/// Can have any number of rows and columns, including 0.
pub struct SparseBinaryMatrix {
    num_cols: usize,
    cols: Vec<usize>,
    row_starts: Vec<usize>,
}

impl SparseBinaryMatrix {
    /// Creates a sparse binary matrix from an array of arrays.
    pub fn from_array_2d<const ROWS: usize, const COLUMNS: usize>(
        array: [[bool; COLUMNS]; ROWS]
    ) -> Self {
        let mut cols = vec![];
        let mut row_starts = Vec::with_capacity(ROWS+1);
        row_starts.push(0);
        for row in array.iter() {
            for (i, &item) in row.iter().enumerate() {
                if item {
                    cols.push(i);
                }
            }
            row_starts.push(cols.len());
        }

        Self {
            num_cols: COLUMNS,
            cols,
            row_starts,
        }
    }

    /// Creates a sparse binary matrix from an iterator of sparse
    /// array rows.
    /// Column indices must be strictly increasing for each row and valid.
    pub fn from_sparse_rows(
        rows: impl Iterator<Item = impl Iterator<Item = usize>>,
        num_cols: usize
    ) -> Result<Self, ()> {
        let mut cols = vec![];
        let mut row_starts = vec![];
        // TODO!! Check the rows are strictly increasing and validly
        // col-indexed or this will go badly wrong.
        // For now just trust the user doesn't fuck it up.
        row_starts.push(0);
        for row in rows {
            cols.extend(row);
            row_starts.push(cols.len());
        }

        Ok(Self {
            num_cols,
            cols,
            row_starts,
        })
    }

    pub fn num_rows(&self) -> usize {
        self.row_starts.len().checked_sub(1).unwrap()
    }

    /// Returns an iterator of rows, which are themselves iterators
    /// over the 1s of the array.
    pub fn ordered_points_rows(&self) -> impl Iterator<Item = impl Iterator<Item = usize>> {
        // TODO: make this more efficient?
        (0..(self.row_starts.len()-1))
            .map(|r| {
                let start = self.row_starts[r];
                let end = self.row_starts[r+1];
                (start..end).map(|i| self.cols[i])
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ordered_points_rows() {
        let o = false; let x = true;
        #[rustfmt::skip]
        let example = [
            [x,x,o,o,o,o],
            [o,x,o,x,o,o],
            [o,o,x,x,x,o],
            [o,o,o,o,o,x],
        ];
        let arr = SparseBinaryMatrix::from_array_2d(example);
        let output = arr.ordered_points_rows()
            .map(|row| row.collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(output, vec![vec![0,1],vec![1,3],vec![2,3,4],vec![5]]);
    }

    // #[test]
    // fn test_ordered_points() {
    //     let o = false; let x = true;
    //     #[rustfmt::skip]
    //     let example = [
    //         [x,x,o,o,o,o],
    //         [o,x,o,x,o,o],
    //         [o,o,x,x,x,o],
    //         [o,o,o,o,o,x],
    //     ];
    //     let arr = SparseBinaryMatrix::from_array_2d(example);
    //     let output = arr.ordered_points()
    //         .collect::<Vec<_>>();
    //     assert_eq!(output, vec![(0,0),(0,1),(1,1),(1,3),(2,2),(2,3),(2,4),(3,5)]);
    // }
}
