#!/bin/bash
set -ueo pipefail

for wat in wat/*.wat; do
    wat2wasm "$wat"
done

mv *.wasm ..
