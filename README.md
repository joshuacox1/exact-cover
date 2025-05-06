# Description

An extremely fast, efficient solver for the generalised exact cover problem.

- Written in a fast programming language with no garbage collector.
- Uses efficient methods (Algorithm X with the &ldquo;dancing links&rdquo; technique representing the grid as a torus of circular doubly-linked lists).
- Solves the generalised exact cover problem (i.e. supports optional secondary constraints).
- Written in a generator style so the solver can be started and stopped.
- Presents a notion of a solver &ldquo;step&rdquo; and methods to advance a step, advance to the next solution, and so on.

The solver uses the [dancing links](https://en.wikipedia.org/wiki/Dancing_Links) technique.;