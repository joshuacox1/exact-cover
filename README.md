# Description

A fast solver for exact cover problems. Uses the dancing links technique

- Written efficiently in a fast programming language with no garbage collector (Rust).
- Has an ergonomic interface supporting strict or lazy input.
- Solves the generalised exact cover problem (i.e. supports optional secondary constraints).
- Written as a state machine so it can yield solutions like an iterator.
- Has modes that yield internal solver state and decisions made. This can be used to display.

The solver uses the [dancing links](https://en.wikipedia.org/wiki/Dancing_Links) technique.