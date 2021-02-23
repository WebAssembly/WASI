#!/bin/bash
set -ueo pipefail

# Top-level test runner. Usage is "run.sh" to run tests in compile-only mode,
# or "run.sh <runwasi>" where <runwasi> is a WASI-capable runtime to run the
# tests in full compile and execute mode.
#
# By default this script will look for `clang` and `clang++` in $PATH and
# assume that they are correctly configured with the sysroot in the default
# location.  Alternatively, exporting $CC and $CXX allow more flexibility. e.g:
#
#  export CXX="<wasi-sdk>/bin/clang++ --sysroot <wasi-sdk>/share/wasi-sysroot"
#  export CCC="<wasi-sdk>/bin/clang --sysroot <wasi-sdk>/share/wasi-sysroot"
#

# Determine the wasm runtime to use, if one is provided.
if [ $# -gt 0 ]; then
    runwasi="$1"
else
    runwasi=""
fi

testdir=$(dirname $0)
CC=${CC:=clang}
CXX=${CXX:=clang++}

echo $CC
echo $CXX

cd $testdir/compile-only
for options in -O0 -O2 "-O2 -flto"; do
    echo "===== Testing compile-only with $options ====="
    for file in *.c; do
        echo "Testing compile-only $file..."
        ../testcase.sh "" "$CC" "$options" "$file"
    done
    for file in *.cc; do
        echo "Testing compile-only $file..."
        ../testcase.sh "" "$CXX" "$options" "$file"
    done
done
cd - >/dev/null

cd $testdir/general
for options in -O0 -O2 "-O2 -flto"; do
    echo "===== Testing with $options ====="
    for file in *.c; do
        echo "Testing $file..."
        ../testcase.sh "$runwasi" "$CC" "$options" "$file"
    done
    for file in *.cc; do
        echo "Testing $file..."
        ../testcase.sh "$runwasi" "$CXX" "$options" "$file"
    done
done
cd - >/dev/null
