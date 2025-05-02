# WASI proposals

WASI APIs are developed as proposals. These proposals go through 5 phases of development (following [the WASI Subgroup's Phase Process]).

You can learn more about contributing new proposals (and other ways to contribute) in our [Contributing] guide.

[the WASI Subgroup's Phase Process]: https://github.com/WebAssembly/WASI/blob/main/Contributing.md#the-phase-process
[Contributing]: https://github.com/WebAssembly/WASI/blob/main/Contributing.md

## Active Proposals

### Phase 5 - The Feature is Standardized (WG)

| Proposal | Champion | Versions |
| -------- | -------- | -------- |

### Phase 4 - Standardize the Feature (WG)

| Proposal | Champion | Versions |
| -------- | -------- | -------- |

### Phase 3 - Implementation Phase (CG + WG)

| Proposal                      | Champion                                                              | Versions |
| ----------------------------- | --------------------------------------------------------------------- | -------- |
| [I/O][wasi-io]                | Dan Gohman                                                            |          |
| [Clocks][wasi-clocks]         | Dan Gohman                                                            |          |
| [Random][wasi-random]         | Dan Gohman                                                            |          |
| [Filesystem][wasi-filesystem] | Dan Gohman                                                            |          |
| [Sockets][wasi-sockets]       | Dave Bakker                                                           |          |
| [CLI][wasi-cli]               | Dan Gohman                                                            |          |
| [HTTP][wasi-http]             | Piotr Sikora, Jiaxiao Zhou, Dan Chiarlone, David Justice, Luke Wagner |          |

### Phase 2 - Proposed Spec Text Available (CG + WG)

| Proposal                                           | Champion                                                              | Versions |
| -------------------------------------------------- | --------------------------------------------------------------------- | -------- |
| [Clocks: Timezone][wasi-clocks]                    | Dan Gohman                                                            |          |
| [CLI: Exit With Code][wasi-cli]                    | Dan Gohman                                                            |          |
| [HTTP: Informational Outbound Response][wasi-http] | Piotr Sikora, Jiaxiao Zhou, Dan Chiarlone, David Justice, Luke Wagner |          |
| [I2C][wasi-i2c]                                    | Friedrich Vandenberghe, Merlijn Sebrechts, Maximilian Seidler         |          |
| [Key-value Store][wasi-kv-store]                   | Jiaxiao Zhou, Dan Chiarlone, David Justice                            |          |
| [Machine Learning (wasi-nn)][wasi-nn]              | Andrew Brown and Mingqiu Sun                                          |          |
| [Runtime Config][wasi-runtime-config]              | Jiaxiao Zhou, Dan Chiarlone, David Justice                            |          |
| [GFX][wasi-gfx]                                    | Mendy Berger, Sean Isom                                               |          |
| [Messaging][wasi-messaging]                        | Jiaxiao Zhou, Dan Chiarlone, David Justice, Taylor Thomas             |          |

### Phase 1 - Feature Proposal (CG)

| Proposal                                                  | Champion                                         | Versions |
| --------------------------------------------------------- | ------------------------------------------------ | -------- |
| [Blob Store][wasi-blob-store]                             | Jiaxiao Zhou, Dan Chiarlone, David Justice       |          |
| [Crypto][wasi-crypto]                                     | Frank Denis and Daiki Ueno                       |          |
| [GPIO][wasi-gpio]                                         | Merlijn Sebrechts, Maximilian Seidler            |          |
| [Distributed Lock Service][wasi-distributed-lock-service] | Jiaxiao Zhou, Dan Chiarlone, David Justice       |          |
| [Logging][wasi-logging]                                   | Dan Gohman                                       |          |
| [Observe][wasi-observe]                                   | Caleb Schoepp                                    |          |
| [Parallel][wasi-parallel]                                 | Andrew Brown                                     |          |
| [Pattern Match][wasi-pattern-match]                       | Jianjun Zhu                                      |          |
| [SPI][wasi-spi]                                           | Merlijn Sebrechts                                |          |
| [SQL][wasi-sql]                                           | Jiaxiao Zhou, Dan Chiarlone, David Justice       |          |
| [SQL Embed][wasi-sql-embed]                               | Robin Brown                                      |          |
| [Threads][wasi-threads]                                   | Alexandru Ene, Marcin Kolny, Andrew Brown        |          |
| [TLS][wasi-tls]                                           | Joel Dice, Dave Bakker, James Sturtevant         |          |
| [URL][wasi-url]                                           | Radu Matei                                       |          |
| [USB][wasi-usb]                                           | Wouter Hennen, Warre Dujardin, Merlijn Sebrechts |          |

### Phase 0 - Pre-Proposal (CG)

**Note:** The pre-proposal phase is simply meant as a way to share ideas. This means that there may be overlap between pre-proposals. It also means that the WASI subgroup has not yet decided that the pre-proposal is in scope for WASI.

| Proposal                                                                         | Champion      | Versions |
| -------------------------------------------------------------------------------- | ------------- | -------- |
| [proxy-wasm/spec][wasi-proxy-wasm] (will advance as multiple, smaller proposals) | Piotr Sikora  |          |
| [OTel][wasi-otel]                                                                | Caleb Schoepp |          |

## Versioning

Once a proposal reaches Phase 3, we expect the champions to start creating releases, following the conventions of semantic versioning (semver). Releases for active proposals are linked in the chart above.

Proposals remain in the 0.x semver range until they reach Phase 5 and are fully standardized. At that point, a 1.0 release should be made available.

For some APIs, it makes sense to add new features after the API itself has reached Phase 5. These feature additions should go through the same standardization process. Once they have reached Phase 5, the minor version number of the release should be incremented.

Some APIs may require backwards-incompatible changes over time. In these cases, we allow proposals to increment the major version number _only if_ the old API can be implemented in terms of the new API. As part of the new version, champions are expected to provide a tool that enables this backwards-compatibility. If that is not possible, then a new API proposal with a new name should be started. The original API can then be deprecated over time if it makes sense to do so.

[WebAssembly CG Phases process]: https://github.com/WebAssembly/meetings/blob/master/process/phases.md
[witx]: https://github.com/WebAssembly/WASI/blob/main/tools/witx-docs.md
[ephemeral/snapshot/old process]: https://github.com/WebAssembly/WASI/blob/master/phases/README.md

[wasi-blob-store]: https://github.com/WebAssembly/wasi-blob-store
[wasi-clocks]: https://github.com/WebAssembly/wasi-clocks
[wasi-crypto]: https://github.com/WebAssembly/wasi-crypto
[wasi-gpio]: https://github.com/WebAssembly/wasi-gpio
[wasi-distributed-lock-service]: https://github.com/WebAssembly/wasi-distributed-lock-service
[wasi-filesystem]: https://github.com/WebAssembly/wasi-filesystem
[wasi-http]: https://github.com/WebAssembly/wasi-http
[wasi-i2c]: https://github.com/WebAssembly/wasi-i2c
[wasi-io]: https://github.com/WebAssembly/wasi-io
[wasi-kv-store]: https://github.com/WebAssembly/wasi-kv-store
[wasi-logging]: https://github.com/WebAssembly/wasi-logging
[wasi-messaging]: https://github.com/WebAssembly/wasi-messaging
[wasi-nn]: https://github.com/WebAssembly/wasi-nn
[wasi-observe]: https://github.com/dylibso/wasi-observe
[wasi-otel]: https://github.com/calebschoepp/wasi-otel
[wasi-parallel]: https://github.com/WebAssembly/wasi-parallel
[wasi-pattern-match]: https://github.com/WebAssembly/wasi-pattern-match
[wasi-proxy-wasm]: https://github.com/proxy-wasm/spec
[wasi-random]: https://github.com/WebAssembly/wasi-random
[wasi-runtime-config]: https://github.com/WebAssembly/wasi-runtime-config
[wasi-sockets]: https://github.com/WebAssembly/wasi-sockets
[wasi-spi]: https://github.com/WebAssembly/wasi-spi
[wasi-sql]: https://github.com/WebAssembly/wasi-sql
[wasi-sql-embed]: https://github.com/WebAssembly/wasi-sql-embed
[wasi-threads]: https://github.com/WebAssembly/wasi-native-threads
[wasi-tls]: https://github.com/WebAssembly/wasi-tls
[wasi-url]: https://github.com/WebAssembly/wasi-url
[wasi-usb]: https://github.com/WebAssembly/wasi-usb
[wasi-gfx]: https://github.com/WebAssembly/wasi-gfx
[wasi-cli]: https://github.com/WebAssembly/wasi-cli
