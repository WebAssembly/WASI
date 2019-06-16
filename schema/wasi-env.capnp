@0xb076a1080f602f71;

using import "wasi-common.capnp".Errno;
using import "wasi-common.capnp".Size;

interface Environ {
  # Read environment variable data.
  # The sizes of the buffers should match that returned by environ_sizes_get().
  get @0 (
    argvBuf :List(Text) # the environment variable string data.
  ) -> (
    error :Errno
  );

  # Return environment variable data sizes.
  sizesGet @1 () -> (
    error :Errno,
    argc :Size, # The number of environment variables.
    argv_buf_size :Size # The size of the environment variable string data.
  );
}