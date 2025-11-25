# WASI CLI World

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

wasi-cli is currently in [Phase 3].

[Phase 3]:  https://github.com/WebAssembly/WASI/blob/main/Proposals.md#phase-3---implementation-phase-cg--wg

### Champions

- Dan Gohman

### Portability Criteria

WASI CLI must have host implementations which can pass the testsuite
on at least Windows, macOS, and Linux.

WASI CLI must have at least two complete independent implementations.

## Table of Contents

- [Introduction](#introduction)
- [Goals [or Motivating Use Cases, or Scenarios]](#goals-or-motivating-use-cases-or-scenarios)
- [Non-goals](#non-goals)
- [API walk-through](#api-walk-through)
  - [Use case 1](#use-case-1)
  - [Use case 2](#use-case-2)
- [Detailed design discussion](#detailed-design-discussion)
  - [Should stdout be an `output-stream`?](#should-stdout-be-an-output-stream)
  - [Should stderr be an `output-stream`?](#should-stderr-be-an-output-stream)
  - [Should environment variables be arguments to `command`?](#should-environment-variables-be-arguments-to-command)
- [Stakeholder Interest & Feedback](#stakeholder-interest--feedback)
- [References & acknowledgements](#references--acknowledgements)

### Introduction

Wasi-cli a [World] proposal for a Command-Line Interface (CLI) environment. It provides APIs commonly available in such environments, such as filesystems and sockets, and also provides command-line facilities such as command-line arguments, environment variables, and stdio.

[World]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md#wit-worlds

### Goals

Wasi-cli aims to be useful for:

 - Interactive command-line argument programs.

 - Servers that use filesystems, sockets, and related APIs and expect to be started with
   a CLI-style command-line.

 - Stream filters that read from standard input and write to standard output.

### Non-goals

Wasi-cli is not aiming to significantly re-imagine the concept of command-line interface programs. While WASI as a whole is exploring ideas such as [Typed Main], wasi-cli sticks to the traditional list-of-strings style command-line arguments.

[Typed Main]: https://sunfishcode.github.io/typed-main-wasi-presentation/

### API walk-through

The full API documentation can be found [here](command.md).

TODO [Walk through of how someone would use this API.]

#### [Use case 1]

[Provide example code snippets and diagrams explaining how the API would be used to solve the given problem]

#### [Use case 2]

[etc.]

### Detailed design discussion

#### Should stdout be an `output-stream`?

For server use cases, standard output (stdout) is typically used as a log,
where it's typically not meaningfully blocking, async, or fallible. It's just
a place for the program to send messages to and forget about them. One option
would be to give such use cases a dedicated API, which would have a single
function to allow printing strings that doesn't return a `result`, meaning it
never fails.

However, it'd only be a minor simplification in practice, and dedicated cloud
or edge use cases should ideally migrate to more specialized worlds than the
wasi-cli world anyway, as they can result in much greater simplifications, so
this doesn't seem worthwhile.

#### Should stderr be an `output-stream`?

This is similar to the question for stdout, but for standard error (stderr),
it's a little more tempting to do something like this because stderr is used
in this logging style by many kinds of applications.

However, it seems better overall to keep stderr consistent with stdout, and
focus our desires for simplification toward other worlds, which can achieve
even greater simplifications.

#### Should environment variables be arguments to `command`?

Environment variables are useful in some non-cli use cases, so leaving them
as separate imports means they can be used from worlds that don't have a
`command` entrypoint.

### Stakeholder Interest & Feedback

TODO before entering Phase 3.

[This should include a list of implementers who have expressed interest in implementing the proposal]

### References & acknowledgements

The concept of wasi-cli has been in development for over a year since the proposal is
posted here, and many people have contributed ideas that have influenced.  Many thanks
for valuable feedback and advice in particular from:

- Luke Wagner
- Pat Hickey
