# `tahini` AST Transformer – Implementation Plan

This TODO list decomposes the work required to build a **pure** `OriginalAST -> TransformedAST` transformer that
performs **monomorphization**, **dollar-operator desugaring**, and **lambda hoisting / closure capture**. Each task is
intentionally narrow to enable incremental, well-tested development.

---

## 0. Project Setup

- [ ] Ensure current build passes via `cargo test` (baseline green tests).
- [x] Ensure `cargo test` is set up and passes (baseline green tests).
- [x] Ensure `cargo test` passes (baseline green tests).
- [ ] Create `src/transformer/` module for new code (`mod transformer;`).
- [x] Ensure build passes via `cargo test` (baseline green tests).
- [x] Create `src/transformer/` module for new code (`mod transformer;`).
- [x] Add empty stubs for new AST data-classes (see §4).

---

## 1. Monomorphization Pass

### 1.1 Analysis

- [ ] Collect **all call-sites** of polymorphic functions / generics.
- [ ] Infer **concrete type substitutions** for each call.
- [ ] Record mapping: `(genericFnId, typeSubstitution) -> monomorphInstanceId`.

### 1.2 Generation

- [ ] Clone original polymorphic function body.
- [ ] Replace **type parameters** with inferred concrete types.
- [ ] Insert **specialized function definitions** into `TransformedAST.Module`.

### 1.3 Deduplication

- [ ] When a (fn, substitution) pair repeats, **reuse** the previously generated instance.

### 1.4 Re-writing Call-sites

- [ ] Replace generic fn calls with calls to corresponding specialized functions.

### 1.5 Edge Cases & Validation

- [ ] Support higher-order generics (generic lambdas as params).
- [ ] Detect & error on **un-instantiated** generics that survive analysis.
- [ ] Unit tests (see §6).

---

## 2. Dollar-Operator (`$`) Transformation Pass

| Pattern              | Target AST                       |
|----------------------|----------------------------------|
| `($ :x ($ :y a))`    | `ChainAccess(a, ["y", "x"])`     |
| `($ :x ($ :y a) 10)` | `ChainAssign(a, ["y", "x"], 10)` |
| `($ [0] a)`          | `IndexAccess(a, [0])`            |
| `($ [0] ($ [0] a))`  | `IndexAccess(a, [0, 0])`         |
| `($ [i] a i)`        | `IndexAssign(a, [i], i)`         |

### 2.1 Detection & Parsing

- [x] Identify root `$` forms during traversal.
- [x] Distinguish **access** (arity 2) vs **assignment** (arity 3).

### 2.2 Chain Construction

- [x] Flatten nested `$` into ordered **segment list** of either *Field* or *Index* segments.
- [ ] Synthesize `ChainAccess` / `ChainAssign` nodes holding:
    * base expression,
    * segment list,
    * (optional) value expression for assignment.

### 2.3 Validation & Edge Cases

- [ ] Support arbitrarily deep nesting.
- [ ] Preserve original source spans on each segment for error reporting.

### 2.4 Tests

- [ ] Positive & negative cases (see §6).

---

## 3. Lambda Hoisting & Closure Capture Pass

### 3.1 Discovery

- [ ] Traverse AST; collect **inline lambda** nodes.

### 3.2 Free-Variable Analysis

- [ ] For each lambda, compute **free variables** (FV) not defined inside lambda.

### 3.3 Hoisting

- [ ] Generate **fresh top-level function name** (`lambda$<N>`).
- [ ] Move lambda body to a new `FnDef` in module scope.
- [ ] Add explicit parameters for captured FVs.

### 3.4 Closure Construction

- [ ] Introduce `MakeClosure { fn_ref, env_tuple }` expression node.
- [ ] Replace original lambda expression with `MakeClosure`.

### 3.5 Call-Site Adjustment

- [ ] Ensure that calling a closure unpacks env before invoking the function.
    * (Can be deferred to later lowering stage if not the transformer's job.)

### 3.6 Tests

- [ ] Lambdas with **no** captures (degenerate env).
- [ ] Shallow capture (`x` in outer let).
- [ ] Deep / nested captures.

---

## 4. New Transformed AST Definition

- [ ] Create `src/transformer/ast_new.rs` with nodes:
    * `Module`, `FnDef`, `Stmt`, `Expr`, …
    * `MonoFnDef` – specialized function with concrete types.
    * `ChainAccess`, `ChainAssign`, `IndexAccess`, `IndexAssign`.
    * `MakeClosure`, `ClosureCall` (if needed).
    * `Type` variants after monomorphization (no generics).
- [ ] Add **`Span` / `SrcLoc`** to every node.
- [ ] Provide `into_orig_span()` helper for tooling.

---

## 5. Transformer Architecture

- [ ] Implement **pure functional** transformer:
  ```rust
  pub fn transform(module: &orig::Module) -> new::Module {
      let mono = monomorph::run(module);
      let chained = dollar::run(&mono);
      let lifted = lambda::run(&chained);
      lifted
  }
  ```
- [ ] Each pass lives in its own sub-module with:
    * traversal + small focused rewrite helpers,
    * no global state; use explicit dictionaries / accumulators.
- [ ] Provide **visitor utilities** to avoid repetitive code.

---

## 6. Tests

### 6.1 Dollar Operator

- [x] `($ :x ($ :y a))` ⇒ `ChainAccess(a, ["y", "x"])`.
- [x] `($ :x ($ :y a) 10)` ⇒ `ChainAssign`.
- [x] Mixing `:` and `[]` segments.

### 6.2 Monomorphization

- [ ] Simple generic identity fn instantiated at `Int` and `Bool` dedupes.
- [ ] Higher-order: generic fn returning generic lambda.

### 6.3 Lambda Hoisting

- [ ] Lambda with no FV.
- [ ] Lambda capturing outer `x`.
- [x] Lambda with no FV.
- [x] Lambda capturing outer `x`.
- [ ] Nested lambdas capturing different scopes.

### 6.4 Property-Based / Fuzz (stretch)

- [ ] Random ASTs remain **semantically equivalent** after round-trip compile.

---

## 7. Documentation & Clean-up

- [ ] Update project README with transformer overview.
- [ ] Inline docstrings / rustdoc on all new modules.
- [ ] Remove obsolete code paths once new pipeline is default.