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
use clocks.monotonic-clock.{instant, duration}
```

For now, the tooling requires the wit files defining types to be colocated with
the wit files using them. The convention for now is to have a "deps" directory
underneath the top-level "wit" directory containing copies of any needed wit
files.

In the future, wit is expected to support an identifier syntax for naming
types in other packages.
