# 🥙 `tahini`

> A low-level, compiled Lisp with a creamy smooth type system

## What is this? 🤔

`tahini` is the spiritual successor to [Hummus](https://github.com/Azer0s/Hummus) - but where Hummus was interpreted and dynamic, `tahini` is compiled and ✨ _blazingly fast_ ✨. It's a Lisp-like systems programming language, with automatic memory management via Boehm GC.

Think of it as **C3 and Clojure had a baby**, but the baby prefers parentheses and had (optional) automatic memory management.

## Features 🚀

- **S-expression syntax** - because who doesn't love parentheses? 
- **Static typing with inference** - let the compiler figure it out
- **Zero-cost C interop** - call C functions like they're native
- **Boehm GC** - because manual memory management is so 1970s
- **Pattern matching** - destructure your data with style
- **First-class functions** - pass 'em around like candy
- **Macros** - code that writes code (coming soon™)

## Quick taste 👅

```lisp
;; Classic hello world
(def stdio (use :header "stdio.h"))

(def main (fn [] i32
  (do
    (stdio/printf "Hello from `tahini`! 🥙\n")
    0)))

;; Higher-order functions? You bet!
(def make-adder (fn [(:n i32)] (fn [i32] i32)
  (fn [(:x i32)] i32
    (+ x n))))

(def add5 (make-adder 5))
(add5 10) ;; => 15

;; Pattern matching on data types
(type result (data
  [:ok i32]
  [:err str]))

(def safe-divide (fn [(:a i32) (:b i32)] (result i32)
  (if (= b 0)
    [:err "Division by zero!"]
    [:ok (/ a b)])))
```

## Why `tahini`? 🥄

Because after making Hummus, the next logical step is obviously `tahini`. Plus:

- **It's fast** - compiles to native code via LLVM
- **It's safe** - garbage collected, no use-after-free
- **It's practical** - seamless C interop means you can use existing libraries
- **It's fun** - Lisp syntax with modern language features

## Building 🔨

```bash
# You'll need Rust and LLVM
cargo build --release

# Run the tahini compiler
./target/release/tahini my-program.th
```

## Project Status 📊

`tahini` is in active development. Here's what works:

- ✅ Parser & AST
- 🚧 Type system
- 🚧 C interop via libclang
- 🚧 Basic code generation
- 🚧 Standard library
- 🚧 Macro system
- 🚧 LLVM backend
- 📝 Optimizations

## Examples 📚

Check out the `examples/` directory for more code samples:

- `closures.th` - Functional programming patterns
- `c_interop.th` - Working with C libraries  
- `data_structures.th` - Records, variants, and pattern matching
- `web_server.th` - A tiny HTTP server (because why not?)

## Contributing 🤝

Found a bug? Want a feature? PRs welcome! Just remember:

1. Keep it simple
2. Keep it fast
3. Keep it lispy

## License 📜

MIT - because sharing is caring

---

Made with 🧆 and parentheses by [@Azer0s](https://github.com/Azer0s)

_"It's not a bug, it's a feature... of the type system"_ ™️ 