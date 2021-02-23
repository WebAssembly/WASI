cd $(dirname $(realpath $0))/witx
cargo run -p witx-cli -- docs ../../phases/snapshot/witx/wasi_snapshot_preview1.witx --output ../../phases/snapshot/docs.md
cargo run -p witx-cli -- docs ../../phases/old/snapshot_0/witx/wasi_unstable.witx --output ../../phases/old/snapshot_0/docs.md
