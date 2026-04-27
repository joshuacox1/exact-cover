// use bumpalo::{Bump};
use arrayvec::ArrayVec;
use itertools::Itertools;

use super::{
    ExactCoverProblem,
    SolverStep
};

const HEAD: usize = 0;
const UNUSED: usize = usize::MAX;

// TODO: change internal layout so we don't waste space
// for size and row label for non-columns.
// TODO: remove all allocations?
// - The root node only uses left and right.
// - Column nodes use LRUD, col points to themselves so unnecessary,
// no row-label.
// - Ordinary nodes use LRUD col row_label. No size.
// The size of this entire structure is something like
// 1 + numcolumns + numones + the stack + #emptyrows (bounded by rows?)
// + the current working solution
#[derive(Debug)]
struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    col: usize,
    row_label: usize,
    size: usize,
}

/// A state of the generator state machine. This is a stack-ified version
/// of the recursive function in Knuth's paper.
#[derive(Debug)]
enum State {
    StartCall,
    AfterColumnChoice { col_node: usize },
    AfterAddOrReplaceRow { r: usize },
    AfterRemoveRow { col_node: usize },
    ResumeCall,
}

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
#[derive(Debug)]
pub struct ExactCoverSolver {
    /// The nodes of the doubly-linked torus.
    x: Vec<Node>,
    // List of node items constituting the current solution.
    o: Vec<usize>,
    // Buffer into which to map row indices of the above for reporting.
    o_rows: Vec<usize>,
    /// Empty rows. The default behaviour of Algorithm X / Dancing Links
    /// entirely ignores empty rows. For each every solution S we need
    /// to add 2^S solutions, one for each subset of empty rows.
    /// TODO: of course test this.
    empty_rows: Vec<usize>,
    // bounded by num columns. TODO: confirm this is true. Assertions?
    stack: Vec<State>,
    solver_step: Option<SolverStep>,
}

impl ExactCoverSolver {
    /// Creates a new exact cover solver from a problem specification.
    pub fn new(problem: &ExactCoverProblem) -> Self {
        let primary_cols = problem.primary_columns();
        let secondary_cols = problem.secondary_columns();
        let ones = problem.matrix.ordered_points_rows();
        let num_cols = primary_cols + secondary_cols;

        // The root node lives at index 0 of the node list.
        // If there are any columns, right should point at the first one.
        // Not sure what happens if no columns. TODO check.
        let root = Node {
            left: primary_cols,
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
                left: if c < primary_cols { c } else { c+1 },
                right: if c < primary_cols { c+2 } else { c+1 },
                up: c+1,
                down: c+1,
                col: c+1,
                row_label: UNUSED,
                size: 0,
            };
            nodes.push(col_header);
        }

        // The last primary column's right wraps around to head.
        nodes[primary_cols].right = HEAD;

        let mut empty_rows = vec![];
        for (i, row) in ones.enumerate() {
            let mut first_of_row = None;

            for j in row {
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

                nodes[left].right = new_index;
                nodes[first_of_row.unwrap()].left = new_index;
                let last_of_col = nodes[col].up;
                nodes[last_of_col].down = new_index;
                nodes[col].up = new_index;

                nodes[j+1].size += 1;
            }

            if let None = first_of_row {
                empty_rows.push(i);
            }
        }

