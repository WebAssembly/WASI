# Wit in WASI

Starting in Preview2, WASI APIs are defined using the [Wit IDL].

[Wit IDL]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md

To set up a git repository, start by cloning the [wasi-proposal-template],
renaming the wit file, and adding content and new files as needed.

[wasi-proposal-template]: https://github.com/WebAssembly/wasi-proposal-template/

## Conventions

### Getters

TODO: Should we have a recommendation for whether to say `get-` or not? Or
maybe even build a concept of getters into wit itself?

## Temporary workarounds

The component model specification and implementations are still in development,
so at this time some features or planned features in the component model are
not ready for use in WASI APIs. Here's a list of those features and what to use
for now in their place.

### Streams

Streams are expected to be available in the Preview 3 timeframe, as part of the
[Component Model async proposal].

As a temporary workaround for use cases that need byte streams, [use] the
`input-stream` and `output-stream` types defined in [wasi-io].

```wit
use io.streams.{input-stream, output-stream}
```

For use cases that need typed streams, another option is to define a [resource]
with a function returning `option<T>` or `result<option<T>, E>` for returning
the elements with `none` indicating the end of the stream.

This resource-based workaround can be used for asynchronous I/O by using
[wasi-io] to poll for multiple streams, however it doesn't support
composing asynchronous work across multiple components, and it has some scaling
limitations. These limitations will be fixed when built in `stream` types are
available.

[Component Model async proposal]: https://docs.google.com/presentation/d/1MNVOZ8hdofO3tI0szg_i-Yoy0N2QPU2C--LzVuoGSlE/edit#slide=id.g1270ef7d5b6_0_662
[use]: #Dependencies
[wasi-io]: https://github.com/WebAssembly/wasi-io
[resource]: #Resources

### Value Imports

As a temporary workaround for the lack of value imports, link-time authority
functions may be used.

In place of a value import like this:

```wit
    /// An example value import.
    /// 
    /// A description of the value import.
    values: list<i32>
```

Define an `instance` function, like this:

```
    /// An example value import.
    /// 
    /// A description of the value import.
    /// 
    /// This [represents a value import](https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Value_Imports).
    instance-values: func() -> list<u32>
```

If the value type contains any handles, for now specify that calling the
function more than once creates new handles each time.

It is often desirable to put these functions in their own interface, so that
worlds may chose to include or exclude them independently of the rest of the
APIs they're associated with.

### Star Imports

TODO: Are there specific patterns we should recommend for working around star imports?

And add this comment:

```wit
    /// This [represents a star import](https://github.com/WebAssembly/WASI/blob/main/WasiWitdocs/WitInWasi.md#Star_Imports).
```

To avoid trouble when migrating to star imports in the future, also avoid
depending on dynamic keys.

### Dependencies

To use types from one package in another, use a `use` declaration:

```wit
use io.streams.{input-stream, output-stream}
```

For now, the tooling requires the wit files defining types to be colocated with
the wit files using them. The convention for now is to have a "deps" directory
underneath the top-level "wit" directory containing copies of any needed wit
files.

In the future, wit is expected to support an identifier syntax for naming
types in other packages.
