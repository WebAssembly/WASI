# CI Scripts

## validate-proposals.js

Validates WIT definitions for changed proposals. Used by the CI workflow.

### Prerequisites

- Node.js
- [Wasm-tools](https://github.com/bytecodealliance/Wasm-tools)
- [wit-deps](https://github.com/bytecodealliance/wit-deps)

### Running Locally

```bash
# Validate specific proposals by simulating changed files
WIT_02_FILES='["proposals/cli/wit/cli.wit"]' node .github/scripts/validate-proposals.js

# Validate 0.3 proposals
WIT_03_FILES='["proposals/http/wit-0.3.0-draft/handler.wit"]' node .github/scripts/validate-proposals.js

# Validate multiple proposals
WIT_02_FILES='["proposals/cli/wit/cli.wit", "proposals/http/wit/proxy.wit"]' node .github/scripts/validate-proposals.js
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `WIT_02_FILES` | JSON array of changed files in `proposals/*/wit/**` |
| `WIT_03_FILES` | JSON array of changed files in `proposals/*/wit-0.3.0-draft/**` |

### What it validates

1. **wit-deps lock** - If `deps.toml` exists, checks that `deps.lock` is up to date
2. **WIT syntax** - Parses WIT files with `Wasm-tools component wit`
3. **Wasm encoding** - Validates Wasm binary encoding with `--Wasm` flag
