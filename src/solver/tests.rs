//! The solver's state is very complex and the step code hard to make sense of.
//! But many invariants must be upheld. Define functions to assert they hold here.
//! These functions are very sloppily written, but they do check what they
//! purport to check and quite stringently.
#![cfg(test)]

// TODO: more test cases. Empty in particular needs to be made to work
// and also potential edge case to the stack rules!
// Ensure it returns nothing after it's done. Very likely as the stack
// is now empty anyway...


use crate::{
    problems::{ExactCoverProblem, NQueens},
    solver::{ExactCoverSolver, ExactCoverSpec, SolverStep},
    sparse_binary_matrix::SparseBinaryMatrix,
};

// The row additions and removals reported in solver steps form valid
// stack operations with the stack containing unique values at all times.
fn valid_unique_entry_row_stack(s: &ExactCoverSpec) {
    let mut solver = ExactCoverSolver::new(s);

    let mut row_stack = vec![];
    for step in solver.iter_steps() {
        match step {
            SolverStep::PushRow(r) => {
                assert!(!row_stack.contains(&r));
                row_stack.push(r);
            },
            SolverStep::AdvanceRow(b4, after) => {
                assert_eq!(*row_stack.last().unwrap(), b4);
                assert!(!row_stack.contains(&after));
                let l = row_stack.len()-1;
                row_stack[l] = after;
            },
            SolverStep::PopRow(r) => {
                let l = row_stack.len()-1;
                assert_eq!(row_stack[l], r);
                row_stack.pop();
            },
            SolverStep::SelectColumn { .. }
              | SolverStep::DeselectColumn(_)
              | SolverStep::ReportSolution(_) => (),
        }
    }
}

// The column selections and uncovers reported in solver steps form valid
// stack operations with the stack containing unique values at all times.
fn valid_unique_entry_col_stack(s: &ExactCoverSpec) {
    let mut solver = ExactCoverSolver::new(s);

    let mut col_stack = vec![];
    for step in solver.iter_steps() {
        match step {
            SolverStep::SelectColumn { col, .. } => {
                assert!(!col_stack.contains(&col));
                col_stack.push(col);
            },
            SolverStep::DeselectColumn(col) => {
                let l = col_stack.len()-1;
                assert_eq!(col_stack[l], col);
                col_stack.pop();
            },
            SolverStep::PushRow(_) |
            SolverStep::AdvanceRow(_, _) |
            SolverStep::PopRow(_) | SolverStep::ReportSolution(_) => (),
        }
    }
}

fn is_exact_cover(m: &SparseBinaryMatrix, cover: &[usize]) -> bool {
    let mut b = vec![0; m.num_cols()];
    for &row in cover {
        let ones = m.get_row(row).unwrap();
        for &one_pos in ones {
            b[one_pos] += 1;
        }
    }

    b.iter().all(|&col| col == 1)
}

// If a solution is reported, it is identical to the current partial solution
// and is an exact cover. The converse is not true as pushing a row
// can get you a solution moments before you then get a good answer.
// todo wtf?!
fn solutions_exactly_the_exact_covers(s: &ExactCoverSpec)  {
    let m = s.matrix();
    let mut solver = ExactCoverSolver::new(s);

    while let Some(SolverStep::ReportSolution(s)) = solver.next_step() {
        assert!(is_exact_cover(&m, &s.0));

        let partial_sol = solver.current_partial_solution().0;
        let partial_sol_is_exact_cov = is_exact_cover(&m, &partial_sol);

        assert!(partial_sol_is_exact_cov);
    }
}

// The row changes as specified in the solver steps should line up
// identically with the claimed partial solutions after every step.
// This is non-trivial as the state machine unroll across the loop
// means that you get off-by-one errors for k depending on the state.
fn step_row_stack_and_partial_solution_identical(s: &ExactCoverSpec) {
    let mut solver = ExactCoverSolver::new(s);

    let mut row_stack = vec![];
    while let Some(step) = solver.next_step() {
        match step {
            SolverStep::PushRow(r) => row_stack.push(r),
            SolverStep::AdvanceRow(_, after) => {
                let l = row_stack.len()-1;
                row_stack[l] = after;
            },
            SolverStep::PopRow(_) => { row_stack.pop(); },
            SolverStep::SelectColumn { .. }
              | SolverStep::DeselectColumn(_)
              | SolverStep::ReportSolution(_) => (),
        }

        let partial = solver.current_partial_solution().0;
        assert_eq!(row_stack, partial);
    }
}

