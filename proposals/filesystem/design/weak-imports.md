Weak Imports
============

It can be useful for WASI programs to weakly depend on certain functions in a
WASI API.  For example, if a WASI API evolves to include a new function a
program might want to continue to run on older systems that don't yet support
the new function.  In this case a weak import mechanism allows the program to
run on older systems and detect the presence of the function at runtime.
Another use case is an API that is not applicable on certain embedding.  In this
case weak import would allow program to continue to run in such an embedding,
albeit with reduced or modified behavior.

*Note*: The term *weak* here refers to a type of symbol reference used by a
linker and comes from the ELF specification.  It is not related to the
JavaScript [weakrefs] proposal or to garbage collection.

WebAssembly itself does not currently provide a mechanism for weak imports.
There is some discussion of adding it to the [spec][spec], and WASI would likely
use such a feature if/when it is added.  In the absence of first class weak
imports this document describes the mechanism used by WASI to specify weak
imports a custom section.  Currently this is only specified for function
imports.

Declaring a weak import
-----------------------

Weak function imports are implemented using two imports for each function.  The
first being the weak function itself and the second being an i32 global which
indicates if the function is present at runtime.  We call this addition import
the guard.

For example, if a module called `wasi:fs` added a new `statvfs` function to its
interface a program could import this new function weakly in the following way:

```wasm
(func $statvfs (import "wasi:fs" "statvfs.weak"))
(global $statvfs_is_present (import "wasm:fs" "statvfs.is_present") i32)
```

These two imports would also need to be added to the `import.weak` custom
section (See below).

On older systems that don't support the new function, `$statvfs_is_present`
would be set to 0 and calling `$statvfs` would result in a trap.

On systems that do support the new function, `$statvfs_is_present` is set to
1 and calling `$statvfs` would work as expected.

Using a weak function import
----------------------------

In order to use the above weak function import its presence should first be
tested for.  In C code this would look something like this:

```c
if (wasm_fs.statvfs) {
  wasm_fs.statvfs(...)
}
```

At the WebAssembly level it might look like this:

```wasm
global.get $statvfs_is_present
if i32
  call $statvfs
end
```

Weak import custom section
--------------------------

A custom section is used to specify which imports are weak, and for each weak
import the name of the corresponding guard (the global import which is used to
signal its presence).  For each module that contains weak imports the
module name is specified once followed by a list of its weak import along with
their corresponding guards.

The name of this custom section is `import.weak` and its contents are as
follows:

| Field           | Type                | Description                         |
| ----------------| ------------------- | ------------------------------------|
| count           | `varuint32`         | count of mod_weak_imports to follow |
| mod_weak_imports| `weak_import_list*` | sequence if weak import lists       |

Each `weak_import_list` has the following content:

| Field         | Type                | Description                           |
| --------------| ------------------- | ------------------------------------- |
| mod_name_len  | `varuint32`         | the length of `mod_name_data` in bytes|
| mod_name_data | `bytes`             | UTF-8 encoding of the module name     |
| count         | `varuint32`         | count of `weak_import` to follow      |
| weak_import   | `weak_import*`      | sequence of `weak_import`             |

Each `weak_import` has the following content:

| Field           | Type             | Description                             |
| --------------- | ---------------- | --------------------------------------- |
| name_len        | `varuint32`      | the length of `name_data` in bytes      |
| name_data       | `bytes`          | UTF-8 encoding of the import name       |
| guard_name_len  | `varuint32`      | the length of `guard_name_data` in bytes|
| guard_name_data | `bytes`          | UTF-8 encoding of the import name       |

[weakrefs]: https://github.com/tc39/proposal-weakrefs
[spec]: https://github.com/WebAssembly/design/issues/1281
