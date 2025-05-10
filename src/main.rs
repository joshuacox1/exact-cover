use exact_cover_solver::{problems::{ExactCoverProblem, NQueens}, solver::SolverStep};

fn main() {
    let queens = NQueens::new(8);
    // let mut b = queens.brute_force();
    // b.sort();
    let mut b2 = queens.solver().search_non_rec_single_step();
    //     .map(|s| NQueens::from_solution(&queens, s))
    //     .collect::<Vec<_>>();
    // b2.sort();

    // println!("b1len {:?}, b2len {:?}", b.len(), b2.len());
    // for (s1, s2) in b.iter().zip(&b2) {
    //     println!("{s1:?} {s2:?}");
    // }
    for s in &b2 {
        println!("{s:?}");
    }
    println!("LEN: b2len {:?}", b2.len());
    println!("LEN: b2lensolutions {:?}", b2.iter().filter(|q| matches!(q, SolverStep::ReportSolution { .. })).count());

    // println!("{:?}", b == b2);
}
