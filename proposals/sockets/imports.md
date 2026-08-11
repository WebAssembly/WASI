<h1><a id="imports"></a>World imports</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_clocks_types_0_3_1"><code>wasi:clocks/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_sockets_types_0_3_1"><code>wasi:sockets/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_sockets_ip_name_lookup_0_3_1"><code>wasi:sockets/ip-name-lookup@0.3.1</code></a></li>
</ul>
</li>
</ul>
<h2><a id="wasi_clocks_types_0_3_1"></a>Import interface wasi:clocks/types@0.3.1</h2>
<p>This interface common types used throughout wasi:clocks.</p>
<hr />
<h3>Types</h3>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><code>u64</code></p>
<p>A duration of time, in nanoseconds.
<h2><a id="wasi_sockets_types_0_3_1"></a>Import interface wasi:sockets/types@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><a href="#duration"><a href="#duration"><code>duration</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>variant error-code</code></h4>
<p>Error codes.</p>
<p>In theory, every API can return any error code.
In practice, API's typically only return the errors documented per API
combined with a couple of errors that are always possible:</p>
<ul>
<li><code>other</code></li>
<li><code>access-denied</code></li>
<li><code>not-supported</code></li>
<li><code>out-of-memory</code></li>
</ul>
<p>See each individual API for what the POSIX equivalents are. They sometimes differ per API.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="error_code.access_denied"></a><code>access-denied</code></p>
<p>Access denied.
<p>POSIX equivalent: EACCES, EPERM</p>
</li>
<li>
<p><a id="error_code.not_supported"></a><code>not-supported</code></p>
<p>The operation is not supported.
<p>POSIX equivalent: EOPNOTSUPP, ENOPROTOOPT, EPFNOSUPPORT, EPROTONOSUPPORT, ESOCKTNOSUPPORT</p>
</li>
<li>
<p><a id="error_code.invalid_argument"></a><code>invalid-argument</code></p>
<p>One of the arguments is invalid.
<p>POSIX equivalent: EINVAL, EDESTADDRREQ, EAFNOSUPPORT</p>
</li>
<li>
<p><a id="error_code.out_of_memory"></a><code>out-of-memory</code></p>
<p>Not enough memory to complete the operation.
<p>POSIX equivalent: ENOMEM, ENOBUFS</p>
</li>
<li>
<p><a id="error_code.timeout"></a><code>timeout</code></p>
<p>The operation timed out before it could finish completely.
<p>POSIX equivalent: ETIMEDOUT</p>
</li>
<li>
<p><a id="error_code.invalid_state"></a><code>invalid-state</code></p>
<p>The operation is not valid in the socket's current state.
</li>
<li>
<p><a id="error_code.address_not_bindable"></a><code>address-not-bindable</code></p>
<p>The local address is not available.
<p>POSIX equivalent: EADDRNOTAVAIL</p>
</li>
<li>
<p><a id="error_code.address_in_use"></a><code>address-in-use</code></p>
<p>A bind operation failed because the provided address is already in
use or because there are no ephemeral ports available.
<p>POSIX equivalent: EADDRINUSE</p>
</li>
<li>
<p><a id="error_code.remote_unreachable"></a><code>remote-unreachable</code></p>
<p>The remote address is not reachable.
<p>POSIX equivalent: EHOSTUNREACH, EHOSTDOWN, ENETDOWN, ENETUNREACH, ENONET</p>
</li>
<li>
<p><a id="error_code.connection_refused"></a><code>connection-refused</code></p>
<p>The connection was forcefully rejected.
<p>POSIX equivalent: ECONNREFUSED</p>
</li>
<li>
<p><a id="error_code.connection_broken"></a><code>connection-broken</code></p>
<p>A write failed because the connection was broken.
<p>POSIX equivalent: EPIPE</p>
</li>
<li>
<p><a id="error_code.connection_reset"></a><code>connection-reset</code></p>
<p>The connection was reset.
<p>POSIX equivalent: ECONNRESET</p>
</li>
<li>
<p><a id="error_code.connection_aborted"></a><code>connection-aborted</code></p>
<p>The connection was aborted.
<p>POSIX equivalent: ECONNABORTED</p>
</li>
<li>
<p><a id="error_code.datagram_too_large"></a><code>datagram-too-large</code></p>
<p>The size of a datagram sent to a UDP socket exceeded the maximum
supported size.
<p>POSIX equivalent: EMSGSIZE</p>
</li>
<li>
<p><a id="error_code.other"></a><code>other</code>: option&lt;<code>string</code>&gt;</p>
<p>A catch-all for errors not captured by the existing variants.
Implementations can use this to extend the error type without
breaking existing code.
</li>
</ul>
<h4><a id="ip_address_family"></a><code>enum ip-address-family</code></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="ip_address_family.ipv4"></a><code>ipv4</code></p>
<p>Similar to `AF_INET` in POSIX.
</li>
<li>
<p><a id="ip_address_family.ipv6"></a><code>ipv6</code></p>
<p>Similar to `AF_INET6` in POSIX.
</li>
</ul>
<h4><a id="ipv4_address"></a><code>tuple ipv4-address</code></h4>
<h5>Tuple Fields</h5>
<ul>
<li><a id="ipv4_address.0"></a><code>0</code>: <code>u8</code></li>
<li><a id="ipv4_address.1"></a><code>1</code>: <code>u8</code></li>
<li><a id="ipv4_address.2"></a><code>2</code>: <code>u8</code></li>
<li><a id="ipv4_address.3"></a><code>3</code>: <code>u8</code></li>
</ul>
<h4><a id="ipv6_address"></a><code>tuple ipv6-address</code></h4>
<h5>Tuple Fields</h5>
<ul>
<li><a id="ipv6_address.0"></a><code>0</code>: <code>u16</code></li>
<li><a id="ipv6_address.1"></a><code>1</code>: <code>u16</code></li>
<li><a id="ipv6_address.2"></a><code>2</code>: <code>u16</code></li>
<li><a id="ipv6_address.3"></a><code>3</code>: <code>u16</code></li>
<li><a id="ipv6_address.4"></a><code>4</code>: <code>u16</code></li>
<li><a id="ipv6_address.5"></a><code>5</code>: <code>u16</code></li>
<li><a id="ipv6_address.6"></a><code>6</code>: <code>u16</code></li>
<li><a id="ipv6_address.7"></a><code>7</code>: <code>u16</code></li>
</ul>
<h4><a id="ip_address"></a><code>variant ip-address</code></h4>
<h5>Variant Cases</h5>
<ul>
<li><a id="ip_address.ipv4"></a><code>ipv4</code>: <a href="#ipv4_address"><a href="#ipv4_address"><code>ipv4-address</code></a></a></li>
<li><a id="ip_address.ipv6"></a><code>ipv6</code>: <a href="#ipv6_address"><a href="#ipv6_address"><code>ipv6-address</code></a></a></li>
</ul>
<h4><a id="ipv4_socket_address"></a><code>record ipv4-socket-address</code></h4>
<h5>Record Fields</h5>
<ul>
<li>
<p><a id="ipv4_socket_address.port"></a><code>port</code>: <code>u16</code></p>
<p>sin_port
</li>
<li>
<p><a id="ipv4_socket_address.address"></a><code>address</code>: <a href="#ipv4_address"><a href="#ipv4_address"><code>ipv4-address</code></a></a></p>
<p>sin_addr
</li>
</ul>
<h4><a id="ipv6_socket_address"></a><code>record ipv6-socket-address</code></h4>
<h5>Record Fields</h5>
<ul>
<li>
<p><a id="ipv6_socket_address.port"></a><code>port</code>: <code>u16</code></p>
<p>sin6_port
</li>
<li>
<p><a id="ipv6_socket_address.flow_info"></a><code>flow-info</code>: <code>u32</code></p>
<p>sin6_flowinfo
</li>
<li>
<p><a id="ipv6_socket_address.address"></a><code>address</code>: <a href="#ipv6_address"><a href="#ipv6_address"><code>ipv6-address</code></a></a></p>
<p>sin6_addr
</li>
<li>
<p><a id="ipv6_socket_address.scope_id"></a><code>scope-id</code>: <code>u32</code></p>
<p>sin6_scope_id
</li>
</ul>
<h4><a id="ip_socket_address"></a><code>variant ip-socket-address</code></h4>
<h5>Variant Cases</h5>
<ul>
<li><a id="ip_socket_address.ipv4"></a><code>ipv4</code>: <a href="#ipv4_socket_address"><a href="#ipv4_socket_address"><code>ipv4-socket-address</code></a></a></li>
<li><a id="ip_socket_address.ipv6"></a><code>ipv6</code>: <a href="#ipv6_socket_address"><a href="#ipv6_socket_address"><code>ipv6-socket-address</code></a></a></li>
</ul>
<h4><a id="tcp_socket"></a><code>resource tcp-socket</code></h4>
<p>A TCP socket resource.</p>
<p>The socket can be in one of the following states:</p>
<ul>
<li><code>unbound</code></li>
<li><code>bound</code> (See note below)</li>
<li><code>listening</code></li>
<li><code>connecting</code></li>
<li><code>connected</code></li>
<li><code>closed</code>
See <a href="https://github.com/WebAssembly/WASI/blob/main/proposals/sockets/TcpSocketOperationalSemantics-0.3.0.md">https://github.com/WebAssembly/WASI/blob/main/proposals/sockets/TcpSocketOperationalSemantics-0.3.0.md</a>
for more information.</li>
</ul>
<p>Note: Except where explicitly mentioned, whenever this documentation uses
the term &quot;bound&quot; without backticks it actually means: in the <code>bound</code> state <em>or higher</em>.
(i.e. <code>bound</code>, <code>listening</code>, <code>connecting</code> or <code>connected</code>)</p>
<p>WASI uses shared ownership semantics: the <a href="#tcp_socket"><code>tcp-socket</code></a> handle and all
derived <code>stream</code> and <code>future</code> values reference a single underlying OS
socket:</p>
<ul>
<li>Send/receive streams remain functional after the original <a href="#tcp_socket"><code>tcp-socket</code></a>
handle is dropped.</li>
<li>The stream returned by <code>listen</code> behaves similarly.</li>
<li>Client sockets returned by <code>tcp-socket::listen</code> are independent and do
not keep the listening socket alive.</li>
</ul>
<p>The OS socket is closed only after the last handle is dropped. This
model has observable effects; for example, it affects when the local
port binding is released.</p>
<p>In addition to the general error codes documented on the
<code>types::error-code</code> type, TCP socket methods may always return
<code>error(invalid-state)</code> when in the <code>closed</code> state.</p>
<h4><a id="udp_socket"></a><code>resource udp-socket</code></h4>
<h2>A UDP socket handle.</h2>
<h3>Functions</h3>
<h4><a id="static_tcp_socket_create"></a><code>[static]tcp-socket.create: func</code></h4>
<p>Create a new TCP socket.</p>
<p>Similar to <code>socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)</code>
in POSIX. On IPv6 sockets, IPV6_V6ONLY is enabled by default and
can't be configured otherwise.</p>
<p>Unlike POSIX, WASI sockets have no notion of a socket-level
<code>O_NONBLOCK</code> flag. Instead they fully rely on the Component Model's
async support.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-supported</code>: The <code>address-family</code> is not supported. (EAFNOSUPPORT)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/socket.2.html">https://man7.org/linux/man-pages/man2/socket.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=socket&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=socket&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="static_tcp_socket_create.address_family"></a><code>address-family</code>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_tcp_socket_create.0"></a> result&lt;own&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_bind"></a><code>[method]tcp-socket.bind: func</code></h4>
<p>Bind the socket to the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is
left to the implementation to decide which network interface(s) to
bind to. If the TCP/UDP port is zero, the socket will be bound to a
random free port.</p>
<p>Bind can be attempted multiple times on the same socket, even with
different arguments on each iteration. But never concurrently and
only as long as the previous bind failed. Once a bind succeeds, the
binding can't be changed anymore.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:          The <code>local-address</code> has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)</li>
<li><code>invalid-argument</code>:          <code>local-address</code> is not a unicast address. (EINVAL)</li>
<li><code>invalid-argument</code>:          <code>local-address</code> is an IPv4-mapped IPv6 address. (EINVAL)</li>
<li><code>invalid-state</code>:             The socket is already bound. (EINVAL)</li>
<li><code>address-in-use</code>:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)</li>
<li><code>address-in-use</code>:            Address is already in use. (EADDRINUSE)</li>
<li><code>address-not-bindable</code>:      <code>local-address</code> is not an address that can be bound to. (EADDRNOTAVAIL)</li>
</ul>
<h1>Implementors note</h1>
<p>The bind operation shouldn't be affected by the TIME_WAIT state of a
recently closed socket on the same local address. In practice this
means that the SO_REUSEADDR socket option should be set implicitly
on all platforms, except on Windows where this is the default
behavior and SO_REUSEADDR performs something different.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/bind.2.html">https://man7.org/linux/man-pages/man2/bind.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=bind&amp;sektion=2&amp;format=html">https://man.freebsd.org/cgi/man.cgi?query=bind&amp;sektion=2&amp;format=html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_bind.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_bind.local_address"></a><code>local-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_connect"></a><code>[method]tcp-socket.connect: func</code></h4>
<p>Connect to a remote endpoint.</p>
<p>On success, the socket is transitioned into the <code>connected</code> state
and the <code>remote-address</code> of the socket is updated.
The <code>local-address</code> may be updated as well, based on the best network
path to <code>remote-address</code>. If the socket was not already explicitly
bound, this function will implicitly bind the socket to a random
free port.</p>
<p>After a failed connection attempt, the socket will be in the <code>closed</code>
state and the only valid action left is to <code>drop</code> the socket. A single
socket can not be used to connect more than once.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:          The <code>remote-address</code> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-argument</code>:          <code>remote-address</code> is not a unicast address. (EINVAL, ENETUNREACH on Linux, EAFNOSUPPORT on MacOS)</li>
<li><code>invalid-argument</code>:          <code>remote-address</code> is an IPv4-mapped IPv6 address. (EINVAL, EADDRNOTAVAIL on Illumos)</li>
<li><code>invalid-argument</code>:          The IP address in <code>remote-address</code> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EADDRNOTAVAIL on Windows)</li>
<li><code>invalid-argument</code>:          The port in <code>remote-address</code> is set to 0. (EADDRNOTAVAIL on Windows)</li>
<li><code>invalid-state</code>:             The socket is already in the <code>connecting</code> state. (EALREADY)</li>
<li><code>invalid-state</code>:             The socket is already in the <code>connected</code> state. (EISCONN)</li>
<li><code>invalid-state</code>:             The socket is already in the <code>listening</code> state. (EOPNOTSUPP, EINVAL on Windows)</li>
<li><code>timeout</code>:                   Connection timed out. (ETIMEDOUT)</li>
<li><code>connection-refused</code>:        The connection was forcefully rejected. (ECONNREFUSED)</li>
<li><code>connection-reset</code>:          The connection was reset. (ECONNRESET)</li>
<li><code>connection-aborted</code>:        The connection was aborted. (ECONNABORTED)</li>
<li><code>remote-unreachable</code>:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
<li><code>address-in-use</code>:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/connect.2.html">https://man7.org/linux/man-pages/man2/connect.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?connect">https://man.freebsd.org/cgi/man.cgi?connect</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_connect.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_connect.remote_address"></a><code>remote-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_listen"></a><code>[method]tcp-socket.listen: func</code></h4>
<p>Start listening and return a stream of new inbound connections.</p>
<p>Transitions the socket into the <code>listening</code> state. This can be called
at most once per socket.</p>
<p>If the socket is not already explicitly bound, this function will
implicitly bind the socket to a random free port.</p>
<p>Normally, the returned sockets are bound, in the <code>connected</code> state
and immediately ready for I/O. Though, depending on exact timing and
circumstances, a newly accepted connection may already be <code>closed</code>
by the time the server attempts to perform its first I/O on it. This
is true regardless of whether the WASI implementation uses
&quot;synthesized&quot; sockets or not (see Implementors Notes below).</p>
<p>The following properties are inherited from the listener socket:</p>
<ul>
<li><code>address-family</code></li>
<li><code>keep-alive-enabled</code></li>
<li><code>keep-alive-idle-time</code></li>
<li><code>keep-alive-interval</code></li>
<li><code>keep-alive-count</code></li>
<li><code>hop-limit</code></li>
<li><code>receive-buffer-size</code></li>
<li><code>send-buffer-size</code></li>
</ul>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:             The socket is already in the <code>connected</code> state. (EISCONN, EINVAL on BSD)</li>
<li><code>invalid-state</code>:             The socket is already in the <code>listening</code> state.</li>
<li><code>address-in-use</code>:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)</li>
</ul>
<h1>Implementors note</h1>
<p>This method returns a single perpetual stream that should only close
on fatal errors (if any). Yet, the POSIX' <code>accept</code> function may also
return transient errors (e.g. ECONNABORTED). The exact details differ
per operation system. For example, the Linux manual mentions:</p>
<blockquote>
<p>Linux accept() passes already-pending network errors on the new
socket as an error code from accept(). This behavior differs from
other BSD socket implementations. For reliable operation the
application should detect the network errors defined for the
protocol after accept() and treat them like EAGAIN by retrying.
In the case of TCP/IP, these are ENETDOWN, EPROTO, ENOPROTOOPT,
EHOSTDOWN, ENONET, EHOSTUNREACH, EOPNOTSUPP, and ENETUNREACH.
Source: https://man7.org/linux/man-pages/man2/accept.2.html</p>
</blockquote>
<p>WASI implementations have two options to handle this:</p>
<ul>
<li>Optionally log it and then skip over non-fatal errors returned by
<code>accept</code>. Guest code never gets to see these failures. Or:</li>
<li>Synthesize a <a href="#tcp_socket"><code>tcp-socket</code></a> resource that exposes the error when
attempting to send or receive on it. Guest code then sees these
failures as regular I/O errors.</li>
</ul>
<p>In either case, the stream returned by this <code>listen</code> method remains
operational.</p>
<p>WASI requires <code>listen</code> to perform an implicit bind if the socket
has not already been bound. Not all platforms (notably Windows)
exhibit this behavior out of the box. On platforms that require it,
the WASI implementation can emulate this behavior by performing
the bind itself if the guest hasn't already done so.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html</a></li>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/listen.2.html">https://man7.org/linux/man-pages/man2/listen.2.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/accept.2.html">https://man7.org/linux/man-pages/man2/accept.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=listen&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=listen&amp;sektion=2</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=accept&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=accept&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_listen.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_listen.0"></a> result&lt;stream&lt;own&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_send"></a><code>[method]tcp-socket.send: func</code></h4>
<p>Transmit data to peer.</p>
<p>The caller should close the stream when it has no more data to send
to the peer. Under normal circumstances this will cause a FIN packet
to be sent out. Closing the stream is equivalent to calling
<code>shutdown(SHUT_WR)</code> in POSIX.</p>
<p>This function may be called at most once and returns once the full
contents of the stream are transmitted or an error is encountered.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:             The socket is not in the <code>connected</code> state. (ENOTCONN)</li>
<li><code>invalid-state</code>:             <code>send</code> has already been called on this socket.</li>
<li><code>connection-broken</code>:         The connection is not writable anymore. (EPIPE, ECONNABORTED on Windows)</li>
<li><code>connection-reset</code>:          The connection was reset. (ECONNRESET)</li>
<li><code>remote-unreachable</code>:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/send.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/send.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/send.2.html">https://man7.org/linux/man-pages/man2/send.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=send&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=send&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_send.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_send.data"></a><code>data</code>: stream&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_send.0"></a> future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_tcp_socket_receive"></a><code>[method]tcp-socket.receive: func</code></h4>
<p>Read data from peer.</p>
<p>Returns a <code>stream</code> of data sent by the peer. The implementation
drops the stream once no more data is available. At that point, the
returned <code>future</code> resolves to:</p>
<ul>
<li><code>ok</code> after a graceful shutdown from the peer (i.e. a FIN packet), or</li>
<li><code>err</code> if the socket was closed abnormally.</li>
</ul>
<p><code>receive</code> may be called only once per socket. Subsequent calls return
a closed stream and a future resolved to <code>err(invalid-state)</code>.</p>
<p>If the caller is not expecting to receive any more data from the peer,
they should drop the stream. Any data still in the receive queue
will be discarded. This is equivalent to calling <code>shutdown(SHUT_RD)</code>
in POSIX.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:             The socket is not in the <code>connected</code> state. (ENOTCONN)</li>
<li><code>invalid-state</code>:             <code>receive</code> has already been called on this socket.</li>
<li><code>connection-reset</code>:          The connection was reset. (ECONNRESET)</li>
<li><code>remote-unreachable</code>:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/recv.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/recv.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/recv.2.html">https://man7.org/linux/man-pages/man2/recv.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recv">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recv</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=recv&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=recv&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_receive.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_receive.0"></a> (stream&lt;<code>u8</code>&gt;, future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;)</li>
</ul>
<h4><a id="method_tcp_socket_get_local_address"></a><code>[method]tcp-socket.get-local-address: func</code></h4>
<p>Get the bound local address.</p>
<p>POSIX mentions:</p>
<blockquote>
<p>If the socket has not been bound to a local name, the value
stored in the object pointed to by <code>address</code> is unspecified.</p>
</blockquote>
<p>WASI is stricter and requires <code>get-local-address</code> to return
<code>invalid-state</code> when the socket hasn't been bound yet.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>: The socket is not bound to any local address.</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/getsockname.2.html">https://man7.org/linux/man-pages/man2/getsockname.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?getsockname">https://man.freebsd.org/cgi/man.cgi?getsockname</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_local_address.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_remote_address"></a><code>[method]tcp-socket.get-remote-address: func</code></h4>
<p>Get the remote address.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>: The socket is not connected to a remote address. (ENOTCONN)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/getpeername.2.html">https://man7.org/linux/man-pages/man2/getpeername.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=getpeername&amp;sektion=2&amp;n=1">https://man.freebsd.org/cgi/man.cgi?query=getpeername&amp;sektion=2&amp;n=1</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_remote_address.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_is_listening"></a><code>[method]tcp-socket.get-is-listening: func</code></h4>
<p>Whether the socket is in the <code>listening</code> state.</p>
<p>Equivalent to the SO_ACCEPTCONN socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_is_listening.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_is_listening.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_tcp_socket_get_address_family"></a><code>[method]tcp-socket.get-address-family: func</code></h4>
<p>Whether this is a IPv4 or IPv6 socket.</p>
<p>This is the value passed to the constructor.</p>
<p>Equivalent to the SO_DOMAIN socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_address_family.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_address_family.0"></a> <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h4><a id="method_tcp_socket_set_listen_backlog_size"></a><code>[method]tcp-socket.set-listen-backlog-size: func</code></h4>
<p>Hints the desired listen queue size. Implementations are free to
ignore this.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
Any other value will never cause an error, but it might be silently
clamped and/or rounded.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-supported</code>:        (set) The platform does not support changing the backlog size after the initial listen.</li>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
<li><code>invalid-state</code>:        (set) The socket is in the <code>connecting</code> or <code>connected</code> state.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_set_listen_backlog_size.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_set_listen_backlog_size.value"></a><code>value</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_set_listen_backlog_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_keep_alive_enabled"></a><code>[method]tcp-socket.get-keep-alive-enabled: func</code></h4>
<p>Enables or disables keepalive.</p>
<p>The keepalive behavior can be adjusted using:</p>
<ul>
<li><code>keep-alive-idle-time</code></li>
<li><code>keep-alive-interval</code></li>
<li><code>keep-alive-count</code>
These properties can be configured while <code>keep-alive-enabled</code> is
false, but only come into effect when <code>keep-alive-enabled</code> is true.</li>
</ul>
<p>Equivalent to the SO_KEEPALIVE socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_enabled.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_enabled.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_set_keep_alive_enabled"></a><code>[method]tcp-socket.set-keep-alive-enabled: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_set_keep_alive_enabled.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_set_keep_alive_enabled.value"></a><code>value</code>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_set_keep_alive_enabled.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_keep_alive_idle_time"></a><code>[method]tcp-socket.get-keep-alive-idle-time: func</code></h4>
<p>Amount of time the connection has to be idle before TCP starts
sending keepalive packets.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
All other values are accepted without error, but may be
clamped or rounded. As a result, the value read back from
this setting may differ from the value that was set.</p>
<p>Equivalent to the TCP_KEEPIDLE socket option. (TCP_KEEPALIVE on MacOS)</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_idle_time.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_idle_time.0"></a> result&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_set_keep_alive_idle_time"></a><code>[method]tcp-socket.set-keep-alive-idle-time: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_set_keep_alive_idle_time.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_set_keep_alive_idle_time.value"></a><code>value</code>: <a href="#duration"><a href="#duration"><code>duration</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_set_keep_alive_idle_time.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_keep_alive_interval"></a><code>[method]tcp-socket.get-keep-alive-interval: func</code></h4>
<p>The time between keepalive packets.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
All other values are accepted without error, but may be
clamped or rounded. As a result, the value read back from
this setting may differ from the value that was set.</p>
<p>Equivalent to the TCP_KEEPINTVL socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_interval.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_interval.0"></a> result&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_set_keep_alive_interval"></a><code>[method]tcp-socket.set-keep-alive-interval: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_set_keep_alive_interval.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_set_keep_alive_interval.value"></a><code>value</code>: <a href="#duration"><a href="#duration"><code>duration</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_set_keep_alive_interval.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_keep_alive_count"></a><code>[method]tcp-socket.get-keep-alive-count: func</code></h4>
<p>The maximum amount of keepalive packets TCP should send before
aborting the connection.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
All other values are accepted without error, but may be
clamped or rounded. As a result, the value read back from
this setting may differ from the value that was set.</p>
<p>Equivalent to the TCP_KEEPCNT socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_count.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_count.0"></a> result&lt;<code>u32</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_set_keep_alive_count"></a><code>[method]tcp-socket.set-keep-alive-count: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_set_keep_alive_count.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_set_keep_alive_count.value"></a><code>value</code>: <code>u32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_set_keep_alive_count.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_hop_limit"></a><code>[method]tcp-socket.get-hop-limit: func</code></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The TTL value must be 1 or higher.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_hop_limit.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_set_hop_limit"></a><code>[method]tcp-socket.set-hop-limit: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_set_hop_limit.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_set_hop_limit.value"></a><code>value</code>: <code>u8</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_set_hop_limit.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_receive_buffer_size"></a><code>[method]tcp-socket.get-receive-buffer-size: func</code></h4>
<p>Kernel buffer space reserved for sending/receiving on this socket.
Implementations usually treat this as a cap the buffer can grow to,
rather than allocating the full amount immediately.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
All other values are accepted without error, but may be
clamped or rounded. As a result, the value read back from
this setting may differ from the value that was set.</p>
<p>This is only a performance hint. The implementation may ignore it or
tweak it based on real traffic patterns.
Linux and macOS appear to behave differently depending on whether a
buffer size was explicitly set. When set, they tend to honor it; when
not set, they dynamically adjust the buffer size as the connection
progresses. This is especially noticeable when comparing the values
from before and after connection establishment.</p>
<p>Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_receive_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_set_receive_buffer_size"></a><code>[method]tcp-socket.set-receive-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_set_receive_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_set_receive_buffer_size.value"></a><code>value</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_set_receive_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_send_buffer_size"></a><code>[method]tcp-socket.get-send-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_send_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_set_send_buffer_size"></a><code>[method]tcp-socket.set-send-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_set_send_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_set_send_buffer_size.value"></a><code>value</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_set_send_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="static_udp_socket_create"></a><code>[static]udp-socket.create: func</code></h4>
<p>Create a new UDP socket.</p>
<p>Similar to <code>socket(AF_INET or AF_INET6, SOCK_DGRAM, IPPROTO_UDP)</code>
in POSIX. On IPv6 sockets, IPV6_V6ONLY is enabled by default and
can't be configured otherwise.</p>
<p>Unlike POSIX, WASI sockets have no notion of a socket-level
<code>O_NONBLOCK</code> flag. Instead they fully rely on the Component Model's
async support.</p>
<h1>References:</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/socket.2.html">https://man7.org/linux/man-pages/man2/socket.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=socket&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=socket&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="static_udp_socket_create.address_family"></a><code>address-family</code>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_udp_socket_create.0"></a> result&lt;own&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_bind"></a><code>[method]udp-socket.bind: func</code></h4>
<p>Bind the socket to the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is
left to the implementation to decide which network interface(s) to
bind to. If the port is zero, the socket will be bound to a random
free port.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:          The <code>local-address</code> has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)</li>
<li><code>invalid-state</code>:             The socket is already bound. (EINVAL)</li>
<li><code>address-in-use</code>:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)</li>
<li><code>address-in-use</code>:            Address is already in use. (EADDRINUSE)</li>
<li><code>address-not-bindable</code>:      <code>local-address</code> is not an address that can be bound to. (EADDRNOTAVAIL)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/bind.2.html">https://man7.org/linux/man-pages/man2/bind.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=bind&amp;sektion=2&amp;format=html">https://man.freebsd.org/cgi/man.cgi?query=bind&amp;sektion=2&amp;format=html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_bind.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_bind.local_address"></a><code>local-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_connect"></a><code>[method]udp-socket.connect: func</code></h4>
<p>Associate this socket with a specific peer address.</p>
<p>On success, the <code>remote-address</code> of the socket is updated.
The <code>local-address</code> may be updated as well, based on the best network
path to <code>remote-address</code>. If the socket was not already explicitly
bound, this function will implicitly bind the socket to a random
free port.</p>
<p>When a UDP socket is &quot;connected&quot;, the <code>send</code> and <code>receive</code> methods
are limited to communicating with that peer only:</p>
<ul>
<li><code>send</code> can only be used to send to this destination.</li>
<li><code>receive</code> will only return datagrams sent from the provided <code>remote-address</code>.</li>
</ul>
<p>The name &quot;connect&quot; was kept to align with the existing POSIX
terminology. Other than that, this function only changes the local
socket configuration and does not generate any network traffic.
The peer is not aware of this &quot;connection&quot;.</p>
<p>This method may be called multiple times on the same socket to change
its association, but only the most recent one will be effective.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:          The <code>remote-address</code> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-argument</code>:          The IP address in <code>remote-address</code> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-argument</code>:          The port in <code>remote-address</code> is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>address-in-use</code>:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)</li>
</ul>
<h1>Implementors note</h1>
<p>If the socket is already connected, some platforms (e.g. Linux)
require a disconnect before connecting to a different peer address.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/connect.2.html">https://man7.org/linux/man-pages/man2/connect.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?connect">https://man.freebsd.org/cgi/man.cgi?connect</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_connect.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_connect.remote_address"></a><code>remote-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_disconnect"></a><code>[method]udp-socket.disconnect: func</code></h4>
<p>Dissociate this socket from its peer address.</p>
<p>After calling this method, <code>send</code> &amp; <code>receive</code> are free to communicate
with any remote address again.</p>
<p>The POSIX equivalent of this is calling <code>connect</code> with an <code>AF_UNSPEC</code> address.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:           The socket is not connected.</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/connect.2.html">https://man7.org/linux/man-pages/man2/connect.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?connect">https://man.freebsd.org/cgi/man.cgi?connect</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_disconnect.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_disconnect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_send"></a><code>[method]udp-socket.send: func</code></h4>
<p>Send a message on the socket to a particular peer.</p>
<p>If the socket is connected, the peer address may be left empty. In
that case this is equivalent to <code>send</code> in POSIX. Otherwise it is
equivalent to <code>sendto</code>.</p>
<p>Additionally, if the socket is connected, a <code>remote-address</code> argument
<em>may</em> be provided but then it must be identical to the address
passed to <code>connect</code>.</p>
<p>If the socket has not been explicitly bound, it will be
implicitly bound to a random free port.</p>
<p>Implementations may trap if the <code>data</code> length exceeds 64 KiB.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:        The <code>remote-address</code> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-argument</code>:        The IP address in <code>remote-address</code> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-argument</code>:        The port in <code>remote-address</code> is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-argument</code>:        The socket is in &quot;connected&quot; mode and <code>remote-address</code> is <code>some</code> value that does not match the address passed to <code>connect</code>. (EISCONN)</li>
<li><code>invalid-argument</code>:        The socket is not &quot;connected&quot; and no value for <code>remote-address</code> was provided. (EDESTADDRREQ)</li>
<li><code>remote-unreachable</code>:      The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
<li><code>connection-refused</code>:      The connection was refused. (ECONNREFUSED)</li>
<li><code>datagram-too-large</code>:      The datagram is too large. (EMSGSIZE)</li>
<li><code>address-in-use</code>:          Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)</li>
</ul>
<h1>Implementors note</h1>
<p>WASI requires <code>send</code> to perform an implicit bind if the socket
has not been bound. Not all platforms (notably Windows) exhibit
this behavior natively. On such platforms, the WASI implementation
should emulate it by performing the bind if the guest has not
already done so.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html</a></li>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/send.2.html">https://man7.org/linux/man-pages/man2/send.2.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/sendmmsg.2.html">https://man7.org/linux/man-pages/man2/sendmmsg.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-sendto">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-sendto</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendmsg">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendmsg</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=send&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=send&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_send.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_send.data"></a><code>data</code>: list&lt;<code>u8</code>&gt;</li>
<li><a id="method_udp_socket_send.remote_address"></a><code>remote-address</code>: option&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_send.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_receive"></a><code>[method]udp-socket.receive: func</code></h4>
<p>Receive a message on the socket.</p>
<p>On success, the return value contains a tuple of the received data
and the address of the sender. Theoretical maximum length of the
data is 64 KiB. Though in practice, it will typically be less than
1500 bytes.</p>
<p>If the socket is connected, the sender address is guaranteed to
match the remote address passed to <code>connect</code>.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:        The socket has not been bound yet.</li>
<li><code>remote-unreachable</code>:   The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
<li><code>connection-refused</code>:   The connection was refused. (ECONNREFUSED)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html</a></li>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/recv.2.html">https://man7.org/linux/man-pages/man2/recv.2.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/recvmmsg.2.html">https://man7.org/linux/man-pages/man2/recvmmsg.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recvfrom">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recvfrom</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/mswsock/nc-mswsock-lpfn_wsarecvmsg">https://learn.microsoft.com/en-us/windows/win32/api/mswsock/nc-mswsock-lpfn_wsarecvmsg</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=recv&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=recv&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_receive.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_receive.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_get_local_address"></a><code>[method]udp-socket.get-local-address: func</code></h4>
<p>Get the current bound address.</p>
<p>POSIX mentions:</p>
<blockquote>
<p>If the socket has not been bound to a local name, the value
stored in the object pointed to by <code>address</code> is unspecified.</p>
</blockquote>
<p>WASI is stricter and requires <code>get-local-address</code> to return
<code>invalid-state</code> when the socket hasn't been bound yet.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>: The socket is not bound to any local address.</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/getsockname.2.html">https://man7.org/linux/man-pages/man2/getsockname.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?getsockname">https://man.freebsd.org/cgi/man.cgi?getsockname</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_get_local_address.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_get_remote_address"></a><code>[method]udp-socket.get-remote-address: func</code></h4>
<p>Get the address the socket is currently &quot;connected&quot; to.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>: The socket is not &quot;connected&quot; to a specific remote address. (ENOTCONN)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/getpeername.2.html">https://man7.org/linux/man-pages/man2/getpeername.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=getpeername&amp;sektion=2&amp;n=1">https://man.freebsd.org/cgi/man.cgi?query=getpeername&amp;sektion=2&amp;n=1</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_get_remote_address.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_get_address_family"></a><code>[method]udp-socket.get-address-family: func</code></h4>
<p>Whether this is a IPv4 or IPv6 socket.</p>
<p>This is the value passed to the constructor.</p>
<p>Equivalent to the SO_DOMAIN socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_get_address_family.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_address_family.0"></a> <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h4><a id="method_udp_socket_get_unicast_hop_limit"></a><code>[method]udp-socket.get-unicast-hop-limit: func</code></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The TTL value must be 1 or higher.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_get_unicast_hop_limit.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_unicast_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_set_unicast_hop_limit"></a><code>[method]udp-socket.set-unicast-hop-limit: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_set_unicast_hop_limit.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_set_unicast_hop_limit.value"></a><code>value</code>: <code>u8</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_set_unicast_hop_limit.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_get_receive_buffer_size"></a><code>[method]udp-socket.get-receive-buffer-size: func</code></h4>
<p>Kernel buffer space reserved for sending/receiving on this socket.
Implementations usually treat this as a cap the buffer can grow to,
rather than allocating the full amount immediately.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
All other values are accepted without error, but may be
clamped or rounded. As a result, the value read back from
this setting may differ from the value that was set.</p>
<p>Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_get_receive_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_set_receive_buffer_size"></a><code>[method]udp-socket.set-receive-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_set_receive_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_set_receive_buffer_size.value"></a><code>value</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_set_receive_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_get_send_buffer_size"></a><code>[method]udp-socket.get-send-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_get_send_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_set_send_buffer_size"></a><code>[method]udp-socket.set-send-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_set_send_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_set_send_buffer_size.value"></a><code>value</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_set_send_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_sockets_ip_name_lookup_0_3_1"></a>Import interface wasi:sockets/ip-name-lookup@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="ip_address"></a><code>type ip-address</code></h4>
<p><a href="#ip_address"><a href="#ip_address"><code>ip-address</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>variant error-code</code></h4>
<p>Lookup error codes.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="error_code.access_denied"></a><code>access-denied</code></p>
<p>Access denied.
<p>POSIX equivalent: EACCES, EPERM</p>
</li>
<li>
<p><a id="error_code.invalid_argument"></a><code>invalid-argument</code></p>
<p>`name` is a syntactically invalid domain name or IP address.
<p>POSIX equivalent: EINVAL</p>
</li>
<li>
<p><a id="error_code.name_unresolvable"></a><code>name-unresolvable</code></p>
<p>Name does not exist or has no suitable associated IP addresses.
<p>POSIX equivalent: EAI_NONAME, EAI_NODATA, EAI_ADDRFAMILY</p>
</li>
<li>
<p><a id="error_code.temporary_resolver_failure"></a><code>temporary-resolver-failure</code></p>
<p>A temporary failure in name resolution occurred.
<p>POSIX equivalent: EAI_AGAIN</p>
</li>
<li>
<p><a id="error_code.permanent_resolver_failure"></a><code>permanent-resolver-failure</code></p>
<p>A permanent failure in name resolution occurred.
<p>POSIX equivalent: EAI_FAIL</p>
</li>
<li>
<p><a id="error_code.other"></a><code>other</code>: option&lt;<code>string</code>&gt;</p>
<p>A catch-all for errors not captured by the existing variants.
Implementations can use this to extend the error type without
breaking existing code.
</li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a id="resolve_addresses"></a><code>resolve-addresses: func</code></h4>
<p>Resolve an internet host name to a list of IP addresses.</p>
<p>Unicode domain names are automatically converted to ASCII using IDNA
encoding. If the input is an IP address string, the address is parsed
and returned as-is without making any external requests.</p>
<p>See the wasi-socket proposal README.md for a comparison with getaddrinfo.</p>
<p>The results are returned in connection order preference.</p>
<p>This function never succeeds with 0 results. It either fails or succeeds
with at least one address. Additionally, this function never returns
IPv4-mapped IPv6 addresses.</p>
<h1>References:</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man3/getaddrinfo.3.html">https://man7.org/linux/man-pages/man3/getaddrinfo.3.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo">https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&amp;sektion=3">https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&amp;sektion=3</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="resolve_addresses.name"></a><code>name</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="resolve_addresses.0"></a> result&lt;list&lt;<a href="#ip_address"><a href="#ip_address"><code>ip-address</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
