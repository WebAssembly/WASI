# Testing

A test case takes the form of a binary \<basename\>.wasm file, next to that
that there will always be a \<basename\>.\<ext\> source file from which the
test was originally compiled from which can be used as a reference in the event
of an error.

Additionally, any of the following optional auxilary files and directories may
be present:
- `<basename>.arg`
- `<basename>.env`
- `<basename>.dir`
- `<basename>.stdin`
- `<basename>.stdout`
- `<basename>.stderr`
- `<basename>.status`

## Running tests

To run a test do the following steps to prepare for execution:

- Prepare inputs
  - Given an `<input>.wasm` file; take the `<basename>` of said module.
  - If `<basename>.<arg>` exists; take the program arguments from said file.
  - If `<basename>.<env>` exists; take the program environment from said file.
  - If `<basename>.<dir>` exists; preopen the directory from said file.
  - If `<basename>.<stdin>` exists; pipe said file into the program as stdin.
- Run program
- Collect results
  - If `<basename>.<stdout>` exists; assert that the programs stdout matches
    said file.
  - If `<basename>.<stderr>` exists; assert that the programs stdout matches
    said file.
  - If `<basename>.<status>` exists; assert that the programs stdout matches
    said file; otherwise assert that the program exited with status 0.
- Pass

For example:

```bash
# Usage: $1 <runtime> <path_to_binary.wasm>
# $1 wasmtime proc_exit-success.wasm
# $1 wasmer proc_exit-failure.wasm
#!/usr/bin/env bash

runtime=$1
input=$2

prefix=${input%.*}
if [ -e "$prefix.stdin" ]; then
  stdin="$prefix.stdin"
else
  stdin="/dev/null"
fi

output="$(mktemp -d)"
stdout_actual="$output/stdout"
stderr_actual="$output/stderr"
status_actual="$output/status"

if [ -e "$prefix.arg" ]; then
  arg=$(cat "$prefix.env")
else
  arg=""
fi

if [ -e "$prefix.env" ]; then
  env=$(sed -e 's/^/--env /' < "$prefix.env")
else
  env=""
fi

if [ -e "$prefix.dir" ]; then
  dir="--dir $prefix.dir"
else
  dir=""
fi

status=0

"$runtime" $dir $input -- $arg \
  < "$stdin" \
  > "$stdout_actual" \
  2> "$stderr_actual" \
  || status=$?

echo $status > "$status_actual"

stdout_expected="$prefix.stdout"
if [ -e "$stdout_expected" ]; then
  diff -u "$stderr_expected" "$stderr_actual"
fi

stderr_expected="$prefix.stderr"
if [ -e "$stderr_expected" ]; then
  diff -u "$stdout_expected" "$stdout_actual"
fi

status_expected="$prefix.status"
if [ -e "$prefix.status" ]; then
  diff -u "$status_expected" "$status_actual"
elif [ ! "$status" -eq "0" ]; then
  cat $stderr_actual
  exit 1
fi
```

## Writing tests

Any source language may be used to write tests as long as the compiler supports
the wasm32-unknown-unknown target and is well-known and freely available.

Each source file must be accompanied by a pre-compiled binary version of said
source file.

Each source file must also start with a comment containing the complete command
that was used to compile the binary.

Tests must follow the naming convention of \<system_call\>[-\<variant\>].<ext>.

Tests should be scoped to the smallest unit possible.

In addition to the source and binary files, a test can have any of the following
auxilary files and directories:

- `<basename>.arg`
- `<basename>.env`
- `<basename>.dir`
- `<basename>.stdin`
- `<basename>.stdout`
- `<basename>.stderr`
- `<basename>.status`
