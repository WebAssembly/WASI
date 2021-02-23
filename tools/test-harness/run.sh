#!/bin/bash
set -ueo pipefail

# Top-level test runner. Usage is "run.sh" to run tests in compile-only mode,
# or "run.sh <runwasi>" where <runwasi> is a WASI-capable runtime to run the
# tests in full compile and execute mode.

# Determine the wasm runtime to use.
runwasi="$1"

tooldir=$(dirname $0)

echo "===== Testing ====="
for file in *.wasm; do
    "$tooldir/testcase.sh" "$runwasi" "$file"
done
cd - >/dev/null
