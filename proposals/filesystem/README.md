# WASI filesystem

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

WASI-filesystem is currently in [Phase 2].

[Phase 2]: https://github.com/WebAssembly/WASI/blob/42fe2a3ca159011b23099c3d10b5b1d9aff2140e/docs/Proposals.md#phase-2---proposed-spec-text-available-cg--wg

### Champions

- Dan Gohman

### Portability Criteria

WASI filesystem must have host implementations which can pass the testsuite
on at least Windows, macOS, and Linux.

WASI filesystem must have at least two complete independent implementations.

## Table of Contents

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
resolved within that directory. For more information about sandbox, see
[WASI filesystem path resolution](path-resolution.md).

WASI filesystem hides some of the surface differences between Windows and
Unix-style filesystems, however much of its behavior, including the
semantics of path lookup, and the semantics of files, directories, and
symlinks, and the constraints on filesystem paths, is host-dependent.

WASI filesystem is not intended to be used as a virtual API for accessing
arbitary resources. Unix's "everything is a file" philosophy is in conflict
with the goals of supporting modularity and the principle of least authority.

Many of the ideas related to doing capability-based filesystem sandboxing with
`openat` come from [CloudABI](https://github.com/NuxiNL/cloudabi) and
[Capsicum](https://wiki.freebsd.org/Capsicum).

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

#### Opening a file

```rust
/// Write "Hello, World" into a file called "greeting.txt" in `dir`.
fn write_hello_world_to_a_file(dir: Descriptor) -> Result<(), Errno> {
    let at_flags = AtFlags::FollowSymlinks;
    let o_flags = OFlags::Create | OFlags::Trunc;
    let descriptor_flags = DescriptorFlags::Write;
    let mode = Mode::Readable;
    let file =
        dir.openat(at_flags, "greeting.txt", o_flags, descriptor_flags, mode)?;
    let message = b"Hello, World\n";
    let mut view = &message[..];
    let mut offset = 0;
    while !view.is_empty() {
        let num_written = file.pwrite(view.to_owned(), 0)?;
        offset += num_written;
        view = &view[num_writen..];
    }
    // The file descriptor is closed when it's dropped!
}
```

Perhaps the biggest change from the preview1 version of openat, called
`path_open`, is the removal of the *rights* flags. Preview1 associates
a set of flags with every file descriptor enumerating which operations
may be performed on it, such as reading, writing, appending, truncating,
and many other operations. In practice, this created a lot of ambiguity
about how it mapped to POSIX semantics, as it doesn't directly correspond
to any feature in POSIX, or in Windows either.

The other major change from preview1 is the introduction of the mode
argument, which controls the permissions of the generated file. There
was no way to control permissions in preview1, so this is new
functionality.

#### Streaming read from a file

TODO

#### Reading from a directory

fn read_entries(dir: Descriptor) -> Result<(), Errno> {
    // TODO: Implement this example.
}

[etc.

### Detailed design discussion

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

### Considered alternatives

#### Fully deterministic filesystem

The main tradeoff with full determinism is that it makes it difficult to access existing filesystems that the Wasm runtime doesn't have full control over. This proposal is aiming to address use cases where users have existing filesystems they want to access.

### Stakeholder Interest & Feedback

TODO before entering Phase 3.

Preview1 has a similar filesystem API, and it's widely exposed in toolchains.

### References & acknowledgements

Many thanks for valuable feedback and advice from:

- [Person 1]
- [Person 2]
- [etc.]
