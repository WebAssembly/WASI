WASI Application ABI
====================

In addition to the APIs defined by the various WASI [modules](modules.md) there
are also certain expectations that the WASI runtime places on an application
that wishes to be portable across WASI implementations.

This document describes how a conforming WASI application is expected to behave
in terms of lifecycle (startup, shutdown, etc) and imports and exports the that
it is expected to include.

Lifecycle
---------

A WASI program may contain any number exports, but the embedded attributes
specific meaning the following optional function exports:

- `__wasi_init` - Called after WebAssemembly instantiation but before any other
  functions.
- `__wasi_main` - Runs the programs main entry points.  May be omitted, for
  example, in the case of a library.
- `__wasi_term` - The logical inverse of `__wasi_init`.  Optionally called
  before module destruction.  No other functions within program will be called
  after this one.

Linear Memory
-------------

All WASI programs are expected to share a linear memory with the embedder.  The
memory can either be imported from the embedder or exports to the embedder.  If
exported the memory must be named `__wasi_memory`.  If imported the import must
be named `memory` from a module names `wasi`.
