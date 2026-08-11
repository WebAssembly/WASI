# WASI Specification v0.3.0

This is version 0.3.0 of the WASI specification, based on the [WebAssembly
Component Model][cm].

## Proposals

The [WebAssembly Interface Type (WIT)][wit] definitions for the proposals included in this
version of the specification are pushed to OCI based on the [Wasm OCI Artifact
layout][wasm-oci]:

- [wasi:random@0.3.0](https://github.com/WebAssembly/WASI/pkgs/container/wasi%2Frandom?tag=0.3.0)
- [wasi:clocks@0.3.0](https://github.com/WebAssembly/WASI/pkgs/container/wasi%2Fclocks?tag=0.3.0)
- [wasi:sockets@0.3.0](https://github.com/WebAssembly/WASI/pkgs/container/wasi%2Fsockets?tag=0.3.0)
- [wasi:filesystem@0.3.0](https://github.com/WebAssembly/WASI/pkgs/container/wasi%2Ffilesystem?tag=0.3.0)
- [wasi:cli@0.3.0](https://github.com/WebAssembly/WASI/pkgs/container/wasi%2Fcli?tag=0.3.0)
- [wasi:http@0.3.0](https://github.com/WebAssembly/WASI/pkgs/container/wasi%2Fhttp?tag=0.3.0)

## Component Model features

Implementing this version of the specification requires the default (ungated)
[Component Model][cm] features, plus the [gated features][gates] adopted by the
WASI Subgroup up to and including this version:

| Gate | Feature | Adopted in |
| --- | --- | --- |
| 🔀 | `async` lift/lower, `future` and `stream` | 0.3.0 |

These are required whether or not an API in this version uses them. See
[Component Model features in WASI][features] for the full adoption record.

[cm]: https://github.com/WebAssembly/component-model
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[wasm-oci]: https://tag-runtime.cncf.io/wgs/wasm/deliverables/wasm-oci-artifact
[gates]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md#gated-features
[features]: https://github.com/WebAssembly/WASI/blob/main/docs/ComponentModelFeatures.md
