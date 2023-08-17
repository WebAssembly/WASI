<h1><a name="example_world">World example-world</a></h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi:sockets_network"><code>wasi:sockets/network</code></a></li>
<li>interface <a href="#wasi:sockets_instance_network"><code>wasi:sockets/instance-network</code></a></li>
<li>interface <a href="#wasi:poll_poll"><code>wasi:poll/poll</code></a></li>
<li>interface <a href="#wasi:sockets_udp"><code>wasi:sockets/udp</code></a></li>
<li>interface <a href="#wasi:sockets_udp_create_socket"><code>wasi:sockets/udp-create-socket</code></a></li>
<li>interface <a href="#wasi:io_streams"><code>wasi:io/streams</code></a></li>
<li>interface <a href="#wasi:sockets_tcp"><code>wasi:sockets/tcp</code></a></li>
<li>interface <a href="#wasi:sockets_tcp_create_socket"><code>wasi:sockets/tcp-create-socket</code></a></li>
<li>interface <a href="#wasi:sockets_ip_name_lookup"><code>wasi:sockets/ip-name-lookup</code></a></li>
</ul>
</li>
</ul>
<h2><a name="wasi:sockets_network">Import interface wasi:sockets/network</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="network"><code>type network</code></a></h4>
<p><code>u32</code></p>
<p>An opaque resource that represents access to (a subset of) the network.
This enables context-based security for networking.
There is no need for this to map 1:1 to a physical network interface.
<p>FYI, In the future this will be replaced by handle types.</p>
<h4><a name="ipv6_address"><code>tuple ipv6-address</code></a></h4>
<h5>Tuple Fields</h5>
<ul>
<li><a name="ipv6_address.0"><code>0</code></a>: <code>u16</code></li>
<li><a name="ipv6_address.1"><code>1</code></a>: <code>u16</code></li>
<li><a name="ipv6_address.2"><code>2</code></a>: <code>u16</code></li>
<li><a name="ipv6_address.3"><code>3</code></a>: <code>u16</code></li>
<li><a name="ipv6_address.4"><code>4</code></a>: <code>u16</code></li>
<li><a name="ipv6_address.5"><code>5</code></a>: <code>u16</code></li>
<li><a name="ipv6_address.6"><code>6</code></a>: <code>u16</code></li>
<li><a name="ipv6_address.7"><code>7</code></a>: <code>u16</code></li>
</ul>
<h4><a name="ipv6_socket_address"><code>record ipv6-socket-address</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="ipv6_socket_address.port"><code>port</code></a>: <code>u16</code></li>
<li><a name="ipv6_socket_address.flow_info"><code>flow-info</code></a>: <code>u32</code></li>
<li><a name="ipv6_socket_address.address"><code>address</code></a>: <a href="#ipv6_address"><a href="#ipv6_address"><code>ipv6-address</code></a></a></li>
<li><a name="ipv6_socket_address.scope_id"><code>scope-id</code></a>: <code>u32</code></li>
</ul>
<h4><a name="ipv4_address"><code>tuple ipv4-address</code></a></h4>
<h5>Tuple Fields</h5>
<ul>
<li><a name="ipv4_address.0"><code>0</code></a>: <code>u8</code></li>
<li><a name="ipv4_address.1"><code>1</code></a>: <code>u8</code></li>
<li><a name="ipv4_address.2"><code>2</code></a>: <code>u8</code></li>
<li><a name="ipv4_address.3"><code>3</code></a>: <code>u8</code></li>
</ul>
<h4><a name="ipv4_socket_address"><code>record ipv4-socket-address</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="ipv4_socket_address.port"><code>port</code></a>: <code>u16</code></li>
<li><a name="ipv4_socket_address.address"><code>address</code></a>: <a href="#ipv4_address"><a href="#ipv4_address"><code>ipv4-address</code></a></a></li>
</ul>
<h4><a name="ip_socket_address"><code>variant ip-socket-address</code></a></h4>
<h5>Variant Cases</h5>
<ul>
<li><a name="ip_socket_address.ipv4"><code>ipv4</code></a>: <a href="#ipv4_socket_address"><a href="#ipv4_socket_address"><code>ipv4-socket-address</code></a></a></li>
<li><a name="ip_socket_address.ipv6"><code>ipv6</code></a>: <a href="#ipv6_socket_address"><a href="#ipv6_socket_address"><code>ipv6-socket-address</code></a></a></li>
</ul>
<h4><a name="ip_address_family"><code>enum ip-address-family</code></a></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="ip_address_family.ipv4"><code>ipv4</code></a></p>
<p>Similar to `AF_INET` in POSIX.
</li>
<li>
<p><a name="ip_address_family.ipv6"><code>ipv6</code></a></p>
<p>Similar to `AF_INET6` in POSIX.
</li>
</ul>
<h4><a name="ip_address"><code>variant ip-address</code></a></h4>
<h5>Variant Cases</h5>
<ul>
<li><a name="ip_address.ipv4"><code>ipv4</code></a>: <a href="#ipv4_address"><a href="#ipv4_address"><code>ipv4-address</code></a></a></li>
<li><a name="ip_address.ipv6"><code>ipv6</code></a>: <a href="#ipv6_address"><a href="#ipv6_address"><code>ipv6-address</code></a></a></li>
</ul>
<h4><a name="error_code"><code>enum error-code</code></a></h4>
<p>Error codes.</p>
<p>In theory, every API can return any error code.
In practice, API's typically only return the errors documented per API
combined with a couple of errors that are always possible:</p>
<ul>
<li><code>unknown</code></li>
<li><code>access-denied</code></li>
<li><code>not-supported</code></li>
<li><code>out-of-memory</code></li>
</ul>
<p>See each individual API for what the POSIX equivalents are. They sometimes differ per API.</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="error_code.unknown"><code>unknown</code></a></p>
<p>Unknown error
</li>
<li>
<p><a name="error_code.access_denied"><code>access-denied</code></a></p>
<p>Access denied.
<p>POSIX equivalent: EACCES, EPERM</p>
</li>
<li>
<p><a name="error_code.not_supported"><code>not-supported</code></a></p>
<p>The operation is not supported.
<p>POSIX equivalent: EOPNOTSUPP</p>
</li>
<li>
<p><a name="error_code.out_of_memory"><code>out-of-memory</code></a></p>
<p>Not enough memory to complete the operation.
<p>POSIX equivalent: ENOMEM, ENOBUFS, EAI_MEMORY</p>
</li>
<li>
<p><a name="error_code.timeout"><code>timeout</code></a></p>
<p>The operation timed out before it could finish completely.
</li>
<li>
<p><a name="error_code.concurrency_conflict"><code>concurrency-conflict</code></a></p>
<p>This operation is incompatible with another asynchronous operation that is already in progress.
</li>
<li>
<p><a name="error_code.not_in_progress"><code>not-in-progress</code></a></p>
<p>Trying to finish an asynchronous operation that:
- has not been started yet, or:
- was already finished by a previous `finish-*` call.
<p>Note: this is scheduled to be removed when <code>future</code>s are natively supported.</p>
</li>
<li>
<p><a name="error_code.would_block"><code>would-block</code></a></p>
<p>The operation has been aborted because it could not be completed immediately.
<p>Note: this is scheduled to be removed when <code>future</code>s are natively supported.</p>
</li>
<li>
<p><a name="error_code.address_family_not_supported"><code>address-family-not-supported</code></a></p>
<p>The specified address-family is not supported.
</li>
<li>
<p><a name="error_code.address_family_mismatch"><code>address-family-mismatch</code></a></p>
<p>An IPv4 address was passed to an IPv6 resource, or vice versa.
</li>
<li>
<p><a name="error_code.invalid_remote_address"><code>invalid-remote-address</code></a></p>
<p>The socket address is not a valid remote address. E.g. the IP address is set to INADDR_ANY, or the port is set to 0.
</li>
<li>
<p><a name="error_code.ipv4_only_operation"><code>ipv4-only-operation</code></a></p>
<p>The operation is only supported on IPv4 resources.
</li>
<li>
<p><a name="error_code.ipv6_only_operation"><code>ipv6-only-operation</code></a></p>
<p>The operation is only supported on IPv6 resources.
</li>
<li>
<p><a name="error_code.new_socket_limit"><code>new-socket-limit</code></a></p>
<p>A new socket resource could not be created because of a system limit.
</li>
<li>
<p><a name="error_code.already_attached"><code>already-attached</code></a></p>
<p>The socket is already attached to another network.
</li>
<li>
<p><a name="error_code.already_bound"><code>already-bound</code></a></p>
<p>The socket is already bound.
</li>
<li>
<p><a name="error_code.already_connected"><code>already-connected</code></a></p>
<p>The socket is already in the Connection state.
</li>
<li>
<p><a name="error_code.not_bound"><code>not-bound</code></a></p>
<p>The socket is not bound to any local address.
</li>
<li>
<p><a name="error_code.not_connected"><code>not-connected</code></a></p>
<p>The socket is not in the Connection state.
</li>
<li>
<p><a name="error_code.address_not_bindable"><code>address-not-bindable</code></a></p>
<p>A bind operation failed because the provided address is not an address that the `network` can bind to.
</li>
<li>
<p><a name="error_code.address_in_use"><code>address-in-use</code></a></p>
<p>A bind operation failed because the provided address is already in use.
</li>
<li>
<p><a name="error_code.ephemeral_ports_exhausted"><code>ephemeral-ports-exhausted</code></a></p>
<p>A bind operation failed because there are no ephemeral ports available.
</li>
<li>
<p><a name="error_code.remote_unreachable"><code>remote-unreachable</code></a></p>
<p>The remote address is not reachable
</li>
<li>
<p><a name="error_code.already_listening"><code>already-listening</code></a></p>
<p>The socket is already in the Listener state.
</li>
<li>
<p><a name="error_code.not_listening"><code>not-listening</code></a></p>
<p>The socket is already in the Listener state.
</li>
<li>
<p><a name="error_code.connection_refused"><code>connection-refused</code></a></p>
<p>The connection was forcefully rejected
</li>
<li>
<p><a name="error_code.connection_reset"><code>connection-reset</code></a></p>
<p>The connection was reset.
</li>
<li>
<p><a name="error_code.datagram_too_large"><code>datagram-too-large</code></a></p>
</li>
<li>
<p><a name="error_code.invalid_name"><code>invalid-name</code></a></p>
<p>The provided name is a syntactically invalid domain name.
</li>
<li>
<p><a name="error_code.name_unresolvable"><code>name-unresolvable</code></a></p>
<p>Name does not exist or has no suitable associated IP addresses.
</li>
<li>
<p><a name="error_code.temporary_resolver_failure"><code>temporary-resolver-failure</code></a></p>
<p>A temporary failure in name resolution occurred.
</li>
<li>
<p><a name="error_code.permanent_resolver_failure"><code>permanent-resolver-failure</code></a></p>
<p>A permanent failure in name resolution occurred.
</li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a name="drop_network"><code>drop-network: func</code></a></h4>
<p>Dispose of the specified <a href="#network"><code>network</code></a>, after which it may no longer be used.</p>
<p>Note: this function is scheduled to be removed when Resources are natively supported in Wit.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_network.this"><code>this</code></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
</ul>
<h2><a name="wasi:sockets_instance_network">Import interface wasi:sockets/instance-network</a></h2>
<p>This interface provides a value-export of the default network handle..</p>
<hr />
<h3>Types</h3>
<h4><a name="network"><code>type network</code></a></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
----
<h3>Functions</h3>
<h4><a name="instance_network"><code>instance-network: func</code></a></h4>
<p>Get a handle to the default network.</p>
<h5>Return values</h5>
<ul>
<li><a name="instance_network.0"></a> <a href="#network"><a href="#network"><code>network</code></a></a></li>
</ul>
<h2><a name="wasi:poll_poll">Import interface wasi:poll/poll</a></h2>
<p>A poll API intended to let users wait for I/O events on multiple handles
at once.</p>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>type pollable</code></a></h4>
<p><code>u32</code></p>
<p>A "pollable" handle.
<p>This is conceptually represents a <code>stream&lt;_, _&gt;</code>, or in other words,
a stream that one can wait on, repeatedly, but which does not itself
produce any data. It's temporary scaffolding until component-model's
async features are ready.</p>
<p>And at present, it is a <code>u32</code> instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.</p>
<p><a href="#pollable"><code>pollable</code></a> lifetimes are not automatically managed. Users must ensure
that they do not outlive the resource they reference.</p>
<p>This <a href="https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources">represents a resource</a>.</p>
<hr />
<h3>Functions</h3>
<h4><a name="drop_pollable"><code>drop-pollable: func</code></a></h4>
<p>Dispose of the specified <a href="#pollable"><code>pollable</code></a>, after which it may no longer
be used.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_pollable.this"><code>this</code></a>: <a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></li>
</ul>
<h4><a name="poll_oneoff"><code>poll-oneoff: func</code></a></h4>
<p>Poll for completion on a set of pollables.</p>
<p>The &quot;oneoff&quot; in the name refers to the fact that this function must do a
linear scan through the entire list of subscriptions, which may be
inefficient if the number is large and the same subscriptions are used
many times. In the future, this is expected to be obsoleted by the
component model async proposal, which will include a scalable waiting
facility.</p>
<p>Note that the return type would ideally be <code>list&lt;bool&gt;</code>, but that would
be more difficult to polyfill given the current state of <code>wit-bindgen</code>.
See <a href="https://github.com/bytecodealliance/preview2-prototyping/pull/11#issuecomment-1329873061">https://github.com/bytecodealliance/preview2-prototyping/pull/11#issuecomment-1329873061</a>
for details.  For now, we use zero to mean &quot;not ready&quot; and non-zero to
mean &quot;ready&quot;.</p>
<h5>Params</h5>
<ul>
<li><a name="poll_oneoff.in"><code>in</code></a>: list&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="poll_oneoff.0"></a> list&lt;<code>u8</code>&gt;</li>
</ul>
<h2><a name="wasi:sockets_udp">Import interface wasi:sockets/udp</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>type pollable</code></a></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
#### <a name="network">`type network`</a>
[`network`](#network)
<p>
#### <a name="error_code">`type error-code`</a>
[`error-code`](#error_code)
<p>
#### <a name="ip_socket_address">`type ip-socket-address`</a>
[`ip-socket-address`](#ip_socket_address)
<p>
#### <a name="ip_address_family">`type ip-address-family`</a>
[`ip-address-family`](#ip_address_family)
<p>
#### <a name="udp_socket">`type udp-socket`</a>
`u32`
<p>A UDP socket handle.
<h4><a name="datagram"><code>record datagram</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="datagram.data"><code>data</code></a>: list&lt;<code>u8</code>&gt;</li>
<li><a name="datagram.remote_address"><a href="#remote_address"><code>remote-address</code></a></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a name="start_bind"><code>start-bind: func</code></a></h4>
<p>Bind the socket to a specific network on the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is left to the implementation to decide which
network interface(s) to bind to.
If the TCP/UDP port is zero, the socket will be bound to a random free port.</p>
<p>When a socket is not explicitly bound, the first invocation to connect will implicitly bind the socket.</p>
<p>Unlike in POSIX, this function is async. This enables interactive WASI hosts to inject permission prompts.</p>
<h1>Typical <code>start</code> errors</h1>
<ul>
<li><code>address-family-mismatch</code>:   The <a href="#local_address"><code>local-address</code></a> has the wrong address family. (EINVAL)</li>
<li><code>already-bound</code>:             The socket is already bound. (EINVAL)</li>
<li><code>concurrency-conflict</code>:      Another <code>bind</code> or <code>connect</code> operation is already in progress. (EALREADY)</li>
</ul>
<h1>Typical <code>finish</code> errors</h1>
<ul>
<li><code>ephemeral-ports-exhausted</code>: No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)</li>
<li><code>address-in-use</code>:            Address is already in use. (EADDRINUSE)</li>
<li><code>address-not-bindable</code>:      <a href="#local_address"><code>local-address</code></a> is not an address that the <a href="#network"><code>network</code></a> can bind to. (EADDRNOTAVAIL)</li>
<li><code>not-in-progress</code>:           A <code>bind</code> operation is not in progress.</li>
<li><code>would-block</code>:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)</li>
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
<li><a name="start_bind.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="start_bind.network"><a href="#network"><code>network</code></a></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
<li><a name="start_bind.local_address"><a href="#local_address"><code>local-address</code></a></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="start_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="finish_bind"><code>finish-bind: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="finish_bind.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="finish_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="start_connect"><code>start-connect: func</code></a></h4>
<p>Set the destination address.</p>
<p>The local-address is updated based on the best network path to <a href="#remote_address"><code>remote-address</code></a>.</p>
<p>When a destination address is set:</p>
<ul>
<li>all receive operations will only return datagrams sent from the provided <a href="#remote_address"><code>remote-address</code></a>.</li>
<li>the <a href="#send"><code>send</code></a> function can only be used to send to this destination.</li>
</ul>
<p>Note that this function does not generate any network traffic and the peer is not aware of this &quot;connection&quot;.</p>
<p>Unlike in POSIX, this function is async. This enables interactive WASI hosts to inject permission prompts.</p>
<h1>Typical <code>start</code> errors</h1>
<ul>
<li><code>address-family-mismatch</code>:   The <a href="#remote_address"><code>remote-address</code></a> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-remote-address</code>:    The IP address in <a href="#remote_address"><code>remote-address</code></a> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-remote-address</code>:    The port in <a href="#remote_address"><code>remote-address</code></a> is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>already-attached</code>:          The socket is already bound to a different network. The <a href="#network"><code>network</code></a> passed to <code>connect</code> must be identical to the one passed to <code>bind</code>.</li>
<li><code>concurrency-conflict</code>:      Another <code>bind</code> or <code>connect</code> operation is already in progress. (EALREADY)</li>
</ul>
<h1>Typical <code>finish</code> errors</h1>
<ul>
<li><code>ephemeral-ports-exhausted</code>: Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)</li>
<li><code>not-in-progress</code>:           A <code>connect</code> operation is not in progress.</li>
<li><code>would-block</code>:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)</li>
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
<li><a name="start_connect.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="start_connect.network"><a href="#network"><code>network</code></a></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
<li><a name="start_connect.remote_address"><a href="#remote_address"><code>remote-address</code></a></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="start_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="finish_connect"><code>finish-connect: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="finish_connect.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="finish_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="receive"><code>receive: func</code></a></h4>
<p>Receive messages on the socket.</p>
<p>This function attempts to receive up to <code>max-results</code> datagrams on the socket without blocking.
The returned list may contain fewer elements than requested, but never more.
If <code>max-results</code> is 0, this function returns successfully with an empty list.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-bound</code>:          The socket is not bound to any local address. (EINVAL)</li>
<li><code>remote-unreachable</code>: The remote address is not reachable. (ECONNREFUSED, ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN)</li>
<li><code>would-block</code>:        There is no pending data available to be read at the moment. (EWOULDBLOCK, EAGAIN)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html</a></li>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/recv.2.html">https://man7.org/linux/man-pages/man2/recv.2.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/recvmmsg.2.html">https://man7.org/linux/man-pages/man2/recvmmsg.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recv">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recv</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recvfrom">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recvfrom</a></li>
<li><a href="https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms741687(v=vs.85)">https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms741687(v=vs.85)</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=recv&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=recv&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="receive.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="receive.max_results"><code>max-results</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="receive.0"></a> result&lt;list&lt;<a href="#datagram"><a href="#datagram"><code>datagram</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="send"><code>send: func</code></a></h4>
<p>Send messages on the socket.</p>
<p>This function attempts to send all provided <code>datagrams</code> on the socket without blocking and
returns how many messages were actually sent (or queued for sending).</p>
<p>This function semantically behaves the same as iterating the <code>datagrams</code> list and sequentially
sending each individual datagram until either the end of the list has been reached or the first error occurred.
If at least one datagram has been sent successfully, this function never returns an error.</p>
<p>If the input list is empty, the function returns <code>ok(0)</code>.</p>
<p>The remote address option is required. To send a message to the &quot;connected&quot; peer,
call <a href="#remote_address"><code>remote-address</code></a> to get their address.</p>
<h1>Typical errors</h1>
<ul>
<li><code>address-family-mismatch</code>: The <a href="#remote_address"><code>remote-address</code></a> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-remote-address</code>:  The IP address in <a href="#remote_address"><code>remote-address</code></a> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-remote-address</code>:  The port in <a href="#remote_address"><code>remote-address</code></a> is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>already-connected</code>:       The socket is in &quot;connected&quot; mode and the <code>datagram.remote-address</code> does not match the address passed to <code>connect</code>. (EISCONN)</li>
<li><code>not-bound</code>:               The socket is not bound to any local address. Unlike POSIX, this function does not perform an implicit bind.</li>
<li><code>remote-unreachable</code>:      The remote address is not reachable. (ECONNREFUSED, ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN)</li>
<li><code>datagram-too-large</code>:      The datagram is too large. (EMSGSIZE)</li>
<li><code>would-block</code>:             The send buffer is currently full. (EWOULDBLOCK, EAGAIN)</li>
</ul>
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
<li><a name="send.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="send.datagrams"><code>datagrams</code></a>: list&lt;<a href="#datagram"><a href="#datagram"><code>datagram</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="send.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="local_address"><code>local-address: func</code></a></h4>
<p>Get the current bound address.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-bound</code>: The socket is not bound to any local address.</li>
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
<li><a name="local_address.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="remote_address"><code>remote-address: func</code></a></h4>
<p>Get the address set with <code>connect</code>.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-connected</code>: The socket is not connected to a remote address. (ENOTCONN)</li>
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
<li><a name="remote_address.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="address_family"><code>address-family: func</code></a></h4>
<p>Whether this is a IPv4 or IPv6 socket.</p>
<p>Equivalent to the SO_DOMAIN socket option.</p>
<h5>Params</h5>
<ul>
<li><a name="address_family.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="address_family.0"></a> <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h4><a name="ipv6_only"><code>ipv6-only: func</code></a></h4>
<p>Whether IPv4 compatibility (dual-stack) mode is disabled or not.</p>
<p>Equivalent to the IPV6_V6ONLY socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>ipv6-only-operation</code>:  (get/set) <code>this</code> socket is an IPv4 socket.</li>
<li><code>already-bound</code>:        (set) The socket is already bound.</li>
<li><code>not-supported</code>:        (set) Host does not support dual-stack sockets. (Implementations are not required to.)</li>
<li><code>concurrency-conflict</code>: (set) Another <code>bind</code> or <code>connect</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="ipv6_only.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="ipv6_only.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_ipv6_only"><code>set-ipv6-only: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_ipv6_only.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="set_ipv6_only.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_ipv6_only.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="unicast_hop_limit"><code>unicast-hop-limit: func</code></a></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>concurrency-conflict</code>: (set) Another <code>bind</code> or <code>connect</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="unicast_hop_limit.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="unicast_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_unicast_hop_limit"><code>set-unicast-hop-limit: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_unicast_hop_limit.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="set_unicast_hop_limit.value"><code>value</code></a>: <code>u8</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_unicast_hop_limit.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="receive_buffer_size"><code>receive-buffer-size: func</code></a></h4>
<p>The kernel buffer space reserved for sends/receives on this socket.</p>
<p>Note #1: an implementation may choose to cap or round the buffer size when setting the value.
In other words, after setting a value, reading the same setting back may return a different value.</p>
<p>Note #2: there is not necessarily a direct relationship between the kernel buffer size and the bytes of
actual data to be sent/received by the application, because the kernel might also use the buffer space
for internal metadata structures.</p>
<p>Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>concurrency-conflict</code>: (set) Another <code>bind</code> or <code>connect</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="receive_buffer_size.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_receive_buffer_size"><code>set-receive-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_receive_buffer_size.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="set_receive_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_receive_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="send_buffer_size"><code>send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="send_buffer_size.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_send_buffer_size"><code>set-send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_send_buffer_size.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="set_send_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_send_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="subscribe"><code>subscribe: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the socket is ready for I/O.</p>
<p>Note: this function is here for WASI Preview2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a name="subscribe.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="subscribe.0"></a> <a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></li>
</ul>
<h4><a name="drop_udp_socket"><code>drop-udp-socket: func</code></a></h4>
<p>Dispose of the specified <a href="#udp_socket"><code>udp-socket</code></a>, after which it may no longer be used.</p>
<p>Note: this function is scheduled to be removed when Resources are natively supported in Wit.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_udp_socket.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h2><a name="wasi:sockets_udp_create_socket">Import interface wasi:sockets/udp-create-socket</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="network"><code>type network</code></a></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
#### <a name="error_code">`type error-code`</a>
[`error-code`](#error_code)
<p>
#### <a name="ip_address_family">`type ip-address-family`</a>
[`ip-address-family`](#ip_address_family)
<p>
#### <a name="udp_socket">`type udp-socket`</a>
[`udp-socket`](#udp_socket)
<p>
----
<h3>Functions</h3>
<h4><a name="create_udp_socket"><code>create-udp-socket: func</code></a></h4>
<p>Create a new UDP socket.</p>
<p>Similar to <code>socket(AF_INET or AF_INET6, SOCK_DGRAM, IPPROTO_UDP)</code> in POSIX.</p>
<p>This function does not require a network capability handle. This is considered to be safe because
at time of creation, the socket is not bound to any <a href="#network"><code>network</code></a> yet. Up to the moment <code>bind</code>/<code>connect</code> is called,
the socket is effectively an in-memory configuration object, unable to communicate with the outside world.</p>
<p>All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-supported</code>:                The host does not support UDP sockets. (EOPNOTSUPP)</li>
<li><code>address-family-not-supported</code>: The specified <a href="#address_family"><code>address-family</code></a> is not supported. (EAFNOSUPPORT)</li>
<li><code>new-socket-limit</code>:             The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)</li>
</ul>
<h1>References:</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/socket.2.html">https://man7.org/linux/man-pages/man2/socket.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=socket&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=socket&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="create_udp_socket.address_family"><a href="#address_family"><code>address-family</code></a></a>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="create_udp_socket.0"></a> result&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:io_streams">Import interface wasi:io/streams</a></h2>
<p>WASI I/O is an I/O abstraction API which is currently focused on providing
stream types.</p>
<p>In the future, the component model is expected to add built-in stream types;
when it does, they are expected to subsume this API.</p>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>type pollable</code></a></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
#### <a name="stream_error">`record stream-error`</a>
<p>An error type returned from a stream operation. Currently this
doesn't provide any additional information.</p>
<h5>Record Fields</h5>
<h4><a name="output_stream"><code>type output-stream</code></a></h4>
<p><code>u32</code></p>
<p>An output bytestream. In the future, this will be replaced by handle
types.
<p>This conceptually represents a <code>stream&lt;u8, _&gt;</code>. It's temporary
scaffolding until component-model's async features are ready.</p>
<p><a href="#output_stream"><code>output-stream</code></a>s are <em>non-blocking</em> to the extent practical on
underlying platforms. Except where specified otherwise, I/O operations also
always return promptly, after the number of bytes that can be written
promptly, which could even be zero. To wait for the stream to be ready to
accept data, the <a href="#subscribe_to_output_stream"><code>subscribe-to-output-stream</code></a> function to obtain a
<a href="#pollable"><code>pollable</code></a> which can be polled for using <code>wasi_poll</code>.</p>
<p>And at present, it is a <code>u32</code> instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.</p>
<p>This <a href="https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources">represents a resource</a>.</p>
<h4><a name="input_stream"><code>type input-stream</code></a></h4>
<p><code>u32</code></p>
<p>An input bytestream. In the future, this will be replaced by handle
types.
<p>This conceptually represents a <code>stream&lt;u8, _&gt;</code>. It's temporary
scaffolding until component-model's async features are ready.</p>
<p><a href="#input_stream"><code>input-stream</code></a>s are <em>non-blocking</em> to the extent practical on underlying
platforms. I/O operations always return promptly; if fewer bytes are
promptly available than requested, they return the number of bytes promptly
available, which could even be zero. To wait for data to be available,
use the <a href="#subscribe_to_input_stream"><code>subscribe-to-input-stream</code></a> function to obtain a <a href="#pollable"><code>pollable</code></a> which
can be polled for using <code>wasi_poll</code>.</p>
<p>And at present, it is a <code>u32</code> instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.</p>
<p>This <a href="https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources">represents a resource</a>.</p>
<hr />
<h3>Functions</h3>
<h4><a name="read"><code>read: func</code></a></h4>
<p>Read bytes from a stream.</p>
<p>This function returns a list of bytes containing the data that was
read, along with a bool which, when true, indicates that the end of the
stream was reached. The returned list will contain up to <code>len</code> bytes; it
may return fewer than requested, but not more.</p>
<p>Once a stream has reached the end, subsequent calls to read or
<a href="#skip"><code>skip</code></a> will always report end-of-stream rather than producing more
data.</p>
<p>If <code>len</code> is 0, it represents a request to read 0 bytes, which should
always succeed, assuming the stream hasn't reached its end yet, and
return an empty list.</p>
<p>The len here is a <code>u64</code>, but some callees may not be able to allocate
a buffer as large as that would imply.
FIXME: describe what happens if allocation fails.</p>
<h5>Params</h5>
<ul>
<li><a name="read.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
<li><a name="read.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="read.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <code>bool</code>), <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="blocking_read"><code>blocking-read: func</code></a></h4>
<p>Read bytes from a stream, with blocking.</p>
<p>This is similar to <a href="#read"><code>read</code></a>, except that it blocks until at least one
byte can be read.</p>
<h5>Params</h5>
<ul>
<li><a name="blocking_read.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
<li><a name="blocking_read.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="blocking_read.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <code>bool</code>), <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="skip"><code>skip: func</code></a></h4>
<p>Skip bytes from a stream.</p>
<p>This is similar to the <a href="#read"><code>read</code></a> function, but avoids copying the
bytes into the instance.</p>
<p>Once a stream has reached the end, subsequent calls to read or
<a href="#skip"><code>skip</code></a> will always report end-of-stream rather than producing more
data.</p>
<p>This function returns the number of bytes skipped, along with a bool
indicating whether the end of the stream was reached. The returned
value will be at most <code>len</code>; it may be less.</p>
<h5>Params</h5>
<ul>
<li><a name="skip.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
<li><a name="skip.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="skip.0"></a> result&lt;(<code>u64</code>, <code>bool</code>), <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="blocking_skip"><code>blocking-skip: func</code></a></h4>
<p>Skip bytes from a stream, with blocking.</p>
<p>This is similar to <a href="#skip"><code>skip</code></a>, except that it blocks until at least one
byte can be consumed.</p>
<h5>Params</h5>
<ul>
<li><a name="blocking_skip.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
<li><a name="blocking_skip.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="blocking_skip.0"></a> result&lt;(<code>u64</code>, <code>bool</code>), <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="subscribe_to_input_stream"><code>subscribe-to-input-stream: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once either the specified stream
has bytes available to read or the other end of the stream has been
closed.</p>
<h5>Params</h5>
<ul>
<li><a name="subscribe_to_input_stream.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="subscribe_to_input_stream.0"></a> <a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></li>
</ul>
<h4><a name="drop_input_stream"><code>drop-input-stream: func</code></a></h4>
<p>Dispose of the specified <a href="#input_stream"><code>input-stream</code></a>, after which it may no longer
be used.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_input_stream.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
</ul>
<h4><a name="write"><code>write: func</code></a></h4>
<p>Write bytes to a stream.</p>
<p>This function returns a <code>u64</code> indicating the number of bytes from
<code>buf</code> that were written; it may be less than the full list.</p>
<h5>Params</h5>
<ul>
<li><a name="write.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="write.buf"><code>buf</code></a>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="write.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="blocking_write"><code>blocking-write: func</code></a></h4>
<p>Write bytes to a stream, with blocking.</p>
<p>This is similar to <a href="#write"><code>write</code></a>, except that it blocks until at least one
byte can be written.</p>
<h5>Params</h5>
<ul>
<li><a name="blocking_write.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="blocking_write.buf"><code>buf</code></a>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="blocking_write.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="write_zeroes"><code>write-zeroes: func</code></a></h4>
<p>Write multiple zero bytes to a stream.</p>
<p>This function returns a <code>u64</code> indicating the number of zero bytes
that were written; it may be less than <code>len</code>.</p>
<h5>Params</h5>
<ul>
<li><a name="write_zeroes.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="write_zeroes.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="write_zeroes.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="blocking_write_zeroes"><code>blocking-write-zeroes: func</code></a></h4>
<p>Write multiple zero bytes to a stream, with blocking.</p>
<p>This is similar to <a href="#write_zeroes"><code>write-zeroes</code></a>, except that it blocks until at least
one byte can be written.</p>
<h5>Params</h5>
<ul>
<li><a name="blocking_write_zeroes.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="blocking_write_zeroes.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="blocking_write_zeroes.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="splice"><code>splice: func</code></a></h4>
<p>Read from one stream and write to another.</p>
<p>This function returns the number of bytes transferred; it may be less
than <code>len</code>.</p>
<p>Unlike other I/O functions, this function blocks until all the data
read from the input stream has been written to the output stream.</p>
<h5>Params</h5>
<ul>
<li><a name="splice.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="splice.src"><code>src</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
<li><a name="splice.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="splice.0"></a> result&lt;(<code>u64</code>, <code>bool</code>), <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="blocking_splice"><code>blocking-splice: func</code></a></h4>
<p>Read from one stream and write to another, with blocking.</p>
<p>This is similar to <a href="#splice"><code>splice</code></a>, except that it blocks until at least
one byte can be read.</p>
<h5>Params</h5>
<ul>
<li><a name="blocking_splice.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="blocking_splice.src"><code>src</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
<li><a name="blocking_splice.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="blocking_splice.0"></a> result&lt;(<code>u64</code>, <code>bool</code>), <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="forward"><code>forward: func</code></a></h4>
<p>Forward the entire contents of an input stream to an output stream.</p>
<p>This function repeatedly reads from the input stream and writes
the data to the output stream, until the end of the input stream
is reached, or an error is encountered.</p>
<p>Unlike other I/O functions, this function blocks until the end
of the input stream is seen and all the data has been written to
the output stream.</p>
<p>This function returns the number of bytes transferred.</p>
<h5>Params</h5>
<ul>
<li><a name="forward.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="forward.src"><code>src</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="forward.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="subscribe_to_output_stream"><code>subscribe-to-output-stream: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once either the specified stream
is ready to accept bytes or the other end of the stream has been closed.</p>
<h5>Params</h5>
<ul>
<li><a name="subscribe_to_output_stream.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="subscribe_to_output_stream.0"></a> <a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></li>
</ul>
<h4><a name="drop_output_stream"><code>drop-output-stream: func</code></a></h4>
<p>Dispose of the specified <a href="#output_stream"><code>output-stream</code></a>, after which it may no longer
be used.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_output_stream.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
</ul>
<h2><a name="wasi:sockets_tcp">Import interface wasi:sockets/tcp</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="input_stream"><code>type input-stream</code></a></h4>
<p><a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></p>
<p>
#### <a name="output_stream">`type output-stream`</a>
[`output-stream`](#output_stream)
<p>
#### <a name="pollable">`type pollable`</a>
[`pollable`](#pollable)
<p>
#### <a name="network">`type network`</a>
[`network`](#network)
<p>
#### <a name="error_code">`type error-code`</a>
[`error-code`](#error_code)
<p>
#### <a name="ip_socket_address">`type ip-socket-address`</a>
[`ip-socket-address`](#ip_socket_address)
<p>
#### <a name="ip_address_family">`type ip-address-family`</a>
[`ip-address-family`](#ip_address_family)
<p>
#### <a name="tcp_socket">`type tcp-socket`</a>
`u32`
<p>A TCP socket handle.
<h4><a name="shutdown_type"><code>enum shutdown-type</code></a></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="shutdown_type.receive"><a href="#receive"><code>receive</code></a></a></p>
<p>Similar to `SHUT_RD` in POSIX.
</li>
<li>
<p><a name="shutdown_type.send"><a href="#send"><code>send</code></a></a></p>
<p>Similar to `SHUT_WR` in POSIX.
</li>
<li>
<p><a name="shutdown_type.both"><code>both</code></a></p>
<p>Similar to `SHUT_RDWR` in POSIX.
</li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a name="start_bind"><code>start-bind: func</code></a></h4>
<p>Bind the socket to a specific network on the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is left to the implementation to decide which
network interface(s) to bind to.
If the TCP/UDP port is zero, the socket will be bound to a random free port.</p>
<p>When a socket is not explicitly bound, the first invocation to a listen or connect operation will
implicitly bind the socket.</p>
<p>Unlike in POSIX, this function is async. This enables interactive WASI hosts to inject permission prompts.</p>
<h1>Typical <code>start</code> errors</h1>
<ul>
<li><code>address-family-mismatch</code>:   The <a href="#local_address"><code>local-address</code></a> has the wrong address family. (EINVAL)</li>
<li><code>already-bound</code>:             The socket is already bound. (EINVAL)</li>
<li><code>concurrency-conflict</code>:      Another <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h1>Typical <code>finish</code> errors</h1>
<ul>
<li><code>ephemeral-ports-exhausted</code>: No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)</li>
<li><code>address-in-use</code>:            Address is already in use. (EADDRINUSE)</li>
<li><code>address-not-bindable</code>:      <a href="#local_address"><code>local-address</code></a> is not an address that the <a href="#network"><code>network</code></a> can bind to. (EADDRNOTAVAIL)</li>
<li><code>not-in-progress</code>:           A <code>bind</code> operation is not in progress.</li>
<li><code>would-block</code>:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)</li>
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
<li><a name="start_bind.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="start_bind.network"><a href="#network"><code>network</code></a></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
<li><a name="start_bind.local_address"><a href="#local_address"><code>local-address</code></a></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="start_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="finish_bind"><code>finish-bind: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="finish_bind.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="finish_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="start_connect"><code>start-connect: func</code></a></h4>
<p>Connect to a remote endpoint.</p>
<p>On success:</p>
<ul>
<li>the socket is transitioned into the Connection state</li>
<li>a pair of streams is returned that can be used to read &amp; write to the connection</li>
</ul>
<h1>Typical <code>start</code> errors</h1>
<ul>
<li><code>address-family-mismatch</code>:   The <a href="#remote_address"><code>remote-address</code></a> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-remote-address</code>:    The IP address in <a href="#remote_address"><code>remote-address</code></a> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EADDRNOTAVAIL on Windows)</li>
<li><code>invalid-remote-address</code>:    The port in <a href="#remote_address"><code>remote-address</code></a> is set to 0. (EADDRNOTAVAIL on Windows)</li>
<li><code>already-attached</code>:          The socket is already attached to a different network. The <a href="#network"><code>network</code></a> passed to <code>connect</code> must be identical to the one passed to <code>bind</code>.</li>
<li><code>already-connected</code>:         The socket is already in the Connection state. (EISCONN)</li>
<li><code>already-listening</code>:         The socket is already in the Listener state. (EOPNOTSUPP, EINVAL on Windows)</li>
<li><code>concurrency-conflict</code>:      Another <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h1>Typical <code>finish</code> errors</h1>
<ul>
<li><code>timeout</code>:                   Connection timed out. (ETIMEDOUT)</li>
<li><code>connection-refused</code>:        The connection was forcefully rejected. (ECONNREFUSED)</li>
<li><code>connection-reset</code>:          The connection was reset. (ECONNRESET)</li>
<li><code>remote-unreachable</code>:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN)</li>
<li><code>ephemeral-ports-exhausted</code>: Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)</li>
<li><code>not-in-progress</code>:           A <code>connect</code> operation is not in progress.</li>
<li><code>would-block</code>:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)</li>
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
<li><a name="start_connect.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="start_connect.network"><a href="#network"><code>network</code></a></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
<li><a name="start_connect.remote_address"><a href="#remote_address"><code>remote-address</code></a></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="start_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="finish_connect"><code>finish-connect: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="finish_connect.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="finish_connect.0"></a> result&lt;(<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>, <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="start_listen"><code>start-listen: func</code></a></h4>
<p>Start listening for new connections.</p>
<p>Transitions the socket into the Listener state.</p>
<p>Unlike POSIX:</p>
<ul>
<li>this function is async. This enables interactive WASI hosts to inject permission prompts.</li>
<li>the socket must already be explicitly bound.</li>
</ul>
<h1>Typical <code>start</code> errors</h1>
<ul>
<li><code>not-bound</code>:                 The socket is not bound to any local address. (EDESTADDRREQ)</li>
<li><code>already-connected</code>:         The socket is already in the Connection state. (EISCONN, EINVAL on BSD)</li>
<li><code>already-listening</code>:         The socket is already in the Listener state.</li>
<li><code>concurrency-conflict</code>:      Another <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EINVAL on BSD)</li>
</ul>
<h1>Typical <code>finish</code> errors</h1>
<ul>
<li><code>ephemeral-ports-exhausted</code>: Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)</li>
<li><code>not-in-progress</code>:           A <code>listen</code> operation is not in progress.</li>
<li><code>would-block</code>:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/listen.2.html">https://man7.org/linux/man-pages/man2/listen.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=listen&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=listen&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="start_listen.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="start_listen.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="finish_listen"><code>finish-listen: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="finish_listen.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="finish_listen.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="accept"><code>accept: func</code></a></h4>
<p>Accept a new client socket.</p>
<p>The returned socket is bound and in the Connection state.</p>
<p>On success, this function returns the newly accepted client socket along with
a pair of streams that can be used to read &amp; write to the connection.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-listening</code>: Socket is not in the Listener state. (EINVAL)</li>
<li><code>would-block</code>:   No pending connections at the moment. (EWOULDBLOCK, EAGAIN)</li>
</ul>
<p>Host implementations must skip over transient errors returned by the native accept syscall.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/accept.2.html">https://man7.org/linux/man-pages/man2/accept.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=accept&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=accept&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="accept.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="accept.0"></a> result&lt;(<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>, <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>, <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="local_address"><code>local-address: func</code></a></h4>
<p>Get the bound local address.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-bound</code>: The socket is not bound to any local address.</li>
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
<li><a name="local_address.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="remote_address"><code>remote-address: func</code></a></h4>
<p>Get the bound remote address.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-connected</code>: The socket is not connected to a remote address. (ENOTCONN)</li>
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
<li><a name="remote_address.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="address_family"><code>address-family: func</code></a></h4>
<p>Whether this is a IPv4 or IPv6 socket.</p>
<p>Equivalent to the SO_DOMAIN socket option.</p>
<h5>Params</h5>
<ul>
<li><a name="address_family.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="address_family.0"></a> <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h4><a name="ipv6_only"><code>ipv6-only: func</code></a></h4>
<p>Whether IPv4 compatibility (dual-stack) mode is disabled or not.</p>
<p>Equivalent to the IPV6_V6ONLY socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>ipv6-only-operation</code>:  (get/set) <code>this</code> socket is an IPv4 socket.</li>
<li><code>already-bound</code>:        (set) The socket is already bound.</li>
<li><code>not-supported</code>:        (set) Host does not support dual-stack sockets. (Implementations are not required to.)</li>
<li><code>concurrency-conflict</code>: (set) A <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="ipv6_only.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="ipv6_only.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_ipv6_only"><code>set-ipv6-only: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_ipv6_only.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_ipv6_only.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_ipv6_only.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_listen_backlog_size"><code>set-listen-backlog-size: func</code></a></h4>
<p>Hints the desired listen queue size. Implementations are free to ignore this.</p>
<h1>Typical errors</h1>
<ul>
<li><code>already-connected</code>:    (set) The socket is already in the Connection state.</li>
<li><code>concurrency-conflict</code>: (set) A <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="set_listen_backlog_size.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_listen_backlog_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_listen_backlog_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="keep_alive"><code>keep-alive: func</code></a></h4>
<p>Equivalent to the SO_KEEPALIVE socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>concurrency-conflict</code>: (set) A <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="keep_alive.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="keep_alive.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_keep_alive"><code>set-keep-alive: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_keep_alive.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_keep_alive.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_keep_alive.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="no_delay"><code>no-delay: func</code></a></h4>
<p>Equivalent to the TCP_NODELAY socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>concurrency-conflict</code>: (set) A <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="no_delay.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="no_delay.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_no_delay"><code>set-no-delay: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_no_delay.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_no_delay.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_no_delay.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="unicast_hop_limit"><code>unicast-hop-limit: func</code></a></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>already-connected</code>:    (set) The socket is already in the Connection state.</li>
<li><code>already-listening</code>:    (set) The socket is already in the Listener state.</li>
<li><code>concurrency-conflict</code>: (set) A <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="unicast_hop_limit.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="unicast_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_unicast_hop_limit"><code>set-unicast-hop-limit: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_unicast_hop_limit.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_unicast_hop_limit.value"><code>value</code></a>: <code>u8</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_unicast_hop_limit.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="receive_buffer_size"><code>receive-buffer-size: func</code></a></h4>
<p>The kernel buffer space reserved for sends/receives on this socket.</p>
<p>Note #1: an implementation may choose to cap or round the buffer size when setting the value.
In other words, after setting a value, reading the same setting back may return a different value.</p>
<p>Note #2: there is not necessarily a direct relationship between the kernel buffer size and the bytes of
actual data to be sent/received by the application, because the kernel might also use the buffer space
for internal metadata structures.</p>
<p>Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>already-connected</code>:    (set) The socket is already in the Connection state.</li>
<li><code>already-listening</code>:    (set) The socket is already in the Listener state.</li>
<li><code>concurrency-conflict</code>: (set) A <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="receive_buffer_size.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_receive_buffer_size"><code>set-receive-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_receive_buffer_size.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_receive_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_receive_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="send_buffer_size"><code>send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="send_buffer_size.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_send_buffer_size"><code>set-send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_send_buffer_size.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_send_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_send_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="subscribe"><code>subscribe: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the socket is ready for I/O.</p>
<p>Note: this function is here for WASI Preview2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a name="subscribe.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="subscribe.0"></a> <a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></li>
</ul>
<h4><a name="shutdown"><code>shutdown: func</code></a></h4>
<p>Initiate a graceful shutdown.</p>
<ul>
<li>receive: the socket is not expecting to receive any more data from the peer. All subsequent read
operations on the <a href="#input_stream"><code>input-stream</code></a> associated with this socket will return an End Of Stream indication.
Any data still in the receive queue at time of calling <a href="#shutdown"><code>shutdown</code></a> will be discarded.</li>
<li>send: the socket is not expecting to send any more data to the peer. All subsequent write
operations on the <a href="#output_stream"><code>output-stream</code></a> associated with this socket will return an error.</li>
<li>both: same effect as receive &amp; send combined.</li>
</ul>
<p>The shutdown function does not close (drop) the socket.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-connected</code>: The socket is not in the Connection state. (ENOTCONN)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/shutdown.2.html">https://man7.org/linux/man-pages/man2/shutdown.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-shutdown">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-shutdown</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=shutdown&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=shutdown&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="shutdown.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="shutdown.shutdown_type"><a href="#shutdown_type"><code>shutdown-type</code></a></a>: <a href="#shutdown_type"><a href="#shutdown_type"><code>shutdown-type</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="shutdown.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="drop_tcp_socket"><code>drop-tcp-socket: func</code></a></h4>
<p>Dispose of the specified <a href="#tcp_socket"><code>tcp-socket</code></a>, after which it may no longer be used.</p>
<p>Similar to the POSIX <code>close</code> function.</p>
<p>Note: this function is scheduled to be removed when Resources are natively supported in Wit.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_tcp_socket.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h2><a name="wasi:sockets_tcp_create_socket">Import interface wasi:sockets/tcp-create-socket</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="network"><code>type network</code></a></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
#### <a name="error_code">`type error-code`</a>
[`error-code`](#error_code)
<p>
#### <a name="ip_address_family">`type ip-address-family`</a>
[`ip-address-family`](#ip_address_family)
<p>
#### <a name="tcp_socket">`type tcp-socket`</a>
[`tcp-socket`](#tcp_socket)
<p>
----
<h3>Functions</h3>
<h4><a name="create_tcp_socket"><code>create-tcp-socket: func</code></a></h4>
<p>Create a new TCP socket.</p>
<p>Similar to <code>socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)</code> in POSIX.</p>
<p>This function does not require a network capability handle. This is considered to be safe because
at time of creation, the socket is not bound to any <a href="#network"><code>network</code></a> yet. Up to the moment <code>bind</code>/<code>listen</code>/<code>connect</code>
is called, the socket is effectively an in-memory configuration object, unable to communicate with the outside world.</p>
<p>All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-supported</code>:                The host does not support TCP sockets. (EOPNOTSUPP)</li>
<li><code>address-family-not-supported</code>: The specified <a href="#address_family"><code>address-family</code></a> is not supported. (EAFNOSUPPORT)</li>
<li><code>new-socket-limit</code>:             The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)</li>
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
<li><a name="create_tcp_socket.address_family"><a href="#address_family"><code>address-family</code></a></a>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="create_tcp_socket.0"></a> result&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:sockets_ip_name_lookup">Import interface wasi:sockets/ip-name-lookup</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>type pollable</code></a></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
#### <a name="network">`type network`</a>
[`network`](#network)
<p>
#### <a name="error_code">`type error-code`</a>
[`error-code`](#error_code)
<p>
#### <a name="ip_address">`type ip-address`</a>
[`ip-address`](#ip_address)
<p>
#### <a name="ip_address_family">`type ip-address-family`</a>
[`ip-address-family`](#ip_address_family)
<p>
#### <a name="resolve_address_stream">`type resolve-address-stream`</a>
`u32`
<p>
----
<h3>Functions</h3>
<h4><a name="resolve_addresses"><code>resolve-addresses: func</code></a></h4>
<p>Resolve an internet host name to a list of IP addresses.</p>
<p>See the wasi-socket proposal README.md for a comparison with getaddrinfo.</p>
<h1>Parameters</h1>
<ul>
<li><code>name</code>: The name to look up. IP addresses are not allowed. Unicode domain names are automatically converted
to ASCII using IDNA encoding.</li>
<li><a href="#address_family"><code>address-family</code></a>: If provided, limit the results to addresses of this specific address family.</li>
<li><code>include-unavailable</code>: When set to true, this function will also return addresses of which the runtime
thinks (or knows) can't be connected to at the moment. For example, this will return IPv6 addresses on
systems without an active IPv6 interface. Notes:</li>
<li>Even when no public IPv6 interfaces are present or active, names like &quot;localhost&quot; can still resolve to an IPv6 address.</li>
<li>Whatever is &quot;available&quot; or &quot;unavailable&quot; is volatile and can change everytime a network cable is unplugged.</li>
</ul>
<p>This function never blocks. It either immediately fails or immediately returns successfully with a <a href="#resolve_address_stream"><code>resolve-address-stream</code></a>
that can be used to (asynchronously) fetch the results.</p>
<p>At the moment, the stream never completes successfully with 0 items. Ie. the first call
to <a href="#resolve_next_address"><code>resolve-next-address</code></a> never returns <code>ok(none)</code>. This may change in the future.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-name</code>:                 <code>name</code> is a syntactically invalid domain name.</li>
<li><code>invalid-name</code>:                 <code>name</code> is an IP address.</li>
<li><code>address-family-not-supported</code>: The specified <a href="#address_family"><code>address-family</code></a> is not supported. (EAI_FAMILY)</li>
</ul>
<h1>References:</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man3/getaddrinfo.3.html">https://man7.org/linux/man-pages/man3/getaddrinfo.3.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo">https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&amp;sektion=3">https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&amp;sektion=3</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="resolve_addresses.network"><a href="#network"><code>network</code></a></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
<li><a name="resolve_addresses.name"><code>name</code></a>: <code>string</code></li>
<li><a name="resolve_addresses.address_family"><a href="#address_family"><code>address-family</code></a></a>: option&lt;<a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a>&gt;</li>
<li><a name="resolve_addresses.include_unavailable"><code>include-unavailable</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="resolve_addresses.0"></a> result&lt;<a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="resolve_next_address"><code>resolve-next-address: func</code></a></h4>
<p>Returns the next address from the resolver.</p>
<p>This function should be called multiple times. On each call, it will
return the next address in connection order preference. If all
addresses have been exhausted, this function returns <code>none</code>.
After which, you should release the stream with <a href="#drop_resolve_address_stream"><code>drop-resolve-address-stream</code></a>.</p>
<p>This function never returns IPv4-mapped IPv6 addresses.</p>
<h1>Typical errors</h1>
<ul>
<li><code>name-unresolvable</code>:          Name does not exist or has no suitable associated IP addresses. (EAI_NONAME, EAI_NODATA, EAI_ADDRFAMILY)</li>
<li><code>temporary-resolver-failure</code>: A temporary failure in name resolution occurred. (EAI_AGAIN)</li>
<li><code>permanent-resolver-failure</code>: A permanent failure in name resolution occurred. (EAI_FAIL)</li>
<li><code>would-block</code>:                A result is not available yet. (EWOULDBLOCK, EAGAIN)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="resolve_next_address.this"><code>this</code></a>: <a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="resolve_next_address.0"></a> result&lt;option&lt;<a href="#ip_address"><a href="#ip_address"><code>ip-address</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="drop_resolve_address_stream"><code>drop-resolve-address-stream: func</code></a></h4>
<p>Dispose of the specified <a href="#resolve_address_stream"><code>resolve-address-stream</code></a>, after which it may no longer be used.</p>
<p>Note: this function is scheduled to be removed when Resources are natively supported in Wit.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_resolve_address_stream.this"><code>this</code></a>: <a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a></li>
</ul>
<h4><a name="subscribe"><code>subscribe: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the stream is ready for I/O.</p>
<p>Note: this function is here for WASI Preview2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a name="subscribe.this"><code>this</code></a>: <a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="subscribe.0"></a> <a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></li>
</ul>
