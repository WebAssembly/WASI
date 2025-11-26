# WASI Sockets

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

[Phase 3](https://github.com/WebAssembly/WASI/blob/main/docs/Proposals.md#phase-3---implementation-phase-cg--wg)

### Champions

- Dave Bakker (@badeend)

### Portability Criteria

- At least two independent production implementations.
- Implementations available for at least Windows, Linux & MacOS.
- A testsuite that passes on the platforms and implementations mentioned above.

## Table of Contents

- [Introduction](#introduction)
- [Goals](#goals)
- [Non-goals](#non-goals)
- [API walk-through](#api-walk-through)
  - [Asynchronous APIs](#asynchronous-apis)
  - [Use case 1](#use-case-1)
  - [Use case 2](#use-case-2)
- [Detailed design discussion](#detailed-design-discussion)
  - [Dualstack sockets](#dualstack-sockets)
  - [Modularity](#modularity)
  - [POSIX compatibility](#posix-compatibility)
  - [Why not getaddrinfo?](#why-not-getaddrinfo)
  - [Security](#security)
  - [Deferred permission requests](#deferred-permission-requests)
- [Considered alternatives](#considered-alternatives)
  - [[Alternative 1]](#alternative-1)
  - [[Alternative 2]](#alternative-2)
- [Stakeholder Interest & Feedback](#stakeholder-interest--feedback)
- [References & acknowledgements](#references--acknowledgements)

### Introduction

This proposal adds TCP & UDP sockets and domain name lookup to WASI. It adds the basic BSD socket interface with the intent to enable server and client networking software running on WebAssembly.

Unlike BSD sockets, WASI sockets require capability handles to create sockets and perform domain name lookups. On top of capability handles, WASI Socket implementations should implement deny-by-default firewalling.

The socket APIs have been split up into standalone protocol-specific WASI modules. Both current and future socket modules can then be tailored to the needs of that specific protocol and progress the standardization process independently.

This proposal introduces 4 new WASI modules:
- [wasi-ip-name-lookup.wit](./wasi-ip-name-lookup.wit)
- [wasi-socket.wit](./wasi-socket.wit)
  - [wasi-socket-ip.wit](./wasi-socket-ip.wit)
    - [wasi-socket-tcp.wit](./wasi-socket-tcp.wit)
    - [wasi-socket-udp.wit](./wasi-socket-udp.wit)

### Goals

- Start out as an MVP; add the bare minimum amount of APIs required to create a basic functioning TCP/UDP application.
- Toolchains must be able to provide a POSIX compatible interface on top of the functions introduced in this proposal.

### Non-goals

- SSL/TLS support
- HTTP(S) support
- Retrieving network-related information of the executing machine, like: installed network interfaces and the computer hostname.

### API walk-through

[Walk through of how someone would use this API.]

#### Asynchronous APIs

At the moment, WIT has no built-in way of expressing asynchronous operations. To work around this limitation, we split up  async functions into two parts: `start-*` and `finish-*`.

Desired signature:

```
operation: func(this, the-inputs...) -> future<result<the-outputs..., error-code>>
```

Temporary workaround:

```
start-operation: func(this, the-inputs...) -> result<_, error-code>
finish-operation: func(this) -> result<the-outputs..., error-code>
```


The semantics are as follows:
- When `start-*` completes successfully:
    - The operation should be considered "in progress".
    - This is the POSIX equivalent of EINPROGRESS.
    - The socket can be polled for completion of the just started operation, using `wasi-poll`.
    - Its corresponding `finish-*` function can be called until it returns something other than the `would-block` error code.
- When `finish-*` returns anything other than `would-block`:
    - The asynchronous operation should be considered "finished" (either successful or failed)
    - Future calls to `finish-*` return the `not-in-progress` error code.
- The documented error codes can be returned from either the `start-*` function or the `finish-*` function. Both are equally correct.

Runtimes that don't need asynchrony, can simply validate the arguments provided to the `start` function and stash them on their internal socket instance and perform the actual syscall in the `finish` function. Conveniently, sockets only allow one of these `start/finish` asynchronous operation to be active at a time.


Example of how to recover blocking semantics in guest code:
```rs
// Pseudo code:
fn blocking-connect(sock: tcp-socket, addr: ip-socket-address) -> result<tuple<input-stream, output-stream>, error-code> {
    
    let pollable = tcp::subscribe(tcp-socket);

    let start-result = tcp::start-connect(sock, addr);
    if (start-result is error) {
        return error;
    }

    while (true) {
        poll::poll-oneoff([ pollable ]);

        let finish-result = tcp::finish-connect(sock);
        if (finish-result is NOT error(would-block)) {
            return finish-result;
        }
    }
}

```

#### Use case: Wasm module per connection

Thanks to the low startup cost of Wasm modules, its feasible for server software with Wasm integration to spawn a Wasm module for each inbound connection. Each module instance is passed only the accepted client socket. This way, all connection handlers are completely isolated from each other. This resembles PHP's "shared nothing" architecture.

#### [Use case 2]

[Provide example code snippets and diagrams explaining how the API would be used to solve the given problem]

### Detailed design discussion

[This section should mostly refer to the .wit.md file that specifies the API. This section is for any discussion of the choices made in the API which don't make sense to document in the spec file itself.]

#### Dualstack sockets

IPv6 sockets returned by this proposal are never dualstack because that can't easily be implemented in a cross platform manner. If an application wants to serve both IPv4 and IPv6 traffic, it should create two sockets; one for IPv4 traffic and one for IPv6 traffic.

This behaviour is deemed acceptable because all existing applications that are truly cross-platform must already handle this scenario. Dualstack support can be part of a future proposal adding it as an opt-in feature.

Related issue: [Emulate dualstack sockets in userspace](https://github.com/WebAssembly/wasi-sockets/issues/1)

#### Modularity

This proposal is not POSIX compatible by itself. The BSD sockets interface is highly generic. The same functions have different semantics depending on which kind of socket they're called on. The man-pages are riddled with conditional documentation. If this had been translated 1:1 into a WASI API using Interface Types, this would have resulted in a proliferation of optional parameters and result types.

Instead, the sockets API has been split up into protocol-specific modules. All BSD socket functions have been pushed into these protocol-specific modules and tailored to their specific needs. Functions, parameters and flags that did not apply within a specific context have been dropped.

A downside of this approach is that functions that do *not* differ per protocol (bind, local_address, connect, shutdown, ...) are duplicated as well.

#### POSIX compatibility

See [Posix-compatibility.md](./Posix-compatibility.md).


#### Why not getaddrinfo?

The proposed [wasi-ip-name-lookup](./wasi-ip-name-lookup.wit) module focuses strictly on translating internet domain names to ip addresses and nothing else.

Like BSD sockets, `getaddrinfo` is very generic and multipurpose by design. The proposed WASI API is *not*. This eliminates many of the other "hats" getaddrinfo has (and potential security holes), like:
- Mapping service names to port numbers (`"https"` -> `443`)
- Mapping service names/ports to socket types (`"https"` -> `SOCK_STREAM`)
- Network interface name translation (`%eth0` -> `1`)
- IP address deserialization (`"127.0.0.1"` -> `Ipv4Address(127, 0, 0, 1)`)
- IP address string canonicalization (`"0:0:0:0:0:0:0:1"` -> `"::1"`)
- Constants lookup for `INADDR_ANY`, `INADDR_LOOPBACK`, `IN6ADDR_ANY_INIT` and `IN6ADDR_LOOPBACK_INIT`.

Many of these functionalities can be shimmed in the libc implementation. Though some require future WASI additions. An example is network interface name translation. That requires a future `if_nametoindex`-like syscall.


#### Security

Wasm modules can not open sockets by themselves without a network capability handle. Even with capability handles, WASI implementations should deny all network access by default. Access should be granted at the most granular level possible. See [Granting Access](./GrantingAccess.md) for examples. Whenever access is denied, the implementation should return EACCES.

This means Wasm modules will get a lot more EACCES errors compared to when running unsandboxed. This might break existing applications that, for example, don't expect creating a TCP client to require special permissions.

At the moment there is no way for a Wasm modules to query which network access permissions it has. The only thing it can do, is to just call the WASI functions it needs and see if they fail.


#### Deferred permission requests

This proposal does not specify how wasm runtimes should handle network permissions. One method could be to let end users declare on the command line which endpoints a wasm component may connect to. Another method could be to somehow let component authors distribute a manifest alongside the component itself, containing the set of permissions that it requires.

Both of these examples depend on the network permissions being known and granted upfront. This is not always feasible and that's usually where dynamic permission requests come into play.

The most likely contenders for permission prompt interception are:
- TCP: `connect`
- TCP: `bind`
- TCP: `listen`
- UDP: `bind`
- UDP: `connect`

Now, again, this proposal does not specify if/how permission prompts should be implemented. However, it does at least facilitate the ability for runtimes to do so. Since waiting for user input takes an unknowable amount of time, the operations listed above have been made asynchronous. POSIX-compatibility layers can simply synchronously block on the returned `future`s.

### TCP State Machine

See [Operational Semantics](./TcpSocketOperationalSemantics.md).

### Considered alternatives

[This section is not required if you already covered considered alternatives in the design discussion above.]

#### [Alternative 1]

[Describe an alternative which was considered, and why you decided against it.]

#### [Alternative 2]

[etc.]

### Stakeholder Interest & Feedback

TODO before entering Phase 3.

[This should include a list of implementers who have expressed interest in implementing the proposal]

### References & acknowledgements

Many thanks for valuable feedback and advice from:

- [Person 1]
- [Person 2]
- [etc.]
