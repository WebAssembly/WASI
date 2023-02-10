# WASI HTTP

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

This proposal currently only contains the proposed Wit interfaces with light
explanation in comments; more work is necessary to fully document the proposal.
The Wit comments annotate where the proposed interface is expected to change in
the short term (for Preview2) once resources and handles are re-added to Wit,
and then after that (for Preview2) once native stream support is added to the
Component Model and Wit.

The `wit` directory currently validates and can generate bindings with:
```
wit-bindgen c wit/ --world proxy
```
or can be manipulated in other ways with:
```
wasm-tools component wit wit/ ...
```

The HTTP proposal depends on the WASI IO and Logging proposals. For simplicity,
the Wit files for these proposals are currently copied into the `wit/deps`
directory and will be updated periodically to match their respective proposals.
As the Wit tooling develops, we should be able to avoid this form of manual
vendoring.

### Current Phase

wasi-http is currently in [Phase 1](https://github.com/WebAssembly/WASI/blob/main/Proposals.md).

### Champions

Piotr Sikora, Jiaxiao Zhou, Dan Chiarlone, David Justice

### TODO

This readme needs to be expanded to cover a number of additional fields suggested in the
[WASI Proposal template](https://github.com/WebAssembly/wasi-proposal-template).
