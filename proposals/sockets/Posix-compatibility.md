# POSIX Compatibility

This document provides an overview of the POSIX interface along with common non-standard extensions and their mapping to functionalities provided by this proposal.


## General

### I/O completion polling (`poll`, `select`, `pselect`, `epoll_*` (non-standard), `kqueue` (non-standard)) <a name="select"></a>
Use [`tcp::subscribe`](tcp)/[`udp::subscribe`](udp) to obtain a `pollable` handle. Then use that to wait for IO events using the [wasi-poll](poll) interface.

### Non-blocking mode (`FIONBIO`, `SOCK_NONBLOCK`, `O_NONBLOCK`) <a name="nonblock"></a>
All WASI sockets are non-blocking and can not be configured to block.
Blocking behaviour can be recreated in userland (or in wasi-libc) by repeatedly calling [`poll::poll-oneoff`](poll) until the operation is complete.

### TCP urgent data (`sockatmark`, `MSG_OOB`, `SO_OOBINLINE`, `SIOCATMARK`) <a name="oob"></a>
Out-of-band (OOB) data is currently not included in this proposal. Application-level usage of the TCP "urgent" flag is rare in practice and discouraged in general. Including it in WASI would probably interfere with the ability to use WASI/ComponentModel `stream`s.

### Peeking (`MSG_PEEK`) <a name="peek"></a>
Peeking support is not provided by this proposal directly. Including it in WASI would probably interfere with the ability to use WASI/ComponentModel `stream`s.

Support for it might be able to be recreated in userland (or in wasi-libc).

### Writing to closed streams (`SIGPIPE`, `SO_NOSIGPIPE`) <a name="sigpipe"></a>
WASI has no concept of 'signals'. Implementations that require it are encouraged to set the `SO_NOSIGPIPE` option to `true`, to increase cross-platform consistency.
Writing to a closed stream in WASI returns a regular error.

### Close-on-exec (`FD_CLOEXEC`, `SOCK_CLOEXEC`, `O_CLOEXEC`) <a name="cloexec"></a>
Not included in proposal. WASI has no concept of UNIX-style processes.


## Functions

### `socket`
- TCP: [`create-tcp-socket`](tcp-create-socket)
- UDP: [`create-udp-socket`](udp-create-socket)

### `connect`
- TCP: [`tcp::start-connect`](tcp) & [`tcp::finish-connect`](tcp)
- UDP: [`udp::start-connect`](udp) & [`udp::finish-connect`](udp)

### `bind`
- TCP: [`tcp::start-bind`](tcp) & [`tcp::finish-bind`](tcp)
- UDP: [`udp::start-bind`](udp) & [`udp::finish-bind`](udp)

