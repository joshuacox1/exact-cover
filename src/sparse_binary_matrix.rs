//! Module holding the sparse binary matrix type.

/// A sparse binary matrix type. Stored in compressed sparse row (CSR) format.
/// Can have any number of rows and columns, including 0.
/// 
/// The main purpose of this type is to pass into the exact cover solver
/// problem. Hence the interface is a bit thin on the ground. You could
/// easily add functions like number of ones in a row, arbitrary access
/// (which is O(#ones in the given row)), etc. One to do in the future maybe.
/// TODO: Add way more ways to make one! Validate! etc
pub struct SparseBinaryMatrix {
    num_cols: usize,
    cols: Vec<usize>,
    row_starts: Vec<usize>,
}

impl SparseBinaryMatrix {
    /// Creates a sparse binary matrix from an array of arrays.
    pub fn from_array_2d<const ROWS: usize, const COLUMNS: usize>(array: [[bool; COLUMNS]; ROWS]) -> Self {
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

    /// Creates a sparse binary matrix from an iterator of array rows.
    pub fn from_array_rows<const COLUMNS: usize>(rows: impl Iterator<Item = [bool; COLUMNS]>) -> Self {
        let mut cols = vec![];
        let mut row_starts = vec![];
        row_starts.push(0);
        for row in rows {
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

    /// Creates a sparse binary matrix from an iterator of sparse array rows.
    /// Column indices must be strictly increasing for each row and valid.
    pub fn from_sparse_rows(
        rows: impl Iterator<Item = impl Iterator<Item = usize>>,
        num_cols: usize
    ) -> Result<Self, ()> {
        let mut cols = vec![];
        let mut row_starts = vec![];
        // TODO!! Check the rows are strictly increasing and validly col-indexed
        // or this will go badly wrong.
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

    // pub fn from_dense_iter_of_iters

    /// The number of rows of the matrix. May be 0.
    pub fn num_rows(&self) -> usize { self.row_starts.len()-1 }

    /// The number of columns of the matrix. May be 0.
    pub fn num_cols(&self) -> usize { self.num_cols }

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

    /// Returns an iterator of the indices of the array's 1s in lexicographic order.
    pub fn ordered_points(&self) -> impl Iterator<Item = (usize,usize)> {
        self.ordered_points_rows()
            .enumerate()
            .flat_map(|(i, row)| row.map(move |j| (i,j)))
    }

    /// Returns the column indices within a row.
    pub fn get_row(&self, row: usize) -> Option<&[usize]> {
        if row < self.num_rows() {
            let start = self.row_starts[row];
            let end = self.row_starts[row+1];
            Some(&self.cols[start..end])
        } else {
            None
        }
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

    #[test]
    fn test_ordered_points() {
        let o = false; let x = true;
        #[rustfmt::skip]
        let example = [
            [x,x,o,o,o,o],
            [o,x,o,x,o,o],
            [o,o,x,x,x,o],
            [o,o,o,o,o,x],
        ];
        let arr = SparseBinaryMatrix::from_array_2d(example);
        let output = arr.ordered_points()
            .collect::<Vec<_>>();
        assert_eq!(output, vec![(0,0),(0,1),(1,1),(1,3),(2,2),(2,3),(2,4),(3,5)]);
    }
}