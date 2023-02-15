# WASI I/O

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

WASI-io is currently in [Phase 2].

[Phase 2]: https://github.com/WebAssembly/WASI/blob/42fe2a3ca159011b23099c3d10b5b1d9aff2140e/docs/Proposals.md#phase-2---proposed-spec-text-available-cg--wg

### Champions

- Dan Gohman

### Phase 4 Advancement Criteria

WASI I/O has not yet proposed its phase-4 advancement criteria.

We anticipate it will involve ensuring it works well for streaming files, sockets, and pipes, and is usable from wasi-libc for implementing POSIX APIs.

## Table of Contents

- [Introduction](#introduction)
- [Goals [or Motivating Use Cases, or Scenarios]](#goals-or-motivating-use-cases-or-scenarios)
- [Non-goals](#non-goals)
- [API walk-through](#api-walk-through)
  - [Use case: copying from input to output using `read`/`write`](#use-case-copying-from-input-to-output-using-readwrite)
  - [Use case: copying from input to output using `splice`](#use-case-copying-from-input-to-output-using-splice)
  - [Use case: copying from input to output using `forward`](#use-case-copying-from-input-to-output-using-forward)
- [Detailed design discussion](#detailed-design-discussion)
  - [Should we have support for non-blocking read/write?](#should-we-have-support-for-non-blocking-read-write)
  - [Why do read/write use u64 sizes?[Tricky design choice 2]](#why-do-read-write-use-u64-sizes)
  - [Why have a `forward` function when you can just `splice` in a loop?](#why-have-a-forward-function-when-you-can-just-splice-in-a-loop)
- [Stakeholder Interest & Feedback](#stakeholder-interest--feedback)
- [References & acknowledgements](#references--acknowledgements)

### Introduction

Wasi I/O is an API providing I/O stream abstractions. There are two
types, `input-stream`, and `output-stream`, which support `read` and
`write`, respectively, as well as a number of utility functions.

### Goals

 - Be usable by wasi-libc to implement POSIX-like file and socket APIs.
 - Support many different kinds of host streams, including files, sockets,
   pipes, character devices, and more.

### Non-goals

 - Support for async. That will be addressed in the component-model async
   design, where we can have the benefit of tigher integration with language
   bindings.
 - Bidirectional streams.

### API walk-through

#### Use Case: copying from input to output using `read`/`write`

```rust
   fn copy_data(input: InputStream, output: OutputStream) -> Result<(), StreamError> {
       const BUFFER_LEN: usize = 4096;
       loop {
           let (data, eos) = input.read(BUFFER_LEN)?;
           let mut remaining = &data[..];
           while !remaining.is_empty() {
               let num_written = output.write(remaining)?;
               remaining = &remaining[num_written..];
           }
           if eos {
               break;
           }
       }
       Ok(())
   }
}
```

#### Use case: copying from input to output using `splice`

```rust
   fn copy_data(input: InputStream, output: OutputStream) -> Result<(), StreamError> {
       loop {
           let (_num_copied, eos) = output.splice(input, u64::MAX)?;
           if eos {
               break;
           }
       }
       Ok(())
   }
}
```

#### Use case: copying from input to output using `forward`

```rust
   fn copy_data(input: InputStream, output: OutputStream) -> Result<(), StreamError> {
       output.forward(input)?;
       Ok(())
   }
}
```

### Detailed design discussion

#### Should we have support for non-blocking read/write?

This may be something we'll need to revisit, but currently, the way
non-blocking streams work is that they perform reads or writes that
read or write fewer bytes than requested.

#### Why do read/write use u64 sizes?

This is to make the API independent of the address space size of
the caller. Callees are still advised to avoid using sizes that
are larger than their instances will be able to allocate.

#### Why have a `forward` function when you can just `splice` in a loop?

This seems like it'll be a common use case, and `forward`
addresses it in a very simple way.

### Stakeholder Interest & Feedback

TODO before entering Phase 3.

Wasi-io is being proposed to be a dependency of wasi-filesystem and
wasi-sockets, and is expected to be a foundational piece of WASI
preview2.

### References & acknowledgements

Many thanks for valuable feedback and advice from:

- Thanks to Luke Wagner for many design functions and the design of
  the component-model async streams which inform the design here.
- Thanks to Calvin Prewitt for the idea to include a `forward` function
  in this API.
