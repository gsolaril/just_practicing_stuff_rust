# 00_NB-STRUCT — Notebook Structure, Style and Authoring Spec

**This is the standing specification for the Rust training notebook series.** It is
written for whoever writes the next notebook — most likely a future me with no memory
of the sessions that produced chapter 01. It records the decisions that were *made*
(and why), the mechanical conventions that must be *reproduced exactly*, and the
mistakes that have already cost time once.

If you are starting from scratch, read this file top to bottom before creating any
notebook. It is deliberately prescriptive: the value of a series comes from
consistency, and consistency comes from not re-deciding things.

> **Scope note.** This document describes *form*. It is not a Rust tutorial and not a
> substitute for reading `01_syntax-printing-and-types_e.ipynb`, which is the
> reference implementation of every rule below. When this document and that notebook
> disagree, the notebook is the truth — and this document should be corrected.

> **File-form note.** This spec is plain Markdown on purpose. It is not a notebook,
> and is not a `.ipynb` file. The workspace tooling takes `.ipynb` input only — never
> pass it this document or any other `.md` file. Its filename deliberately breaks the `{NN}_{title}_{e|p}` pattern so
> it cannot be mistaken for chapter 00 of the course.

## Contents

- [1. What the Series Is](#1-what-the-series-is)
- [2. File Naming and Shorthand](#2-file-naming-and-shorthand)
- [3. The Chapter Plan](#3-the-chapter-plan)
- [4. Anatomy of an Example Notebook](#4-anatomy-of-an-example-notebook)
- [5. Anatomy of a Practice Notebook](#5-anatomy-of-a-practice-notebook)
- [6. The Pairing Rule Between `e` and `p`](#6-the-pairing-rule-between-e-and-p)
- [7. Section 0 Must Explain the Kernel](#7-section-0-must-explain-the-kernel)
- [8. Prose Style](#8-prose-style)
- [9. Cell Granularity](#9-cell-granularity)
- [10. Code Cell Rules](#10-code-cell-rules)
- [11. The Exercise Templates](#11-the-exercise-templates)
- [12. Anchor Links and Headings](#12-anchor-links-and-headings)
- [13. Technical Requirements (the JSON)](#13-technical-requirements-the-json)
- [14. Tooling](#14-tooling)
- [15. Verification Checklist](#15-verification-checklist)
- [16. Hard-Won Gotchas](#16-hard-won-gotchas)
- [17. Verified Environment Facts](#17-verified-environment-facts)
- [18. Progress Tracker](#18-progress-tracker)
- [19. The Retained Tooling](#19-the-retained-tooling)

## 1. What the Series Is

A **Rust training course delivered as Jupyter notebooks**, modelled loosely on the
sibling `training-py` and `training-cpp` folders in the same parent directory.

### The constraints that shape every decision

| Constraint | Consequence |
|---|---|
| The learner is an **experienced programmer** (8 years Python, ~2 of C++), new to Rust. | Pitch at ideas, not at syntax. Spend words on where Rust *differs*. Never explain what a loop is. |
| The learner runs on the **evcxr** Jupyter kernel, not `rustc` directly. | One namespace per notebook; state persists; no `fn main`. Drives §7 and §10 below. |
| **Example notebooks must be runnable without edits.** "I should only need to run it, and never modify it." | Every example code cell must compile and run cleanly, top to bottom, first try. |
| **Practice notebooks are for the learner to edit.** | Stub bodies are `todo!()`; checks are pre-written; the learner touches only the marked region. |
| Delivery is **incremental, chapter by chapter, on explicit go-ahead**. | Finish and validate a whole chapter (both notebooks) before starting the next. |

### The division of labour between the two notebook kinds

- **`_e` — example / lecture.** Prose-led. Section headings, explanations, and code
  cells that demonstrate each point. The learner reads it and runs it.
- **`_p` — practice.** Exercise-led. For every section of the example notebook there
  is at least one exercise with the *same number*, a stub to fill in, and a hidden
  grader.

The example notebook teaches; the practice notebook tests. Neither is complete
alone, and the numbering is what lets the learner jump between them.

## 2. File Naming and Shorthand

```
{NN}_{title-with-dashes}_{e|p}.ipynb
```

| Part | Rule | Example |
|---|---|---|
| `NN` | two digits, zero-padded, chapter number | `01`, `21` |
| title | lowercase, words joined by single dashes, no underscores | `syntax-printing-and-types` |
| suffix | `e` = example, `p` = practice | `_e` |

Full examples:

```
01_syntax-printing-and-types_e.ipynb
01_syntax-printing-and-types_p.ipynb
06_native-collections_e.ipynb
10_structs-oop-and-traits_p.ipynb
```

**Deliberate deviation:** the sibling `training-py` / `training-cpp` folders prefix
notebooks with `g_` (guide) and `p_` (practice). The Rust series does **not** — it
uses the `{NN}_{title}_{e|p}` scheme above, which was chosen explicitly.

### Shorthand convention for conversation

When discussing notebooks in chat, use the compact form:

- `01e` = `01_syntax-printing-and-types_e.ipynb`
- `01p` = `01_syntax-printing-and-types_p.ipynb`

The user uses this shorthand and expects it back.

## 3. The Chapter Plan

Twenty-two chapters, each a title and a stable slug. **Order was chosen by
dependency, not by topic popularity**, and was approved by the user. Three chapters
are insertions: chapter 05 (ownership) — the original eight-chapter plan gained adedicated ownership chapter, which in Rust is not optional material; chapter
09 (logging), inserted ahead of the struct/OOP chapter on explicit request, which
pushed every later chapter up by one; and chapter 20 (PyO3), inserted after the
database chapters and ahead of capstone prep, for Python interop. Each
renumbering is why shipped
notebooks are always re-grepped for stale chapter numbers after a plan change
(§6 rule 3, and the forward-references list at the end of §18).

| # | Title | Slug | Contents |
|---|---|---|---|
| 01 | Syntax, Printing and Types | `syntax-printing-and-types` | items, bindings, `mut`, shadowing, scalars, inference, literals, operators, casting, overflow, floats, `char`/`&str`/`String` preview, printing, `format!`/`dbg!`, tuples, arrays, `const`/`static`/`type`, expressions vs statements, reading the compiler |
| 02 | Logic Blocks and Control Flow | `logic-blocks-and-control-flow` | `if`/`else if`/`else`, `loop`, `while`, `for`, ranges, `match`, `if let`, loop labels, `break` with a value |
| 03 | Functions | `functions` | parameters, return values, tail expressions, multiple returns via tuples, `const fn`, closures (introduction only) |
| 04 | Strings | `strings` | `String` vs `&str`, `&String` deref coercion, UTF-8, slicing and why it can panic, escapes, raw strings, byte vs char iteration, common methods |
| 05 | Ownership and Borrowing | `ownership-and-borrowing` | moves, `Copy` vs move, borrow rules, `&T` / `&mut T`, slices, lifetime elision preview, `Option` in depth |
| 06 | Native Collections | `native-collections` | arrays vs slices, `Vec`, tuples, `String` as a collection, `HashMap`/`HashSet`, `BTreeMap`/`BTreeSet`, `VecDeque`, iterator basics |
| 07 | I/O and Files | `io-and-files` | stdin, stdout vs stderr, command-line args, environment variables, `std::fs`, `BufReader`/`BufWriter`, `Path`/`PathBuf` |
| 08 | Assertions and Type Checks | `assertions-and-type-checks` | `assert!`, `assert_eq!`/`assert_ne!`, `debug_assert!`, `matches!`, `std::any`, `size_of`, `cfg!(debug_assertions)`, what is checkable at runtime vs compile time |
| 09 | Logging | `logging` | `tracing` events, spans and fields; the subscriber/layer model; `tracing-subscriber` fmt output (ANSI colour) and `tracing-appender` file sinks with rotation; level filtering (`EnvFilter`); errors with backtraces via `eyre`/`color-eyre` and `std::backtrace::Backtrace`; the Loguru mapping; evcxr kernel caveats |


Chapter 09 ends the **core track**: after it, the learner can read and write
idiomatic Rust without the language itself getting in the way — and can already
observe their own programs doing it. Chapters 10–22 form the **applied track**,
added on explicit request, and they change one rule: the no-crate rule no longer
applies globally (see the two-track rule below). Chapter 16 (Concurrency and
Async/Await) is the pivot of the applied track — 17, 18, 19 and 20 all assume its
`async`/`await` material.

### The core track (01–09)

Chapters 01–08 are std-only. Chapter 09 (Logging) is the one deliberate exception —
the first crate chapter of the whole series, placed in the core track because
observing a program is treated as basic literacy, not as an applied subject. Its
crate list and its evcxr-specific caveats are spelled out in the Loguru subsection
at the end of this section.

| # | Title | Slug | Contents |
|---|---|---|---|
| 01 | Syntax, Printing and Types | `syntax-printing-and-types` | items, bindings, `mut`, shadowing, scalars, inference, literals, operators, casting, overflow, floats, `char`/`&str`/`String` preview, printing, `format!`/`dbg!`, tuples, arrays, `const`/`static`/`type`, expressions vs statements, reading the compiler |
| 02 | Logic Blocks and Control Flow | `logic-blocks-and-control-flow` | `if`/`else if`/`else`, `loop`, `while`, `for`, ranges, `match`, `if let`, loop labels, `break` with a value |
| 03 | Functions | `functions` | parameters, return values, tail expressions, multiple returns via tuples, `const fn`, closures (introduction only) |
| 04 | Strings | `strings` | `String` vs `&str`, `&String` deref coercion, UTF-8, slicing and why it can panic, escapes, raw strings, byte vs char iteration, common methods |
| 05 | Ownership and Borrowing | `ownership-and-borrowing` | moves, `Copy` vs move, borrow rules, `&T` / `&mut T`, slices, lifetime elision preview, `Option` in depth |
| 06 | Native Collections | `native-collections` | arrays vs slices, `Vec`, tuples, `String` as a collection, `HashMap`/`HashSet`, `BTreeMap`/`BTreeSet`, `VecDeque`, iterator basics |
| 07 | I/O and Files | `io-and-files` | stdin, stdout vs stderr, command-line args, environment variables, `std::fs`, `BufReader`/`BufWriter`, `Path`/`PathBuf` |
| 08 | Assertions and Type Checks | `assertions-and-type-checks` | `assert!`, `assert_eq!`/`assert_ne!`, `debug_assert!`, `matches!`, `std::any`, `size_of`, `cfg!(debug_assertions)`, what is checkable at runtime vs compile time |
| 09 | Logging | `logging` | `tracing` events, spans and fields; the subscriber/layer model; `tracing-subscriber` fmt output (ANSI colour) and `tracing-appender` file sinks with rotation; level filtering (`EnvFilter`); errors with backtraces via `eyre`/`color-eyre` and `std::backtrace::Backtrace`; the Loguru mapping; evcxr kernel caveats |

### The applied track (10–22)

Order is again dependency-driven. 10–14 are std-only and finish the language
core (structs and traits, errors, iterators, modules, smart pointers). 15
introduces the data-domain crates; 16 is the async pivot; 17–19 are the network,
cache and database chapters the learner asked for explicitly; 20 (PyO3) bridges
Rust into the learner's Python world; and 22 closes with a capstone that uses
all of them together. (Chapter 09, back in the core track, is the crate
exception the two-track rule below spells out.)

| # | Title | Slug | Contents |
|---|---|---|---|
| 10 | Structs, OOP and Traits | `structs-oop-and-traits` | structs, `impl`, associated functions, methods, visibility, `derive`, traits, default methods, enums with data, generics, `Display`/`Debug` |
| 11 | Errors in Depth | `errors-in-depth` | error enums, `Box<dyn Error>`, `thiserror`-style manual `impl Display`/`std::error::Error` first, `From` conversions and `?` across error types, `main() -> Result`, panics vs recoverable errors, error conventions in library APIs |
| 12 | Iterators in Depth | `iterators-in-depth` | the `Iterator` trait, adaptors vs consumers, laziness and short-circuiting, `collect` and turbofish, `impl Iterator` return types, `fold`/`try_fold`, custom iterator `impl`, `iter`/`into_iter`/`iter_mut`, allocation behaviour of adaptors |
| 13 | Modules, Crates and Testing | `modules-crates-and-testing` | `mod`/`pub`/`use`/`as`, module trees and paths, `crate::`/`super::`, `cargo` layout, `#[cfg(test)]`, `#[test]`, `#[should_panic]`, `assert_eq!` in tests vs exercises, doc tests, `tests/` integration tests, workspace basics |
| 14 | Smart Pointers and Interior Mutability | `smart-pointers-and-interior-mutability` | `Box`, `Rc`, `Arc`, `Deref`/`Drop`, `Cell`/`RefCell` and the borrow rules they relax, `Weak` cycles, when `Rc<RefCell<T>>` is and is not the answer, `OnceCell`/`OnceLock` |
| 15 | Data Types and Their Libraries | `data-types-and-their-libraries` | time as a domain problem; `chrono` vs `time` (types, `Duration`/`TimeDelta`, time zones, formatting/parsing), the Rust analogue of Python's `pandas` for column/table data (`polars`), `uuid` and `rust_decimal` for identifiers and exact money, choosing representations before libraries |
| 16 | Concurrency and Async/Await | `concurrency-and-async` | OS threads, `Send`/`Sync`, channels (`mpsc`), `Mutex`/`RwLock`/atomics, `async`/`await` syntax and state machines, `Future`, the `tokio` runtime, spawning and joining, `select!`, blocking vs async and when each is right |
| 17 | HTTP Clients and WebSockets | `http-and-websockets` | reqwest blocking vs async clients, methods/headers/status, JSON with serde, query params and timeouts, streaming responses, `tokio-tungstenite` for WebSockets, message framing, ping/pong and reconnect loops, a small live price-feed example |
| 18 | Redis | `redis` | the redis crate with async connections (MultiplexedConnection), string/list/hash/set commands, key expiry and TTLs, pub/sub, pipelines and transactions, serialising values with serde, a cache-aside pattern on top of chapter 17 |
| 19 | PostgreSQL | `postgresql` | sqlx with async pools, compiling SQL macros, queries as/fetch_optional/fetch_all, row mapping to structs, migrations, transactions across await points, NULL mapping to `Option`, pool sizing and timeouts, comparing briefly with tokio-postgres/diesel |
| 20 | PyO3: Rust Inside Python | `pyo3` | why PyO3 (the escape hatch for a Python codebase); `#[pyfunction]` and `#[pymodule]` for functions; `#[pyclass]` and `#[pymethods]` for class-like types; `PyResult` and error conversion across the boundary; `Python<'py>` tokens and the GIL in brief; `IntoPy`/`FromPyObject` conversions and `PyDict`/`PyList`/`PyTuple`; calling Python from Rust; packaging with `maturin` and testing from `pytest`; how it connects to chapters 15–19 |
| 21 | Capstone Prep and Real-World Practice | `capstone-prep` | reading crate docs and CHANGELOGs, `cargo add`/feature flags, error handling across crate boundaries, benchmarking with criterion, structuring a small multi-module service, choosing the capstone scope (logging and tracing were moved out to chapter 09) |
| 22 | Capstone: Market Data Service | `capstone-market-data` | a guided build: ingest a WebSocket feed (17), cache state in Redis (18), persist to PostgreSQL (19), expose results on the command line, async error handling and graceful shutdown throughout; no new concepts, only assembly |

The crate rule is **two-track**. Chapters 01–08, 10–14 and 21's build environment
stay std-only: the project `challenges-rust` has no dependencies for those
chapters. Three chapter groups deliberately introduce crates, because their subjects
*are libraries* — teaching them from `std` alone would be fiction:

- **chapter 09 (Logging)** — the first crate chapter of the series: `tracing`,
  `tracing-subscriber`, `tracing-appender`, and `eyre`/`color-eyre` for
  backtrace-carrying errors;
- **chapters 15–20** — `chrono`/`time`, `polars`, `uuid`, `rust_decimal`, `tokio`,
  `reqwest`, `tokio-tungstenite`, `serde`, `redis`, `sqlx`, and `pyo3`+`maturin`
  (which also needs a `cdylib` cargo project beside the notebook — see §19a).

Each crate chapter must state its crate list and versions up front, and every
version must be verified to compile against the pinned toolchain before the
notebook is written — in evcxr's case, verified through `:dep` in a scratch
kernel session first.

### The Loguru → Rust mapping, and what chapter 09 must settle first

Chapter 09 exists because the author's logging taste was shaped by Python's
**Loguru**: one colourful, well-formatted `stdout` sink; extra sinks such as files
(with rotation) and network collectors like Loki; and `logger.exception()`, which
prints the whole traceback of the exception being handled. Rust's ecosystem
answers each of these, but differently, and the differences are the chapter's
content:

| Loguru habit | Rust idiom | Notes for the chapter |
|---|---|---|
| `logger.info(...)` and friends | `tracing::info!` and siblings — events | macros; levels can be filtered at compile time too |
| colourful formatted stdout | `tracing_subscriber::fmt` layer (ANSI) | the default look is already close to Loguru's |
| file sink + rotation | `tracing_appender::rolling` | needs a writable directory; non-blocking variant exists |
| extra sinks (Loki, network) | additional `Layer`s (e.g. `tracing-loki`) | mention, do not require network access |
| `logger.exception()` + full traceback | **no direct equivalent** | the honest core of the chapter — see below |
| `@logger.catch` decorator | nothing; explicit `Result` plus an `error!` event | Rust has no hidden control flow |

The traceback difference is structural, not incidental. Python runs in an
interpreter and can walk live frames when an exception is raised; Rust compiles to
machine code, frames are inlined, and a backtrace is best-effort. The idiomatic
pattern the chapter teaches instead:

1. capture the error **with context** using `eyre` (or `color-eyre`/`anyhow`),
   which records a `std::backtrace::Backtrace` inside the error object —
   `color-eyre` captures by default; plain `eyre` and panics honour
   `RUST_BACKTRACE=1`;
2. log it through `tracing` (`tracing::error!(error = %err, "...")`) so one event
   prints the message, the error chain and the captured backtrace together;
3. for the panic path, `std::panic::catch_unwind` plus `Backtrace::capture()`.

`rust-loguru` exists but is not the teaching target: it cannot reproduce
interpreter-level traceback capture either, and the industry standard is
`tracing`. The chapter should say this once and move on.

Facts chapter 09 must **verify before writing** (results go into §17):

- evcxr's `:dep crate = "version"` loads crates inside a kernel session; confirm
  the `tracing` family and `color-eyre` compile against the pinned toolchain that
  way.
- The kernel inherits the **Jupyter server's** environment, not the shell's:
  `RUST_LOG`/`RUST_BACKTRACE` may simply be unset. `std::env::set_var` from a cell
  works only *before* the subscriber/backtrace machinery first reads the variable.
- A process has **one** global subscriber (`set_global_default`), which in evcxr
  means one per kernel session. Reconfiguring means restarting the kernel (§7's
  habit); setup code should use `try_init`-style guards, not panic on a second
  initialisation.
- Debug builds usually symbolise fine; note the release-mode caveat (stripped
  symbols, aggressive inlining) once, without dwelling.
- `tracing-appender` needs a writable directory for its rolling files.

Scoping: chapter 09 owns *observing a running program* — events, spans, fields,
levels, sinks, and the error-plus-backtrace pattern that stands in for
`logger.exception()`. Error **type design** stays in chapter 11 (errors in depth),
and chapter 21 (capstone prep) no longer teaches logging; it may assume chapter
09's material as known.

## 4. Anatomy of an Example Notebook

`{NN}_{title}_e.ipynb` opens with three markdown cells, then alternates
markdown/rust for the rest of the file.

| Cell | Kind | Content |
|---|---|---|
| 1 | md | `# <Title>` — one-sentence purpose, plus who it is written for and how to read it |
| 2 | md | `### Contents` — anchor links to every `## N.` heading in the notebook |
| 3 | md | `## 0. Notebook Conventions: How These Cells Run` (see §7) |
| 4 | rust | `// 0.1` — a one-line smoke test, so the learner's first run proves the kernel works |
| 5… | md/rust | `## 1. Title`, then its markdown/code alternation |
| … | md | final numbered section (usually a cheat sheet or a review) |
| last | md | `### What Comes Next` — hand-off to the following chapters |

### The front matter cell (cell 1)

Fixed sub-headings, in this order:

1. Title as `# ...`, then a short paragraph naming the chapter's goal.
2. `## Who this is written for` — states the assumed background explicitly.
3. `## How to read this notebook` — bullets: run cells in order, errors are teaching
   material, this is a reference not a novel.
4. `## The one idea to internalise` — one paragraph naming the chapter's central
   mental model, with the recurring theme that Rust's strictness is a trade.

### The Contents cell (cell 2)

Every `## N. Title` heading appears exactly once, in order, as
`- [N. Title](#slug)`. Sub-headings (`###`) are *not* listed. After the list, one
short sentence pointing at the fastest route back (usually the cheat sheet).

### The body sections

Each section is `## N. Title` in a markdown cell, then one or more code cells. A
section is not one markdown cell and one code cell — it is a *sequence*:

```text
## 7. Operators, Casting and Safe Arithmetic     (markdown cell)
### The operator inventory                       (markdown)
<prose, or a table of operators>
                                                (code cell)  // 7.1 Operators: ...
### Precedence, and one place Rust fixes C       (markdown)
<prose>
### Integer division and remainder               (markdown)
                                                (code cell)  // 7.2 ...
```

Sub-headings (`###`) inside a section are expected and encouraged: they let one
section carry several distinct ideas without becoming a wall of text.

### The closing cell

`### What Comes Next` — names chapters 02–05 by title, gives each a one-line
description, and closes with a short "carry forward" list of the ideas the learner
should be able to use without looking them up.

### Numbering of example code cells

Every code cell's **first line is a caption comment**:

```rust
// N.M Short description of what this cell demonstrates.
```

- `N` matches the enclosing `## N.` section.
- `M` counts from 1 within the section, in file order.
- The description is a sentence fragment ending in a full stop, not a bare label.

So §7 with four demonstration cells yields `// 7.1`, `// 7.2`, `// 7.3`, `// 7.4`.

## 5. Anatomy of a Practice Notebook

`{NN}_{title}_p.ipynb` mirrors the example notebook's numbering exactly.

| Cell | Kind | Content |
|---|---|---|
| 1 | md | `# <Title> — Practice` — states that it is the companion of the `_e` file, by filename |
| 2 | md | `### Contents` — same list of section numbers as the example notebook |
| 3 | md | `## 0. How to Complete These Exercises` (see §7) |
| 4 | md | `### Exercise 0.1 — <title>` |
| 5 | rust | the stub + hidden grader |
| 6,7… | md/rust | alternating exercise/implementation pairs |
| last | md | `### What Comes Next` |

### Section headings in a practice notebook

There are **no `## N. Title` lecture sections** — only `### Exercise N.M — Title`
headings, each immediately followed by its single code cell. The section number
still exists in the Contents list so the learner can navigate by topic, and it
points at the first exercise of that section.

### Exercise counts

Aim for the same granularity as the example notebook's code cells: if `01e` §7 has
four demonstration cells (`// 7.1`–`// 7.4`), `01p` §7 should have four exercises
(Exercise 7.1–7.4). Section 0 always has exactly one exercise — the smoke test.

| Notebook | Sections | Exercises |
|---|---|---|
| `01e` | 0–16 | (30 example code cells) |
| `01p` | 0–16 | 19 exercises, `0.1` through `16.2` |

### What a practice notebook must never do

- **Never leave an exercise without a check block.** An exercise the learner cannot
  self-verify is not an exercise.
- **Never put the answer in the Hint.** A hint names the method, the operator or the
  shape of the solution — not the solution.
- **Never assume an exercise number that the example notebook does not have.** The
  `**See**:` line is a promise; keep it accurate (see §15, step 8).

## 6. The Pairing Rule Between `e` and `p`

The two notebooks of a chapter are **structurally isomorphic**:

```
01e                                    01p
## 0.  Notebook Conventions      <->    ## 0.  How to Complete These Exercises
## 1.  Hello, Rust               <->    ### Exercise 1.1, ### Exercise 1.2
...
## 7.  Operators, Casting...     <->    ### Exercise 7.1 … 7.4
## 16. Cheat Sheet               <->    ## 16. Review: Mixed Problems
### What Comes Next             <->    ### What Comes Next
```

Rules that follow from this:

1. **Same section numbers, same order, same meaning.** §9 in `01e` and §9 in `01p`
   are about the same thing.
2. **Same `### Contents` numbering.** Only the title text of each section may differ
   (a lecture section is a noun phrase; a practice section is "Review" or "Mixed
   Problems").
3. **`**See**:` cross-references must resolve.** Every practice exercise points at
   the example notebook's `§N.M`. If a section is renumbered in `01e`, every
   `**See**` in `01p` that mentions it must be re-checked. This has already drifted
   once and was caught by a grep.
4. **A new example section implies a new exercise.** Enriching `01e` is not finished
   until `01p` has a matching exercise.

### Rebalancing the pair when the example notebook grows

Adding content to an example section (as §7 gained an operator reference, a
precedence table and an `Option`/`Result` primer) has a knock-on effect:

- New `// 7.x` code cells → new Exercise 7.x entries in `01p`.
- Renumbered example code cells → renumbered exercises, and re-checked `**See**`.
- New sections anywhere → renumber everything after them in **both** files, and
  rebuild both Contents lists.

Renumbering is the expensive kind of edit, so **decide the section list for a
chapter before writing either notebook.**

## 7. Section 0 Must Explain the Kernel

Both notebook kinds open with a section 0 that teaches the evcxr execution model,
because every other convention in the series depends on it. The content differs by
kind.

### In the example notebook — `## 0. Notebook Conventions: How These Cells Run`

Must state:

- The kernel is **evcxr**, which wraps each cell into a generated program and
  compiles it incrementally; the learner is effectively typing the body of `main`.
- **No `fn main` in the cells**; the kernel supplies it.
- **State persists across cells**; a binding from cell 3 is live in cell 12.
- **Items are order-independent, statements are not.** Two `fn`s in one cell may
  call each other; two `let`s may not use each other's names early.
- **One namespace per notebook.** Two same-named items in different sections
  collide exactly as they would in one source file.
- **Re-running a cell that defines an item re-defines it**; on a redefinition error,
  restart the kernel and run from the top.
- **A failing cell does not poison the kernel.**
- **`println!` → stdout; `eprintln!` and `dbg!` → stderr.**
- The habit: restart the kernel before investigating "impossible" behaviour; and
  read the compiler's message before changing the code.

### In the practice notebook — `## 0. How to Complete These Exercises`

Must state:

- The same three kernel consequences (no `main`, state persists, items
  order-independent) — compressed, because the learner has already read them.
- **The unique-item-name warning**, called out as a blockquote, because it is the
  single most likely error in a practice notebook: every cell shares one namespace,
  so a redefinition error points at an *earlier* cell.
- **The stub convention**: signature + doc comment + `todo!()`, and the exact panic
  text the learner will see (`not yet implemented`).
- **Warnings are expected**: every unsolved stub whose parameters are untouched
  emits `unused variable`; the list self-cleans as exercises are solved. Only
  errors block.
- **Resetting**: restart the kernel, then run the cells you need.
- Closes with `**See**: §0 and §15 of the example notebook.`

## 8. Prose Style

The overriding instruction from the user: **summarised textbook explanations, not a
bible.** Enough context to make the fact stick, never a restatement of the obvious.

### The sentence pattern

> **Claim** → **why it matters** → **where it bites.**

For example, rather than "Rust does not do implicit conversions", write:

> There are no implicit numeric conversions. Neither widening nor narrowing happens
> by accident: `let b: i64 = a;` where `a: i32` is a compile error, not a silent
> sign-extension. This is why you will write `as` far more often than you expect,
> and why `From`/`Into` exist for the cases where a checked conversion is wanted.

### Rules

- **Pitch at an experienced programmer.** Assume functions, loops, floating-point
  approximation, and what an overflow is are all known. Name them once and move on.
- **Lead with the difference from Python and C++.** The learner already knows the
  concepts; what is new is where Rust diverges. Compare explicitly and often.
- **Tables over paragraphs** whenever three or more things are being enumerated
  (type sizes, operator precedence, macro variants, error codes).
- **Tag code fences by language** — `rust`, `python`, `cpp` — when contrasting. Never
  mix a Python comment into a `rust` fence; use three separate fences.
- **Bold the key term** on first use.
- **No emoji, no exclamation marks, no marketing tone.** Dry and precise.
- **State the toolchain's own behaviour, not the theoretical rule**, when they differ.
  The build is a debug build, so unguarded integer overflow *panics* — say so, and
  teach `wrapping_*`/`checked_*`/`saturating_*` as the explicit alternatives.
- **Deliberate compile errors are content.** Show them as a comment with the actual
  diagnostic text, because reading the compiler is a taught skill (see the example
  notebook's §15).

### Length discipline

- A markdown cell carries **one idea**, and rarely exceeds ~250 words.
- A code cell carries **one demonstration**, and rarely exceeds ~25 lines.
- The instinct to write a single large markdown cell with one code cell under it is
  the failure mode this series exists to avoid. Split it.

## 9. Cell Granularity

This was an explicit user requirement and is the most visible stylistic signature of
the series:

> "Try to avoid monolithic block sections: having a section with multiple markdown
> cells and code cells, each with small and concise snippets and paragraphs of
> content … that is better than having a large markdown cell with only one code
> cell."

The shape to aim for inside one section — a table, so the alternation is visible at a
glance:

| Order | Kind | Content |
|---|---|---|
| 1 | markdown | `## N. Section title` — one short framing paragraph |
| 2 | markdown | `### Sub-idea A` — 1–3 short paragraphs, or a table |
| 3 | rust | `// N.1 — caption` — the demonstration for sub-idea A |
| 4 | markdown | `### Sub-idea B` |
| 5 | rust | `// N.2 — caption` |
| 6 | markdown | `### Sub-idea C` |
| 7 | rust | `// N.3 — caption` |

Two or three sub-ideas per section is the normal size. A section that needs five is
usually two sections.

Practical test before committing a section: **count the sub-headings and the code
captions.** If a section has one markdown cell and one code cell but the markdown is
900 words, it is wrong. If a code cell runs past a screen without printing anything
intermediate, it is probably two demonstrations wearing one caption.

## 10. Code Cell Rules

These are mechanical. They exist because the example notebook must run cleanly on the
first attempt, and because the practice notebook's grader depends on cell-local
behaviour.

### Structural rules

1. **No `fn main`.** The kernel supplies it.
2. **First line is the caption comment** `// N.M description.`
3. **The cell must run in order**, given everything above it has run. Later cells may
   and should lean on earlier bindings.
4. **Item names must be unique across the entire notebook.** One namespace. Always run
   a duplicate-name scan before shipping (`§15`, step 4).
5. **Argument order in `println!` follows the format string**, and a captured binding
   (`{name}`) is preferred over a positional argument when the name is unambiguous.
6. **Deliberate failures are comments**, with the diagnostic they produce spelled out.

### Accuracy rules

7. **Do not teach a semantic fact from memory.** If a cell asserts something about
   casting, overflow, operator precedence or float representation, verify it with a
   throwaway probe program first. The verified facts are recorded in §17.
8. **Integer overflow panics in this build.** Any demonstration of wrapping must use
   `wrapping_add`, `checked_add`, `saturating_add` or `overflowing_add` explicitly.
   Plain `250u8 + 10` is a panic, not a demonstration of wrapping.
9. **`Option::map` closures overflow too.** `250u8.checked_add(5).map(|v| v * 2)`
   panics inside the closure. Pick a small operand for `map` demos.
10. **`unwrap_or`, `unwrap_err` and `unwrap_or_default` consume `self`.** If the cell
    also prints the container afterwards, use `.as_ref().unwrap_or(&-1)`,
    `.clone().unwrap_err()`, or restructure into a `match`. This produced two `E0382`s
    during chapter 01 authoring.
11. **Bare `a * b;` is `unused_must_use`.** Write `let _ = a * b;` in teaching
    examples so the warning list stays empty.
12. **Keep example notebooks at zero warnings.** A warning in `_e` is a bug; in `_p`
    it is expected until the stub is solved.

### Formatting rules

13. Prefer one idea per code cell, with a `println!` that makes the result visible.
14. Align trailing comments in tables of examples; they are read as tables.
15. Identifier style: `snake_case` for values and functions, `SCREAMING_SNAKE_CASE`
    for constants and statics, `UpperCamelCase` for types — and demonstrate the real
    lint (`non_snake_case`, `non_upper_case_globals`) rather than describing it.

## 11. The Exercise Templates

Two reusable shapes. Reproduce them exactly; the learner reads many of them and the
predictable structure is what makes them skimmable.

### Exercise markdown cell

```
### Exercise N.M — Short title, no full stop

**Goal**: one line, in the imperative, saying what will be true when this works.

**Info**: one to three lines of the fact the exercise depends on. This is not a
lecture — it is the missing premise. (Omit if the Goal is self-explanatory.)

**Given**: what is already provided. (Optional; most exercises omit it.)

**Objectives**

- bullet: a concrete, checkable outcome
- bullet: usually includes "keep the signature unchanged"

**See**: §N.M of the example notebook.

**Hint**: a nudge. Name the method or the operator or the shape — never the answer.
```

Field order is **Goal → Info → Given → Objectives → See → Hint**. `Hint` is always
last, so it sits immediately above the code cell and is the last thing read before
the learner starts typing. `See` always precedes it.

### Exercise code cell

```rust
/// One-line contract, phrased as the doc comment the learner would have written.
///
/// TODO: replace `todo!()` with <what is missing>.
fn name(params) -> Ret {
    todo!()
}

// --- check (do not modify) ---
assert_eq!(name(a), b);
assert_eq!(name(c), d);
println!("Exercise N.M passed");
```

Rules for the stub:

- **Doc comment with a TODO line.** The doc comment states the contract; the `TODO`
  line says what to replace.
- **Signature is given and must not change.** The check block is written against it.
- **`todo!()` for function stubs**, or a `// TODO` comment when the exercise is about
  statements rather than a function body.
- **The check block is fenced by `// --- check (do not modify) ---`.**
- **Every check uses a literal expected value.** No computing the answer twice.
- **The last line is `println!("Exercise N.M passed");`** — the learner's success
  signal, and the string the solvability harness greps for.
- **Three to six assertions**, chosen to cover the edges: negative inputs, a zero, a
  boundary. The `01p` §7.4 exercise checks `-7 % 3`, `7 % -3`, `rem_euclid`, and both
  nibbles of `0xAB` for exactly this reason.

### What the exercise must avoid

- No assertion whose expected value was never verified (§15, step 7).
- No exercise that requires reading the example notebook's *output* rather than its
  prose.
- No two functions in one cell with the same name as anything else in the notebook.

## 12. Anchor Links and Headings

Jupyter's markdown renderer produces GitHub-style slugs. The rule, derived from the
existing Contents cells:

| Step | Rule |
|---|---|
| 1 | Take the heading text without the leading `#`s. |
| 2 | Delete backticks and all punctuation *except* hyphens and underscores. |
| 3 | Lowercase. |
| 4 | Replace each run of spaces with a single `-`. |

Worked examples:

| Heading | Anchor |
|---|---|
| `## 7. Operators, Casting and Safe Arithmetic` | `#7-operators-casting-and-safe-arithmetic` |
| ``## 11. `format!`, `dbg!` and Debug vs Display`` | `#11-format-dbg-and-debug-vs-display` |
| ``## 9. `char`, `&str` and `String`: A First Look`` | `#9-char-str-and-string-a-first-look` |
| `## 0. Notebook Conventions: How These Cells Run` | `#0-notebook-conventions-how-these-cells-run` |

Note how `` `&str` `` becomes `str` — the ampersand and the backticks both vanish.
That is not a mistake in the list; it is what the renderer does.

**Only `##`-level headings appear in the Contents.** `###` sub-headings and
`### Exercise N.M` headings are not linked, because there are dozens of them and the
list would stop being navigable.

## 13. Technical Requirements (the JSON)

Every notebook in the series must carry exactly this shape. `_build_nb.py` writes it
directly; the retired `_patch_meta.py` enforced it on hand-built notebooks (§14).

### Notebook level

| Key | Value |
|---|---|
| `nbformat` | `4` |
| `nbformat_minor` | `5` |
| `metadata.kernelspec` | `{"display_name": "Rust", "language": "rust", "name": "rust"}` |
| `metadata.language_info` | see below |

```json
"language_info": {
  "name": "Rust",
  "codemirror_mode": "rust",
  "file_extension": ".rs",
  "mimetype": "text/rust",
  "pygment_lexer": "rust",
  "version": ""
}
```

The `kernelspec.name` is `rust` because the installed kernelspec directory is
`C:/DEV/Rust/jupyter/kernels/rust` (the `evcxr_jupyter` kernel). Do not invent a
different name.

### Cell level

| Cell kind | Requirement |
|---|---|
| code | `metadata.vscode.languageId = "rust"`, `outputs: []`, `execution_count: null` |
| markdown | no special metadata |
| both | `source` is a **list of lines**, each ending in `\n`, with exactly one trailing newline |

### Serialisation

Written with `json.dump(..., ensure_ascii=False, indent=1)` plus a final newline.
`ensure_ascii=False` matters: the notebooks use `§` throughout, and escaped
`\u00a7` sequences make the raw JSON unreadable when diffing.

### Never edit notebook JSON through a shell heredoc

Non-ASCII characters, and `§` in particular, get mangled. Use the notebook editing
tooling, or a Python script that reads and writes UTF-8 explicitly.

## 14. Tooling

The scripts described here are workspace-local build and validation tools, not
deliverables. **Status at series completion:** `_patch_meta.py` is retired (§9);
`_validate.py` was lost in the end-of-series cleanup and reconstructed from the
contract below on 2026-09-30, then re-proven on all three gate shapes; `_build_nb.py`,
`_crate_validate.py` and `_externs.txt` remain (documented in §19). The retired
metadata normaliser is documented below for reference.

### `_patch_meta.py` — metadata normaliser

```bash
python _patch_meta.py 02_logic-blocks-and-control-flow_e.ipynb 02_logic-blocks-and-control-flow_p.ipynb
```

Sets `kernelspec` and `language_info`; adds `metadata.vscode.languageId = "rust"` to
every code cell; initialises `outputs` and `execution_count`; normalises `source` to a
line list with a single trailing newline; writes UTF-8, `indent=1`. Prints a summary
of cells by kind.

Run it on **every** notebook, immediately after creating it. It takes `.ipynb` files
only — never pass it this document or any other `.md` file.

### `_validate.py` — the compile-and-run harness

```bash
python _validate.py 01_syntax-printing-and-types_e.ipynb
```

What it does:

1. Reads every code cell in order.
2. Splits each cell into **items** and **statements**, tracking brace depth, so an
   item's body is never cut in half. An item is a line starting at column 0 with
   `fn `/`pub fn `/`const fn `/`async fn `/`const `/`static `/`type `/`use `/`impl`/
   `struct `/`enum `/`trait `/`pub `/`#[derive`.
3. Hoists all items to the top level, keeps statements in order, and compiles them
   inside a generated `fn main() { ... }`.
4. Writes `_check.rs`, runs `rustc --edition 2024`, then executes `_check.exe`.

This is faithful to evcxr because Rust items are order-independent — hoisting does not
change what compiles. It is deliberately **not** a reimplementation of the kernel: it
does not deduplicate item names, which is what lets it catch real redefinition bugs.

### Reading diagnostics through the validator

The validator prints a summary line, `error: aborting due to N previous errors`, which
sorts **above** the real diagnostics — so `grep "^error"` on its output catches only
that line. To see the actual errors, run the compiler directly:

```bash
rustc --edition 2024 -A warnings -o _check.exe _check.rs
```

To audit which warning *categories* a practice notebook produces:

```bash
python _validate.py 01_syntax-printing-and-types_p.ipynb 2>&1 \
  | grep -E "^warning: " | grep -v "unused variable" | sort -u
```

### The solvability harness

For a practice notebook, this is the only way to know the exercises are answerable and
the expected values are right:

1. Write `_solN.rs` containing a reference implementation of **every** function the
   exercises ask for, plus a **verbatim** copy of every `// --- check (do not modify) ---`
   block from the notebook.
2. Compile and run it.
3. Pass criterion: **every** `Exercise N.M passed` line prints, and the process exits 0.

If a check fails, either the expectation or the reference implementation is wrong — and
finding out which is exactly the point, before the learner does.

## 15. Verification Checklist

Run this for **every chapter**, in order. Steps 3, 6 and 7 are the ones that have
already caught real bugs.

### 1. Fix the section list first

Write down the chapter's `## N.` sections and their titles before writing any prose.
Renumbering afterwards means rebuilding two Contents lists and revisiting every
cross-reference.

### 2. Write `NN_<slug>_e.ipynb`

Front matter, Contents, §0, sections, closing. Keep code cells short and captioned.
Verify any semantic claim you are not certain of with a throwaway probe program.

### 3. Build the notebook, then validate `_e`

**The notebooks are the canonical artifacts.** The plain-text `.src` authoring
aggregates were deleted after series completion: the notebooks carry the gated state
where the two ever diverged (04e), every gate runs on the `.ipynb` directly, and cells
can be re-wrapped into `.src` form losslessly if a chapter is ever revised. `_build_nb.py` is kept
(§19) for that reason. The historical workflow was:

```bash
cat _<NN>v_part*.src > NN_<slug>_e.src   # or author the aggregate directly
python _build_nb.py       NN_<slug>_e.src
python _crate_validate.py NN_<slug>_e.ipynb
```

Under the retired tooling, metadata was patched by `_patch_meta.py` and the notebook
was compiled by `_validate.py`, which wrote `_check.rs` and ran `_check.exe` directly.

**Pass criterion:** `rustc` exits 0, **zero warnings**, and the program runs to
completion (exit 0) producing sensible output. A panic here is a real bug — remember
the debug build and the overflow rules in §10.

### 4. Duplicate-name scan

```bash
python - <<'PY'
import json, re
ITEM = re.compile(r'^\s*(?:pub\s+)?(?:fn|struct|enum|trait|const|static|type)\s+([A-Za-z_][A-Za-z0-9_]*)', re.M)
for f in ['NN_<slug>_e.ipynb', 'NN_<slug>_p.ipynb']:
    nb = json.load(open(f, encoding='utf-8'))
    names = [m for c in nb['cells'] if c['cell_type'] == 'code'
             for m in ITEM.findall(''.join(c['source']))]
    dupes = {n for n in names if names.count(n) > 1}
    print(f, 'items', len(names), 'dupes', dupes)
PY
```

**Pass criterion:** `dupes == {}` for both notebooks. A duplicate is a guaranteed
redefinition error in the learner's kernel.

### 5. Write `NN_<slug>_p.ipynb`

One exercise per example section (at minimum), numbered to match. Use the templates
in §11.

### 6. Build the notebook, then validate `_p`

```bash
cat _<NN>v_part*.src > NN_<slug>_p.src
python _build_nb.py       NN_<slug>_p.src
python _crate_validate.py NN_<slug>_p.ipynb
```

**Pass criterion:** `rustc` exits **0** (no compile errors). The run is *expected* to
abort with `not yet implemented` at the first `todo!()`, with a non-zero exit code —
that is correct behaviour for an unsolved stub. Warnings are expected and must all be
`unused variable` or an unused item, with nothing else in the category audit.

### 7. Prove every exercise solvable

Write `_solN.rs` (see §14), compile, run.

**Pass criterion:** every `Exercise N.M passed` prints.

### 8. Cross-reference grep

```bash
grep -n 'See\*\*: .*§' NN_<slug>_p.ipynb
```

Read every result and confirm the `§N.M` it names really is the section that teaches
that material. This drifted once during chapter 01 when §7 was restructured; the
grep is cheap and the drift is invisible otherwise.

### 9. Delete the scratch files

**Done at series completion.** The following were deleted once every chapter was
gated green: all `_sol*_gen.py` generators, `_sol*.rs` / `_sol*.exe`, `_check.rs` /
`_check.exe`, the `_probe*` scratch programs, every `_NNv_part*.src` intermediate, the
run and validation logs, and the `pg_tx*.log` ledger receipts. `_patch_meta.py` was
retired with them. `_validate.py` went too — a mistake, since `_crate_validate.py`
imports it — and was reconstructed from the contract in §14 on 2026-09-30 and
re-proven (01e 0/0/0, 17e 0/0/0 through the bridge, 01p stub state 0 err / exit 101).
`00_NB-STRUCT.md`, `_build_nb.py`, `_validate.py`,
`_crate_validate.py` and `_externs.txt` remain (§19).

## 16. Hard-Won Gotchas

Every item below has already cost time once. They are ordered roughly by how likely
they are to recur.

### Notebook mechanics

| Symptom | Cause and fix |
|---|---|
| A cell you edited shows the *previous* content | The on-disk file lags briefly after an edit. Read it back independently before re-diagnosing. |
| Editing the wrong cell clobbered text | The notebook summary's entry *N* is cell index *N−1*. Confirm a cell by its **content** immediately before editing, never by assumed position. |
| `Missing viewType` error | The notebook editing tool was used on a plain `.rs` file. Use ordinary file tools for real files. |
| Non-ASCII characters corrupted | The JSON was patched through a shell heredoc. Never do that. |

### evcxr / kernel semantics

| Symptom | Cause and fix |
|---|---|
| "name is defined multiple times" | Item names must be unique across the whole notebook. The error points at an *earlier* cell. |
| Behaviour that cannot be reproduced | Stale kernel state. Restart the kernel and run top to bottom; it is cheap and always correct. |
| A cell defining a function fails on re-run | Re-running redefines the item. Restart the kernel. |
| `use` of a module's item fails with E0432 although the module exists | the validator did not hoist bare `mod x {}` (it nested inside `main()`). `"mod "` is in `ITEM_STARTS`; if a future item kind misbehaves the same way, extend that tuple. |
| `let final = ..` fails with "expected identifier, found reserved keyword" | `final` is a reserved keyword in Rust even though it has no meaning yet. Inside a macro call it is worse: `assert_eq!(final, ..)` reports "no rules expected reserved keyword" from inside `macros/mod.rs`. Rename (ch20 used `total`). |
| `_t: PhantomData` in a struct expression is E0425 | `PhantomData` is not in the prelude and the type-position path in the struct definition does not import it. Spell `std::marker::PhantomData` in the constructor too (ch20 §5). |
| `expected item, found keyword let` on a statement that is fine | The splitter's item brace-counter counts `{`/`}` **inside string literals**: a cell string containing an unpaired `{` (e.g. a junk-JSON fixture `b"{not json"`) left the enclosing `fn` "open" and swallowed the cell's trailing statements into the item. Fix the fixture text (or avoid unbalanced braces in literals); found in ch17 gating. |
| `Ok(x)` in an async block errors with E0283 "cannot infer type of the type parameter E" | With multiple `From<_> for YourError` impls in scope, the `?` after the block leaves the error type ambiguous. Spell it: `Ok::<T, YourError>(x)` (ch22's `run_pipeline` miss-probe). |
| Check code panics with "Cannot drop a runtime" even though the stubs are clean | The CHECK block obeyed the blocking rule too: a reqwest-blocking helper (`ingest`) called inside `rt.block_on(async { .. })` owns its own runtime, and its drop panics. Hoist the blocking call above the `block_on`, pass the data in (ch22 4.1/9.2). |
| A `:dep`-driven crate gets a new rlib hash after a rebuild wave | Cargo re-hashes feature-unified deps (tokio, futures-util) whenever a new crate joins the scratch project. **All `--extern` rlibs handed to rustc must come from the same cargo build wave** — a mismatched tokio pair surfaces as bogus `E0277: AsyncRead not implemented for tokio TcpStream`. When in doubt, rebuild and re-copy every hash. |

### Rust facts that are counter-intuitive

| Symptom | Cause and fix |
|---|---|
| "attempt to multiply with overflow" in a demo | This is a **debug build**: unguarded overflow panics. Use `wrapping_*` / `checked_*` / `saturating_*` to demonstrate the other behaviours. |
| Overflow panic inside a `map` closure | The closure's arithmetic is as unchecked as top-level arithmetic. Use small operands in `map` demos. |
| `E0382: borrow of moved value` when printing a container *and* unwrapping it | `unwrap_or`, `unwrap_err` and friends take `self`. Use `.as_ref()`, `.clone()`, or a `match`. |
| `unused_must_use` warning on a bare arithmetic statement | Wrap it: `let _ = a * b;`. |
| `{x=}` fails to compile | That is Python f-string syntax. Rust's diagnostic suggests `dbg(x)`; teach this deliberately. |
| `{:>8?}` does not pad a bare `&str` | Width is ignored for `Debug` on a `&str`, but honoured for numbers and for container *elements*. `Display` (`{:>8}`) does pad. |
| An error "is not shown" by the validator | The validator's summary line sorts above the diagnostics. Run `rustc` directly. |
| A grep for `§7.1` returns 0 results | The section was renumbered and the cross-reference was not. Run checklist step 8. |
| Rust-level claim in prose disagrees with the compiler | The claim was written from recall. Probe it. Chapter 13 caught two: bare `pantry::restock()` from a sibling module is E0433 (first path segments do not climb to the root — only `super::`/`crate::` reach siblings), and a literal `panic!("..")` payload downcasts as `&str`, not `String`. |

### Process

| Symptom | Cause and fix |
|---|---|
| A "finished" section needs renumbering | The section list should have been fixed before writing. Do that first, every time. |
| The practice notebook has no exercise for new example material | They are a pair. Enriching `_e` is not done until `_p` matches. |
| An exercise has an impossible check | The check's expected value was never verified. Run the solvability harness (checklist step 7) before shipping. |
| The validator's item-section swallows the statement after a `use` | A `use` line carrying a trailing `//` comment hides its `;`. Move the comment above the `use` (ch13 §4 teaches exactly this). Bit again in ch19 while authoring parts — keep comments off `use` lines in part files too. |
| A repeated `use` line is E0252, not tolerated | One namespace per notebook: the second import of a name is a redefinition once cells are flattened. The first cell that needs the name does the `use`; later cells rely on §0's persistence rule. **When authoring in parts, dedupe the `use` lines in the part files themselves** — a dedup pass on the assembled file is lost the next time you rebuild it from the parts (bitten for real in ch18: 11 copies of `use redis::AsyncCommands;`). |
| A crate trait import collides with a notebook-local type of the same name | `use sqlx::Row;` fought ch19's desk type `Row` to E0255. The desk type was renamed `WireRow` (cell alias `Cell`). General rule: name notebook-local types away from trait names you plan to import, and put each `use` in the FIRST cell that needs it (2.1, not earlier — an earlier import is an unused-import warning in the 0-warning `_e` gate). |
| A demo item that only *shows* a construct draws dead-code warnings in `_e` | The 0-warning gate has no `#![allow]` escape (no inner attributes in cell bodies). Every defined item must be exercised by an assert or println in some cell. |
| The solvability harness reports fewer stubs than exercises exist | A stub signature split across multiple lines is invisible to the harness's `fn name(...) {` matcher — the `todo!()` survives and the run aborts with exit 101 mid-check. Keep every `_p` stub signature on ONE line, however wide (ch18: `broadcast` alone cost a debug cycle). |

## 17. Verified Environment Facts

These were established by probe programs rather than recalled, and are safe to teach
directly.

### Toolchain

| Fact | Value |
|---|---|
| `rustc` | 1.98.1 (`48a229cea 2026-09-01`) |
| `cargo` | 1.98.1 (`797e8a9bc 2026-08-05`) |
| Edition | 2024 |
| Toolchain path | `C:\DEV\Rust\.rustup\toolchains\stable-x86_64-pc-windows-gnu` |
| Project | `challenges-rust` 0.1.0, **no dependencies** |
| Kernelspec | `evcxr_jupyter`, installed at `C:/DEV/Rust/jupyter/kernels/rust`, name `rust` |
| `cfg!(debug_assertions)` | **`true`** — the default build is a debug build |

**The debug-build fact is the most consequential one in this document.** Plain
integer arithmetic overflow panics in every notebook cell. `1u32 / 0` panics,
`1u8 << 8` panics, `250u8 + 10` panics. Anything meant to illustrate wrapping,
saturation or overflow detection must say so through a `wrapping_*`, `checked_*`,
`saturating_*` or `overflowing_*` method.

### Module system and tests (probe-verified, ch13)

| Fact | Value |
|---|---|
| `#[test]` fns under plain `rustc` | **stripped** from the compilation, not merely skipped — calling one is E0425 `cannot find function` |
| `#[cfg(test)]` items under plain `rustc` | likewise stripped; `cfg!(test)` folds to `false` |
| `cfg!(test)` false branch | must still **compile** — but a stripped item cannot be named there, only in a comment |
| privacy direction | flows **down**: a child module reads the parent's private items; outside callers get E0603 |
| `pub(super)` | ceiling of exactly one level; decided by the caller's **ancestry**, not the path spelling |
| bare-path first segments | resolve in the current module and **do not climb** to the root: `pantry::restock()` from sibling `kitchen` is E0433 (edition 2024) |
| `pub(crate)` vs `pub` | indistinguishable from inside the crate; the difference is the library boundary (ch15) |
| `use garage::Gear::{self, Reverse};` | legal — `self` in a brace group names the enum itself, alongside its variants |
| `use spices::{self as sp, pepper};` | legal — rename the module and import an item in one line |
| rename-alone `use` | unused-imports warning — `use` writes the lookup row; nothing reading it is dead weight |
| failing `cargo test` | one failure does not stop the run; `test result: FAILED. N passed; M failed` line = non-zero exit |
| doc tests | each `///` fence is a separate test (`test src\lib.rs - bump (line 3) ... ok`); failing ones panic from a temp `rustdoctest…doctest_bundle_*.rs` |

### Smart pointers and interior mutability (probe-verified, ch14)

| Fact | Value |
|---|---|
| sizes | `Box<T>` 8; `Box<dyn Display>` 16 (fat); `Rc`/`Weak`/`Arc` 8; `Cell<i32>` 4 (= T, zero overhead); `RefCell<i32>` 16 (the borrow flag rides along); `OnceCell<i32>` 8 |
| `Rc` mechanics | clone bumps strong, drop decrements, payload frees at 0; `ptr_eq` = same allocation; `get_mut`/`try_unwrap` refuse while shared |
| RefCell violation panic | message `RefCell already borrowed`, **payload a `String`** — `downcast_ref::<&str>()` is `None` |
| `OnceCell::set` on filled | returns `Err(v)` — rejected, not panicked; second `get_or_init` closure never runs |
| `impl Deref` without `Target` | **E0046** (`not all trait items implemented`), not a lifetime error |
| `impl Drop for i32` | E0117 (orphan) **and** E0120 (`Drop` only for local structs/enums/unions) — the pair arrives together |
| `Rc` into `thread::spawn` | E0277; the `Arc<RefCell<_>>` variant's note suggests `std::sync::RwLock` — the ch16 bridge |
| cycle vs `Weak` | a strong cycle never drops (leak is deterministic); a `Weak` back-edge drops cleanly; `upgrade()` → `None` after the last strong drops |
| drop timing | `mem::drop` is immediate; locals LIFO at scope end; evcxr keeps end-of-cell bindings alive — prove timing with inner scopes |

### Data types and their libraries (probe-verified, ch15)

| Fact | Value |
|---|---|
| chrono `weekday()` in 0.4.45 | inherent method is **private** — resolves only through the `Datelike` trait (E0599 otherwise); `Timelike` likewise owns `.hour()`/`.minute()` |
| `NaiveDate::from_ymd_opt` | `None` for `2026-02-30` and `2026-02-29` — the calendar is validated at construction, never wrapped |
| span types | `std::time::Duration` unsigned (timeouts); `chrono::TimeDelta` signed — `to_std()` rejects negatives; `DateTime + TimeDelta` does calendar arithmetic (22:00 + 30h → Oct 1, 04:00) |
| `time::Time::from_hms` | returns **`Result`** (only `*_unchecked` constructors are infallible); `Month` is an enum — month 13 is unwritable, not rejected |
| chrono vs `time` templates | chrono `%`-strings are runtime data; `time`'s `format_description!` is compile-time-checked (needs the `macros` feature) |
| parsing offsets | text **without an offset fails** against `OffsetDateTime` (the type demands one); `PrimitiveDateTime` accepts it; `assume_utc() ==` the RFC3339-parsed instant |
| zone equality | `with_timezone` re-lenses: NYC/Tokyo/UTC views of one instant all compare `==`; `%:z` prints `-05:00` |
| `Decimal` scale arithmetic | multiplication **adds scales**: `from(3)` has scale 0, so 2dp×int = 2dp (`370.35`); two 2-dp factors give 4 (`1.50×2.50 = 3.7500`); `0.1+0.2 == 0.3` exactly |
| `Decimal` rounding | default is banker's (`2.5→2`, `3.5→4`); desks owe `MidpointAwayFromZero` (`2.675→2.68`); `370.3500 == 370.35` (value equality, scale = provenance) |
| uuid v4 anatomy | 36 chars; version nibble `hex[12]='4'`; variant `hex[16]∈{8,9,a,b}`; `Uuid` is a 16-byte `Copy` value; `parse_str` fallible at the boundary |
| polars from rows | `DataFrame::from_rows` names positional columns **`column_1..`**; use `from_rows_and_schema` + `Schema::from_iter([("sym".into(), DataType::String), …])` to declare the schema |
| polars `Row`/`AnyValue` | **not in the prelude** (`polars::frame::row::Row`, `AnyValue::String(&str)` borrows → `Row<'a>` tied to the input slice, consumed immediately by `from_rows*`) |
| polars lazy API | `sort` takes column-**name** arrays (`SortMultipleOptions::with_order_descending`); `group_by`→`agg`→`alias`→`collect`; results chain as plans |
| polars CSV roundtrip | `CsvWriter` → `Vec<u8>`, read via `Cursor` + `CsvReadOptions::with_has_header(true)`; assert `dtypes()` equality — schema is the contract |
| linking polars offline | winapi's **import libs** are required: `native=C:/…/winapi-x86_64-pc-windows-gnu-0.4.0/lib` in `_externs.txt` → `-L native=` (new `_crate_validate.py` convention); two `libtime-*.rlib` exist — the feature-enabled one is the larger, later-built `libtime-d04db6ea6ee6f6ae.rlib`; polars `_check.exe` ≈ 1.8 GB debug |

### Operator and casting semantics (probe-verified)

| Expression | Result |
|---|---|
| `` `1u8 & 3u8 == 1u8` `` | parses as `(1) == 1` → `true` — `&` binds tighter than `==`, unlike C |
| `1u8 << 2 & 4u8` | `4` |
| `!0u8`, `!5u8`, `!true` | `255`, `250`, `false` |
| `-5 % 3`, `5 % -3`, `-5 / 3` | `-2`, `2`, `-1` — `/` truncates toward zero, `%` takes the sign of the left operand |
| `2.9f64 as i32` | `2` — truncates when in range |
| `1e300f64 as i32` | `i32::MAX` — out of range **saturates**, it does not wrap |
| `-1e300f64 as i32` | `i32::MIN` |
| `f64::NAN as i32` | `0` |
| `1e300f64 as u8` | `255` |
| `1u32 / 0` | panics: `attempt to divide by zero` |
| `1u8 << 8` | panics: `attempt to shift left with overflow` |

### `Option` / `Result` accessors

`checked_add` returns an `Option`; `str::parse` returns a `Result`. Both print
usefully under `{:?}` (`Some(255)`, `Ok(42)`, `Err(ParseIntError { kind: InvalidDigit })`).
Their consuming accessors (`unwrap_or`, `unwrap_err`) are what break cells that also
want to print the value — hence the `.as_ref()` rule in §10.

### HTTP clients and WebSockets (probe-verified, ch17)

| Claim | Verified behaviour on rustc 1.98.1 / reqwest 0.12.28 / tokio-tungstenite 0.24.0 |
|---|---|
| Status contract | `send()` is `Ok` for 404/500 — `error_for_status()` is the opt-in bridge; `text()`/`json()` **consume** the response (read status/headers first, E0382 otherwise) |
| Redirects | followed silently by default (`resp.url()` = destination); `Policy::none()` returns the 302 + `Location`; `Policy::limited(n)` bounds the chase |
| Client timeout | `Client::builder().timeout(d)` bounds the whole exchange; on fire, `send()` is `Err` — measured 400ms builder fired at ~500ms wall |
| Streaming | `bytes_stream()` requires the **`stream` feature**; chunk loop `while let Some(item) = stream.next().await`; async-only |
| serde | junk JSON → `Err` at the awaited `json()`; missing field → Err naming the field; `#[serde(default)]` fills; derived structs compare with `PartialEq` |
| WS handshake | `connect_async` returns `(WebSocketStream, Response)`, `resp.status() == 101`; **tungstenite 0.24 REJECTS a client sending `Sec-WebSocket-Protocol` the server does not echo** (`SecWebSocketSubProtocolError`) — negotiation needs `accept_hdr` with a response callback |
| WS close | close is bidirectional: send Close, peer replies, **server must drain after replying** or the client sees `ResetWithoutClosingHandshake` |
| WS server | `accept_async` takes a **tokio** `TcpStream` (std listener `.accept().await` is E0277); a runtime that owns accept loops must be parked (`OnceLock`), not dropped — dropped runtimes kill their spawned tasks |
| `Message` 0.24 | `Message::Ping(Vec<u8>)` (Bytes change is later tungstenite); `is_text`/`to_text`, `is_binary`/`into_data` (consuming) |
| Sub-protocol note | the desk convention: server mirrors the client's requested subprotocol in its 101 response |

### Redis client (probe-verified, ch18)

| Claim | Verified behaviour on rustc 1.98.1 / redis 0.27.6 (tokio-comp+serde) against an in-notebook RESP2 server |
|---|---|
| Handshake | the crate sends `CLIENT SETINFO LIB-NAME/LIB-VER` x2 (both `.ignore()`d), `SELECT` only when the URL names a db; unknown-verb replies to those are tolerated |
| Multiplexed clone | `MultiplexedConnection::clone()` is a handle to the shared socket; dropping a clone keeps the connection; PING answers through either handle |
| Wire quirks probed | the crate's `incr(key, delta)` sends **INCRBY**; `pexpire` sends **PEXPIRE**; `hset_multiple` sends legacy **HMSET** (+OK, no added-count); `set_ex` sends **SETEX** |
| Reply types chosen at call site | un-annotated command = E0282; `get::<Option<String>>`; absent key is `Ok(None)` |
| Pipelines | commands queue and write in one go; **`.ignore()` removes the reply AND its tuple slot** (`(i64,)` for one kept reply) |
| atomic() | wraps the queue in MULTI/EXEC on the wire; queued commands answer `+QUEUED`; EXEC replies with one array decoded into the tuple; guarantee = no interleaving, not rollback |
| Pub/sub | `get_async_pubsub` opens a **dedicated socket** (a muxed connection cannot subscribe); `PUBLISH` returns the receiver count (0 ok); subscribe confirms and message pushes are **3-element arrays with all string elements as BULK strings**; `into_on_message()` consumes the connection, `on_message()` borrows it (E0499 if you unsubscribe while the borrow lives) |
| Errors | two families discriminated by `err.kind()`: server-signalled `ResponseError` (`unknown command 'X'`) vs decode-side `TypeError` (`Response was of incompatible type`) |
| RESP2 protocol notes | every reply type is CRLF-terminated **including bare `+OK`/`:N` and every element inside arrays** — one missing CRLF hangs the client's parser indefinitely (not an error, a wait); partial reads must accumulate: a read timeout mid-command that discards the buffer desyncs the stream |
| TTL semantics | `ttl` = −2 gone / −1 no countdown / n seconds; `expire` on a missing key is `Ok(false)`; expiry deletes server-side (lazy sweep on access like real Redis) |

### Capstone desk (probe-verified, ch22)

| Claim | Verified behaviour on rustc 1.98.1 / the ch19 crate wave (no new crates) |
|---|---|
| Three desks, one namespace | ch17's HTTP server, ch18's RESP2 server, ch19's v3 server ported verbatim with prefixes `md_`/`rd_`/`pg_` (E0428 avoided by construction); ledgers split per notebook (`pg_tx_c.log` / `pg_tx_p.log`) |
| One error spine | `MdServiceError::{Feed,Cache,Store}` + `From` per crate; the sqlx impl keeps `db.message()` + SQLSTATE (`42P01` survives the wrap); layered Display (`feed: …`) |
| Cache-aside proof | the HTTP request log proves it causally: two lookups → one `GET /json` — the origin saw the miss, not the hit; run_pipeline's origin fetch must be CONDITIONAL on the miss or the second pass re-fetches |
| Blocking/async boundary | reqwest blocking inside `block_on` panics with tokio's "Cannot drop a runtime…" — reproduced THREE times while authoring (22e §3.1's first draft, then two check blocks calling `ingest()` inside a runtime); the origin call happens outside `block_on`, always |
| Dead-store probing | `md_pool_on` unwraps by design; probing a dead port THROUGH it panics after a 5s PoolTimedOut — probe via the builder's `Err` directly with a short `acquire_timeout`, then `From` it into the spine |
| Ambiguous `?` error | inside a `block_on(async { Ok(x)? })`-shaped block with multiple `From<MdServiceError>` impls, bare `Ok(..)` is E0283 (cannot infer `E`) — write `Ok::<Option<String>, MdServiceError>(raw)` |
| Harness limits, final form | FN_HEADER needs one-line signatures — 22p lost `store_quote`/`fetch_quote`/`close_all`/`audit_pass` to multi-line sigs (the last caught only when the count came up 25/26); given `MdSummary` needed `Clone` for check-block reuse; `&Ok(s)` moves — clone at the first use |

### Capstone prep tooling (probe-verified, ch21)

| Claim | Verified behaviour on rustc 1.98.1 / criterion 0.8.2 (spike21 desk) |
|---|---|
| criterion setup | `[dev-dependencies] criterion = "0.8"`; `[[bench]] name = "desk", harness = false`; `benches/desk.rs` with `criterion_group!`/`criterion_main!`; first build compiles the dev-dep ring, later bench runs are seconds |
| criterion output | `desk/slow time: [146.61 ns 148.61 ns 151.31 ns]` + a `thrpt:` line only when `Throughput::Elements` is set; `Gnuplot not found, using plotters backend` is a report, not an error; ~9% outliers on a desktop OS are normal and statistically down-weighted |
| Baselines + CI | `cargo bench -- --save-baseline before` stores, `--baseline before` compares (the regression gate); `cargo bench -- slow` name-filters; `cargo bench -- --test` runs each bench ONCE (`Testing desk/slow ... Success`) — compile-and-run without sampling |
| black_box | `std::hint::black_box(&input)` stops the optimizer constant-folding the timed work away |
| cargo add | `--dry-run` prints the resolved version + full feature inventory (`+` enabled, `-` available) then `warning: aborting add due to dry run`; `--features derive` moves a flag from `-` to `+` |
| Registry cache | sources unpack at `$CARGO_HOME/registry/src/<hash>/<name>-<ver>/` with CHANGELOG.md and license; **this machine's CARGO_HOME is `C:\DEV\Rust\.cargo` (non-default)** — check the variable before quoting the path |
| bench correctness gate | `cargo bench` builds release-grade; the sibling integration test (`tests/equiv.rs`: slow == fast) runs before any timing claim |

### PyO3 (probe-verified, ch20)

| Claim | Verified behaviour on rustc 1.98.1 / pyo3 0.29.2 / Python 3.13.5 (spike20 cdylib, pytest 8.4.0) |
|---|---|
| Toolchain setup | **no pyo3-config.txt, no import libs needed** — pyo3 0.29 links via raw-dylib on Windows; discovery runs the `python` on PATH and queries sysconfig (headers found at this install's flat `C:/DEV/Python/include`); first build ~20 s (pyo3+syn/quote+pyo3-ffi), edit loop ~1 s; `cp target/debug/spike20.dll pytest/spike20.pyd` is the whole deployment |
| Boundary scalars | i64/f64/bool/String cross exactly; Python's unbounded int beyond i64 raises OverflowError at the crossing (the shim refuses, body never runs) |
| Arg surface | `Vec<T>`, `HashMap<K,V>`, `Option<T>` (None lossless both ways), tuples all extract via FromPyObject; wrong shape = TypeError from the shim BEFORE the body runs |
| Errors | `PyValueError::new_err` → ValueError; `From<E> for PyErr` decides the payload — `new_err(e.0)` bypasses Display (probe-caught: message lost its prefix; use `e.to_string()`); `std::error::Error: Debug + Display` is E0277 without derive(Debug); `create_exception!(module, DeskError, PyValueError)` + `m.add("DeskError", ...)` gives a catchable class |
| #[setter] errors | pass through VERBATIM: PyValueError surfaces as ValueError, NOT AttributeError (builtin read-only attrs raise AttributeError; a setter chooses its own exception) |
| #[pyclass] | `#[new]` registers the constructor (absent = TypeError on call); `#[pyo3(get, set)]` is per-field opt-in; `__repr__` written in Rust speaks Rust Debug conventions — `{:?}` on String yields DOUBLE quotes |
| Bound/Py | `PyDict::new(py)` born under the `Python<'py>` token; `.into()` detaches to `Py<T>` which survives the token and crosses threads; work in Bound, return Py |
| Calling Python | four verbs import/getattr/call/extract, each a PyResult, chained with `?`; args as Rust tuples (`(x,)`); `Python::eval` takes `&CStr` (E0308 on `&str`) — pass `c"..."`; probed: `CStr::to_bytes` STRIPS the NUL, `to_bytes_with_nul` keeps it |
| Token/GIL | `py: Python<'_>` token params are ELIDED from the Python signature (`attach_probe(py)` is 0-arg in Python; passing an arg = TypeError); `Python::attach`/`detach` are 0.29's spelling of the with_gil/allow_threads pair; GIL sectioning = attach / release for hot Rust / re-attach |
| `final` is reserved | `let final = ...` compiles NOWHERE — E0007-style "expected identifier, found reserved keyword" (with the r#final help), and inside `assert_eq!(final, ..)` it parses as a macro fragment: "no rules expected reserved keyword"; rename to `total` |
| PhantomData shorthand | struct-expr shorthand `_t: PhantomData` is E0425 — `use std::marker::PhantomData;` is NOT in the prelude; spell `std::marker::PhantomData` in the constructor |

### PostgreSQL / sqlx (probe-verified, ch19)

| Claim | Verified behaviour on rustc 1.98.1 / sqlx 0.8.6 (runtime-tokio+postgres, no macros) against an in-notebook v3 wire-protocol server |
|---|---|
| Connect path | `PgConnectOptions::new().host(..).port(..).username(..).database(..)` + `sqlx::postgres::PgPoolOptions::new().max_connections(..).acquire_timeout(..).connect_with(opts)`; bare `sqlx::PgPoolOptions` is E0433 — the type lives under `sqlx::postgres` |
| Wire framing | every message is `[tag][i32 len][body]` where len counts its own 4 bytes but NOT the tag byte (total = 1 + len); **StartupMessage is the exception** — len covers the protocol i32 (196608) + kv payload |
| Handshake | SSLRequest (code 80877103) answered with a bare `N`; sqlx reads ParameterStatus pairs (server_version, client_encoding, DateStyle, integer_datetimes, TimeZone, standard_conforming_strings) and waits for ReadyForQuery, whose status byte flips I→T inside a transaction |
| Extended protocol | sqlx's sequence: Sync, Parse (name+sql), Describe (→ ParameterDescription `t` + RowDescription/NoData), Bind (portal, **statement NAME not SQL**, param formats/values/result formats), Execute, Close (`3` CloseComplete), Terminate `X` |
| Decode rules | int4 = 4-byte BE (fmt 1), text = fmt 0, NULL = i32 −1 with no bytes; `row.get`/`try_get` need `use sqlx::Row;` in scope; un-annotated decode = E0282/E0283 |
| NULL | bind `Option::<String>::None` stores NULL (turbofish needed — bare `None` has no element type); decoding NULL into bare String is a runtime `ColumnDecode` (UnexpectedNullError source), not `""` and not a panic |
| FromRow | hand-written `impl<'r> sqlx::FromRow<'r, PgRow> for T` with per-field `try_get` = exactly what the derive generates; tuples are FromRow destinations too; `query_as::<_, T>` is generic over the binding's destination type |
| Transactions | `pool.begin()` → `Transaction` (ReadyForQuery flips to T); queries with `&mut *tx`; `commit()`/`rollback()` take self by value (`let tx`, no `mut`, for pure rollback); **dropping the Transaction issues the rollback for you** — lands within ~250ms; desk snapshot-restore proves it server-side via count receipts |
| Pool saturation | 1-conn pool with held checkout + 200ms acquire_timeout → `Err(PoolTimedOut)` at ~215ms wall; after release the next acquire succeeds instantly; `size()` = open connections, not in-use |
| Errors | `sqlx::Error::{Database, RowNotFound, ColumnDecode, PoolTimedOut, Protocol}`; `e.code()` is `Cow<str>` (SQLSTATE: 42P01 undefined table, 42601 syntax, 25P01 no tx, 25001 already in tx) → `.unwrap_or_default().into_owned()`; no `DatabaseError` trait import needed for `.code()` on this toolchain |
| Migrations | ordered ledger + loop, skip applied, each migration inside `begin`/`commit` — second pass is a no-op (idempotent); `sqlx::migrate!` embeds a real directory but needs the macros feature (off here) |
| Async-shape trap | closures returning `async move` fail with "lifetime may not live long enough" (E0521-like) — declare `async fn` items instead (same lesson as 18e §9.2) |

### Concurrency and async (probe-verified, ch16)

| Claim | Verified behaviour on rustc 1.98.1 / tokio 1.53.1 |
|---|---|
| `thread::spawn` + `Rc` | E0277 `Rc<i32>` cannot be sent between threads safely; note names `appears within type Rc<i32>`, help suggests `std::sync::Arc<T>` |
| `thread::scope` borrows | borrows last to the **scope's end** (implicit join) — reading the vec inside the scope while chunk-borrows are live is E0502 even after manual `join()`s of every handle; the read belongs after the scope |
| Mutex poisoning trap | a `lock()` result **bound with `let`** (`let p = m.lock()`) keeps the guard alive inside the `Err(PoisonError)`, and the next `lock()` **deadlocks** (probed live, 8s+ hang); inspect inline or `unwrap_or_else(\|p\| p.into_inner())` |
| 10,000 sleepers × 10ms | threads ~260–290ms vs tokio tasks ~25–35ms (~8–11× on the ch16 box, 32 cores); at 500 threads Windows shows **no measurable gap** — use N ≥ 10k for the demo |
| future sizes | `async fn add(a: i32, b: i32)` = **12**; `String` + `usize` + tag across an await = **64**; `[u8; 4096]` across an await = **4098** — compile-time-stable, assert-safe |
| manual runtime in cells | `Runtime::new().unwrap().block_on(async { .. })` works inside the validator's `fn main()` wrapper and inside evcxr cells; **never nest** — creating a runtime inside async succeeds on 1.53 but *dropping* it panics: `Cannot drop a runtime in a context where blocking is not allowed` |
| `select!` arm types | all arms' body expressions must unify to one type (`&'static str` literals fine) |
| `timeout` | `Ok(output)` / `Err(Elapsed)`; on a miss the future is dropped — cancellation as drop, taught as policy |
| sync-channel(0) | `send` blocks until `recv` takes the value — a probe that `join()`ed before `recv()` deadlocked (probed live); recv-first is the pattern |
| `JoinHandle` | `.await` → `Result<T, JoinError>`; `is_finished()` exists on `thread::JoinHandle`; `abort()` kills at the next await point |

## 18. Progress Tracker

Update this table whenever a chapter is finished. It is the fastest way to answer
"where was I?" after a gap.

| # | Slug | `_e` | `_p` | Validated | Notes |
|---|---|---|---|---|---|
| 01 | `syntax-printing-and-types` | ✅ | ✅ | ✅ | 50 cells / 30 code in `_e`; 73 cells / 34 code, 19 exercises in `_p`. §7 restructured into operators / casting / overflow / `Option`+`Result`. `_e` compiles to 810 lines with zero warnings and exit 0. |
| 02 | `logic-blocks-and-control-flow` | ✅ | ✅ | ✅ | 62 cells / 30 code in `_e`; 54 cells / 25 code in `_p`. |
| 03 | `functions` | ✅ | ✅ | ✅ | 49 cells / 15 code in `_e`; 35 cells / 16 code in `_p`. Repaired: `03e` §15.1 renamed `add` to `sum` (the duplicate-definition defect is gone), four uncaptioned code cells got their `// N.M` captions, §0 retitled to the canonical wording, and the notebook metadata now carries the evcxr kernelspec (it had `language_info: python`). `_e` compiles with zero warnings and exits 0. `03p` rebuilt to the `01p`/`04p` standard: 40 cells / 18 code, §0 conventions cell, doc-comment stubs, `// --- check (do not modify) ---` blocks, 19 `**See**` references all resolving. Compiles with only `unused variable` warnings plus two deliberate `#[allow(dead_code)]` diverging stubs; solvability harness 18/18 `Exercise N.M passed`, exit 0. |
| 04 | `strings` | ✅ | ✅ | ✅ | 127 cells / 58 code in `_e`; 126 cells / 61 code in `_p`, one exercise per `_e` demonstration cell plus three mixed review problems in §16. §13 covers `E0277`/`E0308`/`E0502`; §15 ties string allocation cost to HFT latency. `_e` compiles to 512 lines, zero warnings, exit 0; `_p` compiles with only `unused variable` and aborts at the first `todo!()`, which is the expected shape. Solvability harness: 61/61 `Exercise N.M passed`, exit 0. §0 of `_e` retitled to the canonical `## 0. Notebook Conventions: How These Cells Run` (Contents anchor updated with it). |
| 05 | `ownership-and-borrowing` | ✅ | ✅ | ✅ | 77 cells / 47 code in `_e`; 101 cells / 48 code in `_p` — one exercise per `_e` demonstration cell (§0–§14), a §15 pointer cell, and two mixed review problems (§16). §13 documents all five ownership diagnostics (`E0382`, `E0502`, `E0507`, `E0499`, `E0106`) with probe-verified wording. §12 covers the three real-loop patterns (move-into-iteration, mutate-while-iterating, remove-from-map). `_e` compiles with zero warnings and exits 0. Solvability harness: 48/48 `Exercise N.M passed`, exit 0. `05p` §0 documents the two impl-stub warning shapes (`field ... is never read` on stubbed accessors). §5.1 of `03p` and §14.1 here both exercise `-> !` stubs — see the §5.1 note in `03p` about const-fn/never-type stubs. |
| 06 | `native-collections` | ✅ | ✅ | ✅ | 87 cells / 41 code in `_e`; 90 cells / 43 code in `_p` — one exercise per `_e` demonstration cell (§0–§14), plus two mixed review problems (§16). §12 catalogues the container panics with their payload shapes (index/missing-key format a `String` at panic time; `Option::unwrap` panics with a static `&str`) and the E-codes (E0277, E0369, E0502). §13's timing cell shows `insert(0, ..)` ~150× slower than `push_back`/`push_front` at N=30 000. Each `std::collections` type is imported exactly once at first use (§5) — the one-namespace rule extends to `use` lines, documented in both §0s. `_e` compiles with zero warnings and exits 0; `_p` compiles with only `unused variable` warnings and aborts at the first `todo!()`. Solvability harness `_sol6_gen.py`: 43/43 `Exercise N.M passed`, exit 0. Harness findings taught back into the exercises: `rotate_left` asserts `n <= len` (8.2 guards the empty deque), window eviction is pop-before-push (8.1), `BTreeMap::range` yields pairs and has no `.values()` (7.1 hint corrected). |
| 07 | `io-and-files` | ✅ | ✅ | ✅ | 70 cells / 33 code in `_e`; 74 cells / 35 code in `_p` — one exercise per `_e` demonstration cell (§0–§13), plus two mixed review problems (§16). Sandbox pattern: `_e` uses `scratch07/` (created per writing cell, deleted at §12.2), `_p` uses `scratch07p/` with self-sufficient check blocks; `fs::exists` confirmed stable on rustc 1.98. Probe-verified on Windows: `PathBuf::push`/`join` normalise to `\`, `canonicalize` carries the `\\?\` prefix, `File::open(dir)` is `PermissionDenied (os error 5)`, `expect`'s panic payload formats the error with `Debug` (`Os { code: 2, ... }`), oversized `write!` bypasses `BufWriter`, `env::set_var`/`remove_var` are `unsafe` in edition 2024. §12 timing: buffered vs unbuffered ≈ 30–50× at 3000 lines. Validator fact learned: `use` lines must not carry trailing `//` comments (the item collector consumes the next statement; hoisting then emits a top-level `let`), and `use std::fs::{self, ..}` duplicates the earlier `use std::fs;` under E0252. `_e` compiles with zero warnings and exits 0; `_p` compiles with only `unused variable` + (documented in its §0) `unused import` warnings and aborts at the first `todo!()`. Solvability harness `_sol7_gen.py`: 35/35 `Exercise N.M passed`, exit 0. Notebook-wide imports live in stub cells above the check marker (harness-safe); argv/env exercises pass vectors or use notebook-unique keys because the kernel host owns the real ones. |
| 08 | `assertions-and-type-checks` | ✅ | ✅ | ✅ | 40 cells / 18 code in `_e`, sections 0–13; 44 cells / 20 code in `_p` — one exercise per `_e` demonstration cell (§0–§9), plus two mixed review problems (§13, including a `CheckedPrice` newtype that makes downstream asserts unnecessary). Probe-verified on rustc 1.98: `assert_eq!` payloads are eager `String`s but a formatted `assert!` payload downcasts to `&str` (lazy `Arguments` formatting — extractor must try both); const-context assert failure is E0080; `assert_eq!` on a bare struct fires E0369 + E0277×2 (needs `PartialEq` + `Debug`); niche table verified (`Option<&T>` = `&T` = 8, `Option<i32>` = 8 vs `i32` = 4, `String`/`Vec` = 24, `&str` = 16); `size_of` is prelude + const-usable, `type_name` is neither prelude nor `TypeId::of_value`-stable (E0599). Const-eval sharp edge taught in §7.1/7.1: `windows(..).all(..)` is NOT const (E0015) — const twins need `while` loops. **`dyn Any` lesson (harness-caught, probed, taught into both twins): `Box<dyn Any>` itself implements `Any`, so `identify(&boxed)` coerces `&Box<dyn Any>` → `&dyn Any` of the *box* and downcasts fail — pass `&*boxed`.** `_e` compiles with zero warnings and exits 0; `_p` compiles with only `unused variable` + (documented in its §0) `unused import` + stub `never read` warnings and aborts at the first `todo!()`. Solvability harness `_sol8_gen.py`: 20/20 `Exercise N.M passed`, exit 0. A const assert gated on an unsolved stub would be a compile error, not a runtime failure — `_p` exercises keep const asserts pre-solved or fact-returning. |
| 09 | `logging` | ✅ | ✅ | ✅ | 31 cells / 13 code in `_e`, sections 0–12; 35 cells / 16 code in `_p` — one exercise per `_e` demonstration cell (§0, §2.1–§7.1) plus two mixed review problems (§12). First crate chapter; verified versions on this toolchain (via scratch cargo project, dlltool needs `C:/DEV/msys64/mingw64/bin` on PATH): `tracing` 0.1.44, `tracing-subscriber` 0.3.23 (`env-filter` feature), `tracing-appender` 0.2.5, `eyre` 0.6.14. Validator: `_crate_validate.py` wraps `_validate.py` (strips `:`-magics, compiles with `--extern` rlibs listed in `_externs.txt`, absolute paths). Probed truths taught in: the global subscriber is set **once per process and never replaced** (refused `try_init` keeps the old stack receiving events; `init` on an occupied slot panics), so kernel cells demonstrate stacks via **scoped dispatch** (`tracing::dispatcher::with_default`, verified) and 2.1's `try_init` guard handles re-runs; `Backtrace::capture()`'s gate is **cached at the process's first capture call** (set `RUST_BACKTRACE` before it; `force_capture` yields ~40 frames incl. `main`); `wrap_err` is inherent on `eyre::Report` (the `WrapErr` trait is for generic `Result`s); `EnvFilter::max_level_hint()` is the cell-checkable filter API; `LevelFilter` lives in `tracing_core`. ANSI confirmed on by default. `_e` compiles with zero warnings and exits 0 via the bridge validator; `_p` compiles with only `unused variable` + (documented in its §0) `unused import` warnings and aborts at the first `todo!()`. Harness `_sol9_gen.py` (magic-stripping, closure-form MakeWriters): 15/15 `Exercise N.M passed`, exit 0. Harness/template lessons: the stub extractor only matches single-line signatures (12.2 uses a `type` alias), `with_writer` needs closure-form `\|\| Writer` (not a bare value), and `%report` records the chain's *outermost* message (12.1 asserts the wrap headline). |
| 10 | `structs-oop-and-traits` | ✅ | ✅ | ✅ | 53 cells / 25 code in `_e`; 54 cells / 25 code in `_p` — one exercise per `_e` demonstration cell (§0–§13), plus two mixed review problems (§15). `01e`'s promises honoured: operators-as-traits (`Add`/`Mul<u32>` on a `Qty` newtype, `String + &str` operand consumption, E0117 orphan rule + newtype workaround, `f.pad` width-aware `Display`), `#[derive(Debug)]`, Display-by-hand, newtype-from-`type`-alias. Probe-forced corrections: repr(Rust) **reorders** fields (flag/qty/done = 8 B in both written orders; the 12 B shape needs `repr(C)`), dyn/static dispatch ratio wanders ≈0.9–2× (prose teaches the structural costs: no inlining, heap round-trip), E0038 renamed "dyn compatible". Validator-caught E0034 lesson taught into both twins: a type implementing two traits with the same method name makes every receiver ambiguous (dyn twin got its own `flash` method). `_e` compiles 0 warn / 0 err / exit 0; `_p` 0 errors, warnings all in the two families documented in its §0 (unused variable from stubs + stub-induced dead-code `never read`/`never used`), aborts at the first `todo!()`. Harness `_sol10_gen.py`: **25/25 `Exercise N.M passed`, exit 0** (64 exercise fns; 3.1's caught panic prints a panic-hook line to stderr — harmless, exit still 0). Harness upgrade over `_sol9_gen.py`: block scanner tracks enclosing `impl`/`trait` regions (multi-line where-clauses OK), keys stubs `Self::fn` / `Self::Trait::fn` with generic args stripped (`impl Mul<u32> for Qty` → `Qty::Mul::mul`), and **skips default methods inside `trait` blocks** (given code, not stubs). Tracker note: anchor check must pair by text not position (front-matter `##` headings precede numbered ones); practice-notebook Contents intentionally lists only exercise-bearing sections (§14 Mental Model skipped) — 09p set the convention. |
| 11 | `errors-in-depth` | ✅ | ✅ | ✅ | 41 cells / 17 code in `_e`, sections 0–12. Manual `impl Display` + `impl Error` first (empty `{}` body when no source; `Error` demands `Debug` via E0277 naming the trait decl), then `?`/`From` bridges stamping context at boundaries, `source()` chain walks (first hop needs the fully qualified call), `Box<dyn Error>` with std's blanket `From` and downcasts, `map_err` per-callsite stamps, typed `cause()` accessor, `main() -> Result` contract (stderr `Error: <Debug>`, exit 1), panic payloads (`&str` literal / `String` formatted / `String` for custom errors via `panic!("{}", ..)`) + `catch_unwind` + `set_hook`/`take_hook`, stable-only `Backtrace` field pattern with provider-API E0658 note, size table (`Option<Result<u32, ()>>` = 8 via niche through two layers; `Option<Result<&u32, ()>>` = 16), E-codes (E0277×2, E0119 vs std's reflexive `From<T> for T`, E0308, E0252, E0658). Slug traps caught by the anchor check: a standalone `` `?` `` heading and `` `main() -> Result` `` both create ambiguous space-runs when punctuation is stripped — retitled to `` From Conversions `` / `` Returning Result from main ``. `_e` compiles 0 warn / 0 err / exit 0. `_p`: 44 cells / 20 code, 20 exercises — one per `_e` demonstration cell (§0.1–§9.1) plus four mixed review problems in §11. Given complete impls carry the empty-`{}`-body lesson as reading (comment-form `// TODO: add impl Error` exercises are unsolvable-as-shipped: the check blocks cannot compile while the trait impl is missing — E0277 — so those impls are given and the exercise asks *why* the body is empty). Exercise-design lessons the harness forced: every check-block assertion must be reachable from the stub's own parameters (`handshake(2)` hard-coded made the handshake-stamp assertion an `unwrap_err` on `Ok` — attempts became a parameter), and `try_fold` closures need explicit commas in multi-arm `match`es (harness bodies). `assert_eq!` on `Result` values needs the error type `PartialEq` — matches!/expect everywhere else. `_p` compiles 0 errors with warnings only in the three documented families and aborts at the first `todo!()`. Harness `_sol11_gen.py` (kept as template): 20/20 `Exercise N.M passed`, exit 0, **36 exercise fns**; upgrade over `_sol10_gen.py`: replaces only bodies containing `todo!()` (given impls and given free fns stay byte-exact), keys `impl std::error::Error` blocks under the full path `Self::std::error::Error::source`. |
| 12 | `iterators-in-depth` | ✅ | ✅ | ✅ | 46 cells / 20 code in `_e`, sections 0–14. Running example: `Ticks` iterator yielding `Px(u32)` (exact `size_hint` → `FusedIterator` + `impl ExactSizeIterator` in one impl; a second `impl Iterator for Ticks` would be kernel-legal but validator-fatal E0119, so trait methods demo in a usage-only cell). Custom adaptor `Pairs<I>` needs its `I: Iterator` bound **on the struct** (E0220 on `I::Item` otherwise). Probe-verified on rustc 1.98.1: `next()` is fused; laziness proven with an `Rc<Cell<usize>>` closure-call counter (0 after `.map()`, 5 after `collect`, 0 dropped unconsumed); `any()` ≈700ns vs `sum()` ≈239ms over 1e8 (short-circuiting); collect zoo incl. `Vec::from_iter` ≡ `collect`; `Result<Vec,_>`/`Option<Vec,_>` fail-fast collect; `chunks(3)` on 7 = 3 (ragged tail) vs `windows(3)` = 5 (n−k+1); `try_fold`+`checked_add` traps `[u64::MAX, 1]`; un-annotated `collect` is **E0283** (not E0282); twin-branch RPIT (Map vs Filter) is E0308 suggesting `Box<dyn Iterator>`; edition split: RPIT borrowing an arg needs no `+ '_` under edition 2024 (E0700 under 2021); E0689 `1..4i32.map(f)` parses as `1..(4i32.map(f))` — parenthesise the range; E0507 `map(|o| o.field)` over `iter()`, E0382 after `into_iter`; **`advance_by` is nightly on 1.98** (E0658, `iter_advance_by`) — teach `nth`/`skip` instead; adapter sizes: `Range<i32>` 8 / `slice::Iter` 16 / `Map`,`Filter`,`Take` 16 (captureless closures are ZSTs) / `Zip` 40 (padded); `chain` ≈10.1ms vs hand loop ≈7.8ms on 4M u64. `_e` compiles 0 warn / 0 err / exit 0. ch12 is std-only — no forward-ref debt created. `_p`: 50 cells / 23 code, 23 exercises — one per `_e` demonstration cell (§0.1–§11.1) plus three mixed review problems in §14. Exercise-design lessons the harness forced: **RPIT stubs cannot ship as `todo!()`** — the hidden type infers `!` and `!` is not an `Iterator` (E0277), so the `-> impl Iterator` exercises (§8.1, §11.1) ship **placeholder chains** that compile but yield wrong answers the check's asserts reject (documented in `_p` §0); a lifetime that cannot be elided (`&[&'a str] -> Vec<(usize, &'a str)>`) must be given in the signature; and **collect into a map overwrites duplicate keys** — last pair wins, never adds — so the counting idiom is `fold` + `entry().or_insert(0)` (taught in 5.1, where the original exercise claimed the false thing). `_p` compiles 0 errors with warnings only in the two documented families (unused variable + stub-induced `never read`/`never constructed`) and aborts at the first `todo!()`. Harness `_sol12_gen.py` (kept as template): 23/23 `Exercise N.M passed`, exit 0, **36 exercise fns**; upgrade over `_sol11_gen.py`: stub bodies carrying the `// placeholder` marker are replaced too. |
| 13 | `modules-crates-and-testing` | ✅ | ✅ | ✅ | 26 cells / 10 code in `_e`, sections 0–11. Running family: `pantry`{`Flour` (private `brand` + accessor), `restock`, private `secret`, `mod shelf`{`top`, `pub(super) for_parent`, `reach_down`→`super::secret`}, `call_secret`, `use_shelf`}, `kitchen::bake`; single declaration in §1.1 reused by §2.1 (the front-matter "stays live" promise, kept — a second `mod pantry` would be validator-fatal E0428 since the harness flattens all cells). Probe-verified on rustc 1.98.1: **`#[test]` fns are STRIPPED under plain rustc** (E0425 `cannot find function`, not merely inert); `#[cfg(test)]` items likewise; `cfg!(test)` folds false; the `cfg!` false branch must still compile, so a stripped item can only be *named* in a comment there; `should_panic` in-process = `catch_unwind` + downcast — **a literal `panic!("..")` payload downcasts as `&str`, not `String`** (caught live during gating); privacy flows DOWN (child reads parent's private fn); `pub(super)` = ceiling exactly one level, decided by the *caller's ancestry*, not path spelling (E0603, no "almost pub" hint); **bare-path first segments do not climb**: `pantry::restock()` from sibling `kitchen` is E0433 (ed 2024, caught live during gating — corrected the planned "root items are in every descendant's scope" claim), `super::` is the sibling route; `pub(crate)` ≡ `pub` from the root; enum variants importable (`use g::Gear::{self, Reverse};`); `use spices::{self as sp, pepper};` legal; rename-alone `use` = unused warning; failing `cargo test` output captured verbatim (one failure doesn't stop the run; `test result:` = exit code); doc tests: `test src\lib.rs - bump (line 3) ... ok`, temp `rustdoctest…doctest_bundle_*.rs` in failing panic paths, separate output section after unit tests. E-codes table: E0428 (type-namespace wording), E0603, E0616, E0432, E0425 (stripped). §9 mental model, §10 cheat sheet, §11 review 12 Q + What Comes Next (14 Smart Pointers, 15 Data Types, 16 Concurrency). **Tooling change this chapter**: `_validate.py` `ITEM_STARTS` gained `"mod "`, required by every cell-shaped `mod` in ch13 — without it bare `mod x {}` nested in `main()` and `use` of its items went E0432. Kernel-honesty note (§1): cell `mod` is nested inside the wrapper, so `super::` from a cell's module reaches the wrapper/root. Gate: rustc 0 err / 0 warn / run exit 0. **`_p`**: 30 cells / 13 code, 13 exercises — one per `_e` demonstration cell (§0.1–§8.1, skipping §5 which has none) plus three mixed review problems in §11 (review renumbered to §11 to mirror `_e`; 09p's “skip Mental Model” convention superseded — the Contents lists exercise-bearing sections and its links now point at real `## N.` headers, a fix 12p predates: 12p's §1–§11 links are dead in-body). Running families: `depot`/`larder`/`silo`+`mill`/`vault`/`aviary`+`gearbox`/`probe`/`depot_gate`/`suite`/`crate1..3`/`hold`/`raw`+`facade`/`rehearsal`. Exercise-design lessons the harness and validator forced: **a check block must compile in stub state** — 11.2's check originally referenced re-exports that were themselves the exercise (11p's unsolvable-check lesson, re-learned: the `pub use` lines ship as given); **no body may name a `#[cfg(test)]`-gated item — not even a `match cfg!(test)` true arm** (every branch must compile, the E0425 dodge is comment-only, so 6.2 reads the gate instead of calling it); **given-but-dead items draw permanent dead-code warnings** — every demo module needs a consuming call even in stub state (`silo::reserve`, `larder::accept`, `hold::open_stash` consuming `for_parent`); `bad` reports its failing assert's *pair* instead of asserting (a body that asserts `4 == 5` would panic its own check). `_p` compiles 0 errors, 45 warnings all in the three documented families (unused variable / unused import / stub-induced dead code), aborts at the first `todo!()` (exit 101). Harness `_sol13_gen.py` (kept as template): **13/13 `Exercise N.M passed`, exit 0, 41 exercise fns**; upgrades over `_sol12_gen.py`: scanner tracks `mod` blocks at any depth (stubs keyed by full chain — `larder::bin::peek` vs `hold::inner::peek` disambiguation), HEADER_RE so `pub fn`/`pub struct` lines do not open pseudo-blocks, reconstructed headers preserve `pub`/`pub(crate)`/`pub(super)` + indentation (12p could drop `pub`; 13p's checks call stubs by module path, where visibility is load-bearing). |
| 14 | `smart-pointers-and-interior-mutability` | ✅ | ✅ | ✅ | 31 cells / 11 code in `_e`, sections 0–14. Running example: the `Gauge` type (Drop-printing) reused from §3 as §4's observable payload — the `[drop] rc-payload` line proves the count hit zero. Probe-verified on rustc 1.98.1: sizes (`Box<i32>` 8 / `Box<dyn Display>` 16 fat / `Rc`, `Weak`, `Arc` 8 / `Cell<i32>` 4 = zero overhead / `RefCell<i32>` 16 = the flag rides along / `OnceCell<i32>` 8); `Rc` count/`ptr_eq`/`get_mut`/`try_unwrap` refusals; **RefCell double-borrow panic payload is a `String`** (`downcast_ref::<&str>()` → None — chapter 11's literal-`&str` habit misses it; message `RefCell already borrowed`); `OnceCell::set` on filled returns `Err(v)` (no panic); **Deref-without-Target is E0046, not E0106**; `impl Drop for i32` is the E0117+E0120 pair; E0277 `Arc<RefCell<i32>>` into `thread::spawn` carries the ch16 hint (`use std::sync::RwLock instead`); cycle leak proven by *absent* `[drop]` lines vs broken chain dropping both nodes; `Weak::upgrade` → None after last strong drops. Two honest cells: §3.1 proves LIFO with an explicit inner scope (kernel keeps end-of-cell bindings alive — §0 caveat), §10.1's no-leak proof prints both `[drop]` lines inside the cell. E-codes: E0072, E0046, E0117+E0120, E0119, E0597, E0107, E0277. §12 mental model, §13 cheat sheet, §14 review 12 Q + What Comes Next (15 Data Types, 16 Concurrency, 17 Capstone). **`use`-persistence lesson**: repeated `use` lines across cells are E0252 under the validator (one namespace) — first cell that needs a name does the import; column-0 `use` lines still must not carry trailing `//` comments (the §16 gotcha bit again). Gate: rustc 0 err / 0 warn / run exit 0. **`_p`**: 34 cells / 15 code, 14 exercises — one per `_e` demonstration cell (§1.1–§11.1; §10.2 is a *reading* exercise whose check is the silence of never-printing `[drop]` lines) plus three mixed review problems in §14. Running scaffolding: `Course` (the E0072-fixed enum, 16 B probed), `Safe`+`Inner` (two-trait pointer), `Chip`/`Fpga`/`Asic`/`Rack` (dyn + Drop), `Fork`, `Thermometer` (loan methods are free fns over the `RefCell` — methods on the wrapped type cannot be called through the cell), `Cargo` under `Rc<RefCell>`, `Gosling` (Weak back-edge; `[drop]` lines print in-cell), `Critter` (the exhibited leak), `Registry`, `Brick`+`OnceCell` cache, `Pebble` with an `Rc<Cell<Vec>>` log that outlives every drop. Exercise-design lessons: **a trait-impl stub missing `Target` cannot compile** (E0046 in stub state — the `type Target` line ships as given); **a `&mut self` method called through `borrow_mut()`** is the honest registry shape (`&self` + push cannot compile); **the fourth warning family exists** — `variable does not need to be mutable` on a `mut`-param stub is NOT self-cleaning, so stubs rebind into a `mut` local instead; check-block count expectations must state *when* the probe's clone is live (or `strong_count` reads the wrong moment). `_p` compiles 0 errors, 51 warnings all in the documented families, aborts at the first `todo!()` (exit 101). Harness `_sol14_gen.py` (kept as template): **14/14 `Exercise N.M passed`, exit 0, 35 exercise fns**; upgrade over `_sol13_gen.py`: impl methods keyed `Self::Trait::fn` (`Safe::Deref::deref`, `Pebble::Drop::drop`) via the same header scanner that tracks mods. |
| 15 | `data-types-and-their-libraries` | ✅ | ✅ | ✅ | 43 cells / 16 code in `_e`, sections 0–16. First full **crate-consumption** chapter, following the spec row as written — registry reachable, all five crates probed on this toolchain: **chrono 0.4.45**, **time 0.3.55 (features macros+formatting+parsing)**, **uuid 1.x (feature v4)**, **rust_decimal 1.43.0**, **polars 0.55.2 (features lazy+csv+temporal+strings)**. Probed truths taught in: `NaiveDate::from_ymd_opt` returns `None` for Feb 29/30 (calendar validated at construction); **chrono's inherent `weekday` is private in 0.4.45 — the method resolves only through `Datelike`** (E0599 without the trait); `TimeDelta` is signed, `to_std()` refuses negatives; 22:00+30h rolls to Oct 1 04:00; **`time::Time::from_hms` returns `Result`** (only `*_unchecked` are infallible); chrono `%`-templates are runtime data vs `time`'s compile-time `format_description!`; **parsing offset-less text into `OffsetDateTime` fails (the type demands an offset); `PrimitiveDateTime` takes it and `assume_utc` equals the RFC3339-parsed instant**; FixedOffset re-lens keeps instant equality (`-05:00` via `%:z`); Decimal `0.1+0.2 == 0.3` exactly; **`from(3)` carries scale 0 so 2dp×int stays 2dp (`370.35`); two 2-dp factors give 4 dp (`1.50×2.50 = 3.7500`)**; banker's default `2.5→2`/`3.5→4` vs named `MidpointAwayFromZero` `2.675→2.68`; `1/3` = 28 digits; uuid v4 nibbles at hex[12]='4' and hex[16]∈{8,9,a,b}, `Uuid` = 16-byte Copy value; **polars 0.55: `DataFrame::from_rows` names positional columns `column_1..` — `from_rows_and_schema` + `Schema::from_iter([("sym".into(), DataType::String), …])` declares the schema**; **`Row`/`AnyValue` are NOT in the prelude** (`polars::frame::row::Row`, `AnyValue::String` borrows → `Row<'a>` tied to the input slice); `sort` takes column-name arrays; `group_by`+`agg`+`alias`; in-memory CSV roundtrip via `Vec<u8>`+`Cursor` with schema-equality assert. §8→§9 bridge: one `blotter_rows` adapter (structs stay the boundary). One-glob rule held: §9 owns `use polars::prelude::*`, later cells persist. E-codes: E0432 (features), E0599 (Datelike/Timelike), E0433 (unlinked crate), E0252 (re-import), E0308 (float `assert_eq!` forbidden), winapi link row. **Gate plumbing this chapter**: `_externs.txt` gained the five rlibs plus a new `native=` convention (emits `-L native=`) for **winapi's import libs** (`winapi-x86_64-pc-windows-gnu-0.4.0/lib`) — polars does not link without them; `_crate_validate.py` reads them; two `libtime-*.rlib` exist, the feature-enabled one is the larger, later-built `libtime-d04db6ea6ee6f6ae.rlib`; polars-linked `_check.exe` is a **1.8 GB debug binary** and links slowly. §12 = the choosing-representations table (the chapter's exam), §16 = What Comes Next (16 Concurrency, 17 HTTP/WebSockets, 18 Redis). Gate: rustc 0 err / 0 warn / run exit 0. **`_p`**: 37 cells / 18 code (crate cell + 17 exercises), 30 stub fns — one per `_e` demonstration cell (§1.1–§13.1) plus three mixed review problems in §16 (16.1 the desk's day §8–§11, 16.2 the settlement clock §1/2/4/5, 16.3 the audited ledger §6/7/12). Running scaffolding: `Trade`+`blotter_rows` (the borrowing `Row<'a>` adapter, given), `Fill` ledger with `notional()`, `Audit` record. Check discipline: fixed timestamps only (checks stay true next year), uuid ids verified by structure not value, floats only under tolerance — and **f64 Display prints `418.0` as `418`**, so the 8.1 ticket test uses `228.6x100`; §13.1's one given `use chrono::Timelike;` sits ABOVE the check marker so it survives check-only extraction. Stub state: 0 errors, 52 warnings all in the three documented families (49 unused variable, 2 dead code, 1 unread fields), aborts at the first `todo!()` (exit 101). Harness `_sol15_gen.py`: `_sol14_gen.py` machinery (mod-chain/impl keys present but unused — all 15p stubs are free fns) + 30 IMPLS + **externs-bridge compile built in** (`_externs.txt` externs + `native=` dirs; the crate cell needs no stripping — it has no check marker so it drops out), runs `_sol15.exe`: 17/17 `Exercise N.M passed`, exit 0; the known python `FileNotFoundError` AV applies at the in-process launch — run `./_sol15.exe` from bash after gen. Exercise-design lessons the harness forced: `est()` returns the `.unwrap()`ed `FixedOffset` (doc says Option-in, offset-out); a sol body that needs the given `let id = ...` line must re-include it (replacement starts below the signature); `gap_seconds` = close − open (`num_seconds` signs by direction, `b − a` came out negative). |
| 16 | `concurrency-and-async` | ✅ | ✅ | ✅ | 46 cells / 21 code in `_e`, sections 0–14. Second crate-consumption chapter and the applied track's pivot — **tokio 1.53.1 (features `full`)** built in the spike16 scratch project (`C:/Users/GSL/AppData/Local/Temp/spike16`, mingw PATH for dlltool, ~16s build; rlib wired into `_externs.txt` as `tokio=...libtokio-7ff00269cc0b5070.rlib`; no `native=` needed). Probed truths taught in: E0277 on `Rc` across `thread::spawn` with the `Arc` fix in the help; **`thread::scope` borrows end at the scope's implicit join — an in-scope read after manual joins is still E0502** (E0502 caught twice in probes; the canonical demo is chunks_mut fan-out, read after the scope); **the poisoning trap: `let p = m.lock()` keeps the guard alive inside `Err(PoisonError)` and the next `lock()` deadlocks — inspect inline**; 10,000 sleepers threads ~260–290ms vs tasks ~25–35ms (**at 500 Windows shows no gap — N must be ≥10k**); future = compile-time frame, sizes **12 / 64 / 4098** asserted (`add(i32,i32)` / `String`+`usize` across await / `[u8;4096]` across await); manual `Runtime::new().unwrap().block_on` in every async cell (validator wrapper + kernel shape); **runtime-in-runtime: creating succeeds on tokio 1.53 but dropping panics "Cannot drop a runtime..."**; `select!` arms must unify (`&'static str`); `timeout` miss = drop; `sync_channel(0)` recv-first or deadlock (probed hang). Sections: 0 conventions (runtime-per-cell), 1 threads, 2 Send/Sync + scope, 3 mpsc, 4 Mutex/RwLock/poison, 5 atomics (fetch_add previous-value, CAS Err = current), 6 the motivation cell (timing, no asserts), 7 async/await + frame sizes, 8 runtime/spawn/spawn_blocking/abort, 9 select!/timeout/oneshot/bounded mpsc, 10 blocking-vs-async rules table + block_in_place + tokio::sync::Mutex, 11 E-codes, 12 receipts, 13 cheat sheet, 14 review + What Comes Next (17 HTTP/WebSockets, 18 Redis, 19 PostgreSQL, 20 PyO3). Gate: rustc 0 err / 0 warn / run exit 0 (§4.2's poison panic is the only stderr line, by design). **`_p`**: 35 cells / 20 code (crate cell + 19 exercises = one per `_e` demo cell §1.1–§10.2, §6 reading-only) + 3 mixed review in §11 (11.1 std-channel→async-stage pipeline, 11.2 scope fan-out + runtime sum, 11.3 thread workers + shared guard); 38 stub fns (17 sync + 16 async + 5 given). Check discipline: sync stubs build their OWN runtime and are called from sync context (never inside another block_on — the drop-panic); async stubs awaited inside the check's block_on; joins before asserts; 10× sleep margins; no timing asserts; §11.3's check `10x(1+2+3+4)=100` (an earlier draft's 220 was wrong arithmetic, harness-caught). Stub state: 0 errors, 53 warnings all in the three documented families (48 unused variable, 2 unused import, 3 dead code), aborts at the first todo!() (exit 101). Harness `_sol16_gen.py`: `_sol15_gen.py` machinery + 38 IMPLS + **the async upgrade — FN_HEADER matches `async fn` and expand() preserves the `async` keyword on reconstruction** (the first run lost it and produced E0728/E0277 cascades); harness-caught IMPLS bugs fixed: `.iter().map(\|q\| spawn(async move { q*q }))` borrows into 'static futures (E0597/E0521) → `into_iter`/`for &q in` copies; send_into_the_void contract inverted (accepted=false → `.is_ok()`); all 19 `Exercise N.M passed`, exit 0 (AV quirk: run `./_sol16.exe` from bash). Practice Contents lists exercise-bearing sections and points at real `## N.` headers. |
| 17 | `http-and-websockets` | ✅ | ✅ | ✅ | 53 cells / 22 code in `_e`, sections 0–15. Third crate chapter — **reqwest 0.12.28 (blocking+json+charset+stream), serde 1.0.229 (derive), serde_json 1.0.151, tokio-tungstenite 0.24.0 (connect+handshake), futures-util 0.3.34 (sink+std)**, all built in the spike16 project alongside tokio; rlibs wired into `_externs.txt` (ch17 wave: tokio re-hashed to `cbda1df3e27b2637`, futures-util `73abe93854c61ccf`; two libreqwest rlibs exist, stream-enabled = larger `7623492997566db4`). **No external network anywhere**: §0.2 builds a ~40-line raw std-TcpListener HTTP server in-notebook (routes /ok /json /fills /junk /redir /slow /error /echo /q /tok /big), and §8.2 a local WS echo endpoint with its runtime parked in a `OnceLock` (the §8.1 dropped-runtime lesson taught honestly). Probed truths taught in: statuses are data until `error_for_status`; consuming body readers force status/headers-first (E0382 shown in §12.2); redirect policies (`none`/`limited`) with `resp.url()` as ground truth; builder timeout fired at ~500ms on a 400ms policy vs the patient client on the same 1.5s `/slow`; `bytes_stream` on a 64KiB `/big` route (65536 bytes in 4 chunks); serde derive + junk/missing-field/default receipts; WS 101 + Upgrade receipt, text/binary echo round-trips, **the two-sided close handshake with the server-side drain (ResetWithoutClosingHandshake taught)**, and **tungstenite 0.24's strict client-echoes-server subprotocol rule**. Sections: 0 conventions + local server, 1 blocking GET/status/headers, 2 POST, 3 serde, 4 query/headers/redirects, 5 timeouts, 6 streaming, 7 async client (join!/per-call timeout), 8–9 WebSockets, 10 blocking-vs-async table, 11 the reconnect loop (state outside the socket, backoff reset on success), 12 E-codes, 13 receipts, 14 cheat sheet, 15 review + What Comes Next (18 Redis, 19 PostgreSQL, 20 PyO3, 21 Capstone). Gate: rustc 0 err / 0 warn / run exit 0. **`_p`**: 37 cells / 21 code (crate cell + 20 exercises = one per `_e` demo cell; `_e`'s server cells are the given `desk()`/`ws_desk()` helpers, so §8's exercise is 8.3) + 4 mixed review in §12; 29 stub fns (sync + async). Check discipline: every check builds a fresh `desk()` (ephemeral port, no cross-cell server state); turbofish `json::<Vec<Fill>>()` documented in §0; given `ws_desk` merged into the 8.3 stub cell ABOVE the check marker so the harness sees it (the marker-less-cell drop bit here); `within_deadline` takes a full URL (a base+path stub forced the check to pass `/slow` as base). Stub state: 0 errors, 46 warnings all in the three documented families, aborts at the first todo!() (exit 101). Harness `_sol17_gen.py`: `_sol16_gen.py` machinery (async-fn aware) + 29 IMPLS + externs bridge; harness-caught bug: double-escaped `\"seq\"` in generated raw strings printed literal backslashes — IMPLS escapes verified against the check's expected literal. All 20 `Exercise N.M passed`, exit 0 (AV quirk: run `./_sol17.exe` from bash). |
| 18 | `redis` | ✅ | ✅ | ✅ | 46 cells / 18 code in `_e`, sections 0–13. Fourth crate chapter — **redis 0.27.6 (features tokio-comp+serde)** joined the spike16 wave (tokio re-hashed to `0f9a0fb4fe795827`, reqwest `67fb968fb74dcaf1`, serde `bac2f0a2d3a24772`, tokio-tungstenite `9ead2823e8be454c`; serde_json/futures-util kept hashes; all recorded in `_externs.txt`; 16e and 17e re-gated clean on the new wave). **No Redis on the machine and none installed** — the ch17 playbook repeated: §0.2 builds a ~300-line RESP2 server in-notebook on std `TcpListener` (strings/expiry/hashes/lists/sets/PUBLISH-SUBSCRIBE/MULTI-EXEC, lazy expiry sweep, handshake tolerance for CLIENT/SELECT/HELLO, pattern subs with trailing `*`), every cell tests against it. Probe-first paid off: the standalone probe18 project (server.exe + driver.exe) found the crate's wire truths before any notebook cell was written — `incr`→INCRBY, `pexpire`→PEXPIRE, `hset_multiple`→HMSET, `set_ex`→SETEX, `.ignore()` removes the tuple slot, pub/sub on a dedicated socket with bulk-string push frames, and the three RESP2 traps (unterminated CRLF replies hang the client parser, buffer-discarding read loops desync on segmented commands, bare-string elements inside arrays are protocol errors to redis-rs). Sections: 0 conventions+desk, 1 connect/clone, 2 strings, 3 TTL trichotomy, 4 hashes, 5 lists+sets, 6 pipelines+atomic, 7 pub/sub, 8 serde values, 9 cache-aside on a chapter-17 reqwest origin (`set_ex` fused write-and-arm, TTL as the invalidation protocol), 10 E-codes, 11 receipts, 12 cheat sheet, 13 review + What Comes Next. Gate: rustc 0 err / 0 warn / run exit 0; anchors contiguous 0–13. **`_p`**: 30 cells / 16 code (crate cell + 15 exercises = one per `_e` demo cell; §9.1's origin server is the given helper so §9's exercises are 9.2/9.3) + 4 mixed review in §10; 27 stub fns, all async, one-line signatures (FN_HEADER requires `{{` on the signature line). Given `redis_desk()` is the `_e` desk verbatim. Kernel gotcha re-bitten: repeated `use redis::AsyncCommands;` across cells is E0252 — the dedup pass must run on the PART files, not just the assembled file (rebuilding from parts re-introduces repeats). Stub state: 0 errors, 85 warnings all `unused variable` (the only family this chapter), aborts at the first todo!() (exit 101). Harness `_sol18_gen.py`: `_sol17_gen.py` machinery + 27 IMPLS + externs bridge; harness-caught fixes: `lrange` takes `isize`, `pexpire` takes `i64`, turbofish on `query_async` is `::<T>` (one generic — `::<_, T>` is E0107), bare `sadd` without a binding type is E0283; **15/15 `Exercise N.M passed`, exit 0** (AV quirk: run `./_sol18.exe` from bash). |
| 19 | `postgresql` | ✅ | ✅ | ✅ | 40 cells / 12 code in `_e`, sections 0–13. Fifth crate chapter — **sqlx 0.8.6 (features runtime-tokio+postgres, NO macros — no derive, FromRow is hand-written)** joined the spike16 wave; full re-hash: tokio `dce1e8f34dde28e3`, reqwest `911748c308b2904f`, serde `1d2761108e09fc5a`-family (serde `1d2776118e09fc5a`, serde_json `c368b775ce1123f4`), tokio-tungstenite `78ba4eb886013dc4`, futures-util `ff56a726dce2df9f`, redis `b3bfeb30b80119af`, sqlx `f6d65eb45cdc6ced` — all in `_externs.txt`; 16e/17e/18e re-gated clean on the wave. **No PostgreSQL on the machine and none installed** — the ch17/18 playbook raised a level: §0.2 builds a ~250-line PostgreSQL v3 wire-protocol server in-notebook on std TcpListener (StartupMessage/auth-ok/ParameterStatus/ReadyForQuery handshake, SSLRequest→bare `N`, extended protocol Parse/Bind/Describe/Execute/Close/Sync, BEGIN/COMMIT/ROLLBACK as snapshot-restore in a shared `Db`, DataRow binary int4 + text, NULL = i32 −1), every cell tests against it. Probe-first (probe19: pgserv/driver/nulls on Temp) found the wire truths before any cell was written: framing `[tag][i32 len][body]` where len counts its own 4 bytes but NOT the tag byte (StartupMessage the exception — len covers protocol+kv), sqlx's Sync→Parse→Describe→Bind→Execute→Close sequence, the six ParameterStatus pairs sqlx wants (server_version 16.9, client_encoding UTF8, DateStyle, integer_datetimes on, TimeZone UTC, standard_conforming_strings on), ReadyForQuery status byte I/T, **Bind carries the statement NAME not the SQL** (bug found live: offset is `sstart + stmt.len() + 1`, not sql.len()). Probed API truths taught in: `sqlx::postgres::PgPoolOptions` full path (bare `sqlx::PgPoolOptions` is E0433), `PgConnectOptions::new().host().port().username().database()` + `connect_with`, queries with `&mut *tx`, `commit()`/`rollback()` take self by value, `row.get`/`try_get` need `use sqlx::Row;` (imported once, in 2.1), `err.code()` is `Cow<str>` → `.unwrap_or_default().into_owned()`, NULL→bare String = `ColumnDecode` (UnexpectedNullError), PoolTimedOut ~215ms on a 1-conn pool with 200ms acquire_timeout, drop-rollback lands within the 250ms sleep (server snapshot restored, count receipt), migrations idempotent. Kernel fact for the tracker: `static APPLIED: std::sync::Mutex<Vec<&'static str>> = std::sync::Mutex::new(Vec::new());` in §8.1 — const-fn `Mutex::new` in a static initializer works on this toolchain (1.98). Sections: 0 conventions+wire server, 1 connect/pool, 2 fetch shapes, 3 FromRow, 4 NULL/Option, 5 transactions, 6 bind/injection, 7 pool sizing/timeouts, 8 migrations, 9 runtime vs macros/tokio-postgres/diesel, 10 E-codes, 11 receipts, 12 cheat sheet, 13 review + What Comes Next (20 PyO3, 21 Capstone Prep, 22 Capstone). Gate: rustc 0 err / 0 warn / run exit 0; anchors contiguous 0–13. **`_p`**: 28 cells / 14 code (crate cell + 13 exercises = one per `_e` demo cell) + 4 mixed review in §10; 24 stub fns, all async, one-line signatures (FN_HEADER). Given helpers: `pg_desk()`/`pool_on(port)` compress the `_e` desk to two entry points (embedded re-implementation, log `pg_tx_p.log` to avoid clashing with 19e's `pg_tx.log`); §3.1 gives `Fill` + its hand-written FromRow; §8.1 gives the `P_APPLIED` ledger static; §7.1/§10.4 build pools with `PgPoolOptions` directly (the 1-conn cap + 200ms timeout ARE the exercise — `pool_on`'s defaults would hide them). Stub state: 0 errors, 36 warnings all in the three documented families (34 unused variable, 2 stub-induced dead code; `use sqlx::Row;` legitimately used by the given 3.1 impl), aborts at the first todo!() (exit 101). Harness `_sol19_gen.py`: `_sol18_gen.py` machinery + 24 IMPLS + externs bridge; harness-caught fix: `for v in versions` over `&[&'static str]` yields `&&str` — deref at the pushes. **13/13 `Exercise N.M passed`, exit 0** (AV quirk: run `./_sol19.exe` from bash). |
| 20 | `pyo3` | ✅ | ✅ | ✅ | 32 cells / 9 code in `_e`, sections 0–12. Sixth crate chapter by subject, **zero crates by kernel**: pyo3 links as a cdylib against `python313.dll` and cannot load into evcxr, so the plan in the old tracker row held exactly — `_e` teaches through **pre-verified snippets with pytest receipts inline**, plus 9 **runnable std-mirror cells** (the plain-Rust shapes behind the macros: ownership crossings, From-bridged errors, the Bound/Py lifetime split, attach/detach sectioning, CStr views) so the kernel gate stays honest 0/0/0. Probe-first built the desk: `C:/Users/GSL/AppData/Local/Temp/spike20` (cdylib, pyo3 = "0.29" → 0.29.2) + `pytest/test_spike20.py` — **8/8 passed** covering functions, FromPyObject args, return shapes, all three error levels, the Fill class, containers, calling Python, token threading. Toolchain findings (all verified before authoring): **no pyo3-config.txt and no import-lib machinery needed** — pyo3 0.29 uses raw-dylib on Windows, discovers the interpreter through PATH `python` + sysconfig (headers at this install's flat `C:/DEV/Python/include`; Python 3.13.5, pytest 8.4.0); first build ~20 s, edit loop ~1 s; `cp target/debug/spike20.dll pytest/spike20.pyd` is the deployment. Probe-caught truths taught in: `From` impl decides the Python-visible payload (`new_err(e.0)` bypassed Display — receipt failed, fixed to `e.to_string()`); setter Err passes through verbatim (ValueError, not AttributeError); `__repr__` in Rust speaks Debug double quotes; `py.eval` wants `&CStr` (`c"..."`, E0308) and `to_bytes` strips the NUL (20e's own probe caught the off-by-one in a first-draft assert); `py: Python` token params are elided from the Python signature; `final` is a reserved keyword (gate-caught: E0007-style parse error, worse inside `assert_eq!` — rename); `PhantomData` shorthand needs the full path in struct exprs; sections 0 conventions+receipts, 1 boundary, 2 pyfunction, 3 errors, 4 pyclass, 5 containers, 6 calling Python, 7 token/GIL/attach, 8 build loop, 9 E-codes, 10 receipts, 11 cheat sheet, 12 review + WCN (21 Capstone Prep, 22 Capstone). Gate: rustc 0 err / 0 warn / run exit 0; anchors contiguous 0–12. **`_p`**: 26 cells / 12 code, 12 kernel exercises (§1–§7, §9, §10.1–10.4 mixed review) + **§8 Guided cdylib Tasks** (markdown: three prompt-side tasks against spike20 — module-from-scratch with the PyInit-name-mismatch observation, the error bridge with the Display experiment, the Fill class with the refusing setter at the `>>>` prompt) + §11 WCN; 26 stub fns, all sync, one-line signatures; given items: `Spill`/`PyErrMirror`/`shim` (§3.1), `Registry`/`CallError`/`getattr` (§6.1), `Lock` guard (§5.1) and logging `Gil` (§7.1) — two guards because §5's needed no log and two items may not share a name (E0609 caught in stub state, the uniqueness rule re-learned). Stub state: 0 errors, 42 warnings all in the three documented families (40 unused variable, 2 given-helper `never constructed`), aborts at the first todo!() (exit 101). Harness `_sol20_gen.py`: `_sol18_gen.py` machinery byte-identical (externs bridge still runs; std-only means harmless) + 26 IMPLS. **12/12 `Exercise N.M passed`, exit 0** on the first run (AV quirk: run `./_sol20.exe` from bash). |
| 21 | `capstone-prep` | ✅ | ✅ | ✅ | 20 cells / 4 code in `_e`, sections 0–10. Stays std-only in the kernel (two-track rule); the tools live in the **spike21 desk** (`C:/Users/GSL/AppData/Local/Temp/spike21`: dev-dep **criterion 0.8.2**, `[[bench]] desk harness=false`, `benches/desk.rs`, `tests/equiv.rs` gate) with receipts quoted inline: criterion intervals `desk/slow [146.61 148.61 151.31 ns]` vs `desk/fast [82.647 83.181 83.765 ns]` (+`Throughput::Elements` thrpt), gnuplot→plotters report, ~9% outliers normal; `cargo add serde --dry-run` feature inventory (`+`/`-`, abort warning); `cargo bench -- --test` → `Testing desk/slow ... Success`; `--save-baseline`/`--baseline` as the regression gate; registry sources under `$CARGO_HOME/registry/src/…` and **CARGO_HOME is `C:\DEV\Rust\.cargo` on this machine (non-default)**. Sections: 0 conventions+desk map, 1 docs.rs flags/CHANGELOG/registry/SemVer, 2 cargo add/features/tree/lockfile (feature unification cross-ref to ch19's wave), 3 one domain error + From-per-boundary + outermost-PyErr (17–20's crates named), 4 criterion (interval reading, black_box, baselines, CI mode, correctness-first), 5 service layout (`lib/error/feed/cache/store/kernel`; one job per module, deps inward, one façade owns shutdown — ch16's parked-runtime rule scaled), 6 capstone scope (fixed core = ch22's service; deferred: tls/auth/metrics/distributed; logging already ch09's), 7 E-codes (E0432/E0599 as feature stories, bench-unit visibility), 8 receipts, 9 cheat sheet, 10 review + WCN (22 Capstone). Kernel mirrors: desk map, error bridge (`FeedErr`/`CacheErr`→`ServiceError`, receipts `feed: status 404` + `feed: timeout`), Instant-direction timing (461.8µs vs 6.0µs at n=2000, correctness asserted first), service miniature (inventory + pure `price` + façade `run`). Gate: 0 err / 0 warn / exit 0; anchors contiguous 0–10. Gate-caught lessons: duplicate `Feed` between §3.1 and §5.1 mirrors (the uniqueness rule again — §5's renamed `FeedMod`), `assert_eq!` on `Result` needs `PartialEq` on the error (ch11's rule) and `Timeout` never-constructed (0-warning: exercise every variant you define). **`_p`**: 23 cells / 11 code, 11 exercises (21 stub fns; §6 is the scope-sieve table, §7 the E-code decisions), with §0's chapter caveat: **no wall-clock asserts** — §4/§8 parse RECORDED criterion intervals as data, deterministic by construction. Given surfaces: two-crate clients + `ServiceError` (§3.1), `Job` inventory fn (§5.1), `caret_range` given complete (§2.1). Stub state: 0 errors, 27 warnings all in the three documented families, aborts at the first todo!() (exit 101). Harness `_sol21_gen.py` (18p machinery, std-only): **11/11 `Exercise N.M passed`, exit 0** first run (AV quirk: `./_sol21.exe` from bash). Harness-count caveat for the tracker: a check block whose only non-stub fn was GIVEN complete (§2.1's `caret_range`) contributes zero exercise fns, so `exercised` (21) < a naive stub-count — pair counts against check blocks (11), not against `grep -c todo!`. |
| 22 | `capstone-market-data` | ✅ | ✅ | ✅ | 32 cells / 10 code in `_e`, sections 0–12. Pure assembly, as scoped: **no new crates** (the ch19 wave's six load once), and the three wire-protocol desks from chapters 17/18/19 ported verbatim into one namespace with `md_`/`rd_`/`pg_` prefixes — the E0428 threat became the architecture lesson (§21.5's module tree, flattened). Sections: 0 conventions+crate cell, 1 the three desks (feed/cache/store + all-three smoke: /json decodes, PING, count==0), 2 the spine (`MdFill`/`MdQuote` — serde derives both ways; `MdServiceError` with `From` per crate, the sqlx impl keeping message+SQLSTATE; exit contract), 3 ingest + cache-aside (the request log proves it causally: two lookups, one origin request), 4 transactional persistence (batch in one tx; ghost row drop-rolled; `42P01` → `store: …`; ledger tail receipts), 5 the pure kernel (vwap/notional/top, unit-asserted with no desks), 6 the façade (`md_run_service` → `MdSummary`; dead store probed via the builder's Err into the same enum), 7 the CLI (report → 0, service error → 1, usage → 2, stderr honest), 8 graceful shutdown (reverse-order close, runtime drop kills in-flight), 9 E-codes, 10 receipts, 11 cheat sheet, 12 review + series close. Gate: 0/0/0; anchors contiguous 0–12. Gate-caught lessons taught in: trait imports at first use (`sqlx::Row` in the smoke cell, `redis::AsyncCommands` in 3.1, serde derives on both spine structs); **blocking-inside-async reproduced live** — §3.1's first draft fetched the origin inside `block_on` and tokio's drop-panic fired exactly as ch16 documented (origin call hoisted; the lesson is in the prose); `md_pool_on` unwraps so the dead-store probe goes through the builder's Err directly (the first draft panicked after a 5s PoolTimedOut — gate-caught); the pub-sub machinery stripped from the Redis port drew unused/dead warnings until the fields and params went too. **`_p`**: 26 cells / 13 code; given: three desks verbatim (`pg_tx_p.log` keeps both notebooks' ledgers), spine + `MdSummary{Clone}`, the origin fn, the pool builder; 26 stub fns, one-line signatures (three started multi-line — FN_HEADER silently skipped them until the count read 25/26; the last, `audit_pass`, surfaced as a surviving todo). Stub state: 0 errors, 44 warnings all in the three documented families (43 unused variable + 1 `unused import: redis::AsyncCommands` serving unsolved stubs), aborts at the first todo!() (exit 101). Harness `_sol22_gen.py`: 18p machinery + 26 IMPLS; `run_pipeline`'s origin fetch is conditional on the miss (§9.1 asserts two passes → one origin request); the miss-probe block's bare `Ok(raw)` was E0283 (multiple `From<MdServiceError>` impls) → `Ok::<Option<String>, MdServiceError>` turbofish; check code obeys the same blocking rule (4.1/9.2 hoist `ingest` out of `block_on`). **12/12 `Exercise N.M passed`, exit 0** (stderr carries 7.1's deliberate Error/usage receipts; AV quirk: run `./_sol22.exe` from bash). **The series is complete: 22/22 chapters, every `_e` gated 0 errors / 0 warnings / exit 0, every `_p` harness-solved, every claim probe-verified before it was taught.** |

### Forward references already promised by chapter 01

These must be honoured, or the promise must be removed from `01e`:

- `01e` §7 → **chapter 10**, for operators being traits (`Add`, `BitAnd`, and why
  `String + &str` works).
- `01e` §7.4 → **chapter 02**, for `Option` handling beyond `unwrap`.
- `01e` §15 → **chapter 05**, for the borrow checker (`E0382`, `E0502`, `E0499`).
- `01e` closing → **chapters 02–05**, by title, each with a one-line description.
- `01e` §16 cheat sheet → the whole series, as the fastest route back to syntax.

### Cleanup debt, settled

The scratch files named in §9 are gone; what remains is catalogued in §19. Leave the
project scaffolding (`Cargo.toml`, `Cargo.lock`, `src/`, `target/`) alone.

## 19. The Retained Tooling

After the end-of-series cleanup, four workspace files remain around the notebooks.
Each exists for one reason; none is a deliverable.

### `_build_nb.py` — the notebook builder (kept for possible authoring)

The notebooks are the series' source of truth; the `.src` aggregates they were built
from were deleted at series completion (see §15 step 3). The builder remains because
a chapter revision can regenerate a `.src` — wrap each cell as
`<VSCode.Cell language="markdown">` / `"rust">` … `</VSCode.Cell>` — and rebuild:

```bash
python _build_nb.py 17_http-and-websockets_e.src
```

It emits each cell with the exact metadata the series requires — the evcxr
kernelspec, `language_info`, `vscode.languageId: "rust"` on code cells, empty
`outputs` and `execution_count` — so no separate metadata pass is ever needed.
Rebuilding is cell-identical to editing the notebook JSON by hand, and is how every
notebook in the workspace was originally produced.

### `_validate.py` — the compile-and-run harness (std chapters)

Extracts every code cell in notebook order, splits it into **items** and
**statements** while tracking brace depth, hoists the items to top level, and
compiles the result as `fn main() { ... }` — a faithful stand-in for the kernel,
because Rust items are order-independent (see §14 for the full mechanics).

```bash
python _validate.py 01_syntax-printing-and-types_e.ipynb
```

Pass criterion for an `_e`: rustc exits 0 with **zero warnings**, and the run
prints sensible output ending in exit 0. For a `_p`, the run must abort at the
first `todo!()` (exit 101) with 0 errors and warnings only in the documented
families. If the final exe launch fails with the known Windows `WinError 2`
AV, the compile verdict still stands — run `./_check.exe` from bash for the
runtime verdict.

### `_crate_validate.py` — the bridge validator (crate chapters, 09+)

Chapters 09 onward consume external crates, which a bare rustc call cannot see.
The bridge reuses `_validate.py`'s splitter to emit `_check.rs` (also stripping
evcxr `:`-magic lines), then compiles with the `--extern` rlibs declared in
`_externs.txt` and runs the result — same pass criteria as above.

```bash
python _crate_validate.py 19_postgresql_e.ipynb
```

### `_externs.txt` — the frozen crate manifest

One `crate=path/to/libcrate-hash.rlib` line per external crate, pointing at the
exact rlibs built in the spike desks; a `native=<dir>` line adds `-L native=`
for winapi's import libraries (polars does not link without them). The hashes
are frozen evidence: each wave is feature-unified, and the file's own comments
record the rebuild recipe and why tokio re-hashed three times.

The gate depends on these paths — under `C:/Users/GSL/AppData/Local/Temp/` —
staying put. If a desk is ever removed, rebuild it per the recipe and refresh
the hashes here before re-validating.
