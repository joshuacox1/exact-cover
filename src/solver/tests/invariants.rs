// //! The solver's state is very complex and the step code hard to make sense of.
// //! But many invariants must be upheld. Define functions to assert they hold here.
// #![cfg(test)]

// // TODO: more test cases. Empty in particular needs to be made to work
// // and also potential edge case to the stack rules!
// // Ensure it returns nothing after it's done. Very likely as the stack
// // is now empty anyway...

// use crate::{
//     solver::{ExactCoverSolver, ExactCoverProblem, SolverStep, SparseBinaryMatrix},
// };

// use super::cases::*;

// pub(super) trait SolverInvariant {
//     fn assert_invariant(&self, spec: &ExactCoverProblem);
// }

// /// The row additions and removals reported in solver steps form valid
// /// stack operations with the stack containing unique values at all times.
// pub struct ValidUniqueEntryRowStack;

// impl SolverInvariant for ValidUniqueEntryRowStack {
//     fn assert_invariant(&self, spec: &ExactCoverProblem) {
//         let mut solver = ExactCoverSolver::new(spec);
//         let mut row_stack = vec![];
//         let mut n = 0;
//         let mut empty_row_stack_indices = vec![n];
//         for step in solver.iter_steps() {
//             match step {
//                 SolverStep::PushRow(r) => {
//                     assert!(!row_stack.contains(&r));
//                     row_stack.push(r);
//                 },
//                 SolverStep::AdvanceRow(b4, after) => {
//                     assert_eq!(*row_stack.last().unwrap(), b4);
//                     assert!(!row_stack.contains(&after));
//                     let l = row_stack.len()-1;
//                     row_stack[l] = after;
//                 },
//                 SolverStep::PopRow(r) => {
//                     let l = row_stack.len()-1;
//                     assert_eq!(row_stack[l], r);
//                     row_stack.pop();
//                 },
//                 SolverStep::SelectColumn { .. }
//                 | SolverStep::DeselectColumn(_)
//                 | SolverStep::ReportSolution(_) => continue,
//             }

//             n += 1;
//             if row_stack.is_empty() {
//                 empty_row_stack_indices.push(n);
//             }
//         }
//         // After the first step, the next time the row stack is empty
//         // (filtering on row steps only) should be the very end.
//         assert_eq!(empty_row_stack_indices, if n == 0 { vec![0] } else { vec![0,n] });
//     }
// }

// // The column selections and uncovers reported in solver steps form valid
// // stack operations with the stack containing unique values at all times.
// pub struct ValidUniqueEntryColStack;

// impl SolverInvariant for ValidUniqueEntryColStack {
//     fn assert_invariant(&self, spec: &ExactCoverProblem) {
//         let mut solver = ExactCoverSolver::new(spec);
//         let mut col_stack = vec![];
//         let mut n = 0;
//         let mut empty_col_stack_indices = vec![];
//         for step in solver.iter_steps() {
//             match step {
//                 SolverStep::SelectColumn { col, .. } => {
//                     assert!(!col_stack.contains(&col));
//                     col_stack.push(col);
//                 },
//                 SolverStep::DeselectColumn(col) => {
//                     let l = col_stack.len()-1;
//                     assert_eq!(col_stack[l], col);
//                     col_stack.pop();
//                 },
//                 SolverStep::PushRow(_) |
//                 SolverStep::AdvanceRow(_, _) |
//                 SolverStep::PopRow(_) | SolverStep::ReportSolution(_) => (),
//             }

//             n += 1;
//             if col_stack.is_empty() {
//                 empty_col_stack_indices.push(n);
//             }
//         }

//         // After the first step, the next time the col stack is empty
//         // (filtering on col steps only) should be the very end.
//         assert_eq!(empty_col_stack_indices, vec![n]);
//     }
// }


// fn is_exact_cover(m: &ExactCoverProblem, cover: &[usize]) -> bool {
//     let c = m.columns();
//     let p_c = m.primary_columns();
//     let mut b = vec![0; c];
//     for &row in cover {
//         let ones = m.matrix.get_row(row).unwrap();
//         for &one_pos in ones {
//             b[one_pos] += 1;
//         }
//     }

//     // All primary columns are covered exactly once; all secondary columns
//     // are covered at most once.
//     let primaries = b[0..p_c].iter().all(|&col| col == 1);
//     let secondaries = b[p_c..c].iter().all(|&col| col == 0 || col == 1);
//     primaries && secondaries
// }

// pub struct SolutionsExactlyTheExactCovers;

// impl SolverInvariant for SolutionsExactlyTheExactCovers {
//     fn assert_invariant(&self, spec: &ExactCoverProblem) {
//         // TODO: only check row operations so it can be two-way; and knowingly get rid of
//         // empty rows.
//         let mut solver = ExactCoverSolver::new(spec);

//         while let Some(SolverStep::ReportSolution(s)) = solver.next_step() {
//             assert!(is_exact_cover(&spec, &s.0));

//             let partial_sol = solver.current_partial_solution().0;
//             let partial_sol_is_exact_cov = is_exact_cover(&spec, &partial_sol);

//             assert!(partial_sol_is_exact_cov);
//         }
//     }
// }

// // The row changes as specified in the solver steps should line up
// // identically with the claimed partial solutions after every step.
// // This is non-trivial as the state machine unroll across the loop
// // means that you get off-by-one errors for k depending on the state.
// pub struct RowStepStackIdenticalToPartialSolution;

// impl SolverInvariant for RowStepStackIdenticalToPartialSolution {
//     fn assert_invariant(&self, spec: &ExactCoverProblem) {
//         // TODO: only check row operations so it can be two-way; and knowingly get rid of
//         // empty rows.
//         let mut solver = ExactCoverSolver::new(spec);

//         let mut row_stack = vec![];
//         while let Some(step) = solver.next_step() {
//             match step {
//                 SolverStep::PushRow(r) => row_stack.push(r),
//                 SolverStep::AdvanceRow(_, after) => {
//                     let l = row_stack.len()-1;
//                     row_stack[l] = after;
//                 },
//                 SolverStep::PopRow(_) => { row_stack.pop(); },
//                 SolverStep::SelectColumn { .. }
//                 | SolverStep::DeselectColumn(_)
//                 | SolverStep::ReportSolution(_) => (),
//             }

//             let partial = solver.current_partial_solution().0;
//             assert_eq!(row_stack, partial);
//         }
//     }
// }

// // See comment in the cases module. A proc attribute might be in order
// // to generate this Cartesian product.

// #[test] fn knuth_simple_valid_unique_entry_row_stack() { ValidUniqueEntryRowStack.assert_invariant(&KnuthSimple.spec()); }
// #[test] fn knuth_simple_valid_unique_entry_col_stack() { ValidUniqueEntryColStack.assert_invariant(&KnuthSimple.spec()); }
// #[test] fn knuth_simple_solutions_exactly_exact_covers() { SolutionsExactlyTheExactCovers.assert_invariant(&KnuthSimple.spec()); }
// #[test] fn knuth_simple_row_step_stack_exactly_partial_solution() { RowStepStackIdenticalToPartialSolution.assert_invariant(&KnuthSimple.spec()); }

// #[test] fn zero_by_zero_valid_unique_entry_row_stack() { ValidUniqueEntryRowStack.assert_invariant(&ZeroByZero.spec()); }
// #[test] fn zero_by_zero_valid_unique_entry_col_stack() { ValidUniqueEntryColStack.assert_invariant(&ZeroByZero.spec()); }
// #[test] fn zero_by_zero_solutions_exactly_exact_covers() { SolutionsExactlyTheExactCovers.assert_invariant(&ZeroByZero.spec()); }
// #[test] fn zero_by_zero_row_step_stack_exactly_partial_solution() { RowStepStackIdenticalToPartialSolution.assert_invariant(&ZeroByZero.spec()); }

// #[test] fn zero_rows_three_cols_valid_unique_entry_row_stack() { ValidUniqueEntryRowStack.assert_invariant(&ZeroRowsThreeCols.spec()); }
// #[test] fn zero_rows_three_cols_valid_unique_entry_col_stack() { ValidUniqueEntryColStack.assert_invariant(&ZeroRowsThreeCols.spec()); }
// #[test] fn zero_rows_three_cols_solutions_exactly_exact_covers() { SolutionsExactlyTheExactCovers.assert_invariant(&ZeroRowsThreeCols.spec()); }
// #[test] fn zero_rows_three_cols_row_step_stack_exactly_partial_solution() { RowStepStackIdenticalToPartialSolution.assert_invariant(&ZeroRowsThreeCols.spec()); }

// #[test] fn zero_rows_three_cols_all_secondary_valid_unique_entry_row_stack() { ValidUniqueEntryRowStack.assert_invariant(&ZeroRowsThreeColsAllSecondary.spec()); }
// #[test] fn zero_rows_three_cols_all_secondary_valid_unique_entry_col_stack() { ValidUniqueEntryColStack.assert_invariant(&ZeroRowsThreeColsAllSecondary.spec()); }
// #[test] fn zero_rows_three_cols_all_secondary_solutions_exactly_exact_covers() { SolutionsExactlyTheExactCovers.assert_invariant(&ZeroRowsThreeColsAllSecondary.spec()); }
// #[test] fn zero_rows_three_cols_all_secondary_row_step_stack_exactly_partial_solution() { RowStepStackIdenticalToPartialSolution.assert_invariant(&ZeroRowsThreeColsAllSecondary.spec()); }

// #[test] fn three_rows_zero_cols_valid_unique_entry_row_stack() { ValidUniqueEntryRowStack.assert_invariant(&ThreeRowsZeroCols.spec()); }
// #[test] fn three_rows_zero_cols_valid_unique_entry_col_stack() { ValidUniqueEntryColStack.assert_invariant(&ThreeRowsZeroCols.spec()); }
// #[test] fn three_rows_zero_cols_solutions_exactly_exact_covers() { SolutionsExactlyTheExactCovers.assert_invariant(&ThreeRowsZeroCols.spec()); }
// #[test] fn three_rows_zero_cols_row_step_stack_exactly_partial_solution() { RowStepStackIdenticalToPartialSolution.assert_invariant(&ThreeRowsZeroCols.spec()); }
