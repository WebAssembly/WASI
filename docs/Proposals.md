# WASI proposals

This page is under construction. The intent is to follow the CG's
[proposals page], but adapted for [WASI]. Some of the proposals predate our
adoption of this process and so don't fit exactly into the defined phases,
however our intention is to align them going forward.

[WASI]: https://github.com/WebAssembly/WASI
[proposals page]: https://github.com/WebAssembly/proposals/blob/master/README.md

## Active proposals

Proposals follow [this process document](https://github.com/WebAssembly/WASI/blob/main/docs/Process.md).

### Phase 4 - Standardize the Feature (WG)

| Proposal                                                                       | Champion                               |
| ------------------------------------------------------------------------------ | -------------------------------------- |

### Phase 3 - Implementation Phase (CG + WG)

| Proposal                                                                       | Champion                               |
| ------------------------------------------------------------------------------ | -------------------------------------- |

### Phase 2 - Proposed Spec Text Available (CG + WG)

| Proposal                                                                       | Champion                               |
| ------------------------------------------------------------------------------ | -------------------------------------- |
| [I/O][wasi-io]                                                                 | Dan Gohman                             |
| [Filesystem][wasi-filesystem]                                                  | Dan Gohman                             |
| ["Classic" Command-Line][wasi-classic-command]                                 | Dan Gohman                             |
| [Clocks][wasi-clocks]                                                          | Dan Gohman                             |
| [Random][wasi-random]                                                          | Dan Gohman                             |
| [Handle Index][wasi-handle-index]                                                   | Dan Gohman                             |
| [Poll][wasi-poll]                                                                   | Dan Gohman                             |
| [Machine Learning (wasi-nn)][wasi-nn]                                          | Andrew Brown and Mingqiu Sun           |

### Phase 1 - Feature Proposal (CG)

| Proposal                                                                       | Champion                               |
| ------------------------------------------------------------------------------ | -------------------------------------- |
| [Crypto][wasi-crypto]                                                          | Frank Denis and Daiki Ueno             |
| [HTTP][wasi-http]                                                              | Piotr Sikora                           |
| [Parallel][wasi-parallel]                                                      | Andrew Brown                           |

### Phase 0 - Pre-Proposal (CG)

| Proposal                                                                       | Champion                               |
| ------------------------------------------------------------------------------ | -------------------------------------- |
| [Data][wasi-data]                                                              | Bailey Hayes                           |
| [proxy-wasm][wasi-proxy-wasm] (will advance as multiple, smaller proposals)    | Piotr Sikora                           |

### Contributing new proposals

Please see [Contributing to WebAssembly](https://github.com/WebAssembly/WASI/blob/master/Contributing.md) for the most up-to-date information on contributing proposals to standard.

[wasi-clocks]: https://github.com/WebAssembly/wasi-clocks
[wasi-classic-command]: https://github.com/WebAssembly/wasi-classic-command
[wasi-crypto]: https://github.com/WebAssembly/wasi-crypto
[wasi-data]: https://github.com/singlestore-labs/wasi-data
[wasi-filesystem]: https://github.com/WebAssembly/wasi-filesystem
[wasi-io]: https://github.com/WebAssembly/wasi-io
[wasi-misc]: https://github.com/WebAssembly/wasi-misc
[wasi-nn]: https://github.com/WebAssembly/wasi-nn
[wasi-proxy-wasm]: https://github.com/proxy-wasm/spec
[wasi-random]: https://github.com/WebAssembly/wasi-random
[wasi-handle-index]: https://github.com/WebAssembly/wasi-handle-index
[wasi-http]: https://github.com/WebAssembly/wasi-http
[wasi-parallel]: https://github.com/WebAssembly/wasi-parallel
[wasi-poll]: https://github.com/WebAssembly/wasi-poll
