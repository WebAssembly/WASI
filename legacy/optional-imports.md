Optional Imports
================

It can be useful for WASI programs to optionally depend on certain functions in a
WASI API.  For example, if a WASI API evolves to include a new function a
program might want to continue to run on older systems that don't yet support
the new function.  In this case a optional import mechanism allows the program
to run on older systems and detect the presence of the function at runtime.
Another use case is an API that is not applicable on certain embedding.  In this
case optionals imports would allow program to continue to run in such an
embedding, albeit with reduced or modified behavior.

*Note*: In the ELF specification this type of import is known as *weak*.
We chose *optional* because the term weak is already used other context in
WebAssembly and JavaScript, specifically in JavaScript [weakrefs] proposal where
it relates to garbage collection.

WebAssembly itself does not currently provide a mechanism for optional imports.
There is some discussion of adding it to the [spec][spec], and WASI would likely
use such a feature if/when it is added.  In the absence of first class optional
imports this document describes the mechanism used by WASI to specify optional
imports using a custom section.  Currently this is only specified for function
imports.

Declaring an optional import
----------------------------

Optional function imports are implemented using two imports for each function.
The first being the optional function itself and the second being an i32 global
which indicates if the function is present at runtime.  We call this addition
import the guard.

For example, if a module called `wasi:fs` added a new `statvfs` function to its
interface a program could optionally import this new function in the following
way:

```wasm
(func $statvfs (import "wasi:fs" "statvfs.optional"))
(global $statvfs_is_present (import "wasm:fs" "statvfs.is_present") i32)
```

These two imports would also need to be added to the `import.optional` custom
section (See below).

On older systems that don't support the new function, `$statvfs_is_present`
would be set to 0 and calling `$statvfs` would result in a trap.

On systems that do support the new function, `$statvfs_is_present` is set to
1 and calling `$statvfs` would work as expected.

Using an optional function import
---------------------------------

In order to use the above options function its presence should first be tested
for.  In C code this would look something like this:

```c
if (__wasm_is_present(wasm_fs.statvfs)) {
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

Custom section
--------------

A custom section is used to specify which imports are optional, and for each
optional import the name of the corresponding guard (the global import which is
used to signal its presence).  For each module that contains optional imports
the module name is specified once followed by a list of its optional imports
along with their corresponding guards.

The name of this custom section is `import.optional` and its contents are as
follows:

| Field           | Type                   | Description                      |
| ----------------| ---------------------- | -------------------------------- |
| count           | `varuint32`            | count of mod_optionals to follow |
| mod_optionals   | `optional_import_list*`| sequence if optional_import_list |

Each `optional_import_list` has the following content:

| Field         | Type                | Description                           |
| --------------| ------------------- | ------------------------------------- |
| mod_name_len  | `varuint32`         | the length of `mod_name_data` in bytes|
| mod_name_data | `bytes`             | UTF-8 encoding of the module name     |
| count         | `varuint32`         | count of `opt_import` to follow       |
| opt_import    | `opt_import*`       | sequence of `opt_import`              |

Each `opt_import` has the following content:

| Field           | Type             | Description                             |
| --------------- | ---------------- | --------------------------------------- |
| name_len        | `varuint32`      | the length of `name_data` in bytes      |
| name_data       | `bytes`          | UTF-8 encoding of the import name       |
| guard_name_len  | `varuint32`      | the length of `guard_name_data` in bytes|
| guard_name_data | `bytes`          | UTF-8 encoding of the import name       |

[weakrefs]: https://github.com/tc39/proposal-weakrefs
[spec]: https://github.com/WebAssembly/design/issues/1281
