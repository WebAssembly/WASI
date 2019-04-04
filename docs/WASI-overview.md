# WASI: WebAssembly System Interface

WebAssembly System Interface, or WASI, is a family of APIs for WebAssembly
being designed and standardized through the WASI Subgroup of the W3C
WebAssembly Commmunity Group. Initially, the focus is on system-oriented APIs,
covering files, networking, and a few other things. Additional domains are
expected to be added in the future.

WebAssembly is designed to run well on the Web, however it's
[not limited to the Web](https://github.com/WebAssembly/design/blob/master/NonWeb.md).
The core WebAssembly language is independent of its surrounding
environment, and WebAssembly interacts with the outside world
exclusively through APIs. On the Web, it naturally uses the
existing Web APIs provided by browsers.

WASI is an effort to provide general-purpose APIs for supporting
non-Web use cases. The focus is on designing clean and portable APIs which
can be implemented on multiple platforms by multiple engines, and which
don't depend on browser functionality (although they still can run in
browsers; see below).

## Capability-Oriented

WASI's core design follows
[CloudABI](https://cloudabi.org/)'s
(and in turn
[Capsicum](https://www.cl.cam.ac.uk/research/security/capsicum/))'s concept of
[capability-based security](https://en.wikipedia.org/wiki/Capability-based_security),
which fits well into WebAssembly's sandbox model. Files,
directories, network sockets, and other resources are identified
by UNIX-like file descriptors, which are indices into external
tables whose elements represent capabilities. Similar to how core
WebAssembly provides no ability to access the outside world without
calling imported functions, WASI APIs provide no ability to access
the outside world without an associated capability.

For example, instead of a typical
[open](http://pubs.opengroup.org/onlinepubs/009695399/functions/open.html)
system call, WASI provides an
[openat](https://linux.die.net/man/2/openat)-like
system call, requiring the calling process to have a file
descriptor for a directory that contains the file, representing the
capability to open files within that directory. (These ideas are
common in capability-based systems.)

However, the WASI libc implementation still does provide an
implementation of open, by taking the approach of
[libpreopen](https://github.com/musec/libpreopen).
Programs may be granted capabilities for directories on launch, and
the library maintains a mapping from their filesystem path to the
file descriptor indices representing the associated capabilities.
When a program calls open, they look up the file name in the map,
and automatically supply the appropriate directory capability. It
also means WASI doesn't require the use of CloudABI's `program_main`
construct. This eases porting of existing applications without
compromising the underlying capability model. See the diagram below
for how libpreopen fits into the overall software architecture.

WASI also automatically provides file descriptors for standard
input and output, and WASI libc provides a normal `printf`. In
general, WASI is aiming to support a fairly full-featured libc
implementation, with the current implementation work being based on
[musl](http://www.musl-libc.org/).

## Portable System Interface for WebAssembly

WASI is being designed from the ground up for WebAssembly, with
sandboxing, portability, and API tidiness in mind, making natural
use of WebAssembly features such as i64, import functions with
descriptive names and typed arguments, and aiming to avoid being
tied to a particular implementation.

We often call functions in these APIs "syscalls", because they
serve an analogous purpose to system calls in native executables.
However, they're just functions that are provided by the
surrounding environment that can do I/O on behalf of the program.

WASI is starting with a basic POSIX-like set of syscall functions,
though adapted to suit the needs of WebAssembly, such as in
excluding functions such as fork and exec which aren't easily
implementable in some of the places people want to run WebAssembly,
and such as in adopting a capabilities-oriented design.

And, as WebAssembly grows support for
[host bindings](https://github.com/webassembly/host-bindings)
and related features, capabilities can evolve to being represented
as opaque, unforgeable
[reference typed values](https://github.com/WebAssembly/reference-types),
which can allow for finer-grained control over capabilities, and
make the API more accessible beyond the C-like languages that
POSIX-style APIs are typically aimed at.

## WASI Software Architecture

To facilitate use of the WASI API, a libc
implementation called WASI libc is being developed, which presents
a relatively normal musl-based libc interface, implemented on top
of a libpreopen-like layer and a system call wrapper layer (derived
from the "bottom half" of
[cloudlibc](https://github.com/NuxiNL/cloudlibc)).
The system call wrapper layer makes calls to the actual WASI
implementation, which may map these calls to whatever the
surrounding environment provides, whether it's native OS resources,
JS runtime resources, or something else entirely.

[This libc is part of a "sysroot"](https://github.com/CraneStation/wasi-sysroot),
which is a directory containing compiled libraries and C/C++ header
files providing standard library and related facilities laid out in
a standard way to allow compilers to use it directly.

With the [LLVM 8.0](http://llvm.org/)
release, the WebAssembly backend is now officially stable, but LLVM
itself doesn't provide a libc - a standard C library, which you
need to build anything with clang. This is what the WASI-enabled
sysroot provides, so the combination of clang in LLVM 8.0 and the
new WASI-enabled sysroot provides usable Rust and C compilation
environments that can produce executable wasm programs.

![WASI software architecture diagram](wasi-software-architecture.png "WASI software architecture diagram")

## Future Evolution

The first version of WASI is relatively simple, small, and
POSIX-like in order to make it easy for implementers to prototype
it and port existing code to it, making it a good way to start
building momentum and allow us to start getting feedback based on
experience.

Future versions will change based on experience
and feedback with the first version, and add features to address
new use cases. They may also see significant architectural
changes. Because all of the APIs are accessed through regular
WebAssembly imports, APIs can be implemented either by wasm
runtimes directly or by other WebAssembly modules. So if WASI APIs
change significantly, the old APIs can be implemented as a library
on top of the new APIs.

## Can WASI apps run on the Web?

While this isn't the initial focus, it's possible to implement WASI
APIs from JavaScript, since they're just regular WebAssembly imports,
so it's possible to run WASI modules on the Web.

And in the future, it's possible that
[builtin modules](https://github.com/tc39/ecma262/issues/395)
could take these ideas even further allowing easier and tighter
integration between .wasm modules importing WASI and the Web.
