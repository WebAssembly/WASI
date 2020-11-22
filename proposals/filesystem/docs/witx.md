# Know your `witx`

The `witx` file format is an experimental format which is based on the
[module linking] text format (`wit`), (which is in turn based on the
[wat format], which is based on [S-expressions]). It adds some features
using the same syntax as [interface types], some features with syntax
similar to [gc types], as well as a few special features of its own.

`witx` is actively evolving. Expect backwards-incompatible changes,
particularly in the areas where `witx` differs from `wit`.

The initial goal for `witx` is just to have a language suitable for
expressing [WASI] APIs in, to serve as the vocabulary for proposing changes
to existing APIs and proposing new APIs. Initially, while it uses some of
the syntax and concepts from interface types, it doesn't currently imply the
full interface types specification, or the use of the interface types custom
sections.

We expect that eventually we will transition to using the full interface
types specification, with `witx` having minimal additional features. Until then,
the goals here are to remain aligned with interface types and other relevant
WebAssembly standards and proposals wherever practical, and to be an input 
into the design process of interface types.

[module linking]: https://github.com/WebAssembly/module-linking/blob/master/proposals/module-linking/Explainer.md
[interface types]: https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md
[gc types]: https://github.com/WebAssembly/gc
[wat format]: https://webassembly.github.io/spec/core/bikeshed/index.html#text-format%E2%91%A0
[S-expressions]: https://en.wikipedia.org/wiki/S-expression
[WASI]: https://github.com/WebAssembly/WASI
