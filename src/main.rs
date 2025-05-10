use exact_cover_solver::{problems::{ExactCoverProblem, NQueens}, solver::SolverStep, verifications::*};

fn main() {
    let queens = NQueens::new(8);
    // let mut b = queens.brute_force();
    // b.sort();
    let mut b2 = queens.solver().iter_steps().collect::<Vec<_>>();
    let mut b3 = queens.solver().step_simple();
    //     .map(|s| NQueens::from_solution(&queens, s))
    //     .collect::<Vec<_>>();
    // b2.sort();

    // println!("b1len {:?}, b2len {:?}", b.len(), b2.len());
    // for (s1, s2) in b.iter().zip(&b2) {
    //     println!("{s1:?} {s2:?}");
    // }
    for s in &b2 {
        match s {
            SolverStep::ChooseColumn { .. } | SolverStep::UncoverColumn(_) => {
                println!("{s:?}");
            },
            _ => (),
        }
    }
    println!("LEN: b2len {:?}", b2.len());
    // println!("LEN: b3len {:?}", b3.len());
    // println!("LEN: b2lensolutions {:?}", b2.iter().filter(|q| matches!(q, SolverStep::ReportSolution { .. })).count());

    println!("{:?}", b2 == b3);
    println!("{:?}", valid_unique_entry_row_stack(&queens.exact_cover_spec()));
    println!("{:?}", valid_unique_entry_col_stack(&queens.exact_cover_spec()));
    println!("{:?}", solutions_are_exactly_the_exact_covers(&queens.exact_cover_spec()));
    println!("{:?}", row_buildup_and_partial_solution_identical(&queens.exact_cover_spec()));
}