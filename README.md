# WASI 0.2 docs

This branch documents the WASI 0.2 iteration of WASI, also known as Preview 2.

WASI 0.2 is a modular collection of APIs defined using the [Wit IDL]. It
incorporates many of the lessons learned from [Preview 1], including support
for a wider range of source languages, modularity, a more expressive type
system, virtualizability, and more. It covers the `wasi:cli`, `wasi:clocks`,
`wasi:filesystem`, `wasi:http`, `wasi:io`, `wasi:random`, and `wasi:sockets`
interfaces at their 0.2.x versions.

Development of WASI now continues in [WASI 0.3] (Preview 3) on the `main`
branch, which supplants Preview 2 by providing integrated and composable
`async` functionality. Implementations may continue to support WASI 0.2,
either by implementing WASI 0.3 alongside it, or by virtualizing
(polyfilling) 0.2 in terms of 0.3.

The proposals included in WASI 0.2 are documented under [`proposals/`](proposals),
and the tagged interface snapshots are under [`specifications/`](specifications).
See [Preview2.md](docs/Preview2.md) for the Preview 2 inclusion criteria and
overview.

[Preview 1]: https://github.com/WebAssembly/WASI/tree/wasi-0.1
[WASI 0.3]: https://github.com/WebAssembly/WASI/tree/main
[Wit IDL]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
