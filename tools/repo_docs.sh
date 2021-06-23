#!/usr/bin/env bash
set -ex
cd $(dirname $(realpath $0))/witx-cli
cargo run -- docs $1 ../../phases/snapshot/witx/wasi_snapshot_preview1.witx --output ../../phases/snapshot/docs.md
cargo run -- docs $1 ../../phases/old/snapshot_0/witx/wasi_unstable.witx --output ../../phases/old/snapshot_0/docs.md
cargo run -- docs $1 \
  ../../phases/ephemeral/witx/wasi_ephemeral_args.witx \
  ../../phases/ephemeral/witx/wasi_ephemeral_clock.witx \
  ../../phases/ephemeral/witx/wasi_ephemeral_environ.witx \
  ../../phases/ephemeral/witx/wasi_ephemeral_fd.witx \
  ../../phases/ephemeral/witx/wasi_ephemeral_path.witx \
  ../../phases/ephemeral/witx/wasi_ephemeral_poll.witx \
  ../../phases/ephemeral/witx/wasi_ephemeral_proc.witx \
  ../../phases/ephemeral/witx/wasi_ephemeral_random.witx \
  ../../phases/ephemeral/witx/wasi_ephemeral_sched.witx \
  ../../phases/ephemeral/witx/wasi_ephemeral_sock.witx \
  --output ../../phases/ephemeral/docs.md
