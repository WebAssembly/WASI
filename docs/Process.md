# WASI Standardization Process

WASI follows the [WebAssembly CG Phases process], with the following adaptations:

 - Entry into Stage 2 requires [witx] specifications.

 - Starting in Stage 2, proposals may follow WASI's [ephemeral/snapshot/old] process
   to provide a balance between the need for stability so that toolchains and engines
   can sync up, and the need for evolution.

 - The Phase 4's entry requirements for "Two or more Web VMs implement the feature",
   "At least one toolchain implements the feature", and "The formalization and the
   reference interpreter are usually updated (though these two can be done as part
   of step 3 at the Working Group chair's discretion)." are waived.

   In their place, as an additional entry requirement into Phase 2, champion(s) must
   include a set of entry criteria into Phase 4 in their proposal, which the Subgroup
   will vote on as part of Phase 2 approval.

   Phase 4 criteria will vary depending on the API and its expected use cases,
   but may include things like multiple independent production implementations,
   implementations on multiple host platforms, polyfill implementations, and
   bindings in toolchains and libraries. Note that, portability requirements may
   vary between proposals, as not all features will necessarily make sense in all
   host environments.

 - The specific process in Phases 4 and 5 will be determined when we have a
   proposal ready for them.

 - Requirements around the reference interpreter don't apply.

 - WASI proposals don't require formal notation. Formal notation may be used in the
   documentation of a feature, but it isn't expected to be practical for all APIs.

[WebAssembly CG Phases process]: https://github.com/WebAssembly/meetings/blob/master/process/phases.md
[witx]: https://github.com/WebAssembly/WASI/blob/master/docs/witx.md
[ephemeral/snapshot/old process]: https://github.com/WebAssembly/WASI/blob/master/phases/README.md
