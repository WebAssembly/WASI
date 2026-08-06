# WASI Release Process

  ┌─────────────────┐     ┌──────────────┐     ┌─────────────┐
  │ release.sh      │────►│ release.yml  │────►│ Creates PR  │
  │ (manual trigger)│     │ (workflow)   │     │ for review  │
  └─────────────────┘     └──────────────┘     └──────┬──────┘
                                                      │
                          ┌──────────────┐            │ merge
                          │ publish.yml  │◄───────────┘
                          │ (on release) │
                          └──────┬───────┘
                                 │
          ┌──────────────────────┼──────────────────────┐
          ▼                      ▼                      ▼
     ┌───────────┐          ┌─────────┐           ┌─────────┐
     │wasi:random│          │wasi:cli │    ...    │wasi:http│
     │ → GHCR    │          │ → GHCR  │           │ → GHCR  │
     └───────────┘          └─────────┘           └─────────┘

## Usage

The `release.sh` script in `.github/scripts/` cuts a WASI 0.3.x release:

```bash
.github/scripts/release.sh --prev 0.3.0 --next 0.3.1
```

## What the Script Does

The script automates the entire release process:

1. Triggers `release.yml` to bump version numbers and create a PR
2. Waits for the PR to be created and CI to pass
3. Awaits manual review and merge of the PR
4. Creates a GitHub release
5. Waits for `publish.yml` to publish packages to GHCR
6. Validates all packages were published successfully

## Prerequisites for what ships in a release

Feature gates queued for stabilization (`@unstable` → `@since`) must have
passed their phase 3 vote, and any new dependence on a Component Model feature
must have been adopted by a WASI Subgroup vote, before the release is cut. Both
processes are documented in [CONTRIBUTING.md](../CONTRIBUTING.md).
