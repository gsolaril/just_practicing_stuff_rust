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
    "macro_rules",  # hoists like the kernel: later cells can invoke it
)
# Lines that belong to the *next* item when they sit directly above it.
ITEM_LEADS = ("#[", "///", "//!")

CHECK_RS = pathlib.Path("_check.rs")
CHECK_EXE = pathlib.Path("_check.exe")


def _item_start(line):
    return line.startswith(ITEM_STARTS)


def _item_lead(line):
    return line.startswith(ITEM_LEADS)


class _StrState:
    """String/comment lexer state threaded across the lines of one cell.

    Tracks raw strings `r"..."`/`r#"..."#`, plain `"..."` strings and `/* */`
    comments so brace counting never sees delimiters inside them. Raw strings
    (and plain strings, and comments) may span lines; state carries over.
    """

    __slots__ = ("raw", "plain", "block")

    def __init__(self):
        self.raw = None    # inside a raw string: number of '#' guards (r"..." -> 0)
        self.plain = False  # inside a "..." string
        self.block = 0     # nesting depth of /* */ comments

    def open(self):
        return self.raw is not None or self.plain or self.block > 0


def _ident(c):
    return c.isalnum() or c == "_"


def _scan_line(line, st):
    """Scan one line, updating st in place.

    Returns (delta, header_end): the brace-depth change over code outside
    strings/comments, and whether a `{` or `;` appeared there (which ends an
    item's header). Char literals vs lifetimes (`'{'` vs `'a`) are told apart
    by lookahead, so `'{'` never counts as a brace and `&'static` stays sane.
    """
    delta = 0
    header_end = False
    i = 0
    n = len(line)
    while i < n:
        c = line[i]
        if st.raw is not None:
            h = st.raw
            if c == '"':
                j = i + 1
                cnt = 0
                while j < n and line[j] == '#':
                    cnt += 1
                    j += 1
                if cnt >= h:  # closing "### (any surplus '#'s are code)
                    st.raw = None
                    i = i + 1 + h
                    continue
            i += 1
            continue
        if st.plain:
            if c == '\\':
                i += 2
                continue
            if c == '"':
                st.plain = False
            i += 1
            continue
        if st.block:
            if c == '/' and i + 1 < n and line[i + 1] == '*':
                st.block += 1
                i += 2
                continue
            if c == '*' and i + 1 < n and line[i + 1] == '/':
                st.block -= 1
                i += 2
                continue
            i += 1
            continue
        # ---- code context ----
        if c == '/' and i + 1 < n and line[i + 1] == '/':
            break  # line comment: rest of the line is inert
        if c == '/' and i + 1 < n and line[i + 1] == '*':
            st.block += 1
            i += 2
            continue
        if c == '"':
            st.plain = True
            i += 1
            continue
        if c == 'r' and (i == 0 or not _ident(line[i - 1])):
            j = i + 1
            while j < n and line[j] == '#':
                j += 1
            if j < n and line[j] == '"':
                st.raw = j - i - 1
                i = j + 1
                continue
            i += 1
            continue
        if c == 'b' and (i == 0 or not _ident(line[i - 1])):
            j = i + 1
            if j < n and line[j] == 'r':
                k = j + 1
                while k < n and line[k] == '#':
                    k += 1
                if k < n and line[k] == '"':
                    st.raw = k - j - 1
                    i = k + 1
                    continue
            if j < n and line[j] == '"':
                st.plain = True
                i = j + 1
                continue
            i += 1
            continue
        if c == "'":
            if i + 2 < n and line[i + 1] == '\\':
                # Escape inside a char literal: \n \' \\ \u{...}. Find the close.
                j = i + 2
                if line[j] == 'u':
                    k = line.find('}', j)
                    if k != -1 and k + 1 < n and line[k + 1] == "'":
                        i = k + 2
                        continue
                elif j < n and line[j] == "'":
                    i = j + 1
                    continue
                i = j + 1
                continue
            if i + 2 < n and line[i + 2] == "'":
                i += 3  # char literal 'x'
                continue
            i += 1  # lifetime like 'a
            continue
        if c == '{':
            delta += 1
            header_end = True
        elif c == '}':
            delta -= 1
        elif c == ';':
            header_end = True
        i += 1
    return delta, header_end


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
    cell_st = _StrState()
    while i < n:
        line = lines[i]
        if (
            not cell_st.open()
            and line
            and not line[0].isspace()
            and (_item_start(line) or _item_lead(line))
        ):
            block = [line]
            st = _StrState()
            depth, header_end = _scan_line(line, st)
            i += 1
            # Glue: doc/attribute leads, and header continuation lines until the
            # header terminates in `{` or `;` (multi-line signatures, where-clauses;
            # `const X: &str = r#"..."#;` bodies ride along via the string state).
            while i < n and depth == 0 and (
                _item_lead(lines[i]) or not header_end
            ):
                block.append(lines[i])
                d, he = _scan_line(lines[i], st)
                depth += d
                header_end = header_end or he
                i += 1
            # Consume the body until braces balance and no string is open.
            while i < n and (depth > 0 or st.open()):
                block.append(lines[i])
                d, _he = _scan_line(lines[i], st)
                depth += d
                i += 1
            items.extend(block)
            cell_st = st
        else:
            stmts.append(line)
            _scan_line(line, cell_st)
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