fn correct_counters_when_stepping(s: &ExactCoverSpec) {
    let mut solver = ExactCoverSolver::new(s);

    let mut n = 0;
    let mut solns = 0;

    assert_eq!(n, solver.counter_steps());
    assert_eq!(solns, solver.counter_solutions());
    while let Some(step) = solver.next_step() {
        n += 1;
        if let SolverStep::ReportSolution(_) = step {
            solns += 1;
        }

        assert_eq!(n, solver.counter_steps());
        assert_eq!(solns, solver.counter_solutions());
    }

    // Check nothing changes once done
    for _ in 0..20 {
        assert_eq!(n, solver.counter_steps());
        assert_eq!(solns, solver.counter_solutions());
    }
}

fn correct_counters_when_solutioning(s: &ExactCoverSpec) {
    let mut solver = ExactCoverSolver::new(s);
    let mut n = 0;
    let mut solution_step_counts = vec![0];
    while let Some(step) = solver.next_step() {
        n += 1;
        if let SolverStep::ReportSolution(_) = step {
            solution_step_counts.push(n);
        }
    }

    let mut solver = ExactCoverSolver::new(s);
    let mut j = 0;
    assert_eq!(solution_step_counts[j], solver.counter_steps());
    assert_eq!(j as u64, solver.counter_solutions());
    while let Some(_) = solver.next_solution() {
        j += 1;
        assert_eq!(solution_step_counts[j], solver.counter_steps());
        assert_eq!(j as u64, solver.counter_solutions());
    }
}

// Donald Knuth's simple example.
fn knuth_1() -> ExactCoverSpec {
    let o = false; let x = true;
    let arrays = [
        [o,o,x,o,x,x,o],
        [x,o,o,x,o,o,x],
        [o,x,x,o,o,x,o],
        [x,o,o,x,o,o,o],
        [o,x,o,o,o,o,x],
        [o,o,o,x,x,o,x],
    ];
    let matrix = SparseBinaryMatrix::from_array_2d(arrays);
    ExactCoverSpec::new_standard(matrix)
}

fn queens8() -> ExactCoverSpec { NQueens::new(8).exact_cover_spec() }

#[test] fn knuth_1_valid_unique_entry_row_stack()                  { valid_unique_entry_row_stack(&knuth_1());                  }
#[test] fn knuth_1_valid_unique_entry_col_stack()                  { valid_unique_entry_col_stack(&knuth_1());                  }
#[test] fn knuth_1_solutions_exactly_the_exact_covers()            { solutions_exactly_the_exact_covers(&knuth_1());            }
#[test] fn knuth_1_step_row_stack_and_partial_solution_identical() { step_row_stack_and_partial_solution_identical(&knuth_1()); }
#[test] fn knuth_1_correct_counters_when_stepping()                { correct_counters_when_stepping(&knuth_1());                }
#[test] fn knuth_1_correct_counters_when_solutioning()             { correct_counters_when_solutioning(&knuth_1());             }


#[test] fn queens_8_valid_unique_entry_row_stack()                  { valid_unique_entry_row_stack(&queens8());                  }
#[test] fn queens_8_valid_unique_entry_col_stack()                  { valid_unique_entry_col_stack(&queens8());                  }
#[test] fn queens_8_solutions_exactly_the_exact_covers()            { solutions_exactly_the_exact_covers(&queens8());            }
#[test] fn queens_8_step_row_stack_and_partial_solution_identical() { step_row_stack_and_partial_solution_identical(&queens8()); }
#[test] fn queens_8_correct_counters_when_stepping()                { correct_counters_when_stepping(&queens8());                }
#[test] fn queens_8_correct_counters_when_solutioning()             { correct_counters_when_solutioning(&queens8());             }
