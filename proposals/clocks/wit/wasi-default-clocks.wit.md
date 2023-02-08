# WASI Default Clocks API

## `wasi-default-clocks`
```wit
/// WASI Default Clocks provides value-exports of clock handles for monotonic
/// and a wall-clock time, suitable for general-purpose application needs.
default interface wasi-default-clocks {
```

## Imports
```wit
use pkg.wasi-monotonic-clock.{monotonic-clock}
use pkg.wasi-wall-clock.{wall-clock}
```

## `default-monotonic-clock`
```wit
/// Return a default monotonic clock, suitable for general-purpose application
/// needs.
///
/// This allocates a new handle, so applications with frequent need of a clock
/// handle should call this function once and reuse the handle instead of
/// calling this function each time.
default-monotonic-clock: func() -> monotonic-clock
```

## `default-wall-clock`
```wit
/// Return a default wall clock, suitable for general-purpose application
/// needs.
///
/// This allocates a new handle, so applications with frequent need of a clock
/// handle should call this function once and reuse the handle instead of
/// calling this function each time.
default-wall-clock: func() -> wall-clock
```

```wit
}
```
