"""Build a .ipynb from a "<VSCode.Cell language=...>" source file.

Usage: python _build_nb.py <source.src>

The source format is a flat sequence of cells:

    <VSCode.Cell language="markdown">
    ...markdown...
    </VSCode.Cell>
    <VSCode.Cell language="rust">
    ...rust...
    </VSCode.Cell>

Cells are emitted with the kernelspec/language_info this series requires, so the
output is ready to validate without a separate metadata pass.
"""
import json
import pathlib
import re
import sys

CELL = re.compile(
    r'<VSCode\.Cell language="(markdown|rust)">\n(.*?)\n</VSCode\.Cell>',
    re.S,
)

KERNELSPEC = {"display_name": "Rust", "language": "rust", "name": "rust"}
LANGUAGE_INFO = {
    "name": "Rust",
    "codemirror_mode": "rust",
    "file_extension": ".rs",
    "mimetype": "text/rust",
    "pygment_lexer": "rust",
    "version": "",
}


def build(src_path):
    text = pathlib.Path(src_path).read_text(encoding="utf-8")
    cells = []
    for lang, body in CELL.findall(text):
        body = body.rstrip("\n") + "\n"
        source = body.splitlines(keepends=True)
        if lang == "markdown":
            cells.append({
                "cell_type": "markdown",
                "metadata": {},
                "source": source,
            })
        else:
            cells.append({
                "cell_type": "code",
                "execution_count": None,
                "metadata": {"vscode": {"languageId": "rust"}},
                "outputs": [],
                "source": source,
            })

    if not cells:
        sys.exit(f"no cells found in {src_path}")

    notebook = {
        "cells": cells,
        "metadata": {"kernelspec": KERNELSPEC, "language_info": LANGUAGE_INFO},
        "nbformat": 4,
        "nbformat_minor": 5,
    }

    out_path = pathlib.Path(src_path).with_suffix(".ipynb")
    out_path.write_text(
        json.dumps(notebook, ensure_ascii=False, indent=1) + "\n", encoding="utf-8"
    )

    code = sum(1 for c in cells if c["cell_type"] == "code")
    md = len(cells) - code
    print(f"{out_path.name}: {len(cells)} cells ({code} code, {md} markdown)")

    # A cell that does not end in exactly one newline, or a source list whose
    # entries do not all end in a newline, is a rendering bug waiting to happen.
    for i, cell in enumerate(cells, 1):
        text_of = "".join(cell["source"])
        if not text_of.endswith("\n"):
            print(f"  WARN cell {i} does not end with a newline")
        for line in cell["source"][:-1]:
            if not line.endswith("\n"):
                print(f"  WARN cell {i} has an interior line without a newline")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    for arg in sys.argv[1:]:
        build(arg)
