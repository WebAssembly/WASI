# Release

New patch versions of WASI (0.3.x) are released every two months, on the second
Tuesday of the month. This schedule is sometimes also referred to as "the WASI
release train", since releases are determined by a fixed schedule rather than
reaching some feature threshold.

The one exception to the WASI 0.3 release train has been WASI 0.3.0, which was
part of a one-off vote. For WASI 0.2.1-0.2.12 we also followed a two-monthly
release cadence, but with releases happening on the first Tuesday of every two
month rather than the second Tuesday.

Once we are ready to ship WASI 1.0 we may schedule a one-off release outside the release train
again. The release cadence for WASI proposals post 1.0 is still undetermined and
may be subject to change.

## Scheduled WASI 0.3 releases 2026-2027

- WASI 0.3.0: 2026-06-11 (one-off release)
- WASI 0.3.1: 2026-08-11
- WASI 0.3.2: 2026-10-13
- WASI 0.3.3: 2026-12-08
- WASI 0.3.4: 2027-02-09
- WASI 0.3.5: 2027-04-13
- WASI 0.3.6: 2027-06-08
- WASI 0.3.7: 2027-08-10
- WASI 0.3.8: 2027-10-12
- WASI 0.3.9: 2027-12-14

## Creating a new release

The release process is entirely automated via GitHub Actions. To create a new
release, a WASI Co-chair will run the [Release
WASI](https://github.com/WebAssembly/WASI/actions/workflows/release.yml)
workflow which begin the release process. This process has the following steps:

1. Trigger the Release WASI workflow by running `gh workflow run release.yml -f prev_version=0.3.<prev> -f next_version=0.3.<next>`.
1. A PR updating all version numbers is created. This is blocked on CI and review ([example](https://github.com/WebAssembly/WASI/pull/924)).
2. Create a GitHub release by running `gh release create "v0.3.<version>" --generate-notes`.
3. Once the release has been created, a second PR will be automatically created to populate the `specifications/` dir ([example](https://github.com/WebAssembly/WASI/pull/925)).
4. Once this final PR is merged the release process is complete.
