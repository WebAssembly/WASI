# WASI Clocks

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

WASI-clocks is currently in [Phase 2].

[Phase 2]: https://github.com/WebAssembly/WASI/blob/42fe2a3ca159011b23099c3d10b5b1d9aff2140e/docs/Proposals.md#phase-2---proposed-spec-text-available-cg--wg

### Champions

- Dan Gohman

### Phase 4 Advancement Criteria

WASI clocks must have host implementations which can pass the testsuite
on at least Windows, macOS, and Linux.

WASI clocks must have at least two complete independent implementations.

## Table of Contents [if the explainer is longer than one printed page]

- [Introduction](#introduction)
- [Goals](#goals)
- [Non-goals](#non-goals)
- [API walk-through](#api-walk-through)
  - [Use case 1](#use-case-1)
  - [Use case 2](#use-case-2)
- [Detailed design discussion](#detailed-design-discussion)
  - [[Tricky design choice 1]](#tricky-design-choice-1)
  - [[Tricky design choice 2]](#tricky-design-choice-2)
- [Considered alternatives](#considered-alternatives)
  - [[Alternative 1]](#alternative-1)
  - [[Alternative 2]](#alternative-2)
- [Stakeholder Interest & Feedback](#stakeholder-interest--feedback)
- [References & acknowledgements](#references--acknowledgements)

### Introduction

WASI Clocks is a WASI API for reading the current time and measuring elapsed
time.

Unlike many clock APIs, WASI Clocks is capability-oriented. Instead
of having functions that implicitly reference a clock, WASI clocks' APIs are
passed a clock handle.

### Goals

The primary goal of WASI Clocks is to allow users to use WASI programs to
read the current time and to measure elapsed time.

### Non-goals

WASI Clocks is not aiming to cover timezones, daylight savings time, date
formatting, or modifying the time of a clock.

### API walk-through

[Walk through of how someone would use this API.]

#### [Use case 1]

[Provide example code snippets and diagrams explaining how the API would be used to solve the given problem]

#### [Use case 2]

[etc.]

### Detailed design discussion

[This section should mostly refer to the .wit.md file that specifies the API. This section is for any discussion of the choices made in the API which don't make sense to document in the spec file itself.]

### What should the type of a timestamp be?

In POSIX, `clock_gettime` uses a single `timespec` type to represent timestamps
from all clocks, with two fields: seconds and nanoseconds. However, in applications
that just need to measure elapsed time, and don't need to care about wall clock
time, working with seconds and nanoseconds as separate fields adds extra code size
and complexity. For these use cases, a single 64-bit nanoseconds value, which can
measure up to about 584 years, is sufficient and simpler.

For wall clock time, it's still useful to have both seconds and nanoseconds, both
to be able to represent dates in the far future, and to reflect the fact that
code working with wall clock time will often want to treat seconds and fractions
of seconds differently.

And so, this API uses different data types for different types of clocks.

### Considered alternatives

[This section is not required if you already covered considered alternatives in the design discussion above.]

#### [Alternative 1]

[Describe an alternative which was considered, and why you decided against it.]

#### [Alternative 2]

[etc.]

### Stakeholder Interest & Feedback

TODO before entering Phase 3.

[This should include a list of implementers who have expressed interest in implementing the proposal]

### References & acknowledgements

Many thanks for valuable feedback and advice from:

- [Person 1]
- [Person 2]
- [etc.]
