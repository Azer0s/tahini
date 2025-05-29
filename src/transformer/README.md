# `tahini` Transformer

This sub-crate implements the *mid-level* IR transformation pipeline for `tahini` - no mutation of the original AST.
Each pass takes the
previous tree (or slice) and returns a *new* tree.

Currently implemented passes
---------------------------

1. **Dollar-operator desugaring** (`transformer::dollar`)
    * Flattens nested `$` forms into `AccessSegment` chains.
    * Distinguishes **access** vs **assignment**.
    * Fully unit-tested (simple / nested / mixed).

Planned
-------

* Monomorphisation pass (`transformer::monomorph`).
* Integration glue in `transformer::mod.rs` to chain the passes.
* **Lambda hoisting & closure capture** (`transformer::lambda`)
    * Recursively lifts all `Literal::Fn` lambdas to top-level `FnDef`s.
    * Performs free-variable analysis, adds prefixed `cap_<name>` parameters.
    * Replaces original expressions with an intrinsic `$make_closure` call that carries the captured environment.
    * Handles nested lambdas (currying) and produces deterministic names `lambda$<N>`.

Design notes
------------

* The new IR lives in `transformer::ast` to avoid polluting `src/ast.rs`.
* All passes avoid global state; shared state travels via &mut structs.
* Unit tests live next to each pass — run them with `cargo test`. 