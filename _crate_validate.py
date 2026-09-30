"""Bridge validator for crate chapters (09+): runs `_validate.py` to produce
`_check.rs`, strips evcxr `:`-magic lines, then compiles with the `--extern`
rlibs declared in `_externs.txt` and runs the result.

Usage:
    python _crate_validate.py NN_<slug>_e.ipynb

`_externs.txt` (one per line): `crate_name=path/to/libname-hash.rlib`
A `-L dependency=<dir>` is added for each distinct rlib directory.
`native=<dir>` lines (key literally `native`) emit `-L native=<dir>` instead:
import libraries from build scripts (winapi's libwinapi_*.a etc.) that cargo
picks up automatically but a manual rustc call must be told about.
Set RUST_LOG in the environment before calling to test env-filtered output.
"""
import pathlib
import re
import subprocess
import sys

EXTERNS_FILE = pathlib.Path("_externs.txt")


def read_externs():
    externs, dirs, native_dirs = [], [], []
    if not EXTERNS_FILE.exists():
        return externs, dirs, native_dirs
    for line in EXTERNS_FILE.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        name, path = line.split("=", 1)
        if name == "native":
            if path not in native_dirs:
                native_dirs.append(path)
            continue
        externs += ["--extern", f"{name}={path}"]
        d = str(pathlib.Path(path).parent)
        if d not in dirs:
            dirs.append(d)
    return externs, dirs, native_dirs


def build(nb_path):
    """Reuse _validate.py's splitter to emit _check.rs with magics stripped."""
    import _validate

    nb = json_load(nb_path)
    prelude, body = [], []
    for idx, c in enumerate(nb["cells"]):
        if c["cell_type"] != "code":
            continue
        src = "".join(c["source"])
        # evcxr magics are kernel commands, not Rust: remove them.
        src = "\n".join(
            l for l in src.splitlines() if not l.lstrip().startswith(":")
        )
        if not src.strip():
            continue
        items, stmts = _validate.split_cell(src)
        prelude.append(f"// ---- cell {idx} items ----")
        prelude.extend(items)
        body.append(f"// ---- cell {idx} ----")
        body.extend(stmts)
    program = "\n".join(prelude) + "\nfn main() {\n" + "\n".join(body) + "\n}\n"
    out = pathlib.Path("_check.rs")
    out.write_text(program, encoding="utf-8")
    print(f"wrote {out} ({len(program.splitlines())} lines)")
    return out


def json_load(path):
    import json

    with open(path, encoding="utf-8") as f:
        return json.load(f)


def main():
    nb_path = sys.argv[1]
    out = build(nb_path)
    externs, dirs, native_dirs = read_externs()
    exe = pathlib.Path("_check.exe")
    cmd = ["rustc", "--edition", "2024", "-o", str(exe), str(out)]
    for d in native_dirs:
        cmd += ["-L", f"native={d}"]
    for d in dirs:
        cmd += ["-L", f"dependency={d}"]
    cmd += externs
    print("rustc:", " ".join(cmd))
    r = subprocess.run(cmd, capture_output=True, text=True)
    print(r.stdout + r.stderr[-12000:])
    if r.returncode != 0:
        print("COMPILE FAILED")
        return 1
    run = subprocess.run([str(exe)], capture_output=True, text=True)
    print(run.stdout)
    if run.stderr:
        print("--- stderr ---")
        print(run.stderr)
    print(f"exit code: {run.returncode}")
    return run.returncode


if __name__ == "__main__":
    sys.exit(main())
