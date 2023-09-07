# WASI Preview 2

At this time, Preview 2 is in development and has not yet launched.

## Launch criteria

WASI Preview 2 will be considered launched when at least two independent
proposals which define worlds, and all their dependencies, have met the
requirements for inclusion listed below, and the WASI Subgroup has voted
to launch it.

## Introduction

*The following is a draft of an introduction for WASI Preview 2 when it launches.*

WASI Preview 2 represents a major milestone for WASI. It marks the moment
when WASI has fully rebased on the [Wit IDL] and the [component model]
type system and semantics, making it modular, fully virtualizable, and
accessible to a wide variety of source languages.

[Wit IDL]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[component model]: https://github.com/WebAssembly/component-model

## WASI Preview 2 Contents

WASI Preview 2 contains the following APIs:

| Proposal                                                                       | Versions |
| ------------------------------------------------------------------------------ | -------- |

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
