# Component Model features in WASI

WASI APIs are defined in terms of the [Component Model], which develops new WIT
syntax, types, and canonical ABI functionality behind [gated features]. Stable
(`@since`-gated) WASI APIs may only depend on a gated feature after the WASI
Subgroup has voted to adopt it, following the process described in
[CONTRIBUTING.md](../CONTRIBUTING.md#adopting-component-model-features).

This document records which features have been adopted, and the release they
were adopted for. Adoption is cumulative: implementing a given WASI version
requires the default (ungated) Component Model features, plus every feature
adopted in that version and in all earlier ones.

Adopting a feature does not mean that any WASI API uses it. It means that WASI
APIs in that release and later *may* use it, and that runtimes and toolchains
which implement that WASI version must implement the feature regardless of
whether it is used yet.

## Adopted features

This table is the source for the "Component Model features" section of each
released [specification overview](../specifications).

| WASI version | Gate | Feature | Adopted |
| --- | --- | --- | --- |
| 0.2.0 | — | The default (ungated) Component Model features | WASI 0.2 vote (predates this process) |
| 0.3.0 | 🔀 | `async` lift/lower, `future` and `stream` | WASI 0.3 vote (predates this process) |
| 0.3.1 | 🗺️ | The `map<K, V>` type | [2026-08-06](https://github.com/WebAssembly/meetings/blob/main/wasi/2026/WASI-08-06.md) ([#943](https://github.com/WebAssembly/WASI/issues/943)) |
| 0.3.1 | 🏷️ | `implements` and `external-id` annotations on plain-named interface imports and exports | [2026-08-06](https://github.com/WebAssembly/meetings/blob/main/wasi/2026/WASI-08-06.md) ([#942](https://github.com/WebAssembly/WASI/issues/942)) |

Features which have not been adopted are listed under [gated features] in the
Component Model Explainer. WASI proposals may experiment with them in
prerelease versions (such as `0.3.0-rc-*` releases), but stable releases must
remain implementable without them.

CI enforces this: each changed proposal is encoded to a component and validated
with the features in the table above enabled, so a proposal that depends on an
un-adopted feature fails validation.

[Component Model]: https://github.com/WebAssembly/component-model
[gated features]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md#gated-features
