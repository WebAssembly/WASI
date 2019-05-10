# WASI High-Level Goals

(In the spirit of [WebAssembly's High-Level Goals](https://github.com/WebAssembly/design/blob/master/HighLevelGoals.md).)

1. Define a portable, modular, runtime-independent, and WebAssembly-native API
   to serve as a system interface which can be used by WebAssembly code to
   interact with the outside world, that preserves the essential sandboxed
   nature of WebAssembly through a [Capability-based] API design.
2. Specify and implement incrementally:
    * Start with a Minimum Viable Product (MVP) for the standard, covering
      basic API versioning, feature detection, and namespacing.
    * Then add additional features, prioritized by feedback and experience.
3. Supplement API designs with documentation and tests, and, when feasible,
   reference implementations which can be shared between wasm engines.
4. Make a great platform:
    * Work with WebAssembly tool and library authors to help them provide
      WASI support for their users.
    * When being WebAssembly-native means the platform isn't directly
      compatible with existing applications written for other platforms,
      build tools and libraries to provide compatibility.
    * Allow the overall API to evolve over time; to make changes to API
      modules that have been standardized, build implementations of them
      using libraries on top of new API modules to provide compatibility.

[Capability-based]: https://en.wikipedia.org/wiki/Capability-based_security
