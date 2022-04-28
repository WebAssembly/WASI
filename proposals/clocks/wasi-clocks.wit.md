# WASI Clocks API

WASI Clocks is a clock API intended to let users query the current time and
to measure elapsed time.

It is intended to be portable at least between Unix-family platforms and
Windows.

## `instant`
```wit
/// A timestamp in nanoseconds.
type instant = u64
```

## `datetime`
```wit
/// A time and date in seconds plus nanoseconds since 1970-01-01T00:00:00Z.
record datetime {
    seconds: u64,
    nanoseconds: u32,
}
```

## `monotonic_clock`
```wit
/// A monotonic clock is a clock which has an unspecified initial value, and
/// successive reads of the clock will produce non-decreasing values.
///
/// It is intended for measuring elapsed time.
resource monotonic_clock {
```

## `now`
/// Read the current value of the clock.
///
/// As this the clock is monotonic, calling this function repeatedly will produce
/// a sequence of non-decreasing values.
```wit
now: function() -> instant
```

## `resolution`
/// Query the resolution of the clock.
```wit
resolution: function() -> instant
```

```wit
}
```

## `wall_clock`
```wit
/// A wall clock is a clock which measures the date and time according to some
/// external reference.
///
/// External references may be reset, so this clock is not necessarily
/// monotonic, making it unsuitable for measuring elapsed time.
///
/// It is intended for reporting the current date and time for humans.
resource wall_clock {
```

## `now`
/// Read the current value of the clock.
///
/// As this the clock is not monotonic, calling this function repeatedly will
/// not necessarily produce a sequence of non-decreasing values.
```wit
now: function() -> datetime
```

## `resolution`
/// Query the resolution of the clock.
```wit
resolution: function() -> datetime
```

```wit
}
```
