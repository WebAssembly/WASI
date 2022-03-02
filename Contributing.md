# Contributing to WASI

Interested in participating? Please follow
[the same contributing guidelines as the design repository][].

  [the same contributing guidelines as the design repository]: https://github.com/WebAssembly/design/blob/master/Contributing.md

Also, please be sure to read [the README.md](README.md) for this repository.

## Championing a Proposal

If you want to champion a new proposal, here's what you need to do in each phase:

### Phase 0: Gauge interest

You have an idea for an API. To see whether others are interested in pursuing the idea, you should work up a rough description of the API and post it somewhere that is publicly visible. This could be in the WASI issue queue, or in a gist or as its own repo. You can use the [proposal template] if you like, but it's not required in this phase.

Once you've done this, you can optionally have the subgroup discuss the idea by adding a discussion item to the [WASI meeting agenda].

Once you feel ready, you can add a vote to the [WASI meeting agenda] to move to the next stage.

### Phase 1: Write spec text

At this point, the WASI SG chair will create a new repo for the proposal in the WebAssembly GitHub org. This will follow the conventions of the [proposal template]. If you have any questions about how to fill in the spec template, you can reach out to the WASI SG chair.

As part of moving to the next phase, the champions need to define the acceptance criteria for Phase 4. This is because WASI includes APIs that cover a diversity of different domains and use cases, so the acceptance criteria can be very different between different proposals.

Some examples of potential criteria:

- multiple independent production implementations
- implementations on multiple host platforms
- polyfill implementations
- bindings in toolchains and libraries

Note: portability requirements may vary between proposals, as not all features will necessarily make sense in all host environments.

With all this in place, you can add a vote to [WASI meeting agenda] to move to the next stage.

### Phase 2: Work with implementers to prototype and refine the design

At this point, you should be prototyping the API to make sure it works in practice, and you should develop a test suite which can be used by other implementations to validate their spec compliance.

Once the implementation has stabilized, it's again time to add a vote to [WASI meeting agenda] to move to the next stage.

### Phase 3: Validate the design through multiple implementations

At this point, you'll need to get more implementers involved. How many implementations you need depends on the Phase 4 acceptance criteria that you set in Phase 2.

You may need to make changes in response to implementer feedback, but we expect the API to be pretty stable by this point. If implementors uncover especially challenging design issues, the proposal may be sent back to Phase 2 for more development.

Once the implementations are in place, you can add the final WASI SG vote to [WASI meeting agenda]. After this, the proposal advances to a vote in the broader WebAssembly CG.

### Phases 4 & 5: Push it over the finish line

The specific process in Phases 4 and 5 will be determined when we have a proposal ready for them.

Note: While we mostly follow the [WebAssembly CG's Phase Process], the requirements around Web VM implementation, formal notation and the reference interpreter don't apply in the context of WASI.

[proposal template]: https://github.com/WebAssembly/wasi-proposal-template
[WASI meeting agenda]: https://github.com/WebAssembly/meetings/tree/main/wasi
[WebAssembly CG's Phase Process]: https://github.com/WebAssembly/meetings/blob/main/process/phases.md
