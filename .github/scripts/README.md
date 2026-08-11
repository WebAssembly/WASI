# CI Scripts

## cm-features.js

Reads the Component Model adoption record in `docs/ComponentModelFeatures.md`,
which is the single source of truth for which gated features WASI requires.

- `publish.yml` runs it to write the "Component Model features" section of each
  released `specifications/wasi-<version>/Overview.md`.
- `validate-proposals.js` imports `readRecord` from it to build the
  `wasm-tools validate` feature flags.

```bash
# Print the section for a given version
node .github/scripts/cm-features.js 0.3.1
```

## validate-proposals.js

Validates WIT definitions for changed proposals. Used by the CI workflow.

### Prerequisites

- Node.js
- [Wasm-tools](https://github.com/bytecodealliance/Wasm-tools)
- [wit-deps](https://github.com/bytecodealliance/wit-deps)

### Running Locally

```bash
# Validate specific proposals by simulating changed files
WIT_FILES='["proposals/cli/wit/command.wit"]' node .github/scripts/validate-proposals.js

# Validate multiple proposals
WIT_FILES='["proposals/cli/wit/command.wit", "proposals/http/wit/worlds.wit"]' node .github/scripts/validate-proposals.js
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `WIT_FILES` | JSON array of changed files in `proposals/*/wit/**` |

### What it validates

1. **wit-deps lock** - If `deps.toml` exists, checks that `deps.lock` is up to date
2. **WIT syntax** - Parses WIT files with `Wasm-tools component wit`
3. **Wasm encoding** - Validates Wasm binary encoding with `--Wasm` flag
