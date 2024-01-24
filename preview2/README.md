# WASI Preview 2

## Introduction

WASI Preview 2 represents a major milestone for WASI. It marks the moment
when WASI has fully rebased on the [Wit IDL] and the [component model]
type system and semantics, making it modular, fully virtualizable, and
accessible to a wide variety of source languages.

[Wit IDL]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[component model]: https://github.com/WebAssembly/component-model

## WASI Preview 2 Contents

WASI Preview 2 contains the following APIs:

| Proposal           | Versions |
| ------------------ | -------- |
| [wasi-io]          | 0.2.0 |
| [wasi-clocks]      | 0.2.0 |
| [wasi-random]      | 0.2.0 |
| [wasi-filesystem]  | 0.2.0 |
| [wasi-sockets]     | 0.2.0 |
| [wasi-cli]         | 0.2.0 |
| [wasi-http]        | 0.2.0 |

[wasi-io]: https://github.com/WebAssembly/wasi-io
[wasi-clocks]: https://github.com/WebAssembly/wasi-clocks
[wasi-random]: https://github.com/WebAssembly/wasi-random
[wasi-filesystem]: https://github.com/WebAssembly/wasi-filesystem
[wasi-sockets]: https://github.com/WebAssembly/wasi-sockets
[wasi-cli]: https://github.com/WebAssembly/wasi-cli
[wasi-http]: https://github.com/WebAssembly/wasi-http

## WASI Preview 2 Implementations

The portability criteria of the WASI Preview 2 proposals were met by the
following implementations, which were demonstrated to be interoperable by
each passing [Wasmtime's WASI Preview 2 test suite][test-suite].

* [Wasmtime]
* [JCO]

[Wasmtime]: https://github.com/BytecodeAlliance/wasmtime
[JCO]: https://github.com/BytecodeAlliance/jco

We plan to promote the Preview 2 test suite to live in the [wasi-testsuite]
project soon, and add more interoperable implementations to this list when
they are complete.

[wasi-testsuite]: https://github.com/WebAssembly/wasi-testsuite

## Proposal requirements for inclusion in WASI Preview 2.

To be included in Preview 2, a proposal must:

 - Reach phase 3 in the [WASI Subgroup Phase Process]
 - Satisfy its own [portability criteria]
 - Be voted for inclusion by the WASI Subgroup

Proposals may be added to WASI Preview 2 at any time until WASI Preview 3
is launched.

## Witx and Wit

Witx files can be derived from Wit files according to the Canonical ABI. Wasm
engines can implement single-module components using just these derived Witx files
and their existing Witx machinery.

## Looking forward to Preview 3

Preview 3 will add the new `stream` and `future` keywords. The
release criteria of Preview 3 will include that performance will be
measured and addressed.

[WASI Subgroup Phase Process]: https://github.com/WebAssembly/WASI/blob/main/Contributing.md#the-phase-process
[portability criteria]: https://github.com/WebAssembly/WASI/blob/main/Contributing.md#2-feature-description-available-wasi-subgroup
[worlds]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#wit-worlds
[launch criteria]: #wasi-preview-2-launch-criteria
