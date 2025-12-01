[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.4323447.svg)](https://doi.org/10.5281/zenodo.4323447)
    
# WebAssembly System Interface

![WASI](assets/WASI.png)

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
[WASI Preview 2]: https://github.com/WebAssembly/WASI/blob/main/docs/Preview2.md
[Wit IDL]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md

## Find the APIs

Development of each API happens in its own repo, which you can access
from the [proposals list](docs/Proposals.md).

This repo is for general discussion, as well as documenting how we work
and high-level goals.

## Propose a new API

If you would like to create a new proposal, get started with our
[Contributing guide](CONTRIBUTING.md).

All new API proposals should use the new format and the new repo structure that is shown in the [proposal template](https://github.com/WebAssembly/wasi-proposal-template).

See the [Wit in WASI](docs/WitInWasi.md) document for more information about using Wit for WASI proposals.
