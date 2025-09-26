# WASI Clocks

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

WASI-clocks is currently in [Phase 3].

[Phase 3]: https://github.com/WebAssembly/WASI/blob/main/Proposals.md#phase-3---implementation-phase-cg--wg

### Champions

- Dan Gohman

### Portability Criteria

WASI clocks must have host implementations which can pass the testsuite
on at least Windows, macOS, and Linux.

WASI clocks must have at least two complete independent implementations.

## Table of Contents

- [WASI Clocks](#wasi-clocks)
    - [Current Phase](#current-phase)
    - [Champions](#champions)
    - [Portability Criteria](#portability-criteria)
  - [Table of Contents](#table-of-contents)
    - [Introduction](#introduction)
    - [Goals](#goals)
    - [Non-goals](#non-goals)
    - [API walk-through](#api-walk-through)
      - [Measuring elapsed time](#measuring-elapsed-time)
      - [Telling the current human time:](#telling-the-current-human-time)
      - [Retrieving the timezone:](#retrieving-the-timezone)
    - [Detailed design discussion](#detailed-design-discussion)
    - [What should the type of a timestamp be?](#what-should-the-type-of-a-timestamp-be)
    - [Considered alternatives](#considered-alternatives)
      - [Per-process and per-thread clocks](#per-process-and-per-thread-clocks)
    - [Stakeholder Interest \& Feedback](#stakeholder-interest--feedback)
    - [References \& acknowledgements](#references--acknowledgements)
    - [Development](#development)
      - [Generating imports.md](#generating-importsmd)

### Introduction

WASI Clocks is a WASI API for reading the current time and measuring elapsed
time.

Unlike many clock APIs, WASI Clocks is capability-oriented. Instead
of having functions that implicitly reference a clock, WASI Clocks' APIs are
passed a clock handle.

### Goals

The primary goal of WASI Clocks is to allow users to use WASI programs to
read the current time and to measure elapsed time.

### Non-goals

WASI Clocks is not aiming to cover date formatting, or modifying the time of a clock.

### API walk-through

#### Measuring elapsed time

The monotonic clock APIs can be used to measure the elapsed time of a region of code:

```wit
default-monotonic-clock: monotonic-clock
```

```rust
   let start: Mark = monotonic_clock::now(clock);

   // some stuff

   let stop: Mark = monotonic_clock::now(clock);

   let elapsed: Duration = stop - start;
```


#### Telling the current human time:

```rust
    let the_current_time = system_clock::now();

    println!("it has been {} seconds and {} nanoseconds since the Unix epoch!", the_current_time.seconds, the_current_time.nanoseconds);
```

#### Retrieving the timezone:

```rust
    let instant: Instant = system_clock::now();
    let id = timezone::id();
    let offset_h = timezone::utc_offset(instant) as f64 / 3600e9;
    println!("the timezone is {} at UTC{:+}", id, offset_h);
```

### Detailed design discussion

### What should the type of a timestamp be?

In POSIX, `clock_gettime` uses a single `timespec` type to represent timestamps
from all clocks, with two fields: seconds and nanoseconds. However, in applications
that just need to measure elapsed time, and don't need to care about absolute
time, working with seconds and nanoseconds as separate fields adds extra code size
and complexity. For these use cases, a single 64-bit nanoseconds value, which can
measure up to about 584 years, is sufficient and simpler.

For system time, it's still useful to have both seconds and nanoseconds, both
to be able to represent dates in the far future, and to reflect the fact that
code working with system time will often want to treat seconds and fractions
of seconds differently.

And so, this API uses different data types for different types of clocks.

### Considered alternatives

#### Per-process and per-thread clocks

WASI preview1 included two clocks which measured the CPU time of the current process and the current thread, respectively. These clocks are difficult to implement efficiently in WASI implementations that have multiple wasm instances in the same host process, so they've been omitted from this API.

Wasi-libc has support for emulating these clocks, by using the monotonic clock instead, which isn't a technically precise replacement, but is enough to ensure minimal compatibility with existing code.

### Stakeholder Interest & Feedback

TODO before entering Phase 3.

Preview1 has monotonic and wall clock functions, and they're widely exposed in toolchains.

### References & acknowledgements

Many thanks for valuable feedback and advice from:

- [Person 1]
- [Person 2]
- [etc.]

### Development

#### Generating imports.md

The file `imports.md` is generated using [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen).

```bash
wit-bindgen markdown wit --html-in-md --features clocks-timezone
```
