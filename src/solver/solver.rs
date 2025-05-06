use super::iterators::{ExactCoverSolutionIter, ExactCoverStepIter};
use super::output::{Solution, SolverStep};
use super::node::Node;
use super::ExactCoverError;

/// An exact cover solver.
/// 
/// An `ExactCoverSolver` must be initialised with an exact cover problem.
/// It then holds internal state from which it is able to produce
/// all solutions.
/// 
/// The `ExactCoverSolver` exposes two notions of "next": `.next_solution()`
/// and `.next_step()`. `.next_solution()` runs the solver forward until the
/// next solution is found, if one exists; `.next_step()` runs the solver
/// until the next discrete solver step, if there are any. Calls to these
/// may be interleaved with no problem. The solver also exposes
/// iterator wrapper interfaces via `.iter_solutions()` and `.iter_steps()`.
pub struct ExactCoverSolver {
    x: Vec<Node>,
    // Set of row labels. Think about this a bit more...
    o_for_reporting: Vec<usize>,
    /// Empty rows. The default behaviour of Algorithm X / Dancing Links
    /// entirely ignores empty rows. For each every solution S we need
    /// to add 2^S solutions, one for each subset of empty rows.
    /// TODO: of course test this.
    empty_rows: Vec<usize>,
    solution_count: u64,
    step_count: u64,
}

// A generic value for unused values.
const UNUSED: usize = 0;
const HEAD: usize = 0;

impl ExactCoverSolver {
    pub fn from_2d_array<const M: usize, const N: usize> (
        array2d: [[bool; N]; M],
        num_cols: usize,
        num_primary_cols: Option<usize>,
    ) {
        unimplemented!()
    }

    /// Creates an exact cover solver from a matrix. Each `Vec` contains
    /// a bool.
    /// 
    /// Every constituent `Vec` of matrix must be of length `num_cols`.
    /// Returns an error if this is not satisfied. We must also have
    /// `num_primary_cols` <= `num_cols`.
    pub fn from_matrix(
        matrix: Vec<Vec<bool>>,
        num_cols: usize,
        num_primary_cols: Option<usize>,
    ) -> Result<Self, ExactCoverError> {
        let ones = Self::matrix_to_ones(&matrix, num_cols)?;
        Self::from_ones(ones.into_iter(), num_cols, num_primary_cols)
    }

    /// Turns a matrix of booleans into a Vec of iterators of indices where
    /// the `true` values were.
    fn matrix_to_ones(
        matrix: &[Vec<bool>],
        num_cols: usize)
    -> Result<Vec<impl Iterator<Item = usize>>, ExactCoverError> {
        matrix.iter()
            .enumerate()
            .map(|(j, row)| {
                let l = row.len();
                if l != num_cols {
                    Err(ExactCoverError::IncorrectRowLength {
                        row_index: j,
                        bad_length: l,
                    })
                } else {
                    Ok(row.iter()
                        .enumerate()
                        .filter_map(|(i, b)| b.then_some(i)))
                }
            })
            .collect::<Result<Vec<_>, _>>()
    }

    /// Creates an exact cover solver from an iterator of iterators
    /// of indices where 1s lie.
    pub fn from_ones(
        ones: impl Iterator<Item = impl Iterator<Item = usize>>,
        num_cols: usize,
        num_primary_cols: Option<usize>,
    ) -> Result<Self, ExactCoverError> {
        let num_primary_cols = num_primary_cols.unwrap_or(num_cols);
        if num_primary_cols > num_cols {
            return Err(ExactCoverError::BadPrimaryColumnCount);
        }

        // The root node lives at index 0 of the node list.
        // If there are any columns, right should point at the first one.
        // Not sure what happens if no columns. TODO check.
        let root = Node {
            left: 0,
            right: if num_cols > 0 { 1 } else { 0 },
            up: UNUSED, down: UNUSED,
            col: UNUSED, size: UNUSED, row_label: UNUSED,
        };
        let mut nodes = vec![root];

        // The column headers live at indices 1 to
        // num_cols of the node list. Head nodes above num_primary_cols
        // are secondary columns. Primary col headers point left and right; secondary
        // col headers point left and right to themselves.
        // All col headers point up and down to themselves for now and set size to 0.
        for c in 0..num_cols {
            let col_header = Node {
                left: if c < num_primary_cols { c } else { c+1 },
                right: if c < num_primary_cols { c+2 } else { c+1 },
                up: c+1,
                down: c+1,
                col: c+1,
                row_label: UNUSED,
                size: 0,
            };
            nodes.push(col_header);
        }

        // The last primary column's right wraps around to head.
        nodes[num_primary_cols].right = 0;

        let mut empty_rows = vec![];
        for (i, row) in ones.enumerate() {
            let mut first_of_row = None;

            for j in row {
                if j > num_cols {
                    return Err(ExactCoverError::OutOfRangeColumn {
                        row_index: i,
                        bad_col_index: j,
                    });
                }
                let new_index = nodes.len();

                let left; let right;
                match first_of_row {
                    None => {
                        first_of_row = Some(new_index);
                        left = new_index;
                        right = new_index;
                    },
                    Some(f) => {
                        left = nodes[f].left;
                        right = f;
                    }
                }

                let col = j+1;
                let down = col;
                let up = nodes[col].up;
                let row_label = i;

                let new_node = Node {
                    left, right, up, down, col, row_label, size: UNUSED,
                };
                nodes.push(new_node);

                // If this isn't the first node, make the node to the left
                // point right to this.
                if left != new_index {
                    nodes[left].right = new_index;
                }
                nodes[first_of_row.unwrap()].left = new_index;
                nodes[col].up = new_index;

                nodes[j+1].size += 1;
            }

            if let None = first_of_row {
                empty_rows.push(i);
            }
        }

        Ok(Self {
            x: nodes,
            // think about this... extending as appropriate...
            o_for_reporting: vec![],
            empty_rows,
            solution_count: 0,
            step_count: 0,
        })
    }

