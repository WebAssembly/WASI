# WASI Design Principles

## Capability-based security

WASI is built using capability-based security principles. Access to
external resources is always represented by *handles*, which are special
values that are *unforgeable*, meaning there's no way to coerce an
arbitrary integer or other type of value into a handle. WASI is also
aiming to have no *ambient authorities*, meaning that there should
be no way to request a handle purely by providing a string or other
user-controlled identifier providing the name of a resource. With these
two properties, the only ways to obtain access to resources are to be
explicitly given handles, or to perform operations on handles which
return new handles.

Note that this is a different sense of "capability" than [Linux
capabilities](http://man7.org/linux/man-pages/man7/capabilities.7.html)
or the withdrawn [POSIX
capabilities](https://archive.org/details/posix_1003.1e-990310), which
are per-process rather than per-resource.

The simplest representation of handles are values of [reference
type](https://github.com/WebAssembly/reference-types). References in
wasm are inherently unforgeable, so they can represent handles directly.

Some programming languages operate primarily within linear memory,
such as C, C++, and Rust, and there currently is no easy way for
these languages to use references in normal code. And even if it does
become possible, it's likely that source code will still require
annotations to fully opt into references, so it won't always be
feasible to use. For these languages, references are stored in a
[table](https://webassembly.github.io/spec/core/bikeshed/index.html#tables%E2%91%A0)
called a *c-list*. Integer indices into the table then identify
resources, which can be easily passed around or stored in memory. In
some contexts, these indices are called *file descriptors* since they're
similar to what POSIX uses that term for. There are even some tools,
such as wasm-bindgen, which make this fairly easy. (Internally, tools
and engines don't always use actual WebAssembly tables to do this,
however those are implementation details. Conceptually, they work as if
they had tables.)

Integer indices are themselves forgeable, however a program can only
access handles within the c-list it has access to, so isolation can still
be achieved, even between libraries which internally use integer indices,
by witholding access to each library's c-list to the other libraries.
Instances can be given access to some c-lists and not others, or even
no c-lists at all, so it's still possible to establish isolation between
instances.

Witx-specified APIs use a special `handle` keyword to mark parameters
and return values which are handles. In the short term, these are
lowered to integer indices, with an implied table, so that the APIs
can be easily used from C and similar languages today. Once [interface
types](https://github.com/WebAssembly/interface-types) and [type
imports](https://github.com/WebAssembly/proposal-type-imports) are
ready, we expect to make use of them to provide APIs which can be used
either from languages using references or from languages using integer
indices, with tables being used and managed automatically.

## WASI's Scope

WASI started out with a very POSIX-like API, however WASI will grow to
include many APIs that are outside of the scope of POSIX. WASI is a
forum for cooperatively designing APIs, along with a W3C CG Subgroup for
eventually standardizing them.

For example, WASI may include high-level network APIs, such as APIs for
HTTP. This is outside the scope of POSIX, and while some WebAssembly
engines are very interested in implementing it natively, others will
find it too complex and high-level. But one of the great things about
WebAssembly is that there's no syscall instruction, so "syscalls"
in WebAssembly are just calls to imported functions, which could be
native functions provided by the runtime, or could be other WebAssembly
modules. We expect to leverage this capability to provide polyfill
implementations of things like high-level network APIs on top of
low-level APIs, such as a raw socket API, so that engines which wish to
keep things simple and just implement the low-level socket APIs can do
so.

WASI also aims to include domain-specific APIs, such as
database, blockchain, or specialized APIs for embedded
systems. Another key building block for WASI is [optional
imports](https://github.com/WebAssembly/WASI/blob/master/design/optional-imports.md),
which give applications the ability to dynamically test for the
availability of APIs.

## Relationship to POSIX

POSIX specifies a C API rather than an actual system call ABI, with
the expectation that implementation details will differ at the system
call level. In the same way, the primary vehicle for WASI's POSIX
compatibility is libraries such as WASI libc, rather than the WASI API
itself. WASI libc provides a wide range of POSIX-compatible APIs.

In the parts of WASI which do correspond to POSIX functionality, WASI
follows POSIX when it doesn't conflict with WASI's other goals. And,
we consult POSIX even when we aren't strictly following it. POSIX is
valuable for several reasons: POSIX represents a large body of lessons
learned about systems programming, portability, and robustness. POSIX
is available on many existing hosts that we want to port WASI to. And,
there's a large amount of application code that we want to port to WASI
that uses POSIX-style interfaces.

All this said, maximal POSIX conformance is not WASI's primary goal.
Some reasons include:

 - `fork` -- It's not that we can't make `fork` work -- we can, it's
that `fork` carries with it the assumption of copy-on-write memory
optimizations which won't be feasible in many environments where we
want to run WASI, such as nano-processes. There may eventually be
compatibility layers that provide `fork` to help people port POSIX
code to WASI, however there's a difference between providing `fork` as
an optional compatibility layer and having it as a cornerstone of an
ecosystem as it is in POSIX.

   And when we take `fork` out of the focus, it changes the way we think
about a lot of other things, such as `execve`, `dup`, `fcntl`-style file
locking, and even processes.

 - Users and Groups -- POSIX's Users and Groups subsystem are
notoriously inflexible, to the point where much of the computing world
has moved to using containers and VMs and other forms of single-user
environments because traditional multi-user OS functionality doesn't do
what's needed.

   And, when running untrusted code, it isn't desirable to run it as a
user's normal identity, because it shouldn't inherit all of the rights a
user has, but it also doesn't help to run it as user "nobody", as it's
still useful to grant it some rights and restrict it from others.

   And, we are aiming for portability to OS's which don't have
POSIX-style users and groups, and systems which don't have OS's at all.

 - Asynchronous signals and handlers -- The core WebAssembly semantics
don't support these, which would need to change before WASI could
consider supporting them, and there are currently no proposals for doing
so. In POSIX, some interfaces are designed with the assumption that
signals like `SIGPIPE`, `SIGALRM`, `SIGCHLD`, `SIGIO` and others exist
and can cover certain situations, so in the absence of these signals,
those interfaces won't always make sense.

 - Shared filesystem views - One of the unique capabilities WebAssembly
brings to the table is the possibility of shared-nothing linking between
applications and libraries. Shared-nothing means that all communication
is via explicit calls, and the libraries don't share an address space
or any other implicit shared state. But if we run both sides within
the same filesystem view, that would give them a large body of shared
state. Union mounts, mandatory access control systems, user namespaces,
and other techniques can help, but often require complex configuration,
heavy-weight boundaries, and sometimes even admin privileges to set up.

This has wide-ranging implications. Much of POSIX is oriented around
passing around strings, whether through command-line arguments,
environment variables, or paths embedded in files, with the assumption
that there's a shared filesystem view between components. As we said
above, we're de-emphasizing strings, which dovetails with de-emphasizing
shared filesystem views. Instead of having shared state and passing
around values which identify things within the shared state, WASI
prefers to share as little as possible, and use handles which represent
the things which need to be shared.

Compatibility with existing host environments and applications is
important, and we have put a lot of work into WASI libc to provide POSIX
compatibility and support existing applications. There's a lot more work
that can be done, and a lot more we can do to improve compatibility and
user convenience. We're continuing to make progress -- and users are
encouraged to [file bugs](https://github.com/WebAssembly/WASI/issues)
when they find things that don't work or are awkward. This approach
supports existing applications, while also supporting applications and
libraries willing to opt in to enable stronger and more fine-grained
security properties than are possible in regular POSIX.

For example, a typical POSIX-style API might include a function that
accepts a file name to open. That requires the implementation to
have a filesystem view, and to have appropriate permissions within
that filesystem view. WASI APIs typically prefer to instead have a
function which accepts a handle for an already-open file. That way, the
implementation doesn't need a filesystem view, or permissions within
the filesystem. It doesn't even need to care whether there even is a
filesystem. When needed, compatibility with POSIX-style APIs is then
provided as a thin layer on top implementing a simple name-to-handle
mapping.

We recognize that this approach has trade-offs. It often does take more
work to design and implement the compatibility layers needed to support
existing applications than if we just made WASI always expose
POSIX-style APIs directly. It will take more work to port existing
libraries to work with shared-nothing linking. And, even when we do have
compatibility mechanisms, they aren't always the most locally optimal
ones. The compatibility layer overhead is usually quite modest, but it
is present.

However, libraries built to use shared-nothing linking can be used in
more circumstances, because you don't have to have the trust implied by
a shared filesystem view, or the complexity of configuring filesystem
rules for each library. With a better story for libraries and tools to
work together in cooperation with the sandbox, we can build a stronger
ecosystem which makes up for the downsides in the long run.

## Relationship to the Web

It is possible to run WASI code on the Web with the help of polyfills,
however WASI isn't limited to APIs which run well or are easy or
efficient to polyfill on the Web.

That said, where other considerations don't interfere, WASI should use
existing Web standards and work with Web standardization efforts rather
than gratuitously inventing its own versions of them and/or duplicating
efforts.

When using Web standards, WASI APIs should be careful to avoid depending
on JavaScript in the engine in APIs where it isn't essential.

## Use WebAssembly standards and proposals

WASI should align with and build on WebAssembly standards and proposals
where applicable.

For example, WASI seeks to align with and build on [interface
types](https://github.com/WebAssembly/interface-types), [multiple
return values](https://github.com/WebAssembly/multi-value/), [reference
types](https://github.com/WebAssembly/reference-types), [type
imports](https://github.com/WebAssembly/proposal-type-imports), and
more. As of this writing, some of these are early-stage proposals, so
we're not actually depending on them yet, however we are carefully
aligning with them so that we'll be ready when they are.

As another example, WASI's
[witx](https://github.com/WebAssembly/WASI/blob/master/docs/witx.md)
file format is designed to be a
straightforward superset of the [module linking
proposal](https://github.com/WebAssembly/module-linking/blob/master/proposals/module-linking/Explainer.md)'s
.wit format and the [annotations
proposal](https://github.com/WebAssembly/annotations/)'s annotation
syntax.

## Interposition

Interposition in the context of WASI interfaces is the ability for a
Webassembly instance to implement a given WASI interface, and for a
consumer WebAssembly instance to be able to use this implementation
transparently. This can be used to adapt or attenuate the functionality
of a WASI API without changing the code using it.

In WASI, we envision interposition will primarily be configured
through the mechanisms in the module linking' [link-time virtualization
](https://github.com/WebAssembly/module-linking/blob/master/proposals/module-linking/Explainer.md#link-time-virtualization).
Imports are resolved when a module is instantiated, which may happen
during the runtime of a larger logical application, so we can support
interposition of WASI APIs without defining them in terms of explicit
dynamic dispatch mechanisms.

Interposition is sometimes referred to as "virtualization", however we
use "interposition" here because the word "virtualization" has several
related meanings.

## Compatibility

Compatibility with existing applications and libraries, as well as
existing host platforms, is important, but will sometimes be in conflict
with overall API cleanliness, safety, performance, or portability.
Where practical, WASI seeks to keep the WASI API itself free of
compatibility concerns, and provides compatibility through libraries,
such as WASI libc, and tools. This way, applications which don't require
compatibility for compatibility' sake aren't burdened by it.

## Portability

Portability is important to WASI, however the meaning of portability
will be specific to each API.

WASI's modular nature means that engines don't need to implement every
API in WASI, so we don't need to exclude APIs just because some host
environments can't implement them. We prefer APIs which can run across
a wide variety of engines when feasible, but we'll ultimately decide
whether something is "portable enough" on an API-by-API basis.

## Strings

WASI in general de-emphasizes strings in areas where typed interfaces
can be sufficient, and especially when the strings would be serving as
identifiers in a global shared resource pool.

Where strings are passed through APIs, WASI will use [interface
types](https://github.com/WebAssembly/interface-types) to manage the
strings.

Where string encodings are exposed, WASI prefers to use UTF-8
encodings for strings, and to provide explicit length values
rather than NUL-terminated strings, (as [WebAssembly itself
does](https://webassembly.github.io/spec/core/bikeshed/index.html#binary-utf8)).
