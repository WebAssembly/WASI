# WASI's ephemeral/snapshot/old Process

For the standardization process, WASI overall uses a [process]
modeled after the WebAssembly CG's phased process.

For development of features in Phase 2 and later of that process, WASI
has a ephemeral/snapshot/old process, which is designed to allow
for a balance between the need for stability to allow people to build
compatible implementations, libraries, and tools and gain implementation
experience, and the need for proposals to evolve.

[process]: https://github.com/WebAssembly/WASI/blob/master/docs/Process.md

## The ephemeral/snapshot/old Phases

- [`ephemeral`](ephemeral): The development staging area. New API
  proposals API-changing fixes to existing APIs should be submitted
  as Pull Requests making changes to this directory. This directory
  provides no API stability or versioning. APIs in this directory use
  API module names starting with `wasi_ephemeral_`.

- [`snapshot`](snapshot): Usable APIs. APIs in `ephemeral` will be
  occasionally snapshotted and promoted into `snapshot`, with approval
  from the Subgroup, considering the overall suitability of the APIs
  themselves, their documentation, test coverage, and availability of
  polyfills when appropriate. Once merged, the API modules will be
  considered stable, though they may be superseded by newer versions.
  Proposals to promote specific APIs should be submitted as Pull Requests
  that:
    1. `git mv` contents of `phases/snapshot/` to
       `phases/old/snapshot_{old_snapshot_number}`.
    2. `cp -R` contents of `phases/ephemeral/` into `phases/snapshot/`.
    3. Rename files copied into `phases/snapshot/` to substitute `ephemeral`
       for `snapshot` in file names. Append the new snapshot number to each
         name.
    4. Update module names given in `.witx` files according to the previous
       step.
    5. Update tests in `tools/witx/tests/wasi.rs` to point at new snapshot, and
       add a test pointing at the just-archived snapshot under `old`.
    6. Optionally, under `phases/old/snapshot_{old_snapshot_number}, add
       polyfills for superceded APIs using the new APIs.


  Pull Requests may also add additional tests, documentation, or
  polyfills for existing `snapshot` APIs.

- [`old`](old): When APIs in `snapshot` spec are replaced by new
  versions, the old API modules are moved to the `old` directory. When
  possible, `old` APIs may be accompanied by polyfill modules which
  implement their API in terms of newer versions of the API.
