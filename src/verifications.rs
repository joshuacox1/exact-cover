//! The solver's state is very complex and the step code hard to make sense of.
//! But many invariants must be upheld. Define functions to assert they hold here.

use crate::{solver::{ExactCover, ExactCoverSolver, ExactCoverSpec, SolverStep}, sparse_binary_matrix::SparseBinaryMatrix};

// Very badly written: TODO: return an error enum with much more helpful values.
pub fn valid_unique_entry_row_stack(s: &ExactCoverSpec) -> Result<usize, String> {
    let mut solver = ExactCoverSolver::new(s);

    let mut row_stack = vec![];
    let mut n = 0;
    for step in solver.iter_steps() {
        n += 1;
        match step {
            SolverStep::PushRow(r) => {
                if row_stack.contains(&r) {
                    return Err("New row already in stack".to_string());
                }
                row_stack.push(r);
            },
            SolverStep::AdvanceRow(b4, after) => {
                if let Some(&last) = row_stack.last() {
                    if b4 != last {
                        return Err("Before was wrong".to_string());
                    }
                    if row_stack.contains(&after) {
                        return Err("Afer already in stack".to_string());
                    }
                    let l = row_stack.len()-1;
                    row_stack[l] = after;
                } else {
                    return Err("Advancerow but the stack was empty".to_string());
                }
            },
            SolverStep::PopRow(r) => {
                if row_stack.is_empty() {
                    return Err("Tried to pop row but stack was empty".to_string());
                } else {
                    let l = row_stack.len()-1;
                    if row_stack[l] != r {
                        return Err("Tried to pop the wrong row".to_string());
                    }
                    row_stack.pop();
                }
            },
            SolverStep::ChooseColumn { .. }
              | SolverStep::UncoverColumn(_)
              | SolverStep::ReportSolution(_) => (),
        }
    }

    Ok(n)
}

pub fn valid_unique_entry_col_stack(s: &ExactCoverSpec) -> Result<usize, String> {
    let mut solver = ExactCoverSolver::new(s);

    let mut col_stack = vec![];
    let mut n = 0;
    for step in solver.iter_steps() {
        n += 1;
        match step {
            SolverStep::ChooseColumn { col, .. } => {
                if col_stack.contains(&col) {
                    return Err("New col already in stack".to_string());
                }
                col_stack.push(col);
            },
            SolverStep::UncoverColumn(col) => {
                if col_stack.is_empty() {
                    return Err("Tried to uncover col but stack was empty".to_string());
                } else {
                    let l = col_stack.len()-1;
                    if col_stack[l] != col {
                        return Err("Tried to pop the wrong col".to_string());
                    }
                    col_stack.pop();
                }
            },
            SolverStep::PushRow(_) |
            SolverStep::AdvanceRow(_, _) |
            SolverStep::PopRow(_) | SolverStep::ReportSolution(_) => (),
        }
    }

    Ok(n)
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

pub fn solutions_are_exactly_the_exact_covers(s: &ExactCoverSpec) -> Result<usize, String> {
    let m = s.matrix();
    let mut solver = ExactCoverSolver::new(s);

    let mut n = 0;
    while let Some(step) = solver.next_step() {
        n += 1;
        let sol = match step {
            SolverStep::ReportSolution(s) => {
                let ec = is_exact_cover(&m, &s.0);
                Some((s, ec))
            },
            _ => None,
        };

        let partial_sol = solver.current_partial_solution();

        // The solutions are exactly the exact covers.
        let partial_sol_is_exact_cov = is_exact_cover(&m, &partial_sol);

        if partial_sol_is_exact_cov != sol.is_some_and(|(_,ec)| ec) {
            return Err("Partial sols and sol exact covers didn't exactly match".to_string());
        }
    }

    Ok(n)
}

// Very badly written: TODO: return an error enum with much more helpful values.
pub fn row_buildup_and_partial_solution_identical(s: &ExactCoverSpec) -> Result<usize, String> {
    let mut solver = ExactCoverSolver::new(s);

    let mut row_stack = vec![];
    let mut n = 0;
    while let Some(step) = solver.next_step() {
        n += 1;
        match step {
            SolverStep::PushRow(r) => row_stack.push(r),
            SolverStep::AdvanceRow(_, after) => {
                if let Some(_) = row_stack.last() {
                    let l = row_stack.len()-1;
                    row_stack[l] = after;
                } else {
                    return Err("Advancerow but the stack was empty".to_string());
                }
            },
            SolverStep::PopRow(_) => { row_stack.pop(); },
            SolverStep::ChooseColumn { .. }
              | SolverStep::UncoverColumn(_)
              | SolverStep::ReportSolution(_) => (),
        }

        let partial = solver.current_partial_solution();
        if row_stack != partial {
            return Err("Rowstack not equal to partial".to_string());
        }
    }

    Ok(n)
}
