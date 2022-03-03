[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.4323447.svg)](https://doi.org/10.5281/zenodo.4323447)
    
# WebAssembly System Interface

![WASI](WASI.png)

The WebAssembly System Interface is not a monolithic standard system interface,
but is instead a modular collection of standardized APIs. None of the APIs are
required to be implemented to have a compliant runtime. Instead, host
environments can choose which APIs make sense for their use cases.

---
## Important Note: WASI is in transition

WASI is transitioning away from the `witx` format and its early experimental ABI. We are transitioning to Interface Types using the `wit` format and the canonical ABI.

All new API proposals should use the new format and the new repo structure that is shown in the [proposal template](https://github.com/WebAssembly/wasi-proposal-template).

Some APIs can not yet be supported in the `wit` format. The advancement of these proposals will be unblocked with work that is ongoing:

- Proposals that require async/streams are expected to be unblocked in early Q2 2022
- Proposals that depend on libc are expected to be unblocked by work in `wasi-libc` and elsewhere. Until then, implementers of these APIs should continue to use the snapshots in this repo, which use the `witx` format. We will provide updates on the progress of this work in the bi-weekly meetings.

---

## Find the APIs

Development of each API happens in its own repo, which you can access
from the [proposals list](Proposals.md).

This repo is for general discussion, as well as documenting how we work
and high-level goals.

## Propose a new API

If you would like to create a new proposal, get started with our
[Contributing guide](Contributing.md).

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

## WASI Design Principles

### Capability-based security

WASI is built using capability-based security principles. Access to
external resources is always represented by *handles*, which are special
values that are *unforgeable*, meaning there's no way to coerce an
arbitrary integer or other type of value into a handle. WASI is also
aiming to have no *ambient authorities*, meaning that there should
be no way to request a handle purely by providing a string or other
user-controlled identifier providing the name of a resource. With these
two properties, the only ways to obtain access to resources are to be
explicitly given handles, or to perform operations on handles which
return new handles.

Note that this is a different sense of "capability" than [Linux
capabilities](http://man7.org/linux/man-pages/man7/capabilities.7.html)
or the withdrawn [POSIX
capabilities](https://archive.org/details/posix_1003.1e-990310), which
are per-process rather than per-resource.

The simplest representation of handles are values of [reference
type](https://github.com/WebAssembly/reference-types). References in
wasm are inherently unforgeable, so they can represent handles directly.

Some programming languages operate primarily within linear memory,
such as C, C++, and Rust, and there currently is no easy way for
these languages to use references in normal code. And even if it does
become possible, it's likely that source code will still require
annotations to fully opt into references, so it won't always be
feasible to use. For these languages, references are stored in a
[table](https://webassembly.github.io/spec/core/bikeshed/index.html#tables%E2%91%A0)
called a *c-list*. Integer indices into the table then identify
resources, which can be easily passed around or stored in memory. In
some contexts, these indices are called *file descriptors* since they're
similar to what POSIX uses that term for. There are even some tools,
such as wasm-bindgen, which make this fairly easy. (Internally, tools
and engines don't always use actual WebAssembly tables to do this,
however those are implementation details. Conceptually, they work as if
they had tables.)

Integer indices are themselves forgeable, however a program can only
access handles within the c-list it has access to, so isolation can still
be achieved, even between libraries which internally use integer indices,
by witholding access to each library's c-list to the other libraries.
Instances can be given access to some c-lists and not others, or even
no c-lists at all, so it's still possible to establish isolation between
instances.

Witx-specified APIs use a special `handle` keyword to mark parameters
and return values which are handles. In the short term, these are
lowered to integer indices, with an implied table, so that the APIs
can be easily used from C and similar languages today. Once [interface
types](https://github.com/WebAssembly/interface-types) is
ready, we expect to make use of them to provide APIs which can be used
either from languages using references or from languages using integer
indices, with tables being used and managed automatically.

### Interposition

Interposition in the context of WASI interfaces is the ability for a
Webassembly instance to implement a given WASI interface, and for a
consumer WebAssembly instance to be able to use this implementation
transparently. This can be used to adapt or attenuate the functionality
of a WASI API without changing the code using it.

In WASI, we envision interposition will primarily be configured
through the mechanisms in the module linking' [link-time virtualization](https://github.com/WebAssembly/module-linking/blob/main/design/proposals/module-linking/Explainer.md#link-time-virtualization).
Imports are resolved when a module is instantiated, which may happen
during the runtime of a larger logical application, so we can support
interposition of WASI APIs without defining them in terms of explicit
dynamic dispatch mechanisms.

Interposition is sometimes referred to as "virtualization", however we
use "interposition" here because the word "virtualization" has several
related meanings.

### Compatibility

Compatibility with existing applications and libraries, as well as
existing host platforms, is important, but will sometimes be in conflict
with overall API cleanliness, safety, performance, or portability.
Where practical, WASI seeks to keep the WASI API itself free of
compatibility concerns, and provides compatibility through libraries,
such as WASI libc, and tools. This way, applications which don't require
compatibility for compatibility' sake aren't burdened by it.

### Portability

Portability is important to WASI, however the meaning of portability
will be specific to each API.

WASI's modular nature means that engines don't need to implement every
API in WASI, so we don't need to exclude APIs just because some host
environments can't implement them. We prefer APIs which can run across
a wide variety of engines when feasible, but we'll ultimately decide
whether something is "portable enough" on an API-by-API basis.
