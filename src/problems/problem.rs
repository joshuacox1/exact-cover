// use crate::solver::Solution;

// /// A trait for an exact cover problem.
// pub trait ExactCoverProblem<TArgs, TFail, TSolution> {
//     /// Fallibly transforms an input of type `TArgs` into an exact cover
//     /// problem specification.
//     fn to_exact_cover(t: TArgs) -> ProblemSpec<TRows, TRow>;

//     /// s.
//     fn from_solution(s: &Solution) -> TSolution;

//     fn to_solver(t: TArgs) -> 
// }


// /// A problem specification for the 
// pub struct ProblemSpec<T: Iterator<Item = U>, U: Iterator<Item = usize>> {
//     sparse_rows: T,
//     primary_columns: usize,
//     secondary_columns: usize,
// }