        Self {
            x: nodes,
            o: vec![0; num_cols],
            // TODO: this probably isn't the correct
            // number when empty rows come into consideration...
            o_rows: vec![0; num_cols],
            empty_rows,
            stack: {
                let mut s = Vec::with_capacity(num_cols);
                s.push(State::StartCall);
                s
            },
            solver_step: None,
        }
    }

    /// Returns the solver's current partial cover.
    /// The second entry's boolean is `true` if and only if
    /// the current partial cover is an exact cover.
    pub fn solution(&mut self) -> (&[usize], bool) {
        let mut k = self.stack.len();
        match self.stack.last() {
            Some(State::AfterAddOrReplaceRow { .. } | State::ResumeCall) => (),
            None => (), // ?????
            // todo: what is this??
            _ => k = unsafe { k.unchecked_sub(1) },
        }

        for i in 0..k {
            let cell_i = self.o[i];
            let row = self.x[cell_i].row_label;
            self.o_rows[i] = row;
        }

        let result = &self.o_rows[0..k];
        let is_solution = matches!(self.solver_step, Some(SolverStep::ReportSolution));
        // TODO: Is it always correct to return an exact cover when HEAD.right == HEAD?
        (result, is_solution)
    }

    /// Advance the solver one step.
    pub fn advance_step(&mut self) {
        while let Some(st) = self.stack.pop() {
            let k = self.stack.len();
            match st {
                State::StartCall => {
                    if self.x[HEAD].right == HEAD {
                        self.solver_step = Some(SolverStep::ReportSolution);
                    } else {
                        let (col_node, size) = self.least_col_with_least_ones();
                        self.stack.push(State::AfterColumnChoice { col_node });
                        self.cover(col_node);

                        self.solver_step = Some(SolverStep::PushColumn {
                            col: unsafe { col_node.unchecked_sub(1) }, size
                        });
                    }

                    return;
                },
                State::AfterColumnChoice { col_node } => {
                    let r = self.x[col_node].down;
                    if r != col_node {
                        let newrow = self.x[r].row_label;
                        self.o[k] = r;

                        self.stack.push(State::AfterAddOrReplaceRow { r });
                        self.solver_step = Some(SolverStep::PushRow(newrow));
                    } else {
                        self.uncover(col_node);
                        self.solver_step = Some(
                            SolverStep::PopColumn(unsafe { col_node.unchecked_sub(1) })
                        );
                    }

                    return;
                },
                State::AfterAddOrReplaceRow { r } => {
                    let mut j = self.x[r].right;
                    while j != r {
                        self.cover(self.x[j].col);
                        j = self.x[j].right;
                    }

                    self.stack.push(State::ResumeCall);
                    self.stack.push(State::StartCall);
                }
                State::ResumeCall => {
                    // Second half of the loop
                    let mut r = self.o[k];
                    let col_node = self.x[r].col;

                    let mut j = self.x[r].left;
                    while j != r {
                        self.uncover(self.x[j].col);

                        j = self.x[j].left;
                    }

                    let previous_row = self.x[r].row_label;

                    r = self.x[r].down;
                    // First half of the loop again. TODO factor out
                    // though now it's a resumption, so we know to
                    // REPLACE and REMOVE
                    if r != col_node {
                        // TODO: factor out duplication of first half
                        // of the loop.
                        let newrow = self.x[r].row_label;
                        self.o[k] = r;

                        self.stack.push(
                            State::AfterAddOrReplaceRow { r }
                        );

                        self.solver_step = Some(SolverStep::AdvanceRow(
                            previous_row, newrow
                        ));
                    } else {
                        self.stack.push(
                            State::AfterRemoveRow { col_node }
                        );

                        self.solver_step = Some(SolverStep::PopRow(previous_row));
                    }

                    return;
                },
                State::AfterRemoveRow { col_node } => {
                    self.uncover(col_node);
                    self.solver_step = Some(SolverStep::PopColumn(col_node-1));
                    return;
                }
            }
        }

        self.solver_step = None;
    }

    /// Get the latest solver step.
    pub fn get_step(&self) -> Option<SolverStep> { self.solver_step }

    // Returns index of the col node NOT THE COLUMN (so plus 1)
    // and the smallest size. Should not be called if HEAD.right == HEAD.
    fn least_col_with_least_ones(&self) -> (usize, usize) {
        let mut s = usize::MAX;
        let mut j = self.x[HEAD].right;
        // We know at this point that HEAD.right != HEAD.
        // otherwise we exit early in search.
        // so we don't have to worry about returning
        // usize::MAX or a non-col here.
        debug_assert!(j != HEAD);

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
}
