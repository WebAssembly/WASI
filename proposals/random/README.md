# WASI Random

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

WASI-random is currently in [Phase 2].

[Phase 2]: https://github.com/WebAssembly/WASI/blob/42fe2a3ca159011b23099c3d10b5b1d9aff2140e/docs/Proposals.md#phase-2---proposed-spec-text-available-cg--wg

### Champions

- Dan Gohman

### Phase 4 Advancement Criteria

WASI random must have host implementations which can pass the testsuite
on at least Windows, macOS, and Linux.

WASI random must have at least two complete independent implementations.

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

WASI Random is a WASI API for obtaining random data.

### Goals

The primary goal of WASI Random is to allow users to use WASI programs to
obtain high-quality low-level random data.

### Non-goals

WASI Random is not aiming to allow programs to handle errors or to query for
availability. It always succeeds (though on platforms where randomness is
unavailable, programs may fail to be instantiated or may trap).

WASI Random is not aiming to be a full DRBG API. Such an API could be
considered in WASI, but it should be a separate proposal.

And, WASI Random is not include facilities for feeding entropy back into
the system. It is expected that most entropy that applications would observe
should also be observable by the host implementation, and so there should
be little need to feed it back in. There may be other uses for such an API,
but they can be addressed in separate proposals.

WASI Random does not have an async API. It is expected to be implemented with
a CSPRNG which is expected to be sufficiently seeded.

WASI Random does not have an explicit facility for domain separation or
personalization messages. If such features are desired, it would make sense to
define them as custom sections, rather than program data, so that they could
easily be excluded from module caching and possibly also from code signing.
This would make sense as a separate proposal.

WASI Random does not provide an "entropy API" or a "true random" API directly.
The currently expected use cases want a CSPRNG API.

WASI Random does not expose an entropy estimation. It is expected to always
have sufficient entropy to seed a CSPRNG.

WASI Random does not provide any facility for replacing random data with
deterministic data. It is intended to be usable in use cases where determinism
would break application assumptions. Implementations may have debugging
facilities which make this API deterministic, however these should only be
used for debugging, and not production use.

### API walk-through

[Walk through of how someone would use this API.]

#### [Use case 1]

[Provide example code snippets and diagrams explaining how the API would be used to solve the given problem]

#### [Use case 2]

[etc.]

### Detailed design discussion

[This section should mostly refer to the .wit.md file that specifies the API. This section is for any discussion of the choices made in the API which don't make sense to document in the spec file itself.]

### What if the system lacks sufficient entropy during early boot?

Randomness APIs which can fail, or which can be "nonblocking" and return
incomplete results, are error prone and tend to lead applications to resort
to fallbacks which don't tend to be well-tested.

CSPRNGs are believed to be good enough that most systems in most situations
can provide effectively unlimited random data. The main case where this
isn't the case is on systems which have just booted and which have not yet
collected sufficient entropy to initialize their CSPRNGs. In these cases,
this API is designed with the belief that it's better for implementations
to respond to the problem, rather than to pass the responsibility on to
applications.

### Should there be a separate "insecure" random API?

It's a good question. I haven't ruled it out, but I'd like to learn more
about use cases where it would be meaningfully better than just using the
regular random API for everything. In particular, I'm interested in not
just the application, but also the settings in which the application would
be used where this would be relevant.

### What should happen on host platforms with weak or broken randomness APIs?

It's implementations' responsibility to handle these situations. They may do
so by supplementing the host platform APIs with data collected from other
sources, they may refuse to run programs that use Random APIs, or if needed,
they may trap programs dynamically to prevent programs from continuing to
execute with poor data.

Implementations are encouraged to perform regular reseeding (if the host
platform doesn't already do so).

### Should there be a randomness resource, and should the API take a handle?

Programs shouldn't need to be aware of *which* random generator they have, since
the data is random and indistinguishable.

WASI programs using the Random API will have imports specific to the Random API,
because they are distinct from imports used for general-purpose `stream`.

### Should random data be provided as a `stream`?

Reusing the `stream` type is tempting, however it's desirable for users of this
API to be provided actually random data, and not the contents of arbitrary
streams which might be substituted, so it doesn't turn out to be useful to unify
this API with `stream`.

This also ensures that programs using the Random API can be identified by
their imports, as mentioned in the previous question.

### Should the API specify a number of bits of security?

Best practices suggest that implementations should provide at least 196 bits of
security. However, many host platforms' CSPRNG APIs do not currently document
their bits of security, and it doesn't seem desirable to require wasm engines to
run their own CSPRNG on a platform which alreay has one, so for now, the API
does not specify a specific number.

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

- Zach Lym
- Luke Wagner
- Linux Weekly News' many articles about Linux random APIs including [this one].

[this one]: https://lwn.net/Articles/808575/
