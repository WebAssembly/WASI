<h1><a name="imports">World imports</a></h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi:sockets_network_0.2.0_rc_2023_11_10"><code>wasi:sockets/network@0.2.0-rc-2023-11-10</code></a></li>
<li>interface <a href="#wasi:sockets_instance_network_0.2.0_rc_2023_11_10"><code>wasi:sockets/instance-network@0.2.0-rc-2023-11-10</code></a></li>
<li>interface <a href="#wasi:io_poll_0.2.0_rc_2023_11_10"><code>wasi:io/poll@0.2.0-rc-2023-11-10</code></a></li>
<li>interface <a href="#wasi:sockets_udp_0.2.0_rc_2023_11_10"><code>wasi:sockets/udp@0.2.0-rc-2023-11-10</code></a></li>
<li>interface <a href="#wasi:sockets_udp_create_socket_0.2.0_rc_2023_11_10"><code>wasi:sockets/udp-create-socket@0.2.0-rc-2023-11-10</code></a></li>
<li>interface <a href="#wasi:io_error_0.2.0_rc_2023_11_10"><code>wasi:io/error@0.2.0-rc-2023-11-10</code></a></li>
<li>interface <a href="#wasi:io_streams_0.2.0_rc_2023_11_10"><code>wasi:io/streams@0.2.0-rc-2023-11-10</code></a></li>
<li>interface <a href="#wasi:sockets_tcp_0.2.0_rc_2023_11_10"><code>wasi:sockets/tcp@0.2.0-rc-2023-11-10</code></a></li>
<li>interface <a href="#wasi:sockets_tcp_create_socket_0.2.0_rc_2023_11_10"><code>wasi:sockets/tcp-create-socket@0.2.0-rc-2023-11-10</code></a></li>
<li>interface <a href="#wasi:sockets_ip_name_lookup_0.2.0_rc_2023_11_10"><code>wasi:sockets/ip-name-lookup@0.2.0-rc-2023-11-10</code></a></li>
</ul>
</li>
</ul>
<h2><a name="wasi:sockets_network_0.2.0_rc_2023_11_10">Import interface wasi:sockets/network@0.2.0-rc-2023-11-10</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="network"><code>resource network</code></a></h4>
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
<h4><a name="ipv4_address"><code>tuple ipv4-address</code></a></h4>
<h5>Tuple Fields</h5>
<ul>
<li><a name="ipv4_address.0"><code>0</code></a>: <code>u8</code></li>
<li><a name="ipv4_address.1"><code>1</code></a>: <code>u8</code></li>
<li><a name="ipv4_address.2"><code>2</code></a>: <code>u8</code></li>
<li><a name="ipv4_address.3"><code>3</code></a>: <code>u8</code></li>
</ul>
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
<h4><a name="ip_address"><code>variant ip-address</code></a></h4>
<h5>Variant Cases</h5>
<ul>
<li><a name="ip_address.ipv4"><code>ipv4</code></a>: <a href="#ipv4_address"><a href="#ipv4_address"><code>ipv4-address</code></a></a></li>
<li><a name="ip_address.ipv6"><code>ipv6</code></a>: <a href="#ipv6_address"><a href="#ipv6_address"><code>ipv6-address</code></a></a></li>
</ul>
<h4><a name="ipv4_socket_address"><code>record ipv4-socket-address</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="ipv4_socket_address.port"><code>port</code></a>: <code>u16</code></li>
<li><a name="ipv4_socket_address.address"><code>address</code></a>: <a href="#ipv4_address"><a href="#ipv4_address"><code>ipv4-address</code></a></a></li>
</ul>
<h4><a name="ipv6_socket_address"><code>record ipv6-socket-address</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="ipv6_socket_address.port"><code>port</code></a>: <code>u16</code></li>
<li><a name="ipv6_socket_address.flow_info"><code>flow-info</code></a>: <code>u32</code></li>
<li><a name="ipv6_socket_address.address"><code>address</code></a>: <a href="#ipv6_address"><a href="#ipv6_address"><code>ipv6-address</code></a></a></li>
<li><a name="ipv6_socket_address.scope_id"><code>scope-id</code></a>: <code>u32</code></li>
</ul>
<h4><a name="ip_socket_address"><code>variant ip-socket-address</code></a></h4>
<h5>Variant Cases</h5>
<ul>
<li><a name="ip_socket_address.ipv4"><code>ipv4</code></a>: <a href="#ipv4_socket_address"><a href="#ipv4_socket_address"><code>ipv4-socket-address</code></a></a></li>
<li><a name="ip_socket_address.ipv6"><code>ipv6</code></a>: <a href="#ipv6_socket_address"><a href="#ipv6_socket_address"><code>ipv6-socket-address</code></a></a></li>
</ul>
<h2><a name="wasi:sockets_instance_network_0.2.0_rc_2023_11_10">Import interface wasi:sockets/instance-network@0.2.0-rc-2023-11-10</a></h2>
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
<li><a name="instance_network.0"></a> own&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:io_poll_0.2.0_rc_2023_11_10">Import interface wasi:io/poll@0.2.0-rc-2023-11-10</a></h2>
<p>A poll API intended to let users wait for I/O events on multiple handles
at once.</p>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>resource pollable</code></a></h4>
<hr />
<h3>Functions</h3>
<h4><a name="method_pollable.ready"><code>[method]pollable.ready: func</code></a></h4>
<p>Return the readiness of a pollable. This function never blocks.</p>
<p>Returns <code>true</code> when the pollable is ready, and <code>false</code> otherwise.</p>
<h5>Params</h5>
<ul>
<li><a name="method_pollable.ready.self"><code>self</code></a>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_pollable.ready.0"></a> <code>bool</code></li>
</ul>
<h4><a name="method_pollable.block"><code>[method]pollable.block: func</code></a></h4>
<p><code>block</code> returns immediately if the pollable is ready, and otherwise
blocks until ready.</p>
<p>This function is equivalent to calling <code>poll.poll</code> on a list
containing only this pollable.</p>
<h5>Params</h5>
<ul>
<li><a name="method_pollable.block.self"><code>self</code></a>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a name="poll"><code>poll: func</code></a></h4>
<p>Poll for completion on a set of pollables.</p>
<p>This function takes a list of pollables, which identify I/O sources of
interest, and waits until one or more of the events is ready for I/O.</p>
<p>The result <code>list&lt;u32&gt;</code> contains one or more indices of handles in the
argument list that is ready for I/O.</p>
<p>If the list contains more elements than can be indexed with a <code>u32</code>
value, this function traps.</p>
<p>A timeout can be implemented by adding a pollable from the
wasi-clocks API to the list.</p>
<p>This function does not return a <code>result</code>; polling in itself does not
do any I/O so it doesn't fail. If any of the I/O sources identified by
the pollables has an error, it is indicated by marking the source as
being reaedy for I/O.</p>
<h5>Params</h5>
<ul>
<li><a name="poll.in"><code>in</code></a>: list&lt;borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="poll.0"></a> list&lt;<code>u32</code>&gt;</li>
</ul>
<h2><a name="wasi:sockets_udp_0.2.0_rc_2023_11_10">Import interface wasi:sockets/udp@0.2.0-rc-2023-11-10</a></h2>
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
#### <a name="datagram">`record datagram`</a>
<h5>Record Fields</h5>
<ul>
<li><a name="datagram.data"><code>data</code></a>: list&lt;<code>u8</code>&gt;</li>
<li><a name="datagram.remote_address"><code>remote-address</code></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h4><a name="udp_socket"><code>resource udp-socket</code></a></h4>
<hr />
<h3>Functions</h3>
<h4><a name="method_udp_socket.start_bind"><code>[method]udp-socket.start-bind: func</code></a></h4>
<p>Bind the socket to a specific network on the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is left to the implementation to decide which
network interface(s) to bind to.
If the TCP/UDP port is zero, the socket will be bound to a random free port.</p>
<p>When a socket is not explicitly bound, the first invocation to connect will implicitly bind the socket.</p>
<p>Unlike in POSIX, this function is async. This enables interactive WASI hosts to inject permission prompts.</p>
<h1>Typical <code>start</code> errors</h1>
<ul>
<li><code>address-family-mismatch</code>:   The <code>local-address</code> has the wrong address family. (EINVAL)</li>
<li><code>already-bound</code>:             The socket is already bound. (EINVAL)</li>
<li><code>concurrency-conflict</code>:      Another <code>bind</code> or <code>connect</code> operation is already in progress. (EALREADY)</li>
</ul>
<h1>Typical <code>finish</code> errors</h1>
<ul>
<li><code>ephemeral-ports-exhausted</code>: No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)</li>
<li><code>address-in-use</code>:            Address is already in use. (EADDRINUSE)</li>
<li><code>address-not-bindable</code>:      <code>local-address</code> is not an address that the <a href="#network"><code>network</code></a> can bind to. (EADDRNOTAVAIL)</li>
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
<li><a name="method_udp_socket.start_bind.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a name="method_udp_socket.start_bind.network"><a href="#network"><code>network</code></a></a>: borrow&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
<li><a name="method_udp_socket.start_bind.local_address"><code>local-address</code></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.start_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.finish_bind"><code>[method]udp-socket.finish-bind: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_udp_socket.finish_bind.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.finish_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.start_connect"><code>[method]udp-socket.start-connect: func</code></a></h4>
<p>Set the destination address.</p>
<p>The local-address is updated based on the best network path to <code>remote-address</code>.</p>
<p>When a destination address is set:</p>
<ul>
<li>all receive operations will only return datagrams sent from the provided <code>remote-address</code>.</li>
<li>the <code>send</code> function can only be used to send to this destination.</li>
</ul>
<p>Note that this function does not generate any network traffic and the peer is not aware of this &quot;connection&quot;.</p>
<p>Unlike in POSIX, this function is async. This enables interactive WASI hosts to inject permission prompts.</p>
<h1>Typical <code>start</code> errors</h1>
<ul>
<li><code>address-family-mismatch</code>:   The <code>remote-address</code> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-remote-address</code>:    The IP address in <code>remote-address</code> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-remote-address</code>:    The port in <code>remote-address</code> is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)</li>
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
<li><a name="method_udp_socket.start_connect.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a name="method_udp_socket.start_connect.network"><a href="#network"><code>network</code></a></a>: borrow&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
<li><a name="method_udp_socket.start_connect.remote_address"><code>remote-address</code></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.start_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.finish_connect"><code>[method]udp-socket.finish-connect: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_udp_socket.finish_connect.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.finish_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.receive"><code>[method]udp-socket.receive: func</code></a></h4>
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
<li><a name="method_udp_socket.receive.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a name="method_udp_socket.receive.max_results"><code>max-results</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.receive.0"></a> result&lt;list&lt;<a href="#datagram"><a href="#datagram"><code>datagram</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.send"><code>[method]udp-socket.send: func</code></a></h4>
<p>Send messages on the socket.</p>
<p>This function attempts to send all provided <code>datagrams</code> on the socket without blocking and
returns how many messages were actually sent (or queued for sending).</p>
<p>This function semantically behaves the same as iterating the <code>datagrams</code> list and sequentially
sending each individual datagram until either the end of the list has been reached or the first error occurred.
If at least one datagram has been sent successfully, this function never returns an error.</p>
<p>If the input list is empty, the function returns <code>ok(0)</code>.</p>
<p>The remote address option is required. To send a message to the &quot;connected&quot; peer,
call <code>remote-address</code> to get their address.</p>
<h1>Typical errors</h1>
<ul>
<li><code>address-family-mismatch</code>: The <code>remote-address</code> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-remote-address</code>:  The IP address in <code>remote-address</code> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-remote-address</code>:  The port in <code>remote-address</code> is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)</li>
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
<li><a name="method_udp_socket.send.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a name="method_udp_socket.send.datagrams"><code>datagrams</code></a>: list&lt;<a href="#datagram"><a href="#datagram"><code>datagram</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.send.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.local_address"><code>[method]udp-socket.local-address: func</code></a></h4>
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
<li><a name="method_udp_socket.local_address.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.remote_address"><code>[method]udp-socket.remote-address: func</code></a></h4>
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
<li><a name="method_udp_socket.remote_address.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.address_family"><code>[method]udp-socket.address-family: func</code></a></h4>
<p>Whether this is a IPv4 or IPv6 socket.</p>
<p>Equivalent to the SO_DOMAIN socket option.</p>
<h5>Params</h5>
<ul>
<li><a name="method_udp_socket.address_family.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.address_family.0"></a> <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h4><a name="method_udp_socket.ipv6_only"><code>[method]udp-socket.ipv6-only: func</code></a></h4>
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
<li><a name="method_udp_socket.ipv6_only.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.ipv6_only.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.set_ipv6_only"><code>[method]udp-socket.set-ipv6-only: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_udp_socket.set_ipv6_only.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a name="method_udp_socket.set_ipv6_only.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.set_ipv6_only.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.unicast_hop_limit"><code>[method]udp-socket.unicast-hop-limit: func</code></a></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>concurrency-conflict</code>: (set) Another <code>bind</code> or <code>connect</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="method_udp_socket.unicast_hop_limit.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.unicast_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.set_unicast_hop_limit"><code>[method]udp-socket.set-unicast-hop-limit: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_udp_socket.set_unicast_hop_limit.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a name="method_udp_socket.set_unicast_hop_limit.value"><code>value</code></a>: <code>u8</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.set_unicast_hop_limit.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.receive_buffer_size"><code>[method]udp-socket.receive-buffer-size: func</code></a></h4>
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
<li><a name="method_udp_socket.receive_buffer_size.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.set_receive_buffer_size"><code>[method]udp-socket.set-receive-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_udp_socket.set_receive_buffer_size.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a name="method_udp_socket.set_receive_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.set_receive_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.send_buffer_size"><code>[method]udp-socket.send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_udp_socket.send_buffer_size.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.set_send_buffer_size"><code>[method]udp-socket.set-send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_udp_socket.set_send_buffer_size.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a name="method_udp_socket.set_send_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.set_send_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_udp_socket.subscribe"><code>[method]udp-socket.subscribe: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the socket is ready for I/O.</p>
<p>Note: this function is here for WASI Preview2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a name="method_udp_socket.subscribe.self"><code>self</code></a>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_udp_socket.subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:sockets_udp_create_socket_0.2.0_rc_2023_11_10">Import interface wasi:sockets/udp-create-socket@0.2.0-rc-2023-11-10</a></h2>
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
<li><code>address-family-not-supported</code>: The specified <code>address-family</code> is not supported. (EAFNOSUPPORT)</li>
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
<li><a name="create_udp_socket.address_family"><code>address-family</code></a>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="create_udp_socket.0"></a> result&lt;own&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:io_error_0.2.0_rc_2023_11_10">Import interface wasi:io/error@0.2.0-rc-2023-11-10</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="error"><code>resource error</code></a></h4>
<hr />
<h3>Functions</h3>
<h4><a name="method_error.to_debug_string"><code>[method]error.to-debug-string: func</code></a></h4>
<p>Returns a string that is suitable to assist humans in debugging
this error.</p>
<p>WARNING: The returned string should not be consumed mechanically!
It may change across platforms, hosts, or other implementation
details. Parsing this string is a major platform-compatibility
hazard.</p>
<h5>Params</h5>
<ul>
<li><a name="method_error.to_debug_string.self"><code>self</code></a>: borrow&lt;<a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_error.to_debug_string.0"></a> <code>string</code></li>
</ul>
<h2><a name="wasi:io_streams_0.2.0_rc_2023_11_10">Import interface wasi:io/streams@0.2.0-rc-2023-11-10</a></h2>
<p>WASI I/O is an I/O abstraction API which is currently focused on providing
stream types.</p>
<p>In the future, the component model is expected to add built-in stream types;
when it does, they are expected to subsume this API.</p>
<hr />
<h3>Types</h3>
<h4><a name="error"><code>type error</code></a></h4>
<p><a href="#error"><a href="#error"><code>error</code></a></a></p>
<p>
#### <a name="pollable">`type pollable`</a>
[`pollable`](#pollable)
<p>
#### <a name="stream_error">`variant stream-error`</a>
<p>An error for input-stream and output-stream operations.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a name="stream_error.last_operation_failed"><code>last-operation-failed</code></a>: own&lt;<a href="#error"><a href="#error"><code>error</code></a></a>&gt;</p>
<p>The last operation (a write or flush) failed before completion.
<p>More information is available in the <a href="#error"><code>error</code></a> payload.</p>
</li>
<li>
<p><a name="stream_error.closed"><code>closed</code></a></p>
<p>The stream is closed: no more input will be accepted by the
stream. A closed output-stream will return this error on all
future operations.
</li>
</ul>
<h4><a name="input_stream"><code>resource input-stream</code></a></h4>
<h4><a name="output_stream"><code>resource output-stream</code></a></h4>
<hr />
<h3>Functions</h3>
<h4><a name="method_input_stream.read"><code>[method]input-stream.read: func</code></a></h4>
<p>Perform a non-blocking read from the stream.</p>
<p>This function returns a list of bytes containing the read data,
when successful. The returned list will contain up to <code>len</code> bytes;
it may return fewer than requested, but not more. The list is
empty when no bytes are available for reading at this time. The
pollable given by <code>subscribe</code> will be ready when more bytes are
available.</p>
<p>This function fails with a <a href="#stream_error"><code>stream-error</code></a> when the operation
encounters an error, giving <code>last-operation-failed</code>, or when the
stream is closed, giving <code>closed</code>.</p>
<p>When the caller gives a <code>len</code> of 0, it represents a request to
read 0 bytes. If the stream is still open, this call should
succeed and return an empty list, or otherwise fail with <code>closed</code>.</p>
<p>The <code>len</code> parameter is a <code>u64</code>, which could represent a list of u8 which
is not possible to allocate in wasm32, or not desirable to allocate as
as a return value by the callee. The callee may return a list of bytes
less than <code>len</code> in size while more bytes are available for reading.</p>
<h5>Params</h5>
<ul>
<li><a name="method_input_stream.read.self"><code>self</code></a>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a name="method_input_stream.read.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_input_stream.read.0"></a> result&lt;list&lt;<code>u8</code>&gt;, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_input_stream.blocking_read"><code>[method]input-stream.blocking-read: func</code></a></h4>
<p>Read bytes from a stream, after blocking until at least one byte can
be read. Except for blocking, behavior is identical to <code>read</code>.</p>
<h5>Params</h5>
<ul>
<li><a name="method_input_stream.blocking_read.self"><code>self</code></a>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a name="method_input_stream.blocking_read.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_input_stream.blocking_read.0"></a> result&lt;list&lt;<code>u8</code>&gt;, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_input_stream.skip"><code>[method]input-stream.skip: func</code></a></h4>
<p>Skip bytes from a stream. Returns number of bytes skipped.</p>
<p>Behaves identical to <code>read</code>, except instead of returning a list
of bytes, returns the number of bytes consumed from the stream.</p>
<h5>Params</h5>
<ul>
<li><a name="method_input_stream.skip.self"><code>self</code></a>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a name="method_input_stream.skip.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_input_stream.skip.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_input_stream.blocking_skip"><code>[method]input-stream.blocking-skip: func</code></a></h4>
<p>Skip bytes from a stream, after blocking until at least one byte
can be skipped. Except for blocking behavior, identical to <code>skip</code>.</p>
<h5>Params</h5>
<ul>
<li><a name="method_input_stream.blocking_skip.self"><code>self</code></a>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a name="method_input_stream.blocking_skip.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_input_stream.blocking_skip.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_input_stream.subscribe"><code>[method]input-stream.subscribe: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once either the specified stream
has bytes available to read or the other end of the stream has been
closed.
The created <a href="#pollable"><code>pollable</code></a> is a child resource of the <a href="#input_stream"><code>input-stream</code></a>.
Implementations may trap if the <a href="#input_stream"><code>input-stream</code></a> is dropped before
all derived <a href="#pollable"><code>pollable</code></a>s created with this function are dropped.</p>
<h5>Params</h5>
<ul>
<li><a name="method_input_stream.subscribe.self"><code>self</code></a>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_input_stream.subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.check_write"><code>[method]output-stream.check-write: func</code></a></h4>
<p>Check readiness for writing. This function never blocks.</p>
<p>Returns the number of bytes permitted for the next call to <code>write</code>,
or an error. Calling <code>write</code> with more bytes than this function has
permitted will trap.</p>
<p>When this function returns 0 bytes, the <code>subscribe</code> pollable will
become ready when this function will report at least 1 byte, or an
error.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.check_write.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.check_write.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.write"><code>[method]output-stream.write: func</code></a></h4>
<p>Perform a write. This function never blocks.</p>
<p>Precondition: check-write gave permit of Ok(n) and contents has a
length of less than or equal to n. Otherwise, this function will trap.</p>
<p>returns Err(closed) without writing if the stream has closed since
the last call to check-write provided a permit.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.write.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.write.contents"><code>contents</code></a>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.write.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.blocking_write_and_flush"><code>[method]output-stream.blocking-write-and-flush: func</code></a></h4>
<p>Perform a write of up to 4096 bytes, and then flush the stream. Block
until all of these operations are complete, or an error occurs.</p>
<p>This is a convenience wrapper around the use of <code>check-write</code>,
<code>subscribe</code>, <code>write</code>, and <code>flush</code>, and is implemented with the
following pseudo-code:</p>
<pre><code class="language-text">let pollable = this.subscribe();
while !contents.is_empty() {
  // Wait for the stream to become writable
  poll-one(pollable);
  let Ok(n) = this.check-write(); // eliding error handling
  let len = min(n, contents.len());
  let (chunk, rest) = contents.split_at(len);
  this.write(chunk  );            // eliding error handling
  contents = rest;
}
this.flush();
// Wait for completion of `flush`
poll-one(pollable);
// Check for any errors that arose during `flush`
let _ = this.check-write();         // eliding error handling
</code></pre>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.blocking_write_and_flush.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.blocking_write_and_flush.contents"><code>contents</code></a>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.blocking_write_and_flush.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.flush"><code>[method]output-stream.flush: func</code></a></h4>
<p>Request to flush buffered output. This function never blocks.</p>
<p>This tells the output-stream that the caller intends any buffered
output to be flushed. the output which is expected to be flushed
is all that has been passed to <code>write</code> prior to this call.</p>
<p>Upon calling this function, the <a href="#output_stream"><code>output-stream</code></a> will not accept any
writes (<code>check-write</code> will return <code>ok(0)</code>) until the flush has
completed. The <code>subscribe</code> pollable will become ready when the
flush has completed and the stream can accept more writes.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.flush.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.flush.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.blocking_flush"><code>[method]output-stream.blocking-flush: func</code></a></h4>
<p>Request to flush buffered output, and block until flush completes
and stream is ready for writing again.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.blocking_flush.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.blocking_flush.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.subscribe"><code>[method]output-stream.subscribe: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the output-stream
is ready for more writing, or an error has occured. When this
pollable is ready, <code>check-write</code> will return <code>ok(n)</code> with n&gt;0, or an
error.</p>
<p>If the stream is closed, this pollable is always ready immediately.</p>
<p>The created <a href="#pollable"><code>pollable</code></a> is a child resource of the <a href="#output_stream"><code>output-stream</code></a>.
Implementations may trap if the <a href="#output_stream"><code>output-stream</code></a> is dropped before
all derived <a href="#pollable"><code>pollable</code></a>s created with this function are dropped.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.subscribe.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.write_zeroes"><code>[method]output-stream.write-zeroes: func</code></a></h4>
<p>Write zeroes to a stream.</p>
<p>this should be used precisely like <code>write</code> with the exact same
preconditions (must use check-write first), but instead of
passing a list of bytes, you simply pass the number of zero-bytes
that should be written.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.write_zeroes.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.write_zeroes.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.write_zeroes.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.blocking_write_zeroes_and_flush"><code>[method]output-stream.blocking-write-zeroes-and-flush: func</code></a></h4>
<p>Perform a write of up to 4096 zeroes, and then flush the stream.
Block until all of these operations are complete, or an error
occurs.</p>
<p>This is a convenience wrapper around the use of <code>check-write</code>,
<code>subscribe</code>, <code>write-zeroes</code>, and <code>flush</code>, and is implemented with
the following pseudo-code:</p>
<pre><code class="language-text">let pollable = this.subscribe();
while num_zeroes != 0 {
  // Wait for the stream to become writable
  poll-one(pollable);
  let Ok(n) = this.check-write(); // eliding error handling
  let len = min(n, num_zeroes);
  this.write-zeroes(len);         // eliding error handling
  num_zeroes -= len;
}
this.flush();
// Wait for completion of `flush`
poll-one(pollable);
// Check for any errors that arose during `flush`
let _ = this.check-write();         // eliding error handling
</code></pre>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.blocking_write_zeroes_and_flush.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.blocking_write_zeroes_and_flush.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.blocking_write_zeroes_and_flush.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.splice"><code>[method]output-stream.splice: func</code></a></h4>
<p>Read from one stream and write to another.</p>
<p>The behavior of splice is equivelant to:</p>
<ol>
<li>calling <code>check-write</code> on the <a href="#output_stream"><code>output-stream</code></a></li>
<li>calling <code>read</code> on the <a href="#input_stream"><code>input-stream</code></a> with the smaller of the
<code>check-write</code> permitted length and the <code>len</code> provided to <code>splice</code></li>
<li>calling <code>write</code> on the <a href="#output_stream"><code>output-stream</code></a> with that read data.</li>
</ol>
<p>Any error reported by the call to <code>check-write</code>, <code>read</code>, or
<code>write</code> ends the splice and reports that error.</p>
<p>This function returns the number of bytes transferred; it may be less
than <code>len</code>.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.splice.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.splice.src"><code>src</code></a>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.splice.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.splice.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.blocking_splice"><code>[method]output-stream.blocking-splice: func</code></a></h4>
<p>Read from one stream and write to another, with blocking.</p>
<p>This is similar to <code>splice</code>, except that it blocks until the
<a href="#output_stream"><code>output-stream</code></a> is ready for writing, and the <a href="#input_stream"><code>input-stream</code></a>
is ready for reading, before performing the <code>splice</code>.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.blocking_splice.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.blocking_splice.src"><code>src</code></a>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.blocking_splice.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.blocking_splice.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:sockets_tcp_0.2.0_rc_2023_11_10">Import interface wasi:sockets/tcp@0.2.0-rc-2023-11-10</a></h2>
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
#### <a name="shutdown_type">`enum shutdown-type`</a>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="shutdown_type.receive"><code>receive</code></a></p>
<p>Similar to `SHUT_RD` in POSIX.
</li>
<li>
<p><a name="shutdown_type.send"><code>send</code></a></p>
<p>Similar to `SHUT_WR` in POSIX.
</li>
<li>
<p><a name="shutdown_type.both"><code>both</code></a></p>
<p>Similar to `SHUT_RDWR` in POSIX.
</li>
</ul>
<h4><a name="tcp_socket"><code>resource tcp-socket</code></a></h4>
<hr />
<h3>Functions</h3>
<h4><a name="method_tcp_socket.start_bind"><code>[method]tcp-socket.start-bind: func</code></a></h4>
<p>Bind the socket to a specific network on the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is left to the implementation to decide which
network interface(s) to bind to.
If the TCP/UDP port is zero, the socket will be bound to a random free port.</p>
<p>When a socket is not explicitly bound, the first invocation to a listen or connect operation will
implicitly bind the socket.</p>
<p>Unlike in POSIX, this function is async. This enables interactive WASI hosts to inject permission prompts.</p>
<h1>Typical <code>start</code> errors</h1>
<ul>
<li><code>address-family-mismatch</code>:   The <code>local-address</code> has the wrong address family. (EINVAL)</li>
<li><code>already-bound</code>:             The socket is already bound. (EINVAL)</li>
<li><code>concurrency-conflict</code>:      Another <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h1>Typical <code>finish</code> errors</h1>
<ul>
<li><code>ephemeral-ports-exhausted</code>: No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)</li>
<li><code>address-in-use</code>:            Address is already in use. (EADDRINUSE)</li>
<li><code>address-not-bindable</code>:      <code>local-address</code> is not an address that the <a href="#network"><code>network</code></a> can bind to. (EADDRNOTAVAIL)</li>
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
<li><a name="method_tcp_socket.start_bind.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.start_bind.network"><a href="#network"><code>network</code></a></a>: borrow&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.start_bind.local_address"><code>local-address</code></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.start_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.finish_bind"><code>[method]tcp-socket.finish-bind: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.finish_bind.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.finish_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.start_connect"><code>[method]tcp-socket.start-connect: func</code></a></h4>
<p>Connect to a remote endpoint.</p>
<p>On success:</p>
<ul>
<li>the socket is transitioned into the Connection state</li>
<li>a pair of streams is returned that can be used to read &amp; write to the connection</li>
</ul>
<h1>Typical <code>start</code> errors</h1>
<ul>
<li><code>address-family-mismatch</code>:   The <code>remote-address</code> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-remote-address</code>:    The IP address in <code>remote-address</code> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EADDRNOTAVAIL on Windows)</li>
<li><code>invalid-remote-address</code>:    The port in <code>remote-address</code> is set to 0. (EADDRNOTAVAIL on Windows)</li>
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
<li><a name="method_tcp_socket.start_connect.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.start_connect.network"><a href="#network"><code>network</code></a></a>: borrow&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.start_connect.remote_address"><code>remote-address</code></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.start_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.finish_connect"><code>[method]tcp-socket.finish-connect: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.finish_connect.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.finish_connect.0"></a> result&lt;(own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;, own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.start_listen"><code>[method]tcp-socket.start-listen: func</code></a></h4>
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
<li><a name="method_tcp_socket.start_listen.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.start_listen.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.finish_listen"><code>[method]tcp-socket.finish-listen: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.finish_listen.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.finish_listen.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.accept"><code>[method]tcp-socket.accept: func</code></a></h4>
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
<li><a name="method_tcp_socket.accept.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.accept.0"></a> result&lt;(own&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;, own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;, own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.local_address"><code>[method]tcp-socket.local-address: func</code></a></h4>
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
<li><a name="method_tcp_socket.local_address.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.remote_address"><code>[method]tcp-socket.remote-address: func</code></a></h4>
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
<li><a name="method_tcp_socket.remote_address.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.address_family"><code>[method]tcp-socket.address-family: func</code></a></h4>
<p>Whether this is a IPv4 or IPv6 socket.</p>
<p>Equivalent to the SO_DOMAIN socket option.</p>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.address_family.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.address_family.0"></a> <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h4><a name="method_tcp_socket.ipv6_only"><code>[method]tcp-socket.ipv6-only: func</code></a></h4>
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
<li><a name="method_tcp_socket.ipv6_only.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.ipv6_only.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.set_ipv6_only"><code>[method]tcp-socket.set-ipv6-only: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.set_ipv6_only.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.set_ipv6_only.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.set_ipv6_only.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.set_listen_backlog_size"><code>[method]tcp-socket.set-listen-backlog-size: func</code></a></h4>
<p>Hints the desired listen queue size. Implementations are free to ignore this.</p>
<h1>Typical errors</h1>
<ul>
<li><code>already-connected</code>:    (set) The socket is already in the Connection state.</li>
<li><code>concurrency-conflict</code>: (set) A <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.set_listen_backlog_size.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.set_listen_backlog_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.set_listen_backlog_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.keep_alive"><code>[method]tcp-socket.keep-alive: func</code></a></h4>
<p>Equivalent to the SO_KEEPALIVE socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>concurrency-conflict</code>: (set) A <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.keep_alive.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.keep_alive.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.set_keep_alive"><code>[method]tcp-socket.set-keep-alive: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.set_keep_alive.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.set_keep_alive.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.set_keep_alive.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.no_delay"><code>[method]tcp-socket.no-delay: func</code></a></h4>
<p>Equivalent to the TCP_NODELAY socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>concurrency-conflict</code>: (set) A <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.no_delay.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.no_delay.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.set_no_delay"><code>[method]tcp-socket.set-no-delay: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.set_no_delay.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.set_no_delay.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.set_no_delay.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.unicast_hop_limit"><code>[method]tcp-socket.unicast-hop-limit: func</code></a></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>already-connected</code>:    (set) The socket is already in the Connection state.</li>
<li><code>already-listening</code>:    (set) The socket is already in the Listener state.</li>
<li><code>concurrency-conflict</code>: (set) A <code>bind</code>, <code>connect</code> or <code>listen</code> operation is already in progress. (EALREADY)</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.unicast_hop_limit.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.unicast_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.set_unicast_hop_limit"><code>[method]tcp-socket.set-unicast-hop-limit: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.set_unicast_hop_limit.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.set_unicast_hop_limit.value"><code>value</code></a>: <code>u8</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.set_unicast_hop_limit.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.receive_buffer_size"><code>[method]tcp-socket.receive-buffer-size: func</code></a></h4>
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
<li><a name="method_tcp_socket.receive_buffer_size.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.set_receive_buffer_size"><code>[method]tcp-socket.set-receive-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.set_receive_buffer_size.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.set_receive_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.set_receive_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.send_buffer_size"><code>[method]tcp-socket.send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.send_buffer_size.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.set_send_buffer_size"><code>[method]tcp-socket.set-send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.set_send_buffer_size.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.set_send_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.set_send_buffer_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.subscribe"><code>[method]tcp-socket.subscribe: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the socket is ready for I/O.</p>
<p>Note: this function is here for WASI Preview2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a name="method_tcp_socket.subscribe.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a name="method_tcp_socket.shutdown"><code>[method]tcp-socket.shutdown: func</code></a></h4>
<p>Initiate a graceful shutdown.</p>
<ul>
<li>receive: the socket is not expecting to receive any more data from the peer. All subsequent read
operations on the <a href="#input_stream"><code>input-stream</code></a> associated with this socket will return an End Of Stream indication.
Any data still in the receive queue at time of calling <code>shutdown</code> will be discarded.</li>
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
<li><a name="method_tcp_socket.shutdown.self"><code>self</code></a>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a name="method_tcp_socket.shutdown.shutdown_type"><a href="#shutdown_type"><code>shutdown-type</code></a></a>: <a href="#shutdown_type"><a href="#shutdown_type"><code>shutdown-type</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_tcp_socket.shutdown.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:sockets_tcp_create_socket_0.2.0_rc_2023_11_10">Import interface wasi:sockets/tcp-create-socket@0.2.0-rc-2023-11-10</a></h2>
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
<li><code>address-family-not-supported</code>: The specified <code>address-family</code> is not supported. (EAFNOSUPPORT)</li>
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
<li><a name="create_tcp_socket.address_family"><code>address-family</code></a>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="create_tcp_socket.0"></a> result&lt;own&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:sockets_ip_name_lookup_0.2.0_rc_2023_11_10">Import interface wasi:sockets/ip-name-lookup@0.2.0-rc-2023-11-10</a></h2>
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
#### <a name="resolve_address_stream">`resource resolve-address-stream`</a>
<hr />
<h3>Functions</h3>
<h4><a name="resolve_addresses"><code>resolve-addresses: func</code></a></h4>
<p>Resolve an internet host name to a list of IP addresses.</p>
<p>See the wasi-socket proposal README.md for a comparison with getaddrinfo.</p>
<h1>Parameters</h1>
<ul>
<li><code>name</code>: The name to look up. IP addresses are not allowed. Unicode domain names are automatically converted
to ASCII using IDNA encoding.</li>
<li><code>address-family</code>: If provided, limit the results to addresses of this specific address family.</li>
<li><code>include-unavailable</code>: When set to true, this function will also return addresses of which the runtime
thinks (or knows) can't be connected to at the moment. For example, this will return IPv6 addresses on
systems without an active IPv6 interface. Notes:</li>
<li>Even when no public IPv6 interfaces are present or active, names like &quot;localhost&quot; can still resolve to an IPv6 address.</li>
<li>Whatever is &quot;available&quot; or &quot;unavailable&quot; is volatile and can change everytime a network cable is unplugged.</li>
</ul>
<p>This function never blocks. It either immediately fails or immediately returns successfully with a <a href="#resolve_address_stream"><code>resolve-address-stream</code></a>
that can be used to (asynchronously) fetch the results.</p>
<p>At the moment, the stream never completes successfully with 0 items. Ie. the first call
to <code>resolve-next-address</code> never returns <code>ok(none)</code>. This may change in the future.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-name</code>:                 <code>name</code> is a syntactically invalid domain name.</li>
<li><code>invalid-name</code>:                 <code>name</code> is an IP address.</li>
<li><code>address-family-not-supported</code>: The specified <code>address-family</code> is not supported. (EAI_FAMILY)</li>
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
<li><a name="resolve_addresses.network"><a href="#network"><code>network</code></a></a>: borrow&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
<li><a name="resolve_addresses.name"><code>name</code></a>: <code>string</code></li>
<li><a name="resolve_addresses.address_family"><code>address-family</code></a>: option&lt;<a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a>&gt;</li>
<li><a name="resolve_addresses.include_unavailable"><code>include-unavailable</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="resolve_addresses.0"></a> result&lt;own&lt;<a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_resolve_address_stream.resolve_next_address"><code>[method]resolve-address-stream.resolve-next-address: func</code></a></h4>
<p>Returns the next address from the resolver.</p>
<p>This function should be called multiple times. On each call, it will
return the next address in connection order preference. If all
addresses have been exhausted, this function returns <code>none</code>.</p>
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
<li><a name="method_resolve_address_stream.resolve_next_address.self"><code>self</code></a>: borrow&lt;<a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_resolve_address_stream.resolve_next_address.0"></a> result&lt;option&lt;<a href="#ip_address"><a href="#ip_address"><code>ip-address</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_resolve_address_stream.subscribe"><code>[method]resolve-address-stream.subscribe: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the stream is ready for I/O.</p>
<p>Note: this function is here for WASI Preview2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a name="method_resolve_address_stream.subscribe.self"><code>self</code></a>: borrow&lt;<a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_resolve_address_stream.subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