    // TODO: port the Javascript to a recursive function here, check it
    // works CORRECTLY, and only then work out how to express it as a generator
    // as below.
    fn search() -> Option<Vec<usize>> {
        unimplemented!()
    }

    /// Return the next solution, or None if there are no more solutions.
    pub fn next_solution(&mut self) -> Option<Solution> {
        unimplemented!()
    }

    /// Return the next solver step.
    pub fn next_step(&mut self) -> Option<SolverStep> {
        unimplemented!()
    }

    /// Returns an iterator through solutions.
    pub fn iter_solutions(&mut self) -> ExactCoverSolutionIter {
        ExactCoverSolutionIter { solver: self }
    }

    /// Returns an iterator through solver steps.
    pub fn iter_steps(&mut self) -> ExactCoverStepIter {
        ExactCoverStepIter { solver: self }
    }

    // TODO. When just solving efficiently, we will simply want to return the col
    // (and maybe the size). But for diagnostic purposes, we may want to return
    // all of the cols of relevant size.
    // returns index of the col node. and the smallest size
    fn col_with_least_ones(&self) -> (usize, usize) {
        // We know at this point that HEAD.right != HEAD.
        // otherwise we exit early in search.
        // so we don't have to worry about returning
        // usize::MAX or a non-col here.
        let mut s = usize::MAX;
        let mut j = self.x[HEAD].right;
        let mut min_col = j;
        while j != HEAD {
            let j_size = self.x[j].size;
            if j_size < s {
                min_col = j;
                s = j_size;
            }

            j = self.x[j].right;
        }

        (min_col, s)
    }

    // Covers a column node c. (This is the actual node, so will have
    // index in [1, ... , num_cols].)
    fn cover(&mut self, c: usize) {
        let r = self.x[c].right;
        self.x[r].left = self.x[c].left;
        let l = self.x[c].left;
        self.x[l].right = self.x[c].right;

        let mut i = self.x[c].down;
        while i != c {
            let mut j = self.x[i].right;
            while j != i {
                let d = self.x[j].down;
                self.x[d].up = self.x[j].up;
                let u = self.x[j].up;
                self.x[u].down = self.x[j].down;
                let c_j = self.x[j].col;
                self.x[c_j].size -= 1;

                j = self.x[j].right;
            }

            i = self.x[i].down;
        }
    }

    // Uncovers a column node c. (This is the actual node, so will have
    // index in [1, ... , num_cols].)
    fn uncover(&mut self, c: usize) {
        let mut i = self.x[c].up;
        while i != c {
            let mut j = self.x[i].left;
            while j != i {
                let c_j = self.x[j].col;
                self.x[c_j].size += 1;
                let d = self.x[j].down;
                self.x[d].up = j;
                let u = self.x[j].up;
                self.x[u].down = j;

                j = self.x[j].left;
            }

            i = self.x[i].up;
        }

        let r = self.x[c].right;
        self.x[r].left = c;
        let l = self.x[c].left;
        self.x[l].right = c;
    }

    /// The number of empty columns in the problem provided to the
    /// exact cover solver. n of these will bloat the number of solutions
    /// by 2^n.
    pub fn solution_count(&self) -> u64 { self.solution_count }

    /// The number of empty columns in the problem provided to the
    /// exact cover solver. n of these will bloat the number of solutions
    /// by 2^n.
    pub fn step_count(&self) -> u64 { self.step_count }
}
