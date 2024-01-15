# POSIX Compatibility

This document provides an overview of the POSIX interface along with common non-standard extensions and their mapping to functionalities provided by this proposal.


## General

### I/O completion polling (`poll`, `select`, `pselect`, `epoll_*` (non-standard), `kqueue` (non-standard)) <a name="select"></a>
Use the various `subscribe` methods to obtain a `pollable` handle. Then use that to wait for IO events using the [wasi:io/poll][poll] interface.

### Non-blocking mode (`FIONBIO`, `SOCK_NONBLOCK`, `O_NONBLOCK`) <a name="nonblock"></a>
All WASI sockets are non-blocking and can not be configured to block.
Blocking behaviour can be recreated in userland (or in wasi-libc) by calling [pollable::block][poll] on the relevant pollable.

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
- TCP: [`create-tcp-socket`][tcp-create-socket]
- UDP: [`create-udp-socket`][udp-create-socket]

### `connect`
- TCP: [`tcp-socket::start-connect`][tcp] & [`tcp-socket::finish-connect`][tcp]
- UDP: [`udp-socket::start-connect`][udp] & [`udp-socket::finish-connect`][udp]

### `bind`
- TCP: [`tcp-socket::start-bind`][tcp] & [`tcp-socket::finish-bind`][tcp]
- UDP: [`udp-socket::start-bind`][udp] & [`udp-socket::finish-bind`][udp]

