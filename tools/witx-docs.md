# Know your `witx`

`Witx` is an experimental IDL. The text format is based on a text format
associated with an early version of the [module linking proposal] text
format, which at the time was called `wit`, though it is a different language
than what we now call [Wit]. And that `wit` was in turn based on the
[wat format], which is based on [S-expressions].

Witx adds some features inspired by [interface types], such as limited
`string` and `array` arguments, and some features using for working with
IDL files such as the ability to include one IDL file in another.

The initial goal for `witx` was just to have a language suitable for
expressing [WASI] APIs in, to serve as the vocabulary for proposing changes
to existing APIs and proposing new APIs.

The WASI Subgroup is [migrating] away from `witx` and toward [Wit], because
Wit provides much better support for non-C-like languages, better support for
API virtualization, it has a path to integrating async behavior into WASI
APIs in a comprehensive way, and it supports much more expressive APIs, such
as the ability to have `string`s and other types as return types in addition
to just argument types. At this point, the tooling for Wit is also a lot more
sophisticated and the [Wit language] and [Canonical ABI] have much more
documentation.

This document focused on the witx format.

## Return types

Function declarations in witx can have a special `expected` type, which is
a variant which represents either success or failure, and can return a
specific type off value for each.

For example, the `fd_read` function declaration in Preview1 contains this:

```witx
    (result $error (expected $size (error $errno)))
```

This declares a result named `$error` which returns a value with type
`$size` on success, and a value with type `$errno` on failure.

The `expected` mechanism assumes that the error value is an enum where 0
indicates success, and as such it doesn't return an explicit descriminant
value. In the ABI, the `error` type is returned as the return value and
the success value is handled by adding an argument of pointer type for
the function to store the result into.

The resulting ABI for `fd_read` looks like this:

```c
__wasi_errno_t __wasi_fd_read(
    __wasi_fd_t fd,
    const __wasi_iovec_t *iovs,
    size_t iovs_len,
    __wasi_size_t *retptr0
);
```

## Pointers

Witx supports two pointer types, `pointer` and `const_pointer`, which represent
pointers into the exported linear memory named "memory". `const_pointer` in a
function declaration documents that the function does not use the pointer for
mutating memory. Similar to C, they can point to either a single value or an
contiguous array of values.

Pointer arguments use the `@witx` syntax inspired by the [annotations proposal].

For example, the `poll_oneoff` function has these arguments:

```witx
   (param $in (@witx const_pointer $subscription))
   (param $out (@witx pointer $event))
```

Pointer values are expected to be aligned, to the alignment of their pointee
type. If a misaligned pointer is passed to a function, the function shall trap.

If an out-of-bounds pointer is passed to a function and the function needs
to dereference it, the function shall trap rather than returning
`errno.fault`.

[module linking proposal]: https://github.com/WebAssembly/module-linking/
[interface types]: https://github.com/WebAssembly/interface-types/blob/main/proposals/interface-types/Explainer.md
[wat format]: https://webassembly.github.io/spec/core/bikeshed/index.html#text-format%E2%91%A0
[S-expressions]: https://en.wikipedia.org/wiki/S-expression
[WASI]: https://github.com/WebAssembly/WASI
[Wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[Wit language]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[Canonical ABI]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md
[migrating]: https://github.com/WebAssembly/wasi#important-note-wasi-is-in-transition
[annotations proposal]: https://github.com/WebAssembly/annotations
