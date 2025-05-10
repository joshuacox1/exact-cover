use itertools::Itertools;

use super::{
    ExactCoverSpec, ExactCoverSolutionIter, ExactCoverStepIter,
    ExactCover, SolverStep,
};

// TODO: at some point remove size and row_label
// for non-columns. It's a waste of space for most of the nodes.
#[derive(Debug)]
struct Node {
    pub left: usize,
    pub right: usize,
    pub up: usize,
    pub down: usize,
    pub col: usize,
    pub row_label: usize,
    pub size: usize,
}

/// A state of the generator state machine.
#[derive(Debug)]
enum FinalState {
    Start,
    AfterColumnChoice { col_node: usize },
    AfterAddOrReplaceRow { r: usize },
    AfterRemoveRow { col_node: usize },
    Resume,
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
    x: Vec<Node>,
    // Set of row labels. Think about this a bit more...
    o_for_reporting: Vec<usize>,
    /// Empty rows. The default behaviour of Algorithm X / Dancing Links
    /// entirely ignores empty rows. For each every solution S we need
    /// to add 2^S solutions, one for each subset of empty rows.
    /// TODO: of course test this.
    empty_rows: Vec<usize>,
    counter_solutions: u64,
    counter_steps: u64,

    k: usize,
    stack: Vec<FinalState>,
}

// A generic value for unused values.
const UNUSED: usize = usize::MAX;
const HEAD: usize = 0;

impl ExactCoverSolver {
    /// Creates a new exact cover solver from a problem specification.
    pub fn new(problem: &ExactCoverSpec) -> Self {
        let primary_cols = problem.primary_columns();
        let secondary_cols = problem.secondary_columns();
        let ones = problem.matrix().ordered_points_rows();
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
        nodes[primary_cols].right = 0;

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
            // think about this... extending as appropriate...
            o_for_reporting: vec![0; num_cols],
            empty_rows,
            counter_solutions: 0,
            counter_steps: 0,
            k: 0,
            stack: vec![FinalState::Start],
        }
    }

    /// Get all solutions to the exact cover problem at once.
    /// This function *may* be faster than the generator-written versions
    /// as it does no auxiliary book-keeping. I'm not sure how much the compiler
    /// can optimise those either. One to benchmark.
    /// If it's not faster, delete this function.
    pub fn search_rec(&mut self) -> Vec<ExactCover> {
        let mut solutions = vec![];
        self.search_rec_inner(0, &mut solutions);
        solutions
    }

    /// Source of truth function for steps as it's simple and recursive.
    /// Should equal the generator version always.
    /// Delete this once sufficiently confident in the generator version.
    pub fn step_simple(&mut self) -> Vec<SolverStep> {
        let mut steps = vec![];
        self.step_simple_inner(0, &mut steps);
        steps
    }

    fn step_simple_inner(&mut self, k: usize, steps: &mut Vec<SolverStep>) {
        if self.x[HEAD].right == HEAD {
            let solution = self.o_for_reporting.iter()
                .take(k)
                .map(|&r| self.x[r].row_label)
                .collect::<Vec<_>>();

            // Hahahaha empty rows. Make this work in stateful land!
            // This also breaks the assumptions that rows are added
            // and removed like a stack. No way I'm pretending to add
            // and remove them at the end. Just accept that empty rows get powersetted
            // on at the end.

            // Logically unnecessary as the powerset of the empty set works
            // fine, but to avoid unnecessary work.
            if self.empty_rows.len() > 0 {
                for sub in self.empty_rows.iter().powerset() {
                    let mut new_solution = solution.clone();
                    new_solution.extend(sub.iter().map(|&a| a));
                    steps.push(SolverStep::ReportSolution(ExactCover(new_solution)));
                }
            } else {
                steps.push(SolverStep::ReportSolution(ExactCover(solution)));
            }
            return;
        }

        let (mut col_node, s) = self.least_col_with_least_ones();
        steps.push(SolverStep::ChooseColumn { col: col_node-1, size: s });
        self.cover(col_node);

        let mut r = self.x[col_node].down;
        let mut prev_row = None;
        while r != col_node {
            let newrow = self.x[r].row_label;
            match prev_row {
                None => steps.push(SolverStep::PushRow(newrow)),
                Some(prow) => steps.push(SolverStep::AdvanceRow(prow, newrow)),
            }
            prev_row = Some(newrow);

            self.o_for_reporting[k] = r;

            let mut j = self.x[r].right;
            while j != r {
                self.cover(self.x[j].col);

                j = self.x[j].right;
            }

            self.step_simple_inner(k+1, steps);

            r = self.o_for_reporting[k];
            col_node = self.x[r].col;

            let mut j = self.x[r].left;
            while j != r {
                self.uncover(self.x[j].col);

                j = self.x[j].left;
            }

            r = self.x[r].down;
        }
        if let Some(prow) = prev_row {
            steps.push(SolverStep::PopRow(prow));
        }

        self.uncover(col_node);
        steps.push(SolverStep::UncoverColumn(col_node-1));
        return;
    }

    fn search_rec_inner(&mut self, k: usize, solutions: &mut Vec<ExactCover>) {
        if self.x[HEAD].right == HEAD {
            let solution = ExactCover(self.o_for_reporting.iter()
                .take(k)
                .map(|&r| self.x[r].row_label)
                .collect::<Vec<_>>());
            println!("     SOLUTION: {:?}", solution);
            solutions.push(solution);
            return;
        }

        let (mut col_node, _) = self.least_col_with_least_ones();
        // println!("COLUMN CHOICE: {:?}", col_node-1);
        self.cover(col_node);

        let mut r = self.x[col_node].down;
        let mut prev_row = None;
        while r != col_node {
            let newrow = self.x[r].row_label;
            match prev_row {
                None => {
                    println!("      ADD ROW: {:?}", newrow);
                },
                Some(prow) => {
                    println!("  REPLACE ROW: {:?}  {:?}", prow, newrow);
                }
            }
            prev_row = Some(newrow);

            self.o_for_reporting[k] = r;

            let mut j = self.x[r].right;
            while j != r {
                self.cover(self.x[j].col);

                j = self.x[j].right;
            }

            self.search_rec_inner(k+1, solutions);

            r = self.o_for_reporting[k];
            col_node = self.x[r].col;

            let mut j = self.x[r].left;
            while j != r {
                self.uncover(self.x[j].col);

                j = self.x[j].left;
            }

            r = self.x[r].down;
        }
        if let Some(prow) = prev_row {
            println!("   REMOVE ROW: {:?}", prow);
        }

        self.uncover(col_node);
        return;
    }

    // // Non-recursive `search`. Intermediate position between
    // // recursive unroll and generator/state-machine translation.
    // pub fn search_non_rec(&mut self) -> Vec<ExactCover> {
    //     let mut solutions = vec![];
    //     let mut k = 0;

    //     enum SimpleState {
    //         Start,
    //         Resume,
    //     }

    //     let mut stack = vec![SimpleState::Start];
    //     while let Some(st) = stack.pop() {
    //         match st {
    //             SimpleState::Start => {
    //                 if self.x[HEAD].right == HEAD {
    //                     let solution = ExactCover(self.o_for_reporting.iter()
    //                         .take(k)
    //                         .map(|&r| self.x[r].row_label)
    //                         .collect::<Vec<_>>());
    //                     println!("     SOLUTION: {:?}", solution);
    //                     solutions.push(solution);
    //                     k = k.saturating_sub(1);
    //                     continue;
    //                 }

    //                 let (col_node, _) = self.least_col_with_least_ones();
    //                 println!("COLUMN CHOICE: {:?}", col_node-1);
    //                 self.cover(col_node);

    //                 let r = self.x[col_node].down;
    //                 if r != col_node {
    //                     // TODO: factor out duplication of first half of the loop.
    //                     let newrow = self.x[r].row_label;
    //                     println!("      ADD ROW: {:?}", newrow);
    //                     self.o_for_reporting[k] = r;

    //                     let mut j = self.x[r].right;
    //                     while j != r {
    //                         self.cover(self.x[j].col);
    //                         j = self.x[j].right;
    //                     }

    //                     stack.push(SimpleState::Resume);
    //                     k += 1;
    //                     stack.push(SimpleState::Start);
    //                 } else {
    //                     println!("COLUMN UNCHOICE: {:?}", col_node-1);
    //                     self.uncover(col_node);
    //                     k = k.saturating_sub(1);
    //                 }
    //             },
    //             SimpleState::Resume => {
    //                 // Second half of the loop
    //                 let mut r = self.o_for_reporting[k];
    //                 let col_node = self.x[r].col;

    //                 let mut j = self.x[r].left;
    //                 while j != r {
    //                     self.uncover(self.x[j].col);

    //                     j = self.x[j].left;
    //                 }

    //                 let previous_row = self.x[r].row_label;

    //                 r = self.x[r].down;
    //                 // First half of the loop again. TODO factor out
    //                 // though now it's a resumption, so we know to REPLACE
    //                 // and REMOVE
    //                 if r != col_node {
    //                     // TODO: factor out duplication of first half of the loop.
    //                     let newrow = self.x[r].row_label;
    //                     println!("  REPLACE ROW: {:?}  {:?}", previous_row, newrow);
    //                     self.o_for_reporting[k] = r;

    //                     let mut j = self.x[r].right;
    //                     while j != r {
    //                         self.cover(self.x[j].col);
    //                         j = self.x[j].right;
    //                     }

    //                     stack.push(SimpleState::Resume);
    //                     k += 1;
    //                     stack.push(SimpleState::Start);
    //                 } else {
    //                     println!("   REMOVE ROW: {:?}", previous_row);
    //                     println!("COLUMN UNCHOICE: {:?}", col_node-1);
    //                     self.uncover(col_node);
    //                     k = k.saturating_sub(1);
    //                 }
    //             },
    //         }
    //     }

    //     solutions
    // }

    // // Non-recursive `search`. Intermediate position between
    // // recursive unroll and generator/state-machine translation.
    // pub fn search_non_rec_single_step(&mut self) -> Vec<SolverStep> {
    //     let mut steps = vec![];
    //     let mut k = 0;

    //     enum SimpleState {
    //         Start,
    //         AfterColumnChoice { col_node: usize },
    //         AfterAddOrReplaceRow { r: usize },
    //         AfterRemoveRow { col_node: usize },
    //         Resume,
    //     }

    //     // increment step if we do anything

    //     let mut stack = vec![SimpleState::Start];
    //     while let Some(st) = stack.pop() {
    //         match st {
    //             SimpleState::Start => {
    //                 if self.x[HEAD].right == HEAD {
    //                     let solution = ExactCover(self.o_for_reporting.iter()
    //                         .take(k)
    //                         .map(|&r| self.x[r].row_label)
    //                         .collect::<Vec<_>>());
    //                     println!("     SOLUTION: {:?}", solution);
    //                     k = k.saturating_sub(1);
    //                     self.counter_solutions += 1;
    //                     steps.push(SolverStep::ReportSolution(solution));
    //                 } else {
    //                     let (col_node, size) = self.least_col_with_least_ones();
    //                     stack.push(SimpleState::AfterColumnChoice { col_node });
    //                     self.cover(col_node);

    //                     steps.push(SolverStep::ChooseColumn {
    //                         col: col_node-1, size, other_cols: HashSet::new() });
    //                 }
    //             },
    //             SimpleState::AfterColumnChoice { col_node } => {
    //                 let r = self.x[col_node].down;
    //                 if r != col_node {
    //                     // TODO: factor out duplication of first half of the loop.
    //                     let newrow = self.x[r].row_label;
    //                     self.o_for_reporting[k] = r;

    //                     println!("      ADD ROW: {:?}", newrow);
    //                     stack.push(SimpleState::AfterAddOrReplaceRow { r });
    //                     steps.push(SolverStep::PushRow(newrow));
    //                 } else {
    //                     self.uncover(col_node);

    //                     // TODO: I think k modifications are going to be difficult
    //                     // and might have to happen at the start rather than at the end.
    //                     // Matters for partial solution lookup: it needs to make sense
    //                     // at all times.
    //                     k = k.saturating_sub(1);

    //                     println!("COLUMN UNCHOICE: {:?}", col_node-1);
    //                     // no need to push to stack, we're done
    //                     steps.push(SolverStep::UncoverColumn(col_node-1));
    //                 }
    //             },
    //             SimpleState::AfterAddOrReplaceRow { r } => {
    //                 let mut j = self.x[r].right;
    //                 while j != r {
    //                     self.cover(self.x[j].col);
    //                     j = self.x[j].right;
    //                 }

    //                 stack.push(SimpleState::Resume);
    //                 k += 1;
    //                 stack.push(SimpleState::Start);
    //             }
    //             SimpleState::Resume => {
    //                 // Second half of the loop
    //                 let mut r = self.o_for_reporting[k];
    //                 let col_node = self.x[r].col;

    //                 let mut j = self.x[r].left;
    //                 while j != r {
    //                     self.uncover(self.x[j].col);

    //                     j = self.x[j].left;
    //                 }

    //                 let previous_row = self.x[r].row_label;

    //                 r = self.x[r].down;
    //                 // First half of the loop again. TODO factor out
    //                 // though now it's a resumption, so we know to REPLACE
    //                 // and REMOVE
    //                 if r != col_node {
    //                     // TODO: factor out duplication of first half of the loop.
    //                     let newrow = self.x[r].row_label;
    //                     self.o_for_reporting[k] = r;
    //                     println!("  REPLACE ROW: {:?}  {:?}", previous_row, newrow);

    //                     stack.push(SimpleState::AfterAddOrReplaceRow { r });

    //                     steps.push(SolverStep::AdvanceRow(previous_row, newrow));
    //                 } else {
    //                     println!("   REMOVE ROW: {:?}", previous_row);
    //                     stack.push(SimpleState::AfterRemoveRow { col_node });

    //                     steps.push(SolverStep::PopRow(previous_row));
    //                 }
    //             },
    //             SimpleState::AfterRemoveRow { col_node } => {
    //                 self.uncover(col_node);
    //                 k = k.saturating_sub(1); // comment regarding k as before.
    //                 steps.push(SolverStep::UncoverColumn(col_node-1));
    //             }
    //         }
    //     }

    //     steps
    // }

    /// The current partial solution, i.e. the solver's current row stack.
    pub fn current_partial_solution(&self) -> Vec<usize> {
        self.o_for_reporting.iter()
            .take(self.k)
            .map(|&r| self.x[r].row_label)
            .collect::<Vec<_>>()
    }

    /// Return the next solution if there are any remaining.
    pub fn next_solution(&mut self) -> Option<ExactCover> {
        while let Some(next_step) = self.next_step() {
            if let SolverStep::ReportSolution(s) = next_step {
                return Some(s);
            }
        }
        None
    }
    
    /// Return the next solver step if there are any remaining to take.
    pub fn next_step(&mut self) -> Option<SolverStep> {
        let step = self.next_step_inner();
        if step.is_some() {
            self.counter_steps += 1;
        }
        step
    }

    fn next_step_inner(&mut self) -> Option<SolverStep> {
        while let Some(st) = self.stack.pop() {
            match st {
                FinalState::Start => {
                    if self.x[HEAD].right == HEAD {
                        let solution = ExactCover(self.o_for_reporting.iter()
                            .take(self.k)
                            .map(|&r| self.x[r].row_label)
                            .collect::<Vec<_>>());
                        self.k = self.k.saturating_sub(1);
                        self.counter_solutions += 1;

                        return Some(SolverStep::ReportSolution(solution));
                    } else {
                        let (col_node, size) = self.least_col_with_least_ones();
                        self.stack.push(FinalState::AfterColumnChoice { col_node });
                        self.cover(col_node);

                        return Some(SolverStep::ChooseColumn {
                            col: col_node-1, size });
                    }
                },
                FinalState::AfterColumnChoice { col_node } => {
                    let r = self.x[col_node].down;
                    if r != col_node {
                        // TODO: factor out duplication of first half of the loop.
                        let newrow = self.x[r].row_label;
                        self.o_for_reporting[self.k] = r;

                        self.stack.push(FinalState::AfterAddOrReplaceRow { r });
                        return Some(SolverStep::PushRow(newrow));
                    } else {
                        self.uncover(col_node);

                        // TODO: I think k modifications are going to be difficult
                        // and might have to happen at the start rather than at the end.
                        // Matters for partial solution lookup: it needs to make sense
                        // at all times.
                        self.k = self.k.saturating_sub(1);

                        return Some(SolverStep::UncoverColumn(col_node-1));
                    }
                },
                FinalState::AfterAddOrReplaceRow { r } => {
                    let mut j = self.x[r].right;
                    while j != r {
                        self.cover(self.x[j].col);
                        j = self.x[j].right;
                    }

                    self.stack.push(FinalState::Resume);
                    self.k += 1;
                    self.stack.push(FinalState::Start);
                }
                FinalState::Resume => {
                    // Second half of the loop
                    let mut r = self.o_for_reporting[self.k];
                    let col_node = self.x[r].col;

                    let mut j = self.x[r].left;
                    while j != r {
                        self.uncover(self.x[j].col);

                        j = self.x[j].left;
                    }

                    let previous_row = self.x[r].row_label;

                    r = self.x[r].down;
                    // First half of the loop again. TODO factor out
                    // though now it's a resumption, so we know to REPLACE
                    // and REMOVE
                    if r != col_node {
                        // TODO: factor out duplication of first half of the loop.
                        let newrow = self.x[r].row_label;
                        self.o_for_reporting[self.k] = r;

                        self.stack.push(FinalState::AfterAddOrReplaceRow { r });

                        return Some(SolverStep::AdvanceRow(previous_row, newrow));
                    } else {
                        self.stack.push(FinalState::AfterRemoveRow { col_node });

                        return Some(SolverStep::PopRow(previous_row));
                    }
                },
                FinalState::AfterRemoveRow { col_node } => {
                    self.uncover(col_node);
                    self.k = self.k.saturating_sub(1); // comment regarding k as before.
                    return Some(SolverStep::UncoverColumn(col_node-1));
                }
            }
        }

        None
    }

    /// Returns an iterator through remaining solutions.
    pub fn iter_solutions(&mut self) -> ExactCoverSolutionIter {
        ExactCoverSolutionIter { solver: self }
    }

    /// Returns an iterator through remaining solver steps.
    pub fn iter_steps(&mut self) -> ExactCoverStepIter {
        ExactCoverStepIter { solver: self }
    }

    // TODO. When just solving efficiently, we will simply want to return the col
    // (and maybe the size). But for diagnostic purposes, we may want to return
    // all of the cols of relevant size.
    // returns index of the col node. and the smallest size INDEX OF THE COL NODE
    // not the COLUMN
    fn least_col_with_least_ones(&self) -> (usize, usize) {
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

    /// The number of solutions seen so far.
    pub fn counter_solutions(&self) -> u64 { self.counter_solutions }

    /// The number of solver steps performed so far.
    pub fn counter_steps(&self) -> u64 { self.counter_steps }
}
