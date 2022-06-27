# WASI Default Clocks API

WASI Default Clocks provides value-exports of clock handles for monotonic
and a wall-clock time, suitable for general-purpose application needs.

## Imports
```wit
use { monotonic-clock, wall-clock } from wasi-clocks
```

## `default-monotonic-clock`
```wit
default-monotonic-clock: monotonic-clock
```

## `default-wall-clock`
```wit
default-wall-clock: wall-clock
```
