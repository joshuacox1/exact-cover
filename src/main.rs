use exact_cover_solver::problems::{NQueens, ExactCoverProblem};

fn main() {
    let queens = NQueens::new(8);
    let mut b = queens.brute_force();
    b.sort();
    let mut b2 = queens.solver().all_solutions().iter()
        .map(|s| NQueens::from_solution(&queens, s))
        .collect::<Vec<_>>();
    b2.sort();

    println!("b1len {:?}, b2len {:?}", b.len(), b2.len());
    for (s1, s2) in b.iter().zip(&b2) {
        println!("{s1:?} {s2:?}");
    }

    println!("{:?}", b == b2);
}
