WASI Application ABI
====================

In addition to the APIs defined by the various WASI [modules](modules.md) there
are also certain expectations that the WASI runtime places on an application
that wishes to be portable across WASI implementations.

This document describes how a conforming WASI application is expected to behave
in terms of lifecycle (startup, shutdown, etc) and any exports it is expected to
include.

Current Unstable ABI
--------------------

There are two kinds of modules:

 - A *command* exports a function named `_start`, with no arguments and no return
   values. Environments shall call this function once, after instantiating the
   module and all of its dependencies. After this function exits, the instance
   is considered terminated and no further use of any of its exports should be
   permitted.

 - A *reactor* exports a function named `_initialize`, with no arguments and no
   return values. Environments shall call this function once, after instantiating
   the module and all of its dependencies. After this function exits, the instance
   remains live, and its exports may be accessed.

These kinds are mutually exclusive; implementations should report an error if
asked to instantiate a module containing exports which declare it to be of
multiple kinds.

Regardless of the kind, all programs accessing WASI APIs also export a linear
memory with the name `memory`. Pointers in WASI API calls are relative to this
memory's index space.

In the future, as the underlying WebAssembly platform offers more features, we
we hope to eliminate the requirement to export all of linear memory.

Planned Stable ABI
------------------

There is ongoing discussion about what the stable ABI might look like:

- https://github.com/WebAssembly/WASI/issues/13
- https://github.com/WebAssembly/WASI/issues/19
- https://github.com/WebAssembly/WASI/issues/24
