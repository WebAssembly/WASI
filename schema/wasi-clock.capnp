@0xdb04d36135e780f4;

using import "wasi-common.capnp".Errno;
using import "wasi-common.capnp".Timestamp;

  
enum ClockId {
  # The store-wide monotonic clock, which is defined as a clock measuring real time, whose value cannot be adjusted and which cannot have negative clock jumps.
  # The epoch of this clock is undefined. The absolute time value of this clock therefore has no meaning.
  monotonic @0;
  # The CPU-time clock associated with the current process.
  processcputimeid @1;
  #  The clock measuring real time. Time value zero corresponds with 1970-01-01T00:00:00Z.
  realtime @2;
  # The CPU-time clock associated with the current thread.
  threadcputimeid @3;
}

interface Clock {
  # Return the resolution of a clock.
  # Implementations are required to provide a non-zero value for supported clocks. For unsupported clocks, return `einval`.
  # Note: This is similar to clock_getres in POSIX.
  resGet @0 (
    clock_id :ClockId, # The clock for which to return the resolution.
  ) -> (
    error :Errno,
    resolution :Timestamp # The resolution of the clock.
  );

  # Return the time value of a clock.
  # Note: This is similar to clock_gettime in POSIX.
  timeGet @1 (
    clockId :ClockId, # The clock for which to return the time.
    precision :Timestamp, # The maximum lag (exclusive) that the returned time value may have, compared to its actual value.
  ) -> (
    error :Errno,
    time :Timestamp # The time value of the clock.
  );
}

