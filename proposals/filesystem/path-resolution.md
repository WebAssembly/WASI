# WASI filesystem path resolution

wasi-filesystem uses a filesystem path sandboxing scheme modeled after the
system used in [CloudABI], which is also similar to the system used in
[Capsicum].

On Linux, it corresponds to the `RESOLVE_BENEATH` behavior in
[Linux's `openat2`]. In FreeBSD, it corresponds to the `O_RESOLVE_BENEATH`
behavior in [FreeBSD's `open`]. However, path resolution can also be
implemented manually using `openat` and `readlinkat` or similar primitives.

## Sandboxing overview

All functions in wasi-filesystem which operate on filesystem paths take
a pair of values: a base directory handle, and a relative path. Absolute
paths are not permitted, and there is no global namespace. All path
accesses are relative to a base directory handle.

Path resolution is constrained to occur within the sub-filesystem referenced
by the base handle. Information about the filesystem outside of the base
directory handles is not visible. In particular, it's not permitted to use
paths that temporarily step outside the sandbox with something like
"../../../stuff/here", even if the final resolved path is back inside the
sandbox, because that would leak information about the existence of
directories outside the sandbox.

Importantly, the sandboxing is designed to be implementable even in the presence
of outside processes accessing the same filesystem, including renaming,
unlinking, and creating new files and directories.

## Symlinks

Creating a symlink with an absolute path string fails with a "not permitted"
error.

Other than that, symlinks may be created with any string, provided the
underlying filesystem implementation supports it.

Sandboxing for symlink strings is performed at the time of an access, when a
path is being resolved, and not at the time that the symlink is created or
moved. This ensures that the sandbox is respected even if there are symlinks
created or renamed by other entities with access to the filesystem.

## Host Implementation

### Implementing path resolution manually

Plain `openat` doesn't perform any sandboxing; it will readily open paths
containing ".." or starting with "/", or symlinks to paths containing ".."
or starting with "/". It has an `O_NOFOLLOW` flag, however this flag only
applies to the last component of the path (eg. the "c" in "a/b/c"). So
the strategy for using `openat` to implement sandboxing is to split paths
into components (eg. "a", "b", "c") and open them one component at a time,
so that each component can be opened with `O_NOFOLLOW`.

If the `openat` call fails, and the OS error code indicates that it *was*
a symlink (eg. `ELOOP`), then call `readlinkat` to read the link contents,
split the contents into components, and prepend these new components to the
component list. If it starts with an absolute path, that's an attempt to
jump outside the sandbox, so path resolution should fail with an
"access denied" error message.

If a path component is "..", instead of opening it, pop an item off of the
component list. If the list was empty, that represents an attempt to use
".." to step outside the sandbox, so path resolution should fail with an
"access denied" error message.

### Implementation notes

On Linux, `openat2` with `RESOLVE_BENEATH` may be used as an optimization to
implement many system calls other than just "open" by utilizing Linux's
`O_PATH` and "/proc/self/fd" features.

On Windows, the [`NtCreateFile`] function can accept a directory handle and
can behave like an `openat` function, which can be used in the
[manual algorithm](implementing-path-resolution-manually).

The Rust library [cap-std] implements WASI's filesystem sandboxing semantics,
but is otherwise independent of WASI or Wasm, so it can be reused in other
settings. It uses `openat2` and `NtCreateFile` and other optimizations.

cloudabi-utils has an [implementation of the manual technique in C], though
that repository is no longer maintained.

[implementation of the manual technique in C]: https://github.com/NuxiNL/cloudabi-utils/blob/master/src/libemulator/posix.c#L1205
[cap-std]: https://github.com/bytecodealliance/cap-std
[Linux's `openat2`]: https://man7.org/linux/man-pages/man2/openat2.2.html
[CloudABI]: https://github.com/NuxiNL/cloudabi
[Capsicum]: https://wiki.freebsd.org/Capsicum
[FreeBSD's `open`]: https://man.freebsd.org/cgi/man.cgi?sektion=2&query=open
[`NtCreateFile`]: https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntcreatefile
