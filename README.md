# Exact Cover Solver

work in progress

A fast, efficient solver for the exact cover problem.

- Written in a fast programming language with no garbage collector.
- Uses efficient methods ([Algorithm X](https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X) with the [dancing links](https://en.wikipedia.org/wiki/Dancing_Links) technique representing the grid as a torus of circular doubly-linked lists). See [Donald Knuth&#8217;s paper](https://arxiv.org/pdf/cs/0011047) on the topic.
- Solves the generalised exact cover problem (i.e. supports optional secondary constraints).
- Returns correct output in all cases including duplicate and empty rows. (To be clear, adding $n$ empty rows multiplies the number of solutions by $2^n$.)
- Written in a generator style so the solver can be started and stopped.
- Presents a notion of a solver &ldquo;step&rdquo; and methods to advance a step, advance to the next solution, and so on.
- Has a simple accompanying executable to read and write solutions to files.

## TODOs

- Full special handling for empty rows
- Get all tests working and make the assertions maximally tight
- Fill out `SparseBinaryMatrix` with more methods
- Clarify {Exact, Partial}Cover type interfaces and the ExactCoverProblem interface
- Add other problems (pentominos, Sudoku, Kakuro, etc.)
- Investigate proc macros for test generation, in particular product of cases and invariants

```rust
fn meow() {
	let o = false; let x = true;
	let matrix = SparseBinaryMatrix::from_arrays([
        [o,o,x,o,x,x,o],
        [x,o,o,x,o,o,x],
        [o,x,x,o,o,x,o],
        [x,o,o,x,o,o,o],
        [o,x,o,o,o,o,x],
        [o,o,o,x,x,o,x],
	]);
	let problem = ExactCoverProblem::new(matrix, 0).unwrap();

	let mut solver = ExactCoverSolver::new(&problem);
	let mut num_solutions = 0;
	while let Some(next_step) = solver.advance() {
		match next_step {
			PushColumn(c) => println!("Pushing column {c}"),
			PopColumn(c) => println!("Popping column {c}"),
			PushRow(r) => println!("Pushing row {r}"),
			PopRow(r) => println!("Pushing row {r}"),
			ReportSolution => {
				let (cover, is_exact) = solver.get();
				assert!(is_exact);
				num_solutions += 1;
				println!("Found exact cover: {cover:?}");
			}
		}
	}

	println!("Total number of solutions: {num_solutions}");
}
```