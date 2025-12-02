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
     ┌─────────┐           ┌─────────┐           ┌─────────┐
     │ wasi:io │           │wasi:cli │    ...    │wasi:http│
     │ → GHCR  │           │ → GHCR  │           │ → GHCR  │
     └─────────┘           └─────────┘           └─────────┘

## Usage

The unified `release.sh` script in `.github/scripts/` handles both patch and RC releases:

```bash
# Patch release (0.2.x stable)
.github/scripts/release.sh --type patch --prev 0.2.8 --next 0.2.9

# RC release (0.3.0-rc-YYYY-MM-DD)
.github/scripts/release.sh --type rc --prev-rc-date 2025-09-16
.github/scripts/release.sh --type rc  # First RC, no previous date
```

## What the Script Does

The script automates the entire release process:

1. Triggers `release.yml` to bump version numbers and create a PR
2. Waits for the PR to be created and CI to pass
3. Awaits manual review and merge of the PR
4. Creates a GitHub release (with `--prerelease` flag for RC)
5. Waits for `publish.yml` to publish packages to GHCR
6. Validates all packages were published successfully
