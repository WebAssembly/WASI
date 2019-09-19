# WASI development process

## WASI uses a 3-phase process:

- [`ephemeral`](ephemeral): The development staging area. New API
  proposals API-changing fixes to existing APIs should be submitted
  as Pull Requests making changes to this directory. This directory
  provides no API stability or versioning. APIs in this directory use
  API module names starting with `wasi_ephemeral_`.

- [`unstable`](unstable): Usable APIs. APIs in `ephemeral` will be
  occasionally snapshotted and promoted into `unstable`, with approval
  from the Subgroup, considering the overall suitability of the APIs
  themselves, their documentation, test coverage, and availability of
  polyfills when appropriate. Once merged, the API modules will be
  considered stable, though they may be superseded by newer versions.
  Proposals to promote specific APIs should be submitted as Pull Requests
  that:
   - Move any superseded files out of `unstable` into `old`.
   - Optionally add polyfills for superseded APIs using `unstable` APIs.
   - Move all files supporting the APIs out of `ephemeral` into `unstable`.
   - Rename the API modules from `wasi_ephemeral_` to `wasi_unstable_`
     and append a version number.

  Pull Requests may also add additional tests, documentation, or
  polyfills for existing `unstable` APIs.

- [`old`](old): When APIs in `current` spec are replaced by new
  versions, the old API modules are moved to the `old` directory. When
  possible, `old` APIs may be accompanied by polyfill modules which
  implement their API in terms of newer versions of the API.

## Rationale

### Relationship to the CG's phases

When WASI becomes more mature, such that we have an established base
and we're adding incremental functionality to it, we may want to adopt
a process like [the CG's phases]. However, right now, everything in
WASI is undergoing changes, so we have a greater need to iterate with
flexibility.

### Relationship to standards

WASI should eventually become a standard at the level of WebAssembly
itself. Right now, it needs a lot of work before it's ready. The
`unstable` tree is meant to serve a practical purpose for people who
want to work with APIs today, with the understanding that everything
is still evolving. It's not meant as a replacement for proper
standardization, which will happen once the overall API is more
mature.

[the CG's phases]: https://github.com/WebAssembly/meetings/blob/master/process/phases.md
