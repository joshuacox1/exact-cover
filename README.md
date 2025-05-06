# Exact Cover Solver

A fast, efficient solver for the exact cover problem.

- Written in a fast programming language with no garbage collector.
- Uses efficient methods ([Algorithm X](https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X) with the [dancing links](https://en.wikipedia.org/wiki/Dancing_Links) technique representing the grid as a torus of circular doubly-linked lists). See [Donald Knuth&#8217;s paper](https://arxiv.org/pdf/cs/0011047) on the topic.
- Solves the generalised exact cover problem (i.e. supports optional secondary constraints).
- Returns correct output in all cases including duplicate and empty rows. (To be clear, adding $n$ empty rows multiplies the number of solutions by $2^n$.)
- Written in a generator style so the solver can be started and stopped.
- Presents a notion of a solver &ldquo;step&rdquo; and methods to advance a step, advance to the next solution, and so on.
- Has a simple accompanying executable to read and write solutions to files.
