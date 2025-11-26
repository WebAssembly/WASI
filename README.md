[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.4323447.svg)](https://doi.org/10.5281/zenodo.4323447)
    
# WebAssembly System Interface

![WASI](WASI.png)

The WebAssembly System Interface (WASI) is a set of APIs for WASI being
developed for eventual standardization by the WASI Subgroup, which is a
subgroup of the WebAssembly Community Group.

WASI started with launching what is now called [Preview 1], an API using
the witx IDL, and it is now widely used. Its major influences are POSIX and
CloudABI.

[WASI Preview 2] is now stable, and is a modular collection of
APIs defined with the [Wit IDL], and it incorporates many of the lessons
learned from Preview 1, including adding support for a wider range of
source languages, modularity, a more expressive type system,
virtualizability, and more.

[Preview 1]: https://github.com/WebAssembly/WASI/tree/main/legacy/README.md
[WASI Preview 2]: https://github.com/WebAssembly/WASI/blob/main/wasip2/README.md
[Wit IDL]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md

## Find the APIs

Development of each API happens in its own repo, which you can access
from the [proposals list](docs/Proposals.md).

This repo is for general discussion, as well as documenting how we work
and high-level goals.

## Propose a new API

If you would like to create a new proposal, get started with our
[Contributing guide](docs/Contributing.md).

All new API proposals should use the new format and the new repo structure that is shown in the [proposal template](https://github.com/WebAssembly/wasi-proposal-template).

See the [Wit in WASI](docs/WitInWasi.md) document for more information about using Wit for WASI proposals.

## WASI High Level Goals

(In the spirit of [WebAssembly's High-Level Goals](https://github.com/WebAssembly/design/blob/main/HighLevelGoals.md).)

1. Define a set of portable, modular, runtime-independent, and
   WebAssembly-native APIs which can be used by WebAssembly code to interact
   with the outside world. These APIs preserve the essential sandboxed nature of
   WebAssembly through a [Capability-based] API design.
2. Specify and implement incrementally. Start with a Minimum Viable Product
   (MVP), then adding additional features, prioritized by feedback and
   experience.
3. Supplement API designs with documentation and tests, and, when feasible,
   reference implementations which can be shared between wasm engines.
4. Make a great platform:
    * Work with WebAssembly tool and library authors to help them provide
      WASI support for their users.
    * When being WebAssembly-native means the platform isn't directly
      compatible with existing applications written for other platforms,
      design to enable compatibility to be provided by tools and libraries.
    * Allow the overall API to evolve over time; to make changes to API
      modules that have been standardized, build implementations of them
      using libraries on top of new API modules to provide compatibility.

[Capability-based]: https://en.wikipedia.org/wiki/Capability-based_security
