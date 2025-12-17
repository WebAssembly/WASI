#!/usr/bin/env bash
set -ex
cd $(dirname $(realpath $0))/witx-cli
cargo run -- docs $1 ../../preview1/witx/wasi_snapshot_preview1.witx --output ../../preview1/docs.md
cargo run -- docs $1 ../../preview0/witx/wasi_unstable.witx --output ../../preview0/docs.md
