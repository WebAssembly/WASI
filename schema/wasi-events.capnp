@0x9bc3b97f0db7d1a2;
using import "wasi-common.capnp".Errno;
using import "wasi-common.capnp".Timestamp;
using import "wasi-common.capnp".Size;
using import "wasi-fs.capnp".FileDescriptor;
using import "wasi-fs.capnp".Filesize;
using import "wasi-clock.capnp".ClockId;


interface Events {
  using Userdata = UInt64; # User-provided value that may be attached to objects that is retained when extracted from the implementation.
  
  enum EventType {
    clock @0; # The time value of clock __wasi_subscription_t::u.clock.clock_id has reached timestamp __wasi_subscription_t::u.clock.timeout.
    fdRead @1; # File descriptor __wasi_subscription_t::u.fd_readwrite.fd has data available for reading. This event always triggers for regular files.
    fdWrite @2; # File descriptor __wasi_subscription_t::u.fd_readwrite.fd has capacity available for writing. This event always triggers for regular files.
  }

  # The state of the file descriptor subscribed to with __WASI_EVENTTYPE_FD_READ or __WASI_EVENTTYPE_FD_WRITE.
  enum Eventrwflags {
    eventFdReadwriteHangup @0; # The peer of this socket has closed or disconnected.
  }

  # Subscription to an event.
  struct Subscription {
    userdata @0 :Userdata; # User-provided value that is attached to the subscription in the implementation and returned through __wasi_event_t::userdata.
    type @1 :EventType; # The type of the event to which to subscribe.
    union {
      # When type is __WASI_EVENTTYPE_CLOCK
      clock :group {
        identifier @2 :Userdata; # The user-defined unique identifier of the clock.
        clockId @3 :ClockId; # The clock against which to compare the timestamp.
        timeout @4 :Timestamp; # The absolute or relative timestamp.
        precision @5 :Timestamp; # The amount of time that the implementation may wait additionally to coalesce with other events.
      }
      # When type is __WASI_EVENTTYPE_FD_READ or __WASI_EVENTTYPE_FD_WRITE
      fdReadwrite :group {
        fd @6 :FileDescriptor; # The file descriptor on which to wait for it to become ready for reading or writing.
      }
    }
  }
  
  # An event that occurred.
  struct Event {
    userdata @0 :Userdata; # User-provided value that got attached to __wasi_subscription_t::userdata.
    error @1 :Errno; # If non-zero, an error that occurred while processing the subscription request.
    type @2 :EventType; # The type of the event that occurred.
    union {
      # When type is __WASI_EVENTTYPE_FD_READ or __WASI_EVENTTYPE_FD_WRITE
      fdReadwrite :group {
        nbytes @3 :Filesize; # The number of bytes available for reading or writing.
        flags @4 :Eventrwflags; # The state of the file descriptor. 
      }
      else @5 :Void;
    }
  } 

  # Concurrently poll for the occurrence of a set of events.
  pollOneoff @0 (
    in :Subscription, # The events to which to subscribe.
    nsubscriptions :Size # Both the number of subscriptions and events. 
  ) -> (
    error :Errno,
    nevents :Size, # The number of events stored.
    out :Event # The events that have occurred.
  );
} 
