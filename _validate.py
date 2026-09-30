"""Compile-and-run harness for the Rust notebooks, std chapters in particular.

Usage:
    python _validate.py 01_syntax-printing-and-types_e.ipynb

What it does:

1. Reads every code cell in order.
2. Splits each cell into **items** and **statements**, tracking brace depth, so an
   item's body is never cut in half. An item is a line starting at column 0 with
   `fn `/`pub fn `/`const fn `/`async fn `/`const `/`static `/`type `/`use `/`impl`/
   `struct `/`enum `/`trait `/`mod `/`pub `/`#[` (and the doc comments / attributes
   that immediately precede such a line are collected into the item).
3. Hoists all items to the top level, keeps statements in order, and compiles them
   inside a generated `fn main() { ... }`.
4. Writes `_check.rs`, runs `rustc --edition 2024`, then executes `_check.exe`.

This is faithful to evcxr because Rust items are order-independent — hoisting does not
change what compiles. It is deliberately **not** a reimplementation of the kernel: it
does not deduplicate item names, which is what lets it catch real redefinition bugs.

Crate chapters (09+) go through `_crate_validate.py`, which strips evcxr `:`-magics
and compiles with the `--extern` rlibs from `_externs.txt`, reusing `split_cell` here.
"""
import json
import pathlib
import subprocess
import sys

ITEM_STARTS = (
    "pub fn ", "const fn ", "async fn ", "pub(crate) fn ",
    "fn ", "const ", "static ", "type ", "use ", "impl",
    "struct ", "enum ", "trait ", "mod ", "pub ",
)
# Lines that belong to the *next* item when they sit directly above it.
ITEM_LEADS = ("#[", "///", "//!")

CHECK_RS = pathlib.Path("_check.rs")
CHECK_EXE = pathlib.Path("_check.exe")


def _item_start(line):
    return line.startswith(ITEM_STARTS)


def _item_lead(line):
    return line.startswith(ITEM_LEADS)


def _opens_block(line):
    """True while an item header is still incomplete: no `{` and no `;` yet."""
    stripped = line
    pos = stripped.find("//")
    if pos != -1:
        stripped = stripped[:pos]
    return "{" not in stripped and ";" not in stripped


def _depth_delta(line):
    """Brace-depth change over one line, ignoring braces inside line comments."""
    code = line
    pos = code.find("//")
    if pos != -1:
        code = code[:pos]
    return code.count("{") - code.count("}")


def split_cell(src):
    """Split one cell's source into (item_lines, statement_lines).

    Items are emitted as a flat list of whole lines (attributes, doc comments and
    the full brace-balanced body included); statements keep their order. Line-based,
    so the caller can reassemble with plain "\\n".join(...).
    """
    items, stmts = [], []
    lines = src.split("\n")
    i = 0
    n = len(lines)
    while i < n:
        line = lines[i]
        if line and not line[0].isspace() and (_item_start(line) or _item_lead(line)):
            block = [line]
            depth = _depth_delta(line)
            i += 1
            # Glue: doc/attribute leads, and header continuation lines until the
            # header terminates in `{` or `;` (multi-line signatures, where-clauses).
            while i < n and depth == 0 and (
                _item_lead(lines[i]) or _opens_block(block[-1])
            ):
                block.append(lines[i])
                depth += _depth_delta(lines[i])
                i += 1
            # Consume the body until braces balance again.
            while i < n and depth > 0:
                block.append(lines[i])
                depth += _depth_delta(lines[i])
                i += 1
            items.extend(block)
        else:
            stmts.append(line)
            i += 1
    return items, stmts


def json_load(path):
    with open(path, encoding="utf-8") as fh:
        return json.load(fh)


def build(nb_path):
    """Emit _check.rs from a notebook's code cells (items hoisted, statements in main)."""
    nb = json_load(nb_path)
    prelude, body = [], []
    for idx, cell in enumerate(nb["cells"]):
        if cell["cell_type"] != "code":
            continue
        src = "".join(cell["source"])
        # evcxr magics are kernel commands, not Rust: remove them.
        src = "\n".join(
            l for l in src.splitlines() if not l.lstrip().startswith(":")
        )
        if not src.strip():
            continue
        items, stmts = split_cell(src)
        prelude.append(f"// ---- cell {idx} items ----")
        prelude.extend(items)
        body.append(f"// ---- cell {idx} ----")
        body.extend(stmts)
    program = "\n".join(prelude) + "\nfn main() {\n" + "\n".join(body) + "\n}\n"
    CHECK_RS.write_text(program, encoding="utf-8")
    print(f"wrote {CHECK_RS} ({len(program.splitlines())} lines)")
    return CHECK_RS


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    for nb_path in sys.argv[1:]:
        build(nb_path)
        cmd = ["rustc", "--edition", "2024", "-o", str(CHECK_EXE), str(CHECK_RS)]
        print("rustc:", " ".join(cmd))
        r = subprocess.run(cmd, capture_output=True, text=True)
        print(r.stdout + r.stderr[-12000:])
        if r.returncode != 0:
            print("COMPILE FAILED")
            return 1
        run = subprocess.run([str(CHECK_EXE)], capture_output=True, text=True)
        print(run.stdout)
        if run.stderr:
            print("--- stderr ---")
            print(run.stderr)
        print(f"exit code: {run.returncode}")
        if run.returncode != 0:
            return run.returncode
    return 0


if __name__ == "__main__":
    sys.exit(main())
