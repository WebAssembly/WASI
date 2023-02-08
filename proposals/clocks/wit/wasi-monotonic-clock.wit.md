# WASI Monotonic Clock API

## `wasi-clocks`
```wit
/// WASI Monotonic Clock is a clock API intended to let users measure elapsed
/// time.
///
/// It is intended to be portable at least between Unix-family platforms and
/// Windows.
default interface wasi-monotonic-clock {
```

## Imports
```wit
use pkg.wasi-poll.{pollable}
```

## `instant`
```wit
/// A timestamp in nanoseconds.
type instant = u64
```

## `monotonic-clock`
```wit
/// A monotonic clock is a clock which has an unspecified initial value, and
/// successive reads of the clock will produce non-decreasing values.
///
/// It is intended for measuring elapsed time.
// TODO(resource monotonic-clock {)
type monotonic-clock = u32
```

## `now`
```wit
/// Read the current value of the clock.
///
/// The clock is monotonic, therefore calling this function repeatedly will produce
/// a sequence of non-decreasing values.
now: func(this: monotonic-clock) -> instant
```

## `resolution`
```wit
/// Query the resolution of the clock.
resolution: func(this: monotonic-clock) -> instant
```

## `subscribe`
```wit
/// Create a `pollable` which will resolve once the specified time has been reached.
subscribe: func(this: monotonic-clock, when: instant, absolute: bool) -> pollable
```

## `drop-monotonic-clock`
```wit
/// Dispose of the specified `monotonic-clock`, after which it may no longer
/// be used.
// TODO(} /* resource monotonic-clock */)
drop-monotonic-clock: func(this: monotonic-clock)
```

```wit
}
```