### `listen`
- TCP: [`tcp::start-listen`](tcp) & [`tcp::finish-listen`](tcp). The `backlog` parameter has been split out into a distinct function [`tcp::set-listen-backlog-size`](tcp) ([See #34](https://github.com/WebAssembly/wasi-sockets/issues/34)).
- UDP: N/A

### `accept`, `accept4` (non-standard)
- TCP: [`tcp::accept`](tcp)
- UDP: N/A

To collect the remote address, call [`tcp::remote-address`](tcp) on the newly accepted client socket.

Some platforms provide an `accept4` variant with additional flags. None of these flags make sense in the context of this proposal. See [SOCK_NONBLOCK](#nonblock) & [SOCK_CLOEXEC](#cloexec).

### `getsockname`, `getpeername`
- TCP: [`tcp::local-address`](tcp) & [`tcp::remote-address`](tcp)
- UDP: [`udp::local-address`](udp) & [`udp::remote-address`](udp)

### `read`, `readv`, `recv`, `recvfrom`, `recvmsg`, `recvmmsg` (non-standard)

TCP sockets can be read using [`streams::(blocking-)read`](streams). UDP sockets can be read using [`udp::receive`](udp).

The various POSIX functions should be implementable on top of these two functions.

None of the flags are directly present in WASI Sockets:
- `MSG_DONTWAIT`: This is [always the case](#nonblock).
- `MSG_OOB` (TCP): [Not supported](#oob)
- `MSG_OOB` (UDP): N/A
- `MSG_PEEK`: [No direct support](#peek)
- `MSG_TRUNC` (TCP): N/A
- `MSG_TRUNC` (UDP): Not needed, the returned data array always has the exact perfect size.
- `MSG_WAITALL` (TCP): Emulatable in userspace.
- `MSG_WAITALL` (UDP): N/A
- `MSG_EOR`: N/A (not supported on TCP & UDP sockets)
- `MSG_CMSG_CLOEXEC`: N/A (only used on Unix domain sockets)

### `write`, `writev`, `send`, `sendto`, `sendmsg`, `sendmmsg` (non-standard)

TCP sockets can be written to using [`streams::(blocking-)write`](streams). UDP sockets can be written to using [`udp::send`](udp).

The various POSIX functions should be implementable on top of these two functions.

None of the flags are directly present in WASI Sockets:
- `MSG_DONTROUTE`: Not included in proposal at the moment.
- `MSG_DONTWAIT`: This is [always the case](#nonblock).
- `MSG_NOSIGNAL`: This is [always the case](#sigpipe).
- `MSG_OOB` (TCP): [Not supported](#oob)
- `MSG_OOB` (UDP): N/A
- `MSG_EOR`: N/A (not supported on TCP & UDP sockets)


### `sendfile` (non-standard)
- TCP: Part of the WASI Streams proposal as [`output-stream::forward`](streams)
- UDP: N/A

### `shutdown`
- TCP: [`tcp::shutdown`](tcp)
- UDP: N/A

### `sockatmark`
- TCP: Not supported, see [OOB](#oob).
- UDP: N/A

### `close`
Dropping a handle performs an effective `close`.

### `socketpair`, `connectat` (non-standard), `bindat` (non-standard)
Specifically for UNIX domain sockets. Out of scope for this proposal.

### `fcntl`
- `F_GETFL`/`F_SETFL` > `O_NONBLOCK`: [Not needed](#nonblock).
- `F_SETFD`/`F_GETFD` > `FD_CLOEXEC`: [Not included](#cloexec).

### `ioctl`
- `SIOCATMARK`: [Not included](#oob).
- `FIONREAD`: Currently not included. See [#17](https://github.com/WebAssembly/wasi-sockets/issues/17).

### `getsockopt`, `setsockopt`
Socket options have been split out into distinct functions. See table below.




## Socket options

POSIX defines the signatures of the `getsockopt` & `setsockopt` functions, but does not provide much guidance on the individual socket options themselves.
Because of this lack of a central authority, a list has been compiled of the options that are used "in the wild".

The results are not intended to be an exhaustive overview of all possible network applications, but rather to provide input on which options are worth standardizing in WASI.

Additionally, most columns have been populated semi-automatically by grepping through the respective codebases. The results have not been manually verified and therefore may not be 100% correct.

Columns:
- "Socket option": The native option constant.
- "WASI":
	- ✅ = Included in proposal.
	- ⛔ = Consciously decided _not_ to include in WASI. See notes for explanation.
	- ❔ = Not included (yet), for no particular reason.
- The rest:
	- ✅ = Option is provided by the platform / depended upon by the application.
	- ❌ = Option is not provided / not used.

> Note: GitHub clips the table content. Scroll left and right to see all columns, or use the Code View.

| Option                          | WASI      | POSIX  | Linux  | Windows | MacOS   | FreeBSD | JVM   | .NET   | Rust  | Node.js | Go  | OpenSSL  | nginx | curl  | msquic | exim  | Notes |
|---------------------------------|-----------|--------|--------|---------|---------|---------|-------|--------|-------|---------|-----|----------|-------|-------|--------|-------|-|
| SO_ERROR                        | ⛔       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ✅  | ✅      | ✅    | ✅    | ✅    | ❌   | Not necessary. WIT has (or will have) native support for asynchronous results. |
| SO_DOMAIN                       | ✅       | ❌     | ✅    | ✅      | ❌     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | [`tcp::address-family`](tcp)<br/>[`udp::address-family`](udp) <br/><br/> SO_PROTOCOL_INFO on Windows. |
| SO_TYPE                         | ✅*      | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ❌      | ✅  | ✅      | ✅    | ✅    | ❌    | ✅   | * indirectly through the type of the socket resource. |
| SO_PROTOCOL                     | ✅*      | ❌     | ✅    | ✅      | ❌     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ✅   | * indirectly through the type of the socket resource. SO_PROTOCOL_INFO on Windows. |
| SO_ACCEPTCONN                   | ❔       | ✅     | ✅    | ✅      | ✅     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_V6ONLY                     | ✅       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ✅  | ✅      | ❌    | ✅    | ✅    | ✅   | [`tcp::(set-)ipv6-only`](tcp)<br/>[`udp::(set-)ipv6-only`](udp) |
| IP_HDRINCL                      | ⛔       | ❌     | ✅    | ✅      | ✅     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Out of scope. Raw sockets only. |
| IPV6_HDRINCL                    | ⛔       | ❌     | ✅    | ✅      | ❌     | ❌      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Out of scope. Raw sockets only. |
| IP_TTL                          | ✅       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | [`tcp::(set-)unicast-hop-limit`](tcp)<br/>[`udp::(set-)unicast-hop-limit`](udp) |
| IPV6_UNICAST_HOPS               | ✅       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | [`tcp::(set-)unicast-hop-limit`](tcp)<br/>[`udp::(set-)unicast-hop-limit`](udp) |
| IP_RECVTTL                      | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVHOPLIMIT               | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_TOS                          | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ✅   | |
| IPV6_TCLASS                     | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ✅    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ✅   | |
| IP_RECVTOS                      | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ✅    | ❌   | |
| IPV6_RECVTCLASS                 | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ✅    | ❌   | |
| IP_RECVPKTINFO                  | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ✅      | ✅    | ❌    | ✅    | ❌   | IP_PKTINFO on Linux & Windows, IP_RECVDSTADDR+IP_RECVIF on MacOS & FreeBSD. |
| IPV6_RECVPKTINFO                | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ✅      | ✅    | ❌    | ✅    | ❌   | IPV6_PKTINFO on Windows. |
| IP_DONTFRAG                     | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ❌      | ❌  | ✅      | ✅    | ❌    | ✅    | ❌   | IP_DONTFRAGMENT on Windows, implementable using IP_MTU_DISCOVER on Linux. |
| IPV6_DONTFRAG                   | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ❌      | ❌  | ✅      | ✅    | ❌    | ✅    | ❌   | |
| IP_MTU_DISCOVER                 | ❔       | ❌     | ✅    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ✅      | ✅    | ✅    | ✅    | ❌   | |
| IPV6_MTU_DISCOVER               | ❔       | ❌     | ✅    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ✅      | ✅    | ✅    | ✅    | ❌   | |
| SO_RCVBUF                       | ✅       | ✅     | ✅    | ❌      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ✅  | ❌      | ✅    | ❌    | ✅    | ❌   | [`tcp::(set-)receive-buffer-size`](tcp)<br/>[`udp::(set-)receive-buffer-size`](udp) |
| SO_SNDBUF                       | ✅       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ✅  | ❌      | ✅    | ✅    | ❌    | ❌   | [`tcp::(set-)send-buffer-size`](tcp)<br/>[`udp::(set-)send-buffer-size`](udp) |
| SO_RCVLOWAT                     | ❔       | ✅     | ✅    | ❌      | ✅     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_SNDLOWAT                     | ❔       | ✅     | ✅    | ❌      | ✅     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| SO_RCVTIMEO                     | ⛔       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ❌      | ❌  | ✅      | ❌    | ✅    | ❌    | ❌   | WASI sockets are always non-blocking. Timeouts can be recreated in libc. |
| SO_SNDTIMEO                     | ⛔       | ✅     | ❌    | ✅      | ✅     | ✅      | ❌    | ✅    | ✅   | ❌      | ❌  | ✅      | ❌    | ❌    | ❌    | ❌   | WASI sockets are always non-blocking. Timeouts can be recreated in libc. |
| SO_KEEPALIVE                    | ✅       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ✅  | ✅      | ✅    | ✅    | ❌    | ✅   | [`tcp::(set-)keep-alive`](tcp) |
| TCP_KEEPCNT                     | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| TCP_KEEPIDLE                    | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ✅  | ❌      | ✅    | ✅    | ❌    | ❌   | TCP_KEEPALIVE on MacOS |
| TCP_KEEPINTVL                   | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ✅  | ❌      | ✅    | ✅    | ❌    | ❌   | |
| TCP_NODELAY                     | ✅       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ✅  | ✅      | ✅    | ✅    | ❌    | ✅   | [`tcp::(set-)no-delay`](tcp) |
| TCP_CORK                        | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ✅   | TCP_NOPUSH on MacOS & FreeBSD |
| SO_LINGER                       | ❔       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ✅  | ✅      | ✅    | ❌    | ❌    | ❌   | |
| SO_OOBINLINE                    | ⛔       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Not supported, see [OOB](#oob) |
| SO_DEBUG                        | ❔       | ✅     | ✅    | ✅      | ✅     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_DONTROUTE                    | ❔       | ✅     | ✅    | ✅      | ✅     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_REUSEADDR                    | ❔       | ✅     | ✅    | ✅*     | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ✅  | ✅      | ✅    | ✅    | ❌    | ✅   | Roughly equivalent to the inverse of SO_EXCLUSIVEADDRUSE on Windows. |
| SO_REUSEPORT                    | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ✅    | ❌   | ✅      | ✅  | ❌      | ✅    | ❌    | ✅    | ❌   | |
| SO_REUSEPORT_LB                 | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| IP_BIND_ADDRESS_NO_PORT         | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ✅    | ❌    | ❌   | |
| SO_ATTACH_REUSEPORT_CBPF        | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ✅    | ❌   | |
| SO_ATTACH_REUSEPORT_EBPF        | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| SO_DETACH_REUSEPORT_BPF         | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_REUSPORT_LB_NUMA            | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_BROADCAST                    | ❔       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ✅  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_ADD_MEMBERSHIP               | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ✅  | ❌      | ❌    | ❌    | ❌    | ❌   | Equivalent to MCAST_JOIN_GROUP |
| IPV6_JOIN_GROUP                 | ❔       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ✅  | ❌      | ❌    | ❌    | ❌    | ❌   | Equivalent to MCAST_JOIN_GROUP, alias of IPV6_ADD_MEMBERSHIP |
| IP_ADD_SOURCE_MEMBERSHIP        | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Equivalent to MCAST_JOIN_SOURCE_GROUP |
| IP_DROP_MEMBERSHIP              | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Equivalent to MCAST_LEAVE_GROUP |
| IPV6_LEAVE_GROUP                | ❔       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Equivalent to MCAST_LEAVE_GROUP, alias of IPV6_DROP_MEMBERSHIP |
| IP_DROP_SOURCE_MEMBERSHIP       | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Equivalent to MCAST_LEAVE_SOURCE_GROUP |
| IP_MULTICAST_IF                 | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ✅  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_MULTICAST_IF               | ❔       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ✅  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_MULTICAST_LOOP               | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ✅  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_MULTICAST_LOOP             | ❔       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ✅  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_MULTICAST_TTL                | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ✅   | ✅      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_MULTICAST_HOPS             | ❔       | ✅     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ✅      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_BLOCK_SOURCE                 | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Equivalent to MCAST_BLOCK_SOURCE |
| IP_UNBLOCK_SOURCE               | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ✅    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Equivalent to MCAST_UNBLOCK_SOURCE |
| IP_MSFILTER                     | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_INFO                        | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ✅   | |
| TCP_FASTOPEN                    | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ✅      | ✅    | ❌    | ❌    | ✅   | |
| TCP_FASTOPEN_CONNECT            | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ✅      | ❌    | ✅    | ❌    | ✅   | |
| TCP_FASTOPEN_KEY                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_FASTOPEN_NO_COOKIE          | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_FASTOPEN_FORCE_ENABLE       | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_FASTOPEN_FORCE_HEURISTICS   | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_BINDTODEVICE                 | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ✅      | ✅  | ❌      | ❌    | ✅    | ❌    | ❌   | |
| SO_BINDTOIFINDEX                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_BOUND_IF                     | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_BOUND_IF                   | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_IPSEC_POLICY                 | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_MINTTL                       | ❔       | ❌     | ✅    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_MINHOPCOUNT                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_MTU                          | ❔       | ❌     | ✅    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ✅      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_MTU                        | ❔       | ❌     | ✅    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ✅      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_PATHMTU                    | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVPATHMTU                | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_USE_MIN_MTU                | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_OPTIONS                      | ❔       | ❌     | ✅    | ✅      | ✅     | ✅      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ✅   | |
| IP_RECVOPTS                     | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RECVORIGDSTADDR              | ❔       | ❌     | ✅    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | IP_ORIGDSTADDR on FreeBSD |
| IP_RECVRETOPTS                  | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Alias of IP_RETOPTS |
| IP_UNICAST_IF                   | ❔       | ❌     | ✅    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ✅    | ❌   | |
| IPV6_UNICAST_IF                 | ❔       | ❌     | ✅    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ✅    | ❌   | |
| IPV6_2292DSTOPTS                | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_2292HOPLIMIT               | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_2292HOPOPTS                | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_2292PKTINFO                | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_2292PKTOPTIONS             | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_2292RTHDR                  | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_AUTOFLOWLABEL              | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_CHECKSUM                   | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_DSTOPTS                    | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_HOPOPTS                    | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_IPSEC_POLICY               | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_NEXTHOP                    | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVDSTOPTS                | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVHOPOPTS                | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVORIGDSTADDR            | ❔       | ❌     | ✅    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | IPV6_ORIGDSTADDR on FreeBSD |
| IPV6_RECVRTHDR                  | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RTHDR                      | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RTHDRDSTOPTS               | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TIMESTAMP                    | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_CONGESTION                  | ❔       | ❌     | ✅    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_MAXSEG                      | ❔       | ❌     | ✅    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_MD5SIG                      | ❔       | ❌     | ✅    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_NOTSENT_LOWAT               | ❔       | ❌     | ✅    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| UDP_ENCAP                       | ❔       | ❌     | ✅    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_CHECKSUM                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_FREEBIND                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_MULTICAST_ALL                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_NODEFRAG                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_PASSSEC                      | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_PKTOPTIONS                   | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RECVERR                      | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ✅      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RECVERR_RFC4884              | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RECVFRAGSIZE                 | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_ROUTER_ALERT                 | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_TRANSPARENT                  | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| IP_XFRM_POLICY                  | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_ADDR_PREFERENCES           | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_ADDRFORM                   | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_AUTHHDR                    | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_FLOWINFO                   | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_FLOWINFO_SEND              | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_FLOWLABEL_MGR              | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_FREEBIND                   | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_JOIN_ANYCAST               | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_LEAVE_ANYCAST              | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_MULTICAST_ALL              | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVERR                    | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ✅      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVERR_RFC4884            | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVFRAGSIZE               | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_ROUTER_ALERT               | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_ROUTER_ALERT_ISOLATE       | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_TRANSPARENT                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| IPV6_XFRM_POLICY                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_ATTACH_FILTER                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_BPF_EXTENSIONS               | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_BSDCOMPAT                    | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_BUF_LOCK                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_BUSY_POLL                    | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_BUSY_POLL_BUDGET             | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_CNX_ADVICE                   | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_COOKIE                       | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| SO_DETACH_FILTER                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_INCOMING_CPU                 | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_INCOMING_NAPI_ID             | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ✅    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_LOCK_FILTER                  | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_MARK                         | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_MEMINFO                      | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NETNS_COOKIE                 | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NO_CHECK                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NOFCS                        | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PASSCRED                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PASSSEC                      | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PEEK_OFF                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PEERCRED                     | ⛔       | ❌     | ✅    | ❌      | ❌     | ❌      | ✅    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Out of scope; UNIX domain sockets only. |
| SO_PEERNAME                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PEERSEC                      | ⛔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | Out of scope; UNIX domain sockets only. |
| SO_PREFER_BUSY_POLL             | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PRIORITY                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_RCVBUFFORCE                  | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_RCVMARK                      | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_RESERVE_MEM                  | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_RXQ_OVFL                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_SELECT_ERR_QUEUE             | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_SNDBUFFORCE                  | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TIMESTAMPING                 | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TIMESTAMPNS                  | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TXREHASH                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TXTIME                       | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_WIFI_STATUS                  | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_ZEROCOPY                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_CC_INFO                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_CM_INQ                      | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_DEFER_ACCEPT                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| TCP_INQ                         | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_LINGER2                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_MD5SIG_EXT                  | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_QUEUE_SEQ                   | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_QUICKACK                    | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ✅    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ✅   | |
| TCP_REPAIR                      | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_REPAIR_OPTIONS              | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_REPAIR_QUEUE                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_REPAIR_WINDOW               | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_SAVE_SYN                    | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_SAVED_SYN                   | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_SYNCNT                      | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_THIN_DUPACK                 | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_THIN_LINEAR_TIMEOUTS        | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_TIMESTAMP                   | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_TX_DELAY                    | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_ULP                         | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_USER_TIMEOUT                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_WINDOW_CLAMP                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_ZEROCOPY_RECEIVE            | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| UDP_CORK                        | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| UDP_GRO                         | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ✅    | ❌   | |
| UDP_NO_CHECK6_RX                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| UDP_NO_CHECK6_TX                | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| UDP_SEGMENT                     | ❔       | ❌     | ✅    | ❌      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| IP_ADD_IFLIST                   | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_DEL_IFLIST                   | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_GET_IFLIST                   | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_IFLIST                       | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_ORIGINAL_ARRIVAL_IF          | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_ORIGINAL_ARRIVAL_IF          | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RECEIVE_BROADCAST            | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_USER_MTU                     | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_WFP_REDIRECT_CONTEXT         | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_WFP_REDIRECT_RECORDS         | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_ADD_IFLIST                 | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_DEL_IFLIST                 | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_GET_IFLIST                 | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_IFLIST                     | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_PROTECTION_LEVEL           | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVIF                     | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_USER_MTU                   | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_BSP_STATE                    | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_CONDITIONAL_ACCEPT           | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_CONNDATA                     | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_CONNDATALEN                  | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_CONNECT_TIME                 | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_CONNOPT                      | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_CONNOPTLEN                   | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_DISCDATA                     | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_DISCDATALEN                  | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_DISCOPT                      | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_DISCOPTLEN                   | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_GROUP_ID                     | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_GROUP_PRIORITY               | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_MAX_MSG_SIZE                 | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_MAXDG                        | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_MAXPATHDG                    | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_OPENTYPE                     | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PAUSE_ACCEPT                 | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PORT_SCALABILITY             | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PROTOCOL_INFO                | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PROTOCOL_INFOA               | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_PROTOCOL_INFOW               | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_RANDOMIZE_PORT               | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_REUSE_MULTICASTPORT          | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_REUSE_UNICASTPORT            | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ✅    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_UPDATE_ACCEPT_CONTEXT        | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_UPDATE_CONNECT_CONTEXT       | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_BSDURGENT                   | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_EXPEDITED_1122              | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_FAIL_CONNECT_ON_ICMP_ERROR  | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_ICMP_ERROR_INFO             | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_MAXRT                       | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_TIMESTAMPS                  | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| UDP_CHECKSUM_COVERAGE           | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| UDP_NOCHECKSUM                  | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| UDP_RECV_MAX_COALESCED_SIZE     | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ✅    | ❌   | |
| UDP_SEND_MSG_SIZE               | ❔       | ❌     | ❌    | ✅      | ❌     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ✅    | ❌   | |
| IP_FAITH                        | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_MULTICAST_IFINDEX            | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_NAT__XXX                     | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_STRIPHDR                     | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_TRAFFIC_MGT_BACKGROUND       | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_3542DSTOPTS                | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_3542HOPLIMIT               | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_3542HOPOPTS                | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_3542NEXTHOP                | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_3542PKTINFO                | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_3542RTHDR                  | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RTHDR_LOOSE                | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RTHDR_STRICT               | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RTHDR_TYPE_0               | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_AWDL_UNRESTRICTED            | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_CFIL_SOCK_ID                 | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_DELEGATED                    | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_DELEGATED_UUID               | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_DONTTRUNC                    | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_EXECPATH                     | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_EXTENDED_BK_IDLE             | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_FLOW_DIVERT_TOKEN            | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_FLUSH                        | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_INTCOPROC_ALLOW              | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_LINGER_SEC                   | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_MARK_CELLFALLBACK            | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_MPKL_SEND_INFO               | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NECP_ATTRIBUTES              | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NECP_CLIENTUUID              | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NECP_LISTENUUID              | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NET_SERVICE_TYPE             | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NETSVC_MARKING_LEVEL         | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NKE                          | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NOADDRERR                    | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NOAPNFALLBK                  | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NOTIFYCONFLICT               | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NOWAKEFROMSLEEP              | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NP_EXTENSIONS                | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NREAD                        | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NUMRCVPKT                    | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NWRITE                       | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_OPPORTUNISTIC                | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_QOSMARKING_POLICY_OVERRIDE   | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_RANDOMPORT                   | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_RECV_ANYIF                   | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_RESTRICTIONS                 | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_REUSESHAREUID                | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_STATISTICS_EVENT             | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TC_NET_SERVICE_OFFSET        | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TC_NETSVC_SIG                | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TIMESTAMP_CONTINUOUS         | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TIMESTAMP_MONOTONIC          | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TRAFFIC_MGT_BACKGROUND       | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_UPCALLCLOSEWAIT              | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_WANT_KEV_SOCKET_CLOSED       | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_WANTMORE                     | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_WANTOOBFLAG                  | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| MPTCP_ALTERNATE_PORT            | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| MPTCP_EXPECTED_PROGRESS_TARGET  | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| MPTCP_FORCE_ENABLE              | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| MPTCP_FORCE_VERSION             | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| MPTCP_SERVICE_TYPE              | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| PERSIST_TIMEOUT                 | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_ADAPTIVE_READ_TIMEOUT       | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_ADAPTIVE_WRITE_TIMEOUT      | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_CONNECTION_INFO             | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_CONNECTIONTIMEOUT           | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_DISABLE_BLACKHOLE_DETECTION | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_ECN_MODE                    | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_ENABLE_ECN                  | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_KEEPALIVE_OFFLOAD           | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_MEASURE_BW_BURST            | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_MEASURE_SND_BW              | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_NOTIFY_ACKNOWLEDGEMENT      | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_NOTIMEWAIT                  | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_PEER_PID                    | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_RXT_CONNDROPTIME            | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_RXT_FINDROP                 | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_RXT_MINIMUM_TIMEOUT         | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_SENDMOREACKS                | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| UDP_KEEPALIVE_OFFLOAD           | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| UDP_NOCKSUM                     | ❔       | ❌     | ❌    | ❌      | ✅     | ❌      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| ICMP6_FILTER                    | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_MULTICAST_VIF                | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_PORTRANGE                    | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ✅  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RSVP_OFF                     | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RSVP_ON                      | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RSVP_VIF_OFF                 | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RSVP_VIF_ON                  | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_2292NEXTHOP                | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_BINDV6ONLY                 | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_FAITH                      | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_MSFILTER                   | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_PKTOPTIONS                 | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_PORTRANGE                  | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ✅  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_PREFER_TEMPADDR            | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVRTHDRDSTOPTS           | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_ACCEPTFILTER                 | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| SO_LABEL                        | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NOSIGPIPE                    | ⛔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ✅      | ❌  | ❌      | ❌    | ✅    | ❌    | ❌   | Not supported, see [SIGPIPE](#sigpipe) |
| SO_PEERLABEL                    | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_USELOOPBACK                  | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_NOOPT                       | ❔       | ❌     | ❌    | ❌      | ✅     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_BINDANY                      | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| IP_BINDMULTI                    | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_FLOWID                       | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_FLOWTYPE                     | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_MAX_MEMBERSHIPS              | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_ONESBCAST                    | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RECVFLOWID                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RECVRSSBUCKETID              | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RSS_LISTEN_BUCKET            | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_RSSBUCKETID                  | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IP_SENDSRCADDR                  | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| IP_VLAN_PCP                     | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_AUTH_LEVEL                 | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_BINDANY                    | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| IPV6_BINDMULTI                  | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_ESP_NETWORK_LEVEL          | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_ESP_TRANS_LEVEL            | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_FLOWID                     | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_FLOWTYPE                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_IPCOMP_LEVEL               | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVFLOWID                 | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RECVRSSBUCKETID            | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RSS_LISTEN_BUCKET          | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_RSSBUCKETID                | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| IPV6_VLAN_PCP                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_BINTIME                      | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_LISTENINCQLEN                | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_LISTENQLEN                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ✅   | |
| SO_LISTENQLIMIT                 | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_MAX_PACING_RATE              | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NO_DDP                       | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_NO_OFFLOAD                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_RERROR                       | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_SETFIB                       | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ✅    | ❌    | ❌    | ❌   | |
| SO_TS_BINTIME                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TS_CLOCK                     | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TS_CLOCK_MAX                 | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TS_DEFAULT                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TS_MONOTONIC                 | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TS_REALTIME                  | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_TS_REALTIME_MICRO            | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| SO_USER_COOKIE                  | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_CCALGOOPT                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_DEFER_OPTIONS               | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_DELACK                      | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_FAST_RSM_HACK               | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_FIN_IS_RST                  | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_FUNCTION_ALIAS              | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_FUNCTION_BLK                | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_HDWR_RATE_CAP               | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_HDWR_UP_ONLY                | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_IDLE_REDUCE                 | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_IWND_NB                     | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_IWND_NSEG                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_KEEPINIT                    | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_LOG                         | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_LOG_LIMIT                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_LOG_TAG                     | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_LOGBUF                      | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_LOGDUMP                     | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_LOGDUMPID                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_LOGID                       | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_LOGID_CNT                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_LRD                         | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_MAXPEAKRATE                 | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_MAXUNACKTIME                | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_PCAP_IN                     | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_PCAP_OUT                    | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_PERF_INFO                   | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_PROC_ACCOUNTING             | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_REMOTE_UDP_ENCAPS_PORT      | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_RXTLS_ENABLE                | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_RXTLS_MODE                  | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_STATS                       | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_TXTLS_ENABLE                | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_TXTLS_MODE                  | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_USE_CMP_ACKS                | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |
| TCP_USER_LOG                    | ❔       | ❌     | ❌    | ❌      | ❌     | ✅      | ❌    | ❌    | ❌   | ❌      | ❌  | ❌      | ❌    | ❌    | ❌    | ❌   | |






[ip-name-lookup]: https://github.com/WebAssembly/wasi-sockets/blob/main/wit/ip-name-lookup.wit
[tcp-create-socket]: https://github.com/WebAssembly/wasi-sockets/blob/main/wit/tcp-create-socket.wit
[tcp]: https://github.com/WebAssembly/wasi-sockets/blob/main/wit/tcp.wit
[udp-create-socket]: https://github.com/WebAssembly/wasi-sockets/blob/main/wit/udp-create-socket.wit
[udp]: https://github.com/WebAssembly/wasi-sockets/blob/main/wit/udp.wit
[poll]: https://github.com/WebAssembly/wasi-poll/blob/main/wit/poll.wit
[streams]: https://github.com/WebAssembly/wasi-io/blob/main/wit/streams.wit
