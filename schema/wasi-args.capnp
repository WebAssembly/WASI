@0xb47a0b67171659f7;

using import "wasi-common.capnp".Errno;
using import "wasi-common.capnp".Size;

interface Args {
  # Read command-line argument data.
  # The sizes of the buffers should match that returned by argsSizesGet().
  get @0 (
    argvBuf :List(Text) # A pointer to a buffer to write the argument string data.
  ) -> (
    error :Errno
  );

  # Return command-line argument data sizes.
  sizesGet @1 () -> (
    error :Errno,
    argc :Size, # The number of arguments.
    argv_buf_size :Size # The size of the argument string data.
  );
}