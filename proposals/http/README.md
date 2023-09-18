# WASI HTTP

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

wasi-http is currently in [Phase 1](https://github.com/WebAssembly/WASI/blob/main/Proposals.md#phase-1---feature-proposal-cg).

### Champions

* Piotr Sikora
* Jiaxiao Zhou
* Dan Chiarlone
* David Justice
* Luke Wagner

### Phase 4 Advancement Criteria

WASI-http must have at least two complete independent implementations. One
implementation must execute in a browser and may be implemented in terms of the
[Fetch API] using JavaScript. The other implementation must be implemented
in a non-browser WebAssembly runtime and demonstrate embeddability in a
Web server.

[Fetch API]: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API

### Introduction

The WASI-http proposal defines a collection of [interfaces] for sending and
receiving HTTP requests and responses. WASI-http additionally defines a
[world], `wasi:http/proxy`, that circumscribes a minimal execution environment
for wasm HTTP [proxies].

[Interfaces]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#wit-interfaces
[World]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#wit-worlds
[Proxies]: https://httpwg.org/http-core/draft-ietf-httpbis-semantics-latest.html#intermediaries

### Goals

The proposal intends to abstract over HTTP version and transport protocol
choices (such as HTTP/1.1, HTTP/2 or HTTP/3) by mapping directly to the
abstract [HTTP Semantics], allowing hosts to (mostly) transparently use any of
these.

The `wasi:http/proxy` world is meant to be implementable by a wide variety of
hosts including Web [service workers], forward- and reverse-[proxies] and
[origin servers] by requiring a minimal set of additional runtime support.

The `wasi:http/proxy` world is meant to support flexible auto-scaling
("serverless") execution by moving the core `accept()` loop into the host and
allowing the host to dynamically spin up wasm instances in response to arriving
requests.

The `wasi:http/proxy` world is meant to allow the chaining of HTTP
intermediaries to be implemented directly in terms of [Component Model] linking.
(Fully realizing this goal will require additional features only available in
the [Preview 3] timeframe.)

[HTTP Semantics]: https://httpwg.org/http-core/draft-ietf-httpbis-semantics-latest.html
[Service Workers]: https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API
[Origin Servers]: https://httpwg.org/http-core/draft-ietf-httpbis-semantics-latest.html#origin.server
[Component Model]: https://github.com/WebAssembly/component-model/
[Preview 3]: https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#streams

### Non-goals

WASI-http does not intend to define a more fully-featured cloud execution
environment (for this, see the [wasi-cloud-core] proposal).

[wasi-cloud-core]: https://github.com/WebAssembly/wasi-cloud-core

### API walk-through

The proposal can be understood by first reading the comments of [`proxy.wit`],
then [`handler.wit`] and finally [`types.wit`].

[`proxy.wit`]: ./wit/proxy.wit
[`handler.wit`]: ./wit/handler.wit
[`types.wit`]: ./wit/types.wit

### Working with the WIT

Bindings can be generated from the `wit` directory via:
```
wit-bindgen c wit/ --world proxy
```
and can be validated and otherwise manipulated via:
```
wasm-tools component wit wit/ ...
```

The `wit/deps` directory contains a live snapshot of the contents of several
other WASI proposals upon which this proposal depends. It is automatically
updated by running [`wit-deps update`](https://crates.io/crates/wit-deps-cli)
in the root directory, which fetches the live contents of the `main` branch of
each proposal. As things stablize, `wit/deps.toml` will be updated to refer to
versioned releases.

### References & acknowledgements

* This proposal was seeded by and developed in consultation with
  [proxy-wasm](https://github.com/proxy-wasm/spec).

