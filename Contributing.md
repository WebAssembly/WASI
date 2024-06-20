# Contributing to WASI

Interested in participating? Please follow
[the same contributing guidelines as the design repository][].

[the same contributing guidelines as the design repository]: https://github.com/WebAssembly/design/blob/master/Contributing.md

Also, please be sure to read [the README.md](README.md) for this repository.

To contribute to an [existing proposal](https://github.com/WebAssembly/WASI/blob/main/Proposals.md),
refer to the linked proposal repository.

The start a new proposal, the first step is to file an issue in the
[WASI repository](https://github.com/WebAssembly/WASI/issues) presenting
the idea. A good API proposal should discuss the scope of the API,
the use cases, and the places it would be expected to be implemented.
Then proceed with the rest of the steps in phase 0 described below.

If you have any questions about any step of the process, please reach out
to one of the WASI Subgroup (SG) chairs.

## The Phase Process

The following process is modeled after [WebAssembly CG's Phase Process],
though it differs in several areas, to reflect the unique needs of APIs.

Something is out-of-scope if it doesn't fit the [WASI Subgroup's charter](https://github.com/WebAssembly/WASI/blob/main/Charter.md) and there's agreement that the charter should not be amended to cover the proposal.

In general, the process moves forward through a series of numbered phases.
However, if issues are uncovered or consensus devolves,
proposals should back up to the appropriate prior step.

No vote is required for a proposal to enter phase 0. To advance from one phase
to another, a vote proposing the advancement is added to a
[WASI Subgroup meeting](https://github.com/WebAssembly/meetings/tree/main/wasi) agenda
through a pull request, and the SG votes on whether to approve it, evaluating
whether the new phase's entry requirements have been met.

### 0. Pre-Proposal [Individual Contributor]

Entry requirements:

  * A WASI Subgroup (SG) member has an idea. Notably, no SG vote is required to begin phase 0.

During this phase:

  1. An issue is filed on the [WASI repository](https://github.com/WebAssembly/WASI/issues) to present the idea.
  1. Discussion on the API occurs on the issue.
  1. A champion or champions emerge. They may add the proposal to the [proposal list](https://github.com/WebAssembly/WASI/blob/main/Proposals.md) at phase 0.
  1. The champion(s) put together a description of the API in their own GitHub repository or on the issue. You can use the [proposal template] if you like, but it's not required in this phase.

### 1. Feature Proposal [WASI Subgroup]

Entry requirements:

  * There is general interest within the SG in this API.
  * The SG believes the API is in-scope and will plausibly be workable.

During this phase:

  1. If the proposal is not already listed, it should be added to the [proposal list](https://github.com/WebAssembly/WASI/blob/main/Proposals.md) at this time.
  1. A new repository, forking the [proposal template] repo, is created by one of the SG chairs, or transferred to the WebAssembly organization by the champion.
  1. The champion will attempt to reach broad consensus in the Subgroup.
  1. Pull requests and issues are used to iterate on the design of the API. Specifically, an overview document must be produced that specifies the API with reasonably precise and complete language before attempting to move to phase 2 (meaning it is sufficiently precise to be implemented following this description, without obvious holes or ambiguities).
  1. If relevant to demonstrate the viability of a API, prototype implementations of the API are implemented by interested embedders (possibly on a branch).

Additionally during this phase:

 * The champions define the *portability criteria* for Phase 4.

   This is intended to translate the spirit of the CG Phase Process' "Two or more Web VMs" requirement to meet WASI's needs. The criteria should establish at least:
    - Two or more implementations: Each proposal should say what kinds of implementations.
    - Portability: WASI APIs should be portable, however that can mean different things to different use cases, and no one definition covers everything. Consequently, each proposal should define criteria establishing its specific portability requirements.
    - Practicality: It's important that WASI APIs be implementable and usable in real-world use cases, so each proposal should define criteria establishing a sufficient level of confidence.
    - Testing: APIs will have different needs in terms of environments needed to test them, so each proposal should define criteria establishing what form the testing will take.

### 2. Feature Description Available [WASI Subgroup]

Entry requirements:

   * The portability criteria are documented in the proposal.
   * Precise and complete overview document is available in a proposal repo around which a reasonably high level of consensus exists.
   * A [wit](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) description of the API exists.
   * All dependencies of the wit description must have reached phase 2.

During this phase:

   * One or more implementations proceed on prototyping the API.
   * A plan is developed for how the portability criteria will be met.

## 3. Implementation Phase [WASI Subgroup]

Entry requirements:

   * The portability criteria must be either met or there must be a plan for how they're expected to be met.
   * All dependencies of the wit descriptions must have reached phase 3.

During this phase, the following proceeds in parallel:

   * Implementations are built
   * Toolchains, libraries, and other tools using the API are built
   * Remaining open questions are resolved.
   * The plan for satisfying the portability criteria is followed, though the plan may change over time.

### Phases 4 & 5: To be determined

Phases 4 and 5 are where a feature is finished and standardized. As WASI matures, the WASI Subgroup will coordinate with its parent WebAssembly Community Group and the WebAssembly Working Group to define a process for standardization.

[proposal template]: https://github.com/WebAssembly/wasi-proposal-template
[WASI meeting agenda]: https://github.com/WebAssembly/meetings/tree/main/wasi
[WebAssembly CG's Phase Process]: https://github.com/WebAssembly/meetings/blob/main/process/phases.md

## Filing changes to existing phase 3 proposals

Extending existing phase 3 WASI proposals follow a different process than filing
new proposals. Because the scope of a phase 3 proposal is already set, further
changes to its APIs are tracked independently as phase 2 proposals behind an
`@unstable` gate in WIT. Once an extension is sufficiently developed, and meets
the phase 3 criteria, it must go through a vote in the WASI SG to get reach
phase 3. Once that's done, the `@unstable` gate can be replaced with `@since`,
and the extension can be included in a future WASI release.

To submit an extension to an existing phase 3 WASI proposal, the following
process should be followed:

1. File a PR to a WASI proposal repo with the feature extensions behind an
  `@unstable` gate. Feature gate names all exist in a shared namespace, so they
  should be prefixed with the parent proposal name. An unstable "timezone"
  feature for the "clocks" proposal should be named `clocks-timezone`.
2. Accepting changes to proposals is done at the discretion of the proposal
  champions. They will review and work with the PR submitter to get it to a
  state where it can be merged, or explain why the extension is presently not
  the right fit for the existing proposal.
3. Once the champion is ready to merge the proposal, they will submit a PR to
  the WASI repository (this repository) to file for a new phase 2 feature.
4. Once the feature is tracked on the WASI repository the champion can now merge
  the extension. This would also be a good time to inform the WASI SG that an
  extension has landed - in the interest of keeping relevant parties informed.
5. Implementers should now be free to begin implementing the extension behind
  feature flags. The goal at this phase is to implement and iterate on the
  extension until it is ready to advance to phase 3.
6. Once the champion believes the phase 3 advancement criteria are met, they
  should bring it to the WASI SG for a vote.
7. Once the proposal is voted to advance to phase 3, the `@unstable` gate should
  be replaced with an `@since` gate containing the version of the next WASI
  release. It is encouraged to preserve the `feature` field in the `@since` gate to
  help the transition from the `@unstable` feature to the newly stabilized
  `@since` gate.
8. The proposal is now ready to be released as part of the next WASI version.
