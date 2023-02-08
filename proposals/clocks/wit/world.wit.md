This file contains a world that imports all interfaces in this proposal. Its
primary purpose is to allow unified documentation to be easily generated for
the whole proposal.

```wit
default world wasi-clocks {
    import wasi-monotonic-clock: pkg.wasi-monotonic-clock
    import wasi-wall-clock: pkg.wasi-wall-clock
    import wasi-timezone: pkg.timezone
    import wasi-default-clocks: pkg.wasi-default-clocks
}
```
