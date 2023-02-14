# Legacy WASI docs

This directory documents the "preview0" and "preview1" iterations of WASI.

Preview0 corresponded to the import module name `wasi_unstable`. It was
also called `snapshot_0` in some places. It was short-lived, and the changes
to preview1 were minor, so the focus here is on preview1.

Preview1 corresponds to the import module name `wasi_snapshot_preview1`.

There was some work under the name "ephemeral" towards an update of preview1
however it is no longer being actively developed. The name "preview2" now
refers to the new wit-based iteration of WASI instead.

Preview1 was defined using the witx IDL and associated tooling. Witx was
an s-expression-based IDL derived from WebAssembly's wat text format, adding
several extensions. It had a low-level C-like type system that emphasized raw
pointers, and callees were expected to have access to the caller's entire
linear memory, exported as "memory". It also had an implied global file
descriptor table.

Some features in preview1 were not widely supported by engines:
 - The `proc_raise` function, because Wasm itself has no signal-handling
   facilities, and a process wishing to terminate abnormally typically
   uses a Wasm trap instead of calling `proc_raise`.

 - The `process_cputime_id` and `thread_cputime_id`, because in many
   engines, Wasm instances are not one-to-one with host processes, so the
   usual host APIs aren't sufficient to implement these.

One function has been added to preview1:
 - `sock_accept`, allowing limited socket use cases to be supported.
   Sockets support with `listen` and `connect` is being added in preview2.
