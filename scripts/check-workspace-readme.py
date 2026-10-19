#!/usr/bin/env python3
"""Compile and execute the root README against the local mti library.

Crate READMEs are included in cargo doctests. The root README lives outside
published packages, so it is tested separately without adding a crate dependency.
"""

import argparse
import json
import os
from pathlib import Path
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--all-features", action="store_true")
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    command = [
        os.environ.get("CARGO", "cargo"), "build", "--package", "mti", "--lib",
        "--message-format=json",
    ]
    if args.all_features:
        command.append("--all-features")
    result = subprocess.run(command, cwd=root, stdout=subprocess.PIPE, text=True)
    library = None
    for line in result.stdout.splitlines():
        message = json.loads(line)
        if message.get("reason") == "compiler-message":
            print(message["message"]["rendered"], end="")
        if message.get("reason") == "compiler-artifact" and message["target"]["name"] == "mti":
            library = next((Path(path) for path in message["filenames"] if path.endswith(".rlib")), None)
    result.check_returncode()
    if library is None:
        raise RuntimeError("Cargo did not produce the mti library artifact")
    subprocess.run([
        os.environ.get("RUSTDOC", "rustdoc"), "--test", "--edition=2021",
        str(root / "README.md"), "--extern", f"mti={library}",
        "-L", f"dependency={library.parent / 'deps'}",
    ], cwd=root, check=True)


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as error:
        raise SystemExit(error.returncode) from error
