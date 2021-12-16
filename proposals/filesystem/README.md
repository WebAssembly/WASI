# WASI filesystem

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

WASI-filesystem is currently in [Phase 2].

[Phase 2]: https://github.com/WebAssembly/WASI/blob/42fe2a3ca159011b23099c3d10b5b1d9aff2140e/docs/Proposals.md#phase-2---proposed-spec-text-available-cg--wg

### Champions

- Dan Gohman

### Phase 4 Advancement Criteria

WASI filesystem must have host implementations which can pass the testsuite
on at least Windows, macOS, and Linux.

WASI filesystem must have at least two complete independent implementations.

## Table of Contents [if the explainer is longer than one printed page]

- [Introduction](#introduction)
- [Goals](#goals)
- [Non-goals](#non-goals)
- [API walk-through](#api-walk-through)
  - [Use case 1](#use-case-1)
  - [Use case 2](#use-case-2)
- [Detailed design discussion](#detailed-design-discussion)
  - [[Tricky design choice 1]](#tricky-design-choice-1)
  - [[Tricky design choice 2]](#tricky-design-choice-2)
- [Considered alternatives](#considered-alternatives)
  - [[Alternative 1]](#alternative-1)
  - [[Alternative 2]](#alternative-2)
- [Stakeholder Interest & Feedback](#stakeholder-interest--feedback)
- [References & acknowledgements](#references--acknowledgements)

### Introduction

WASI filesystem is a WASI API primarily for accessing host filesystems. It
has function for opening, reading, and writing files, and for working with
directories.

Unlike many filesystem APIs, WASI filesystem is capability-oriented. Instead
of having functions that implicitly reference a filesystem namespace,
WASI filesystems' APIs are passed a directory handle along with a path, and
the path is looked up relative to the given handle, and sandboxed to be
resolved within that directory.

WASI filesystem hides some of the surface differences between Windows and
Unix-style filesystems, however much of its behavior, indluding the
semantics of path lookup, and the semantics of files, directories, and
symlinks, and the constraints on filesystem paths, is host-dependent.

WASI filesystem is not intended to be used as a virtual API for accessing
arbitary resources. Unix's "everything is a file" philosophy is in conflict
with the goals of supporting modularity and the principle of least authority.

### Goals

The primary goal of WASI filesystem is to allow users to use WASI programs to
access their existing filesystems in a straightforward and efficient manner.

### Non-goals

WASI filesystem is not aiming for deterministic semantics. That would either
require restricting it to fully controlled private filesystems, which would
conflict with the goal of giving users access to their existing filesystems,
or requiring implementations to do a lot of extra work to emulate specific
defined behaviors, which would conflict with the goal of being efficient.

### API walk-through

[Walk through of how someone would use this API.]

#### [Use case 1]

[Provide example code snippets and diagrams explaining how the API would be used to solve the given problem]

#### [Use case 2]

[etc.]

### Detailed design discussion

[This section should mostly refer to the .wit.md file that specifies the API. This section is for any discussion of the choices made in the API which don't make sense to document in the spec file itself.]

#### Should WASI filesystem be case-sensitive, case-insensitive, or platform-dependent?

Even just among popular platforms, there are case-sensitive and
case-insensitive filesystems in wide use.

It would be nice to have an API which presented consistent behavior across
platforms, so that applications don't have to worry about subtle differences,
and subtle bugs due to those differences.

However, implementing case sensitivity on a case-insensitive filesystem, or
case-insensitivity on a case-sensitive filesystem, are both tricky to do.

One issue is that case insensitivity depends on a Unicode version, so the
details can differ between different case-insensitive platforms. Another
issue is tha WASI filesystem in general can't assume it has exclusive access
to the filesystem, so approaches that involve checking for files with names
that differ only by case can race with other processes creating new files.

#### [Tricky design choice 2]

[etc.]

### Considered alternatives

[This section is not required if you already covered considered alternatives in the design discussion above.]

#### [Alternative 1]

[Describe an alternative which was considered, and why you decided against it.]

#### [Alternative 2]

[etc.]

### Stakeholder Interest & Feedback

TODO before entering Phase 3.

[This should include a list of implementers who have expressed interest in implementing the proposal]

### References & acknowledgements

Many thanks for valuable feedback and advice from:

- [Person 1]
- [Person 2]
- [etc.]
