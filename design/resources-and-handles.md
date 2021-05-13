# Resources and Handles
Resources and handles in WASI is the standard way to represent access to
OS-dependent resources, such as processes (WIP), threads (WIP), sockets (WIP),
and files (WIP). In addition, resources and handles can be used to represent
resources that you do not want to copy around willy nilly, such as large image
buffers.

Handles are implemented as their own type, so they cannot be fudged with unlike
`int` handles returned in C (e.g. by adding/subtracting one). However, this
makes it harder to interact with APIs that may still use ints under the hood.
In WASI, the [Canonical ABI][canonincal-abi] is used as a stepping stone for code
that still needs to operate on integer handles. Handles for the Canonical ABI
are passed via regular integers, where that integer is an index into the
WebAssembly module's memory. At that position in memory should be a WebAssembly
handle. The primary intent for this is that existing users that expect integers
to pass around to functions as handles can transparently use the new handles as
they are rolled out, without having to modify their existing code. At the time
of writing, it may be a while before resources and handles come to fruition, so
it's a good way to ensure that critical features can get done now while
preparing for the future.

An example visualization of a canonical reference can be seen below:

```
0   1   2   3   4   5   6   7   8
+---+---+---+---+---+---+---+---+---+---
| H | A | N | D | L | E |   |   |   |...
+---+---+---+---+---+---+---+---+---+---
^ the canonincal reference `file` points to 0 in memory at this position in
  memory, the implementation-defined bytes for the handle are located. special
  care is taken to ensure that you cannot fudge with the canonincal reference
  to invoke bad behavior.

;; pseudo-code, i can't write/read `wat` very well :p
i32 canonincal_reference = open_canonincal_resource("asdf")

;; prints "0"
print(canonincal_reference)

;; decrements the amount of
;; things pointing to the
;; resource, which may drop the
;; resource at this point
drop_canonincal_reference(canonincal_reference)
```

Handles cannot be copied by value, unlike other primitives. Instead, they must
be explicitly cloned with the `resource_clone_$resource` method, and explicitly
dropped with the `resource_drop_$resource` method. This makes managing the
lifetime of the underlying resource the handle points to easier.

In WAT, one way one can define a resource with handles is by the following:

```wat
;; NOTE: this is pseudo-code, i think i saw the code for htis somewhwere but i
;; can't seem to find where i saw it, so i'm just doing what my brain remembers
(export resource MyResource $R1)
(func resource_clone_MyResource (param (handle $R1)))
(func resource_drop_MyResource (param (handle $R1)))
(func make_resource (result (handle $R1)))
```

and can import and use the resource like so:

```wat
(import "module.wasm" (resource $R1))
(func use_resource (
  ;; i have no idea what i'm doing honestly lol
  ((resource $R1) x (expr make_resource))
  (resource_drop_MyResource(x))
))
```

Resources and handles go hand in hand, and are widely used throughout the WASI
ecosystem. An example of resources and handles in practical use are in the
following areas:

- [Files](#Files)
- [Network Sockets](#Network-Sockets)

## Files

Handles and resources can be found in use in the [wasi-filesystem][wasi-filesystem]
proposal. A simple example of opening and closing a file may be provided at a
later date, when the ABI stabilizes more.

## Network Sockets

Handles and resources can be found in use in networking, for reading and
writing data to a socket, as well as closing the socket when it is no longer in
use. A simple example of opening and closing a socket may be provided at a
later date, when the ABI stabilizes more.

[canonincal-abi]: https://github.com/WebAssembly/interface-types/pull/132
[wasi-filesystem]: https://github.com/WebAssembly/wasi-filesystem
