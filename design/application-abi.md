WASI Application ABI
====================

In addition to the APIs defined by the various WASI [modules](modules.md) there
are also certain expectations that the WASI runtime places on an application
that wishes to be portable across WASI implementations.

This document describes how a conforming WASI application is expected to behave
in terms of lifecycle (startup, shutdown, etc) and any exports the that
it is expected to include.

Current Unstable ABI
--------------------

The current WASI unstable ABI specifies only two exports from a WASI
application:

- `_start` - the program entry point
- `memory` - linear memory used by the program

The `_start` start export must be WebAssembly function and will be used as the
program entry point.  This is the default name used by `lld` when linking
WebAssembly modules.  The embedder is expected to call this function once the
module is instantiated.

Many of current WASI unstable APIs require a sharing of linear memory between
the application and the embedder.  In order to use any such APIs the WASI module
is expected to export its linear memory under the name `memory`.

Planned Stable ABI
------------------

There is ongoing discussion about what the stable ABI might look like:

- #13
- #19
- #24