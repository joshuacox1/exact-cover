use std::time::{Duration, Instant};
use std::thread::sleep;
use itertools::Itertools;

use exact_cover_solver::{
    ExactCoverSolver, ExactCoverRepresentable, SolverStep
};
use exact_cover_solver::NQueens;


fn main() {
    let n = 15;
    let queens = NQueens::new(n).exact_cover_problem();

    let mut solver = ExactCoverSolver::new(&queens);

    // let mut w = vec![];
    while let Some(s) = solver.next_solution() {
        let q = solver.iter_steps().count();
        let s = s.to_sorted();
        println!("{q}: {s:?}");
    }

    // for (q, s) in &w {
    // }

    // let now1 = Instant::now();
    // let sol1: usize = solver1.iter_solutions().map(|ec| *ec.to_sorted().last().unwrap()).sum();
    // println!("Sol propersolver: {} {}", sol1, now1.elapsed().as_micros());

    // let now3 = Instant::now();
    // let sol3 = solver3.search_non_rec();
    // println!("Sol searchnonrec: {} {}", sol3.len(), now3.elapsed().as_micros());
    // let now2 = Instant::now();
    // let sol2 = solver2.search_rec();
    // println!("Sol searchrec: {} {}", sol2.len(), now2.elapsed().as_micros());

    // println!("Made NQueens object: {}", now.elapsed().as_micros());
    // let spec = queens10.exact_cover_spec();
    // println!("Made spec: {}", now.elapsed().as_micros());
    // let mut solver = ExactCoverSolver::new(&spec);
    // println!("Constructed solver: {}", now.elapsed().as_micros());
    // for i in solver.iter_solutions() {
        
    // }
    // println!("Found {} solutions: {}", solver.counter_solutions(), now.elapsed().as_micros());



    // // let mut b = queens.brute_force();
    // // b.sort();
    // let mut b2 = queens.exact_cover_solver().iter_steps().collect::<Vec<_>>();
    // let mut b3 = queens.exact_cover_solver().step_simple();
    // //     .map(|s| NQueens::from_solution(&queens, s))
    // //     .collect::<Vec<_>>();
    // // b2.sort();

    // // println!("b1len {:?}, b2len {:?}", b.len(), b2.len());
    // // for (s1, s2) in b.iter().zip(&b2) {
    // //     println!("{s1:?} {s2:?}");
    // // }
    // for s in &b2 {
    //     println!("{s:?}");
    //     // match s {
    //     //     SolverStep::ChooseColumn { .. } | SolverStep::UncoverColumn(_) => {
    //     //         println!("{s:?}");
    //     //     },
    //     //     _ => (),
    //     // }
    // }
    // println!("LEN: b2len {:?}", b2.len());
    // // println!("LEN: b3len {:?}", b3.len());
    // // println!("LEN: b2lensolutions {:?}", b2.iter().filter(|q| matches!(q, SolverStep::ReportSolution { .. })).count());

    // println!("{:?}", b2 == b3);
}
