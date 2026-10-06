# Just practicing Rust stuff

A 22-chapter Rust course written as **Jupyter notebooks**, running on the
**[evcxr](https://github.com/evcxr/evcxr)** Rust kernel. Every cell in every notebook
actually runs; nothing is pseudocode.

Each chapter ships as a pair:

| Notebook | What it is | How to read it |
|---|---|---|
| `NN_<title>_e.ipynb` | **Example**:  worked demonstrations, top to bottom | read along, run as you go |
| `NN_<title>_p.ipynb` | **Practice**:  one exercise per demonstration cell, auto-graded | replace the `todo!()`, make the checks pass |

## Objectives

- Learn idiomatic Rust from zero, in dependency order:  not topic-popularity order.
- Treat the compiler as the teacher... deliberate error demos (`E0382`, `E0277`, …) are
  catalogued per chapter.
- Practice with a grader: every exercise cell ends in a check block; when the cell runs
  clean you see `Exercise N.M passed`.
- No fiction: every semantic claim in the prose was probe-verified against the pinned
  toolchain before it was taught.

## Requirements

Windows (the course was authored and verified on Windows 11, rustc 1.98
`x86_64-pc-windows-gnu`, Python 3.13)... but actually any OS with a working toolchain will do.

### 1. Rust

```bash
winget install Rustlang.Rustup     # or: https://rustup.rs
rustup default stable
rustc --version
```

### 2. Python + Jupyter

```bash
py -m pip install jupyterlab       # any Python ≥ 3.10
```

### 3. The Rust kernel (`evcxr`)

```bash
cargo install --locked evcxr_jupyter
evcxr_jupyter --install
jupyter kernelspec list            # should list:  rust
```

### 4. Open and run

```bash
jupyter lab
```

…or use **VS Code** with the *Python* + *Jupyter* extensions and select the **Rust**
kernel (the notebooks were authored in VS Code).

> **Crate chapters (09, 15–20):** each starts with a `:dep …` cell that pulls crates
> from crates.io:  first run needs network and can be slow to compile (chapter 15's
> `polars` in particular). Chapter 20 additionally walks through building a small
> `cdylib` project locally; the notebook contains the full recipe.

## Contents

Chapters 01–09 are the **core track**: the language, plus logging as basic literacy.
Chapters 10–22 are the **applied track**: structs → errors → iterators → modules →
smart pointers, then crates for time/data, the async pivot, network/cache/database,
Python interop, and a capstone that assembles it all.

> **DLC (23–26):** the story didn't end at the capstone. Same twin-notebook format,
> same discipline — processes & CLI (23), advanced string manipulation (24), unit
> tests (25), and macros (26).

| # | Chapter | Topics in one line |
|---|---|---|
| 01 | Syntax, Printing and Types | bindings, scalars, operators, casting, `format!`, tuples/arrays |
| 02 | Logic Blocks and Control Flow | `if` / `loop` / `while` / `for`, `match`, `if let`, labels |
| 03 | Functions | parameters, returns, `const fn`, closures (intro) |
| 04 | Strings | `String` vs `&str`, UTF-8, slicing, raw strings |
| 05 | Ownership and Borrowing | moves, borrows, NLL, slices, `Option` in depth |
| 06 | Native Collections | `Vec`, `HashMap`/`BTreeMap`, `VecDeque`, iterator basics |
| 07 | I/O and Files | stdin/out/err, args, env, `std::fs`, `Path` |
| 08 | Assertions and Type Checks | `assert!` family, `matches!`, `size_of`, `std::any` |
| 09 | Logging | `tracing` events/spans, `EnvFilter`, `eyre` errors, file sinks |
| 10 | Structs, OOP and Traits | `impl`, traits, enums with data, generics, `Display`/`Debug` |
| 11 | Errors in Depth | error enums, `From` + `?`, `Box<dyn Error>`, panics vs errors |
| 12 | Iterators in Depth | adaptors, laziness, `collect` + turbofish, custom `Iterator` impls |
| 13 | Modules, Crates and Testing | `mod`/`pub`/`use`, cargo layout, `#[test]`, doc tests |
| 14 | Smart Pointers & Interior Mutability | `Box`/`Rc`/`Arc`, `RefCell`, `Weak`, `OnceCell` |
| 15 | Data Types and Their Libraries | `chrono` vs `time`, `polars`, `uuid`, `rust_decimal` |
| 16 | Concurrency and Async/Await | threads, channels, `Mutex`/atomics, tokio, `select!` |
| 17 | HTTP Clients and WebSockets | `reqwest`, serde JSON, streaming, `tokio-tungstenite`, reconnect |
| 18 | Redis | `redis` async, TTLs, pub/sub, pipelines, cache-aside |
| 19 | PostgreSQL | `sqlx` pools, row mapping, transactions, NULL ↔ `Option` |
| 20 | PyO3: Rust Inside Python | `#[pyfunction]`/`#[pyclass]`, error bridge, cdylib + `pytest` |
| 21 | Capstone Prep and Real-World Practice | crate docs/features, criterion, service layout |
| 22 | Capstone: Market Data Service | WebSocket feed → Redis cache → PostgreSQL, CLI + shutdown |
| 23 | CLI and Processes | `input()` via piped child stdin, `std::process::Command` sync, `tokio::process` async, `clap` |
| 24 | Advanced String Manipulation | `regex`, CSV/JSON → `Vec`/`HashMap`, `polars` DataFrames, `scraper` basics |
| 25 | Unit Tests | `#[test]`, the `assert` family, `#[should_panic]`, fixtures & teardown, `cargo test` |
| 26 | Macros | `macro_rules!`, fragment specifiers, repetition, hygiene |

## How the practice notebooks work

- Every exercise declares a signature, a doc-comment contract, and a `todo!()` body.
- Below it sits a `// --- check (do not modify) ---` block:  **that block is the grader**.
- Replace the `todo!()` and re-run the cell; `Exercise N.M passed` means done.
- Solve it the way the doc comment says: some checks fail to *compile* if you cheat the
  borrow checker.
- Notebooks share **one namespace** (exactly like the kernel): names are unique across
  the whole notebook, `use` lines import once, and cells are meant to run **top to
  bottom**.

## Repository layout

```text
.
├── 00_NB-STRUCT.md        # the governing spec: authoring rules, gates, stuff, tracker
├── NN_<slug>_e.ipynb      # 22 example notebooks — the canonical course artifacts
├── NN_<slug>_p.ipynb      # 22 practice twins
├── _build_nb.py           # optional: rebuild a notebook from a regenerated plain-text cell source
├── _validate.py           # compile-and-run gate for std-only notebooks
├── _crate_validate.py     # gate for crate chapters (offline, via prebuilt rlibs)
└── _externs.txt           # frozen rlib manifest used by the crate gate
```

### Tooling (only if you want to rebuild/re-gate)

```bash
# gate an std-only notebook: 0 errors, 0 warnings, exit 0
python _validate.py NN_<slug>_e.ipynb

# gate a crate chapter notebook (needs the rlibs listed in _externs.txt)
python _crate_validate.py NN_<slug>_e.ipynb

# (only when revising a chapter) rebuild from a regenerated plain-text cell source
python _build_nb.py NN_<slug>_e.src
```

The crates the gate needs are prebuilt rlibs whose absolute paths live in
`_externs.txt`. Don't move or delete those build directories if you plan to re-gate.
The notebooks themselves are the canonical artifacts — the plain-text `.src` sources
they were authored from were removed after the series shipped.

## Status

- **The series is complete**: 22/22 episodes, every example notebook correctly executable: 0 errors or warnings. Every practice notebook proven solvable (all exercises pass their checks when solved).
- The [spec](00_NB-STRUCT.md) is the deep reference:  per-chapter facts, verified toolchain behaviours, and a progress tracker. This README is only the front door.

## (No) license

Personal studying material. Do whatever you want with it, just don't be a "boton" (🇦🇷).
