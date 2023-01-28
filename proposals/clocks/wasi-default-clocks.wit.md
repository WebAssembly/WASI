# WASI Default Clocks API

WASI Default Clocks provides value-exports of clock handles for monotonic
and a wall-clock time, suitable for general-purpose application needs.

## Imports
```wit
use { monotonic-clock, wall-clock } from wasi-clocks
```

## `default-monotonic-clock`
```wit
/// Return a default monotonic clock, suitable for general-purpose application
/// needs.
///
/// This allocates a new handle, so applications with frequent need of a clock
/// handle should call this function once and reuse the handle instead of
/// calling this function each time.
default-monotonic-clock: monotonic-clock
```

## `default-wall-clock`
/// Return a default wall clock, suitable for general-purpose application
/// needs.
///
/// This allocates a new handle, so applications with frequent need of a clock
/// handle should call this function once and reuse the handle instead of
/// calling this function each time.
```wit
default-wall-clock: wall-clock
```