### `listen`
- TCP: [`tcp-socket::start-listen`][tcp] & [`tcp-socket::finish-listen`][tcp]. The `backlog` parameter has been split out into a distinct function [`tcp-socket::set-listen-backlog-size`][tcp] ([See #34][34]).
- UDP: N/A

### `accept`, `accept4` (non-standard)
- TCP: [`tcp-socket::accept`][tcp]
- UDP: N/A

To collect the remote address, call `tcp-socket::remote-address` on the newly accepted client socket.

Some platforms provide an `accept4` variant with additional flags. None of these flags make sense in the context of this proposal. See [SOCK_NONBLOCK](#nonblock) & [SOCK_CLOEXEC](#cloexec).

### `getsockname`, `getpeername`
- TCP: [`tcp-socket::local-address`][tcp] & [`tcp-socket::remote-address`][tcp]
- UDP: [`udp-socket::local-address`][udp] & [`udp-socket::remote-address`][udp]

### `read`, `readv`, `recv`, `recvfrom`, `recvmsg`, `recvmmsg` (non-standard)

TCP sockets can be read using the `input-stream` returned by connect or accept.
UDP sockets can be read using the `incoming-datagram-stream` returned by `udp-socket::stream`.

The various POSIX functions should be implementable on top of these two resources.

None of the flags are directly present in WASI Sockets:
- `MSG_DONTWAIT`: This is [always the case](#nonblock).
- `MSG_OOB` on TCP sockets: [Not supported](#oob)
- `MSG_OOB` on UDP sockets: N/A
- `MSG_PEEK`: [No direct support](#peek)
- `MSG_TRUNC` on TCP sockets: N/A
- `MSG_TRUNC` on UDP sockets: Not needed, the returned data array always has the exact perfect size.
- `MSG_WAITALL` on TCP sockets: Emulatable in userspace.
- `MSG_WAITALL` on UDP sockets: N/A
- `MSG_EOR`: N/A (not supported on TCP & UDP sockets)
- `MSG_CMSG_CLOEXEC`: N/A (only used on Unix domain sockets)

Receiving ancillary messages: None supported as of yet. But see the various "RECV" socket options below.

### `write`, `writev`, `send`, `sendto`, `sendmsg`, `sendmmsg` (non-standard)

TCP sockets can be written to using the `output-stream` returned by connect or accept.
UDP sockets can be written to using the `outgoing-datagram-stream` returned by `udp-socket::stream`.

The various POSIX functions should be implementable on top of these two resources.

None of the flags are directly present in WASI Sockets:
- `MSG_DONTROUTE`: Not included in proposal at the moment.
- `MSG_DONTWAIT`: This is [always the case](#nonblock).
- `MSG_NOSIGNAL`: This is [always the case](#sigpipe).
- `MSG_OOB` on TCP sockets: [Not supported](#oob)
- `MSG_OOB` on UDP sockets: N/A
- `MSG_EOR`: N/A (not supported on TCP & UDP sockets)

Sending ancillary messages: None supported as of yet.

### `sendfile` (non-standard)
- TCP: Part of the [wasi:io/streams][streams] proposal as `output-stream::splice`
- UDP: N/A

### `shutdown`
- TCP: [`tcp-socket::shutdown`][tcp]
- UDP: N/A

### `sockatmark`
- TCP: Not supported, see [OOB](#oob).
- UDP: N/A

### `close`
Dropping the socket resource effectively performs a `close`.

### `socketpair`, `connectat` (non-standard), `bindat` (non-standard)
Specifically for UNIX domain sockets. Out of scope for this proposal.

### `fcntl`
- `F_GETFL`/`F_SETFL` > `O_NONBLOCK`: [Not needed](#nonblock).
- `F_SETFD`/`F_GETFD` > `FD_CLOEXEC`: [Not included](#cloexec).

### `ioctl`
- `SIOCATMARK`: [Not included](#oob).
- `FIONREAD`: Currently not included. See [#17][17].

### `getsockopt`, `setsockopt`
Socket options have been split out into distinct functions. See table below.




## Socket options

POSIX defines the signatures of the `getsockopt` & `setsockopt` functions, but does not provide much guidance on the individual socket options themselves.
Because of this lack of a central authority, a list has been compiled of the options that are used "in the wild".

The results are not intended to be an exhaustive overview of all possible network applications, but rather to provide input on which options are worth standardizing in WASI.

Additionally, most columns have been populated semi-automatically by grepping through the respective codebases. The results have not been manually verified and therefore may not be 100% correct.

Legend:
- ✅ = Included in proposal.
- ⚠️ = Partially supported.
- ⛔ = Consciously decided _not_ to include in WASI. See notes for explanation.
- ❔ = Not included (yet), for no particular reason.


|    | Option                           | Notes                                   | Used/implemented by |
|----| ---------------------------------|-----------------------------------------|---------------------|
| ✅ | SO_DOMAIN <br/><sub>SO_PROTOCOL_INFO on Windows</sub> | [`tcp-socket::address-family`][tcp]<br/>[`udp-socket::address-family`][udp] | linux, windows, freebsd, .net |
| ✅ | SO_ACCEPTCONN                   | [`tcp-socket::is-listening`][tcp]                                                                 | posix, linux, windows, macos, freebsd, .net |
| ✅ | IP_TTL                          | [`tcp-socket::(set-)hop-limit`][tcp]<br/>[`udp-socket::(set-)unicast-hop-limit`][udp]             | linux, windows, macos, freebsd, jvm, .net, rust, libuv |
| ✅ | IPV6_UNICAST_HOPS               | [`tcp-socket::(set-)hop-limit`][tcp]<br/>[`udp-socket::(set-)unicast-hop-limit`][udp]             | posix, linux, windows, macos, freebsd, jvm, .net, libuv |
| ✅ | SO_RCVBUF                       | [`tcp-socket::(set-)receive-buffer-size`][tcp]<br/>[`udp-socket::(set-)receive-buffer-size`][udp] | posix, linux, windows, macos, freebsd, jvm, .net, libuv, go, nginx, msquic |
| ✅ | SO_SNDBUF                       | [`tcp-socket::(set-)send-buffer-size`][tcp]<br/>[`udp-socket::(set-)send-buffer-size`][udp]       | posix, linux, windows, macos, freebsd, jvm, .net, libuv, go, nginx, curl |
| ✅ | SO_KEEPALIVE                    | [`tcp-socket::(set-)keep-alive-enabled`][tcp]                                                     | posix, linux, windows, macos, freebsd, jvm, .net, libuv, go, openssl, nginx, curl, exim |
| ✅ | TCP_KEEPIDLE <br/><sub>TCP_KEEPALIVE on MacOS</sub> | [`tcp-socket::(set-)keep-alive-idle-time`][tcp]                               | linux, windows, macos, freebsd, jvm, .net, libuv, go, nginx, curl |
| ✅ | TCP_KEEPINTVL                   | [`tcp-socket::(set-)keep-alive-interval`][tcp]                                                    | linux, windows, macos, freebsd, jvm, .net, libuv, go, nginx, curl |
| ✅ | TCP_KEEPCNT                     | [`tcp-socket::(set-)keep-alive-count`][tcp]                                                       | linux, windows, macos, freebsd, jvm, .net, libuv, nginx |
| ✅ | SO_REUSEADDR for TCP            | Enabled by default. See [`tcp-socket::bind`][tcp]                              | posix, linux, windows, macos, freebsd, jvm, .net, libuv, go, openssl, nginx, curl, exim |
| ⚠️ | IPV6_V6ONLY                     | In WASI this always `true`. [#1][1]                                            | posix, linux, windows, macos, freebsd, jvm, .net, libuv, go, openssl, curl, msquic, exim |
| ⛔ | SO_ERROR                        | Not necessary. WIT has (or will have) native support for asynchronous results. | posix, linux, windows, macos, freebsd, jvm, .net, rust, libuv, go, openssl, nginx, curl, msquic |
| ⛔ | SO_TYPE                         | Can be inferred from the socket resource type.                                 | posix, linux, windows, macos, freebsd, jvm, .net, go, openssl, nginx, curl, exim |
| ⛔ | SO_PROTOCOL <br/><sub>SO_PROTOCOL_INFO on Windows</sub> | Can be inferred from the socket resource type.         | linux, windows, freebsd, .net, exim |
| ⛔ | IP_HDRINCL                      | Out of scope. Raw sockets only.                                                | linux, windows, macos, freebsd, .net |
| ⛔ | IPV6_HDRINCL                    | Out of scope. Raw sockets only.                                                | linux, windows, .net |
| ⛔ | SO_RCVTIMEO                     | WASI sockets are always non-blocking. Timeouts can be recreated in libc.       | posix, linux, windows, macos, freebsd, jvm, .net, rust, openssl, curl |
| ⛔ | SO_SNDTIMEO                     | WASI sockets are always non-blocking. Timeouts can be recreated in libc.       | posix, windows, macos, freebsd, .net, rust, openssl |
| ⛔ | SO_OOBINLINE                    | Not supported, see [OOB](#oob)                                                 | posix, linux, windows, macos, freebsd, jvm, .net |
| ⛔ | SO_PEERCRED                     | Out of scope; UNIX domain sockets only.                                        | linux, jvm, .net |
| ⛔ | SO_PEERSEC                      | Out of scope; UNIX domain sockets only.                                        | linux |
| ⛔ | SO_NOSIGPIPE                    | Not supported, see [SIGPIPE](#sigpipe)                                         | macos, freebsd, libuv, curl |
| ❔ | IP_RECVPKTINFO <br/><sub>IP_PKTINFO on Linux & Windows</sub><br/><sub>IP_RECVDSTADDR+IP_RECVIF on MacOS & FreeBSD</sub> | [#77][77] | linux, windows, macos, freebsd, .net, openssl, nginx, msquic |
| ❔ | IPV6_RECVPKTINFO <br/><sub>IPV6_PKTINFO on Windows</sub> | [#77][77]       | linux, windows, macos, freebsd, .net, openssl, nginx, msquic |
| ❔ | IP_RECVTOS                      | [#78][78]                                | linux, windows, macos, freebsd, msquic |
| ❔ | IPV6_RECVTCLASS                 | [#78][78]                                | linux, windows, macos, freebsd, msquic |
| ❔ | IP_TOS                          | [#78][78]                                | linux, windows, macos, freebsd, jvm, .net, exim |
| ❔ | IPV6_TCLASS                     | [#78][78]                                | linux, macos, freebsd, jvm, .net, exim   |
| ❔ | TCP_ECN_MODE                    | [#78][78]                                | macos |
| ❔ | TCP_ENABLE_ECN                  | [#78][78]                                | macos |
| ❔ | SO_LINGER                       | [#80][80]                                | posix, linux, windows, macos, freebsd, jvm, .net, rust, libuv, go, openssl, nginx |
| ❔ | IP_DONTFRAG <br/><sub>IP_DONTFRAGMENT on Windows</sub> | [#79][79]         | linux, windows, macos, freebsd, jvm, .net, openssl, nginx, msquic |
| ❔ | IPV6_DONTFRAG                   | [#79][79]                                | linux, windows, macos, freebsd, jvm, .net, openssl, nginx, msquic |
| ❔ | IP_MTU_DISCOVER                 | [#79][79]                                | linux, windows, openssl, nginx, curl, msquic |
| ❔ | IPV6_MTU_DISCOVER               | [#79][79]                                | linux, windows, openssl, nginx, curl, msquic |
| ❔ | TCP_NODELAY                     | [#75][75]                                | posix, linux, windows, macos, freebsd, jvm, .net, rust, libuv, go, openssl, nginx, curl, exim |
| ❔ | TCP_CORK <br/><sub>TCP_NOPUSH on MacOS & FreeBSD</sub> | [#75][75]         | linux, macos, freebsd, nginx, exim |
| ❔ | SO_REUSEADDR for UDP            | [#74][74]                                | posix, linux, windows, macos, freebsd, jvm, .net, libuv, go, openssl, nginx, curl, exim |
| ❔ | SO_EXCLUSIVEADDRUSE             | [#74][74]                                | windows |
| ❔ | SO_RANDOMIZE_PORT               | [#74][74]                                | windows |
| ❔ | SO_RANDOMPORT                   | [#74][74]                                | macos |
| ❔ | IP_BIND_ADDRESS_NO_PORT         | [#74][74]                                | linux, nginx, curl |
| ❔ | SO_PORT_SCALABILITY             | [#74][74]                                | windows |
| ❔ | SO_REUSE_UNICASTPORT            | [#74][74]                                | windows, .net |
| ❔ | SO_REUSEPORT                    | [#74][74]                                | linux, macos, freebsd, .net, libuv, go, nginx, msquic |
| ❔ | SO_REUSEPORT_LB                 | [#74][74]                                | freebsd, nginx |
| ❔ | SO_ATTACH_REUSEPORT_CBPF        | [#74][74]                                | linux, msquic |
| ❔ | SO_ATTACH_REUSEPORT_EBPF        | [#74][74]                                | linux, nginx |
| ❔ | SO_DETACH_REUSEPORT_BPF         | [#74][74]                                | linux |
| ❔ | TCP_REUSPORT_LB_NUMA            | [#74][74]                                | freebsd |
| ❔ | SO_INCOMING_CPU                 | [#74][74]                                | linux |
| ❔ | SO_INCOMING_NAPI_ID             | [#74][74]                                | linux, jvm |
| ❔ | SO_BINDTODEVICE                 | [#74][74]                                | linux, libuv, go, curl |
| ❔ | SO_BINDTOIFINDEX                | [#74][74]                                | linux |
| ❔ | IP_UNICAST_IF                   | [#74][74]                                | linux, windows, msquic |
| ❔ | IPV6_UNICAST_IF                 | [#74][74]                                | linux, windows, msquic |
| ❔ | IP_BOUND_IF                     | [#74][74]                                | macos |
| ❔ | IPV6_BOUND_IF                   | [#74][74]                                | macos |
| ❔ | IP_FREEBIND                     | [#74][74]                                | linux |
| ❔ | IPV6_FREEBIND                   | [#74][74]                                | linux |
| ❔ | IP_TRANSPARENT                  | [#74][74]                                | linux, nginx |
| ❔ | IPV6_TRANSPARENT                | [#74][74]                                | linux, nginx |
| ❔ | IP_BINDANY                      | [#74][74]                                | freebsd, nginx |
| ❔ | IPV6_BINDANY                    | [#74][74]                                | freebsd, nginx |
| ❔ | SO_REUSE_MULTICASTPORT          | [#74][74], [#73][73]                     | windows |
| ❔ | SO_BROADCAST                    | [#73][73]                                | posix, linux, windows, macos, freebsd, jvm, .net, rust, libuv, go |
| ❔ | MCAST_JOIN_GROUP <br/><sub>Supersedes: IP_ADD_MEMBERSHIP</sub><br/><sub>Supersedes: IPV6_JOIN_GROUP</sub><br/><sub>Supersedes: IPV6_ADD_MEMBERSHIP</sub> | [#73][73] | posix, linux, windows, macos, freebsd, jvm, .net, rust, libuv, go |
| ❔ | MCAST_LEAVE_GROUP <br/><sub>Supersedes: IP_DROP_MEMBERSHIP</sub><br/><sub>Supersedes: IPV6_LEAVE_GROUP</sub><br/><sub>Supersedes: IPV6_DROP_MEMBERSHIP</sub> | [#73][73] | posix, linux, windows, macos, freebsd, jvm, .net, rust, libuv |
| ❔ | MCAST_JOIN_SOURCE_GROUP <br/><sub>Supersedes: IP_ADD_SOURCE_MEMBERSHIP</sub>    | [#73][73] | linux, windows, macos, freebsd, jvm, .net, libuv |
| ❔ | MCAST_LEAVE_SOURCE_GROUP <br/><sub>Supersedes: IP_DROP_SOURCE_MEMBERSHIP</sub>  | [#73][73] | linux, windows, macos, freebsd, jvm, .net, libuv |
| ❔ | MCAST_BLOCK_SOURCE <br/><sub>Supersedes: IP_BLOCK_SOURCE</sub>                  | [#73][73] | linux, windows, macos, freebsd, jvm, .net |
| ❔ | MCAST_UNBLOCK_SOURCE <br/><sub>Supersedes: IP_UNBLOCK_SOURCE</sub>              | [#73][73] | linux, windows, macos, freebsd, jvm, .net |
| ❔ | IP_MSFILTER                     | [#73][73]                                | linux, windows, macos, freebsd |
| ❔ | IPV6_MSFILTER                   | [#73][73]                                | macos, freebsd |
| ❔ | IP_MULTICAST_IF                 | [#73][73]                                | linux, windows, macos, freebsd, jvm, .net, libuv, go |
| ❔ | IPV6_MULTICAST_IF               | [#73][73]                                | posix, linux, windows, macos, freebsd, jvm, .net, libuv, go |
| ❔ | IP_MULTICAST_LOOP               | [#73][73]                                | linux, windows, macos, freebsd, jvm, .net, rust, libuv, go |
| ❔ | IPV6_MULTICAST_LOOP             | [#73][73]                                | posix, linux, windows, macos, freebsd, jvm, .net, rust, libuv, go |
| ❔ | IP_MULTICAST_TTL                | [#73][73]                                | linux, windows, macos, freebsd, jvm, .net, rust, libuv |
| ❔ | IPV6_MULTICAST_HOPS             | [#73][73]                                | posix, linux, windows, macos, freebsd, jvm, .net, libuv |
| ❔ | IP_MULTICAST_ALL                | [#73][73]                                | linux |
| ❔ | IPV6_MULTICAST_ALL              | [#73][73]                                | linux |
| ❔ | IP_MULTICAST_IFINDEX            | [#73][73]                                | macos |
| ❔ | TCP_FASTOPEN                    | [#81][81]                                | linux, windows, macos, freebsd, openssl, nginx, exim |
| ❔ | TCP_FASTOPEN_CONNECT            | [#81][81]                                | linux, openssl, curl, exim   |
| ❔ | TCP_FASTOPEN_KEY                | [#81][81]                                | linux |
| ❔ | TCP_FASTOPEN_NO_COOKIE          | [#81][81]                                | linux |
| ❔ | TCP_FASTOPEN_FORCE_ENABLE       | [#81][81]                                | macos |
| ❔ | TCP_FASTOPEN_FORCE_HEURISTICS   | [#81][81]                                | macos |
| ❔ | SO_SNDLOWAT                     | Not usefully implemented on Linux & Windows. | posix, linux, macos, freebsd, .net, nginx |
| ❔ | SO_RCVLOWAT                     |                                          | posix, linux, macos, freebsd, .net |
| ❔ | IP_RECVTTL                      |                                          | linux, windows, macos, freebsd |
| ❔ | IPV6_RECVHOPLIMIT               |                                          | linux, macos, freebsd |
| ❔ | SO_DEBUG                        |                                          | posix, linux, windows, macos, freebsd, .net |
| ❔ | SO_DONTROUTE                    |                                          | posix, linux, windows, macos, freebsd, .net |
| ❔ | TCP_INFO <br/><sub>via ioctl on Windows</sub> |                            | linux, windows, macos, freebsd, nginx, exim |
| ❔ | IP_IPSEC_POLICY                 |                                          | linux, macos, freebsd |
| ❔ | IP_MINTTL                       |                                          | linux, freebsd |
| ❔ | IPV6_MINHOPCOUNT                |                                          | linux |
| ❔ | IP_MTU                          |                                          | linux, windows, openssl |
| ❔ | IPV6_MTU                        |                                          | linux, windows, openssl |
| ❔ | IPV6_PATHMTU                    |                                          | linux, macos, freebsd |
| ❔ | IPV6_RECVPATHMTU                |                                          | linux, macos, freebsd |
| ❔ | IPV6_USE_MIN_MTU                |                                          | linux, macos, freebsd |
| ❔ | IP_OPTIONS                      |                                          | linux, windows, macos, freebsd, .net, exim |
| ❔ | IP_RECVOPTS                     |                                          | linux, macos, freebsd |
| ❔ | IP_RECVORIGDSTADDR <br/><sub>IP_ORIGDSTADDR on FreeBSD</sub> |             | linux, freebsd |
| ❔ | IP_RECVRETOPTS <br/><sub>Alias: IP_RETOPTS</sub> |                         | linux, macos, freebsd |
| ❔ | IPV6_2292DSTOPTS                |                                          | linux, macos, freebsd |
| ❔ | IPV6_2292HOPLIMIT               |                                          | linux, macos, freebsd |
| ❔ | IPV6_2292HOPOPTS                |                                          | linux, macos, freebsd |
| ❔ | IPV6_2292PKTINFO                |                                          | linux, macos, freebsd |
| ❔ | IPV6_2292PKTOPTIONS             |                                          | linux, macos, freebsd |
| ❔ | IPV6_2292RTHDR                  |                                          | linux, macos, freebsd |
| ❔ | IPV6_AUTOFLOWLABEL              |                                          | linux, macos, freebsd |
| ❔ | IPV6_CHECKSUM                   |                                          | linux, macos, freebsd |
| ❔ | IPV6_DSTOPTS                    |                                          | linux, macos, freebsd |
| ❔ | IPV6_HOPOPTS                    |                                          | linux, macos, freebsd |
| ❔ | IPV6_IPSEC_POLICY               |                                          | linux, macos, freebsd |
| ❔ | IPV6_NEXTHOP                    |                                          | linux, macos, freebsd |
| ❔ | IPV6_RECVDSTOPTS                |                                          | linux, macos, freebsd |
| ❔ | IPV6_RECVHOPOPTS                |                                          | linux, macos, freebsd |
| ❔ | IPV6_RECVORIGDSTADDR <br/><sub>IPV6_ORIGDSTADDR on FreeBSD</sub> |         | linux, freebsd |
| ❔ | IPV6_RECVRTHDR                  |                                          | linux, macos, freebsd |
| ❔ | IPV6_RTHDR                      |                                          | linux, macos, freebsd |
| ❔ | IPV6_RTHDRDSTOPTS               |                                          | linux, macos, freebsd |
| ❔ | SO_TIMESTAMP                    |                                          | linux, macos, freebsd |
| ❔ | TCP_CONGESTION                  |                                          | linux, freebsd |
| ❔ | TCP_MAXSEG                      |                                          | linux, macos, freebsd |
| ❔ | TCP_MD5SIG                      |                                          | linux, freebsd |
| ❔ | TCP_NOTSENT_LOWAT               |                                          | linux, macos |
| ❔ | UDP_ENCAP                       |                                          | linux, freebsd |
| ❔ | IP_CHECKSUM                     |                                          | linux |
| ❔ | IP_NODEFRAG                     |                                          | linux |
| ❔ | IP_PASSSEC                      |                                          | linux |
| ❔ | IP_PKTOPTIONS                   |                                          | linux |
| ❔ | IP_RECVERR                      |                                          | linux, libuv |
| ❔ | IP_RECVERR_RFC4884              |                                          | linux |
| ❔ | IP_RECVFRAGSIZE                 |                                          | linux |
| ❔ | IP_ROUTER_ALERT                 |                                          | linux |
| ❔ | IP_XFRM_POLICY                  |                                          | linux |
| ❔ | IPV6_ADDR_PREFERENCES           |                                          | linux |
| ❔ | IPV6_ADDRFORM                   |                                          | linux |
| ❔ | IPV6_AUTHHDR                    |                                          | linux |
| ❔ | IPV6_FLOWINFO                   |                                          | linux |
| ❔ | IPV6_FLOWINFO_SEND              |                                          | linux |
| ❔ | IPV6_FLOWLABEL_MGR              |                                          | linux |
| ❔ | IPV6_JOIN_ANYCAST               |                                          | linux |
| ❔ | IPV6_LEAVE_ANYCAST              |                                          | linux |
| ❔ | IPV6_RECVERR                    |                                          | linux, libuv |
| ❔ | IPV6_RECVERR_RFC4884            |                                          | linux |
| ❔ | IPV6_RECVFRAGSIZE               |                                          | linux |
| ❔ | IPV6_ROUTER_ALERT               |                                          | linux |
| ❔ | IPV6_ROUTER_ALERT_ISOLATE       |                                          | linux |
| ❔ | IPV6_XFRM_POLICY                |                                          | linux |
| ❔ | SO_ATTACH_FILTER                |                                          | linux |
| ❔ | SO_BPF_EXTENSIONS               |                                          | linux |
| ❔ | SO_BSDCOMPAT                    |                                          | linux |
| ❔ | SO_BUF_LOCK                     |                                          | linux |
| ❔ | SO_BUSY_POLL                    |                                          | linux |
| ❔ | SO_BUSY_POLL_BUDGET             |                                          | linux |
| ❔ | SO_CNX_ADVICE                   |                                          | linux |
| ❔ | SO_COOKIE                       |                                          | linux, nginx |
| ❔ | SO_DETACH_FILTER                |                                          | linux |
| ❔ | SO_LOCK_FILTER                  |                                          | linux |
| ❔ | SO_MARK                         |                                          | linux |
| ❔ | SO_MEMINFO                      |                                          | linux |
| ❔ | SO_NETNS_COOKIE                 |                                          | linux |
| ❔ | SO_NO_CHECK                     |                                          | linux |
| ❔ | SO_NOFCS                        |                                          | linux |
| ❔ | SO_PASSCRED                     |                                          | linux |
| ❔ | SO_PASSSEC                      |                                          | linux |
| ❔ | SO_PEEK_OFF                     |                                          | linux |
| ❔ | SO_PEERNAME                     |                                          | linux |
| ❔ | SO_PREFER_BUSY_POLL             |                                          | linux |
| ❔ | SO_PRIORITY                     |                                          | linux |
| ❔ | SO_RCVBUFFORCE                  |                                          | linux |
| ❔ | SO_RCVMARK                      |                                          | linux |
| ❔ | SO_RESERVE_MEM                  |                                          | linux |
| ❔ | SO_RXQ_OVFL                     |                                          | linux |
| ❔ | SO_SELECT_ERR_QUEUE             |                                          | linux |
| ❔ | SO_SNDBUFFORCE                  |                                          | linux |
| ❔ | SO_TIMESTAMPING                 |                                          | linux |
| ❔ | SO_TIMESTAMPNS                  |                                          | linux |
| ❔ | SO_TXREHASH                     |                                          | linux |
| ❔ | SO_TXTIME                       |                                          | linux |
| ❔ | SO_WIFI_STATUS                  |                                          | linux |
| ❔ | SO_ZEROCOPY                     |                                          | linux |
| ❔ | TCP_CC_INFO                     |                                          | linux |
| ❔ | TCP_CM_INQ                      |                                          | linux |
| ❔ | TCP_DEFER_ACCEPT                |                                          | linux, nginx |
| ❔ | TCP_INQ                         |                                          | linux |
| ❔ | TCP_LINGER2                     |                                          | linux |
| ❔ | TCP_MD5SIG_EXT                  |                                          | linux |
| ❔ | TCP_QUEUE_SEQ                   |                                          | linux |
| ❔ | TCP_QUICKACK                    |                                          | linux, jvm, exim |
| ❔ | TCP_REPAIR                      |                                          | linux |
| ❔ | TCP_REPAIR_OPTIONS              |                                          | linux |
| ❔ | TCP_REPAIR_QUEUE                |                                          | linux |
| ❔ | TCP_REPAIR_WINDOW               |                                          | linux |
| ❔ | TCP_SAVE_SYN                    |                                          | linux |
| ❔ | TCP_SAVED_SYN                   |                                          | linux |
| ❔ | TCP_SYNCNT                      |                                          | linux |
| ❔ | TCP_THIN_DUPACK                 |                                          | linux |
| ❔ | TCP_THIN_LINEAR_TIMEOUTS        |                                          | linux |
| ❔ | TCP_TIMESTAMP                   |                                          | linux |
| ❔ | TCP_TX_DELAY                    |                                          | linux |
| ❔ | TCP_ULP                         |                                          | linux |
| ❔ | TCP_USER_TIMEOUT                |                                          | linux |
| ❔ | TCP_WINDOW_CLAMP                |                                          | linux |
| ❔ | TCP_ZEROCOPY_RECEIVE            |                                          | linux |
| ❔ | UDP_CORK                        |                                          | linux |
| ❔ | UDP_GRO                         |                                          | linux, msquic |
| ❔ | UDP_NO_CHECK6_RX                |                                          | linux |
| ❔ | UDP_NO_CHECK6_TX                |                                          | linux |
| ❔ | UDP_SEGMENT                     |                                          | linux, nginx |
| ❔ | IP_ADD_IFLIST                   |                                          | windows |
| ❔ | IP_DEL_IFLIST                   |                                          | windows |
| ❔ | IP_GET_IFLIST                   |                                          | windows |
| ❔ | IP_IFLIST                       |                                          | windows |
| ❔ | IP_ORIGINAL_ARRIVAL_IF          |                                          | windows |
| ❔ | IP_ORIGINAL_ARRIVAL_IF          |                                          | windows |
| ❔ | IP_RECEIVE_BROADCAST            |                                          | windows |
| ❔ | IP_USER_MTU                     |                                          | windows |
| ❔ | IP_WFP_REDIRECT_CONTEXT         |                                          | windows |
| ❔ | IP_WFP_REDIRECT_RECORDS         |                                          | windows |
| ❔ | IPV6_ADD_IFLIST                 |                                          | windows |
| ❔ | IPV6_DEL_IFLIST                 |                                          | windows |
| ❔ | IPV6_GET_IFLIST                 |                                          | windows |
| ❔ | IPV6_IFLIST                     |                                          | windows |
| ❔ | IPV6_PROTECTION_LEVEL           |                                          | windows |
| ❔ | IPV6_RECVIF                     |                                          | windows |
| ❔ | IPV6_USER_MTU                   |                                          | windows |
| ❔ | SO_BSP_STATE                    |                                          | windows |
| ❔ | SO_CONDITIONAL_ACCEPT           |                                          | windows |
| ❔ | SO_CONNDATA                     |                                          | windows |
| ❔ | SO_CONNDATALEN                  |                                          | windows |
| ❔ | SO_CONNECT_TIME                 |                                          | windows |
| ❔ | SO_CONNOPT                      |                                          | windows |
| ❔ | SO_CONNOPTLEN                   |                                          | windows |
| ❔ | SO_DISCDATA                     |                                          | windows |
| ❔ | SO_DISCDATALEN                  |                                          | windows |
| ❔ | SO_DISCOPT                      |                                          | windows |
| ❔ | SO_DISCOPTLEN                   |                                          | windows |
| ❔ | SO_GROUP_ID                     |                                          | windows |
| ❔ | SO_GROUP_PRIORITY               |                                          | windows |
| ❔ | SO_MAX_MSG_SIZE                 |                                          | windows |
| ❔ | SO_MAXDG                        |                                          | windows |
| ❔ | SO_MAXPATHDG                    |                                          | windows |
| ❔ | SO_OPENTYPE                     |                                          | windows |
| ❔ | SO_PAUSE_ACCEPT                 |                                          | windows |
| ❔ | SO_PROTOCOL_INFO                |                                          | windows |
| ❔ | SO_PROTOCOL_INFOA               |                                          | windows |
| ❔ | SO_PROTOCOL_INFOW               |                                          | windows |
| ❔ | SO_UPDATE_ACCEPT_CONTEXT        |                                          | windows |
| ❔ | SO_UPDATE_CONNECT_CONTEXT       |                                          | windows |
| ❔ | TCP_BSDURGENT                   |                                          | windows |
| ❔ | TCP_EXPEDITED_1122              |                                          | windows |
| ❔ | TCP_FAIL_CONNECT_ON_ICMP_ERROR  |                                          | windows |
| ❔ | TCP_ICMP_ERROR_INFO             |                                          | windows |
| ❔ | TCP_MAXRT                       |                                          | windows |
| ❔ | TCP_TIMESTAMPS                  |                                          | windows |
| ❔ | UDP_CHECKSUM_COVERAGE           |                                          | windows |
| ❔ | UDP_NOCHECKSUM                  |                                          | windows |
| ❔ | UDP_RECV_MAX_COALESCED_SIZE     |                                          | windows, msquic |
| ❔ | UDP_SEND_MSG_SIZE               |                                          | windows, msquic |
| ❔ | IP_FAITH                        |                                          | macos |
| ❔ | IP_NAT__XXX                     |                                          | macos |
| ❔ | IP_STRIPHDR                     |                                          | macos |
| ❔ | IP_TRAFFIC_MGT_BACKGROUND       |                                          | macos |
| ❔ | IPV6_3542DSTOPTS                |                                          | macos |
| ❔ | IPV6_3542HOPLIMIT               |                                          | macos |
| ❔ | IPV6_3542HOPOPTS                |                                          | macos |
| ❔ | IPV6_3542NEXTHOP                |                                          | macos |
| ❔ | IPV6_3542PKTINFO                |                                          | macos |
| ❔ | IPV6_3542RTHDR                  |                                          | macos |
| ❔ | IPV6_RTHDR_LOOSE                |                                          | macos |
| ❔ | IPV6_RTHDR_STRICT               |                                          | macos |
| ❔ | IPV6_RTHDR_TYPE_0               |                                          | macos |
| ❔ | SO_AWDL_UNRESTRICTED            |                                          | macos |
| ❔ | SO_CFIL_SOCK_ID                 |                                          | macos |
| ❔ | SO_DELEGATED                    |                                          | macos |
| ❔ | SO_DELEGATED_UUID               |                                          | macos |
| ❔ | SO_DONTTRUNC                    |                                          | macos |
| ❔ | SO_EXECPATH                     |                                          | macos |
| ❔ | SO_EXTENDED_BK_IDLE             |                                          | macos |
| ❔ | SO_FLOW_DIVERT_TOKEN            |                                          | macos |
| ❔ | SO_FLUSH                        |                                          | macos |
| ❔ | SO_INTCOPROC_ALLOW              |                                          | macos |
| ❔ | SO_LINGER_SEC                   |                                          | macos |
| ❔ | SO_MARK_CELLFALLBACK            |                                          | macos |
| ❔ | SO_MPKL_SEND_INFO               |                                          | macos |
| ❔ | SO_NECP_ATTRIBUTES              |                                          | macos |
| ❔ | SO_NECP_CLIENTUUID              |                                          | macos |
| ❔ | SO_NECP_LISTENUUID              |                                          | macos |
| ❔ | SO_NET_SERVICE_TYPE             |                                          | macos |
| ❔ | SO_NETSVC_MARKING_LEVEL         |                                          | macos |
| ❔ | SO_NKE                          |                                          | macos |
| ❔ | SO_NOADDRERR                    |                                          | macos |
| ❔ | SO_NOAPNFALLBK                  |                                          | macos |
| ❔ | SO_NOTIFYCONFLICT               |                                          | macos |
| ❔ | SO_NOWAKEFROMSLEEP              |                                          | macos |
| ❔ | SO_NP_EXTENSIONS                |                                          | macos |
| ❔ | SO_NREAD                        |                                          | macos |
| ❔ | SO_NUMRCVPKT                    |                                          | macos |
| ❔ | SO_NWRITE                       |                                          | macos |
| ❔ | SO_OPPORTUNISTIC                |                                          | macos |
| ❔ | SO_QOSMARKING_POLICY_OVERRIDE   |                                          | macos |
| ❔ | SO_RECV_ANYIF                   |                                          | macos |
| ❔ | SO_RESTRICTIONS                 |                                          | macos |
| ❔ | SO_REUSESHAREUID                |                                          | macos |
| ❔ | SO_STATISTICS_EVENT             |                                          | macos |
| ❔ | SO_TC_NET_SERVICE_OFFSET        |                                          | macos |
| ❔ | SO_TC_NETSVC_SIG                |                                          | macos |
| ❔ | SO_TIMESTAMP_CONTINUOUS         |                                          | macos |
| ❔ | SO_TIMESTAMP_MONOTONIC          |                                          | macos |
| ❔ | SO_TRAFFIC_MGT_BACKGROUND       |                                          | macos |
| ❔ | SO_UPCALLCLOSEWAIT              |                                          | macos |
| ❔ | SO_WANT_KEV_SOCKET_CLOSED       |                                          | macos |
| ❔ | SO_WANTMORE                     |                                          | macos |
| ❔ | SO_WANTOOBFLAG                  |                                          | macos |
| ❔ | MPTCP_ALTERNATE_PORT            |                                          | macos |
| ❔ | MPTCP_EXPECTED_PROGRESS_TARGET  |                                          | macos |
| ❔ | MPTCP_FORCE_ENABLE              |                                          | macos |
| ❔ | MPTCP_FORCE_VERSION             |                                          | macos |
| ❔ | MPTCP_SERVICE_TYPE              |                                          | macos |
| ❔ | PERSIST_TIMEOUT                 |                                          | macos |
| ❔ | TCP_ADAPTIVE_READ_TIMEOUT       |                                          | macos |
| ❔ | TCP_ADAPTIVE_WRITE_TIMEOUT      |                                          | macos |
| ❔ | TCP_CONNECTION_INFO             |                                          | macos |
| ❔ | TCP_CONNECTIONTIMEOUT           |                                          | macos |
| ❔ | TCP_DISABLE_BLACKHOLE_DETECTION |                                          | macos |
| ❔ | TCP_KEEPALIVE_OFFLOAD           |                                          | macos |
| ❔ | TCP_MEASURE_BW_BURST            |                                          | macos |
| ❔ | TCP_MEASURE_SND_BW              |                                          | macos |
| ❔ | TCP_NOTIFY_ACKNOWLEDGEMENT      |                                          | macos |
| ❔ | TCP_NOTIMEWAIT                  |                                          | macos |
| ❔ | TCP_PEER_PID                    |                                          | macos |
| ❔ | TCP_RXT_CONNDROPTIME            |                                          | macos |
| ❔ | TCP_RXT_FINDROP                 |                                          | macos |
| ❔ | TCP_RXT_MINIMUM_TIMEOUT         |                                          | macos |
| ❔ | TCP_SENDMOREACKS                |                                          | macos |
| ❔ | UDP_KEEPALIVE_OFFLOAD           |                                          | macos |
| ❔ | UDP_NOCKSUM                     |                                          | macos |
| ❔ | ICMP6_FILTER                    |                                          | macos, freebsd |
| ❔ | IP_MULTICAST_VIF                |                                          | macos, freebsd |
| ❔ | IP_PORTRANGE                    |                                          | macos, freebsd, go |
| ❔ | IP_RSVP_OFF                     |                                          | macos, freebsd |
| ❔ | IP_RSVP_ON                      |                                          | macos, freebsd |
| ❔ | IP_RSVP_VIF_OFF                 |                                          | macos, freebsd |
| ❔ | IP_RSVP_VIF_ON                  |                                          | macos, freebsd |
| ❔ | IPV6_2292NEXTHOP                |                                          | macos, freebsd |
| ❔ | IPV6_BINDV6ONLY                 |                                          | macos, freebsd |
| ❔ | IPV6_FAITH                      |                                          | macos, freebsd |
| ❔ | IPV6_PKTOPTIONS                 |                                          | macos, freebsd |
| ❔ | IPV6_PORTRANGE                  |                                          | macos, freebsd, go |
| ❔ | IPV6_PREFER_TEMPADDR            |                                          | macos, freebsd |
| ❔ | IPV6_RECVRTHDRDSTOPTS           |                                          | macos, freebsd |
| ❔ | SO_ACCEPTFILTER                 |                                          | macos, freebsd, nginx |
| ❔ | SO_LABEL                        |                                          | macos, freebsd |
| ❔ | SO_PEERLABEL                    |                                          | macos, freebsd |
| ❔ | SO_USELOOPBACK                  |                                          | macos, freebsd |
| ❔ | TCP_NOOPT                       |                                          | macos, freebsd |
| ❔ | IP_BINDMULTI                    |                                          | freebsd |
| ❔ | IP_FLOWID                       |                                          | freebsd |
| ❔ | IP_FLOWTYPE                     |                                          | freebsd |
| ❔ | IP_MAX_MEMBERSHIPS              |                                          | freebsd |
| ❔ | IP_ONESBCAST                    |                                          | freebsd |
| ❔ | IP_RECVFLOWID                   |                                          | freebsd |
| ❔ | IP_RECVRSSBUCKETID              |                                          | freebsd |
| ❔ | IP_RSS_LISTEN_BUCKET            |                                          | freebsd |
| ❔ | IP_RSSBUCKETID                  |                                          | freebsd |
| ❔ | IP_SENDSRCADDR                  |                                          | freebsd, nginx |
| ❔ | IP_VLAN_PCP                     |                                          | freebsd |
| ❔ | IPV6_AUTH_LEVEL                 |                                          | freebsd |
| ❔ | IPV6_BINDMULTI                  |                                          | freebsd |
| ❔ | IPV6_ESP_NETWORK_LEVEL          |                                          | freebsd |
| ❔ | IPV6_ESP_TRANS_LEVEL            |                                          | freebsd |
| ❔ | IPV6_FLOWID                     |                                          | freebsd |
| ❔ | IPV6_FLOWTYPE                   |                                          | freebsd |
| ❔ | IPV6_IPCOMP_LEVEL               |                                          | freebsd |
| ❔ | IPV6_RECVFLOWID                 |                                          | freebsd |
| ❔ | IPV6_RECVRSSBUCKETID            |                                          | freebsd |
| ❔ | IPV6_RSS_LISTEN_BUCKET          |                                          | freebsd |
| ❔ | IPV6_RSSBUCKETID                |                                          | freebsd |
| ❔ | IPV6_VLAN_PCP                   |                                          | freebsd |
| ❔ | SO_BINTIME                      |                                          | freebsd |
| ❔ | SO_LISTENINCQLEN                |                                          | freebsd |
| ❔ | SO_LISTENQLEN                   |                                          | freebsd, exim |
| ❔ | SO_LISTENQLIMIT                 |                                          | freebsd |
| ❔ | SO_MAX_PACING_RATE              |                                          | freebsd |
| ❔ | SO_NO_DDP                       |                                          | freebsd |
| ❔ | SO_NO_OFFLOAD                   |                                          | freebsd |
| ❔ | SO_RERROR                       |                                          | freebsd |
| ❔ | SO_SETFIB                       |                                          | freebsd, nginx |
| ❔ | SO_TS_BINTIME                   |                                          | freebsd |
| ❔ | SO_TS_CLOCK                     |                                          | freebsd |
| ❔ | SO_TS_CLOCK_MAX                 |                                          | freebsd |
| ❔ | SO_TS_DEFAULT                   |                                          | freebsd |
| ❔ | SO_TS_MONOTONIC                 |                                          | freebsd |
| ❔ | SO_TS_REALTIME                  |                                          | freebsd |
| ❔ | SO_TS_REALTIME_MICRO            |                                          | freebsd |
| ❔ | SO_USER_COOKIE                  |                                          | freebsd |
| ❔ | TCP_CCALGOOPT                   |                                          | freebsd |
| ❔ | TCP_DEFER_OPTIONS               |                                          | freebsd |
| ❔ | TCP_DELACK                      |                                          | freebsd |
| ❔ | TCP_FAST_RSM_HACK               |                                          | freebsd |
| ❔ | TCP_FIN_IS_RST                  |                                          | freebsd |
| ❔ | TCP_FUNCTION_ALIAS              |                                          | freebsd |
| ❔ | TCP_FUNCTION_BLK                |                                          | freebsd |
| ❔ | TCP_HDWR_RATE_CAP               |                                          | freebsd |
| ❔ | TCP_HDWR_UP_ONLY                |                                          | freebsd |
| ❔ | TCP_IDLE_REDUCE                 |                                          | freebsd |
| ❔ | TCP_IWND_NB                     |                                          | freebsd |
| ❔ | TCP_IWND_NSEG                   |                                          | freebsd |
| ❔ | TCP_KEEPINIT                    |                                          | freebsd |
| ❔ | TCP_LOG                         |                                          | freebsd |
| ❔ | TCP_LOG_LIMIT                   |                                          | freebsd |
| ❔ | TCP_LOG_TAG                     |                                          | freebsd |
| ❔ | TCP_LOGBUF                      |                                          | freebsd |
| ❔ | TCP_LOGDUMP                     |                                          | freebsd |
| ❔ | TCP_LOGDUMPID                   |                                          | freebsd |
| ❔ | TCP_LOGID                       |                                          | freebsd |
| ❔ | TCP_LOGID_CNT                   |                                          | freebsd |
| ❔ | TCP_LRD                         |                                          | freebsd |
| ❔ | TCP_MAXPEAKRATE                 |                                          | freebsd |
| ❔ | TCP_MAXUNACKTIME                |                                          | freebsd |
| ❔ | TCP_PCAP_IN                     |                                          | freebsd |
| ❔ | TCP_PCAP_OUT                    |                                          | freebsd |
| ❔ | TCP_PERF_INFO                   |                                          | freebsd |
| ❔ | TCP_PROC_ACCOUNTING             |                                          | freebsd |
| ❔ | TCP_REMOTE_UDP_ENCAPS_PORT      |                                          | freebsd |
| ❔ | TCP_RXTLS_ENABLE                |                                          | freebsd |
| ❔ | TCP_RXTLS_MODE                  |                                          | freebsd |
| ❔ | TCP_STATS                       |                                          | freebsd |
| ❔ | TCP_TXTLS_ENABLE                |                                          | freebsd |
| ❔ | TCP_TXTLS_MODE                  |                                          | freebsd |
| ❔ | TCP_USE_CMP_ACKS                |                                          | freebsd |
| ❔ | TCP_USER_LOG                    |                                          | freebsd |

[1]: https://github.com/WebAssembly/wasi-sockets/issues/1
[17]: https://github.com/WebAssembly/wasi-sockets/issues/17
[34]: https://github.com/WebAssembly/wasi-sockets/issues/34
[73]: https://github.com/WebAssembly/wasi-sockets/issues/73
[74]: https://github.com/WebAssembly/wasi-sockets/issues/74
[75]: https://github.com/WebAssembly/wasi-sockets/issues/75
[77]: https://github.com/WebAssembly/wasi-sockets/issues/77
[78]: https://github.com/WebAssembly/wasi-sockets/issues/78
[79]: https://github.com/WebAssembly/wasi-sockets/issues/79
[80]: https://github.com/WebAssembly/wasi-sockets/issues/80
[81]: https://github.com/WebAssembly/wasi-sockets/issues/81
[ip-name-lookup]: https://github.com/WebAssembly/wasi-sockets/blob/main/wit/ip-name-lookup.wit
[tcp-create-socket]: https://github.com/WebAssembly/wasi-sockets/blob/main/wit/tcp-create-socket.wit
[tcp]: https://github.com/WebAssembly/wasi-sockets/blob/main/wit/tcp.wit
[udp-create-socket]: https://github.com/WebAssembly/wasi-sockets/blob/main/wit/udp-create-socket.wit
[udp]: https://github.com/WebAssembly/wasi-sockets/blob/main/wit/udp.wit
[poll]: https://github.com/WebAssembly/wasi-poll/blob/main/wit/poll.wit
[streams]: https://github.com/WebAssembly/wasi-io/blob/main/wit/streams.wit
****
