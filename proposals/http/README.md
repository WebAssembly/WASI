# WASI HTTP

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

This proposal currently only contains the proposed Wit interfaces with light
explanation in comments; more work is necessary to fully document the proposal.
The Wit comments annotate where the proposed interface is expected to change in
the short term (for Preview2) once resources and handles are re-added to Wit,
and then after that (for Preview3) once native stream support is added to the
Component Model and Wit.

The `wit` directory currently validates and can generate bindings with:
```
wit-bindgen c wit/ --world proxy
```
or can be manipulated in other ways with:
```
wasm-tools component wit wit/ ...
```

The `wit/deps` directory contains a live snapshot of the contents of several
other WASI proposals upon which this proposal depends. It is automatically
updated by running [`depit update`](https://crates.io/crates/depit-cli) in the
root directory, which fetches the live contents of the `main` branch of each
proposal. As things stablize, `wit/deps.toml` will be updated to refer to
versioned releases.

### Current Phase

wasi-http is currently in [Phase 1](https://github.com/WebAssembly/WASI/blob/main/Proposals.md).

### Champions

Piotr Sikora, Jiaxiao Zhou, Dan Chiarlone, David Justice

### TODO

This readme needs to be expanded to cover a number of additional fields suggested in the
[WASI Proposal template](https://github.com/WebAssembly/wasi-proposal-template).
