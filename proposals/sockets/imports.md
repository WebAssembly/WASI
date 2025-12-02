<h1><a id="imports"></a>World imports</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_io_error_0_2_9"><code>wasi:io/error@0.2.9</code></a></li>
<li>interface <a href="#wasi_sockets_network_0_2_9"><code>wasi:sockets/network@0.2.9</code></a></li>
<li>interface <a href="#wasi_sockets_instance_network_0_2_9"><code>wasi:sockets/instance-network@0.2.9</code></a></li>
<li>interface <a href="#wasi_io_poll_0_2_9"><code>wasi:io/poll@0.2.9</code></a></li>
<li>interface <a href="#wasi_sockets_udp_0_2_9"><code>wasi:sockets/udp@0.2.9</code></a></li>
<li>interface <a href="#wasi_sockets_udp_create_socket_0_2_9"><code>wasi:sockets/udp-create-socket@0.2.9</code></a></li>
<li>interface <a href="#wasi_io_streams_0_2_9"><code>wasi:io/streams@0.2.9</code></a></li>
<li>interface <a href="#wasi_clocks_monotonic_clock_0_2_9"><code>wasi:clocks/monotonic-clock@0.2.9</code></a></li>
<li>interface <a href="#wasi_sockets_tcp_0_2_9"><code>wasi:sockets/tcp@0.2.9</code></a></li>
<li>interface <a href="#wasi_sockets_tcp_create_socket_0_2_9"><code>wasi:sockets/tcp-create-socket@0.2.9</code></a></li>
<li>interface <a href="#wasi_sockets_ip_name_lookup_0_2_9"><code>wasi:sockets/ip-name-lookup@0.2.9</code></a></li>
</ul>
</li>
</ul>
<h2><a id="wasi_io_error_0_2_9"></a>Import interface wasi:io/error@0.2.9</h2>
<hr />
<h3>Types</h3>
<h4><a id="error"></a><code>resource error</code></h4>
<p>A resource which represents some error information.</p>
<p>The only method provided by this resource is <code>to-debug-string</code>,
which provides some human-readable information about the error.</p>
<p>In the <code>wasi:io</code> package, this resource is returned through the
<code>wasi:io/streams.stream-error</code> type.</p>
<p>To provide more specific error information, other interfaces may
offer functions to &quot;downcast&quot; this error into more specific types. For example,
errors returned from streams derived from filesystem types can be described using
the filesystem's own error-code type. This is done using the function
<code>wasi:filesystem/types.filesystem-error-code</code>, which takes a <code>borrow&lt;error&gt;</code>
parameter and returns an <code>option&lt;wasi:filesystem/types.error-code&gt;</code>.</p>
<h2>The set of functions which can &quot;downcast&quot; an <a href="#error"><code>error</code></a> into a more
concrete type is open.</h2>
<h3>Functions</h3>
<h4><a id="method_error_to_debug_string"></a><code>[method]error.to-debug-string: func</code></h4>
<p>Returns a string that is suitable to assist humans in debugging
this error.</p>
<p>WARNING: The returned string should not be consumed mechanically!
It may change across platforms, hosts, or other implementation
details. Parsing this string is a major platform-compatibility
hazard.</p>
<h5>Params</h5>
<ul>
<li><a id="method_error_to_debug_string.self"></a><code>self</code>: borrow&lt;<a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_error_to_debug_string.0"></a> <code>string</code></li>
</ul>
<h2><a id="wasi_sockets_network_0_2_9"></a>Import interface wasi:sockets/network@0.2.9</h2>
<hr />
<h3>Types</h3>
<h4><a id="error"></a><code>type error</code></h4>
<p><a href="#error"><a href="#error"><code>error</code></a></a></p>
<p>
<h4><a id="network"></a><code>resource network</code></h4>
<p>An opaque resource that represents access to (a subset of) the network.
This enables context-based security for networking.
There is no need for this to map 1:1 to a physical network interface.</p>
<h4><a id="error_code"></a><code>enum error-code</code></h4>
<p>Error codes.</p>
<p>In theory, every API can return any error code.
In practice, API's typically only return the errors documented per API
combined with a couple of errors that are always possible:</p>
<ul>
<li><code>unknown</code></li>
<li><code>access-denied</code></li>
<li><code>not-supported</code></li>
<li><code>out-of-memory</code></li>
<li><code>concurrency-conflict</code></li>
</ul>
<p>See each individual API for what the POSIX equivalents are. They sometimes differ per API.</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="error_code.unknown"></a><code>unknown</code></p>
<p>Unknown error
</li>
<li>
<p><a id="error_code.access_denied"></a><code>access-denied</code></p>
<p>Access denied.
<p>POSIX equivalent: EACCES, EPERM</p>
</li>
<li>
<p><a id="error_code.not_supported"></a><code>not-supported</code></p>
<p>The operation is not supported.
<p>POSIX equivalent: EOPNOTSUPP</p>
</li>
<li>
<p><a id="error_code.invalid_argument"></a><code>invalid-argument</code></p>
<p>One of the arguments is invalid.
<p>POSIX equivalent: EINVAL</p>
</li>
<li>
<p><a id="error_code.out_of_memory"></a><code>out-of-memory</code></p>
<p>Not enough memory to complete the operation.
<p>POSIX equivalent: ENOMEM, ENOBUFS, EAI_MEMORY</p>
</li>
<li>
<p><a id="error_code.timeout"></a><code>timeout</code></p>
<p>The operation timed out before it could finish completely.
</li>
<li>
<p><a id="error_code.concurrency_conflict"></a><code>concurrency-conflict</code></p>
<p>This operation is incompatible with another asynchronous operation that is already in progress.
<p>POSIX equivalent: EALREADY</p>
</li>
<li>
<p><a id="error_code.not_in_progress"></a><code>not-in-progress</code></p>
<p>Trying to finish an asynchronous operation that:
- has not been started yet, or:
- was already finished by a previous `finish-*` call.
<p>Note: this is scheduled to be removed when <code>future</code>s are natively supported.</p>
</li>
<li>
<p><a id="error_code.would_block"></a><code>would-block</code></p>
<p>The operation has been aborted because it could not be completed immediately.
<p>Note: this is scheduled to be removed when <code>future</code>s are natively supported.</p>
</li>
<li>
<p><a id="error_code.invalid_state"></a><code>invalid-state</code></p>
<p>The operation is not valid in the socket's current state.
</li>
<li>
<p><a id="error_code.new_socket_limit"></a><code>new-socket-limit</code></p>
<p>A new socket resource could not be created because of a system limit.
</li>
<li>
<p><a id="error_code.address_not_bindable"></a><code>address-not-bindable</code></p>
<p>A bind operation failed because the provided address is not an address that the `network` can bind to.
</li>
<li>
<p><a id="error_code.address_in_use"></a><code>address-in-use</code></p>
<p>A bind operation failed because the provided address is already in use or because there are no ephemeral ports available.
</li>
<li>
<p><a id="error_code.remote_unreachable"></a><code>remote-unreachable</code></p>
<p>The remote address is not reachable
</li>
<li>
<p><a id="error_code.connection_refused"></a><code>connection-refused</code></p>
<p>The TCP connection was forcefully rejected
</li>
<li>
<p><a id="error_code.connection_reset"></a><code>connection-reset</code></p>
<p>The TCP connection was reset.
</li>
<li>
<p><a id="error_code.connection_aborted"></a><code>connection-aborted</code></p>
<p>A TCP connection was aborted.
</li>
<li>
<p><a id="error_code.datagram_too_large"></a><code>datagram-too-large</code></p>
<p>The size of a datagram sent to a UDP socket exceeded the maximum
supported size.
</li>
<li>
<p><a id="error_code.name_unresolvable"></a><code>name-unresolvable</code></p>
<p>Name does not exist or has no suitable associated IP addresses.
</li>
<li>
<p><a id="error_code.temporary_resolver_failure"></a><code>temporary-resolver-failure</code></p>
<p>A temporary failure in name resolution occurred.
</li>
<li>
<p><a id="error_code.permanent_resolver_failure"></a><code>permanent-resolver-failure</code></p>
<p>A permanent failure in name resolution occurred.
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
<hr />
<h3>Functions</h3>
<h4><a id="network_error_code"></a><code>network-error-code: func</code></h4>
<p>Attempts to extract a network-related <a href="#error_code"><code>error-code</code></a> from the stream
<a href="#error"><code>error</code></a> provided.</p>
<p>Stream operations which return <a href="#stream_error.last_operation_failed"><code>stream-error::last-operation-failed</code></a>
have a payload with more information about the operation that failed.
This payload can be passed through to this function to see if there's
network-related information about the error to return.</p>
<p>Note that this function is fallible because not all stream-related
errors are network-related errors.</p>
<h5>Params</h5>
<ul>
<li><a id="network_error_code.err"></a><code>err</code>: borrow&lt;<a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="network_error_code.0"></a> option&lt;<a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_sockets_instance_network_0_2_9"></a>Import interface wasi:sockets/instance-network@0.2.9</h2>
<p>This interface provides a value-export of the default network handle..</p>
<hr />
<h3>Types</h3>
<h4><a id="network"></a><code>type network</code></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="instance_network"></a><code>instance-network: func</code></h4>
<p>Get a handle to the default network.</p>
<h5>Return values</h5>
<ul>
<li><a id="instance_network.0"></a> own&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_io_poll_0_2_9"></a>Import interface wasi:io/poll@0.2.9</h2>
<p>A poll API intended to let users wait for I/O events on multiple handles
at once.</p>
<hr />
<h3>Types</h3>
<h4><a id="pollable"></a><code>resource pollable</code></h4>
<h2><a href="#pollable"><code>pollable</code></a> represents a single I/O event which may be ready, or not.</h2>
<h3>Functions</h3>
<h4><a id="method_pollable_ready"></a><code>[method]pollable.ready: func</code></h4>
<p>Return the readiness of a pollable. This function never blocks.</p>
<p>Returns <code>true</code> when the pollable is ready, and <code>false</code> otherwise.</p>
<h5>Params</h5>
<ul>
<li><a id="method_pollable_ready.self"></a><code>self</code>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_pollable_ready.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_pollable_block"></a><code>[method]pollable.block: func</code></h4>
<p><code>block</code> returns immediately if the pollable is ready, and otherwise
blocks until ready.</p>
<p>This function is equivalent to calling <code>poll.poll</code> on a list
containing only this pollable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_pollable_block.self"></a><code>self</code>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="poll"></a><code>poll: func</code></h4>
<p>Poll for completion on a set of pollables.</p>
<p>This function takes a list of pollables, which identify I/O sources of
interest, and waits until one or more of the events is ready for I/O.</p>
<p>The result <code>list&lt;u32&gt;</code> contains one or more indices of handles in the
argument list that is ready for I/O.</p>
<p>This function traps if either:</p>
<ul>
<li>the list is empty, or:</li>
<li>the list contains more elements than can be indexed with a <code>u32</code> value.</li>
</ul>
<p>A timeout can be implemented by adding a pollable from the
wasi-clocks API to the list.</p>
<p>This function does not return a <code>result</code>; polling in itself does not
do any I/O so it doesn't fail. If any of the I/O sources identified by
the pollables has an error, it is indicated by marking the source as
being ready for I/O.</p>
<h5>Params</h5>
<ul>
<li><a id="poll.in"></a><code>in</code>: list&lt;borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="poll.0"></a> list&lt;<code>u32</code>&gt;</li>
</ul>
<h2><a id="wasi_sockets_udp_0_2_9"></a>Import interface wasi:sockets/udp@0.2.9</h2>
<hr />
<h3>Types</h3>
<h4><a id="pollable"></a><code>type pollable</code></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
<h4><a id="network"></a><code>type network</code></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<h4><a id="ip_socket_address"></a><code>type ip-socket-address</code></h4>
<p><a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></p>
<p>
<h4><a id="ip_address_family"></a><code>type ip-address-family</code></h4>
<p><a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></p>
<p>
<h4><a id="incoming_datagram"></a><code>record incoming-datagram</code></h4>
<p>A received datagram.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a id="incoming_datagram.data"></a><code>data</code>: list&lt;<code>u8</code>&gt;</p>
<p>The payload.
<p>Theoretical max size: ~64 KiB. In practice, typically less than 1500 bytes.</p>
</li>
<li>
<p><a id="incoming_datagram.remote_address"></a><code>remote-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></p>
<p>The source address.
<p>This field is guaranteed to match the remote address the stream was initialized with, if any.</p>
<p>Equivalent to the <code>src_addr</code> out parameter of <code>recvfrom</code>.</p>
</li>
</ul>
<h4><a id="outgoing_datagram"></a><code>record outgoing-datagram</code></h4>
<p>A datagram to be sent out.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a id="outgoing_datagram.data"></a><code>data</code>: list&lt;<code>u8</code>&gt;</p>
<p>The payload.
</li>
<li>
<p><a id="outgoing_datagram.remote_address"></a><code>remote-address</code>: option&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>&gt;</p>
<p>The destination address.
<p>The requirements on this field depend on how the stream was initialized:</p>
<ul>
<li>with a remote address: this field must be None or match the stream's remote address exactly.</li>
<li>without a remote address: this field is required.</li>
</ul>
<p>If this value is None, the send operation is equivalent to <code>send</code> in POSIX. Otherwise it is equivalent to <code>sendto</code>.</p>
</li>
</ul>
<h4><a id="udp_socket"></a><code>resource udp-socket</code></h4>
<p>A UDP socket handle.</p>
<h4><a id="incoming_datagram_stream"></a><code>resource incoming-datagram-stream</code></h4>
<h4><a id="outgoing_datagram_stream"></a><code>resource outgoing-datagram-stream</code></h4>
<hr />
<h3>Functions</h3>
<h4><a id="method_udp_socket_start_bind"></a><code>[method]udp-socket.start-bind: func</code></h4>
<p>Bind the socket to a specific network on the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is left to the implementation to decide which
network interface(s) to bind to.
If the port is zero, the socket will be bound to a random free port.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:          The <code>local-address</code> has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)</li>
<li><code>invalid-state</code>:             The socket is already bound. (EINVAL)</li>
<li><code>address-in-use</code>:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)</li>
<li><code>address-in-use</code>:            Address is already in use. (EADDRINUSE)</li>
<li><code>address-not-bindable</code>:      <code>local-address</code> is not an address that the <a href="#network"><code>network</code></a> can bind to. (EADDRNOTAVAIL)</li>
<li><code>not-in-progress</code>:           A <code>bind</code> operation is not in progress.</li>
<li><code>would-block</code>:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)</li>
</ul>
<h1>Implementors note</h1>
<p>Unlike in POSIX, in WASI the bind operation is async. This enables
interactive WASI hosts to inject permission prompts. Runtimes that
don't want to make use of this ability can simply call the native
<code>bind</code> as part of either <code>start-bind</code> or <code>finish-bind</code>.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/bind.2.html">https://man7.org/linux/man-pages/man2/bind.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=bind&amp;sektion=2&amp;format=html">https://man.freebsd.org/cgi/man.cgi?query=bind&amp;sektion=2&amp;format=html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_start_bind.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_start_bind.network"></a><a href="#network"><code>network</code></a>: borrow&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
<li><a id="method_udp_socket_start_bind.local_address"></a><code>local-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_start_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_finish_bind"></a><code>[method]udp-socket.finish-bind: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_finish_bind.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_finish_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_stream"></a><code>[method]udp-socket.stream: func</code></h4>
<p>Set up inbound &amp; outbound communication channels, optionally to a specific peer.</p>
<p>This function only changes the local socket configuration and does not generate any network traffic.
On success, the <code>remote-address</code> of the socket is updated. The <code>local-address</code> may be updated as well,
based on the best network path to <code>remote-address</code>.</p>
<p>When a <code>remote-address</code> is provided, the returned streams are limited to communicating with that specific peer:</p>
<ul>
<li><code>send</code> can only be used to send to this destination.</li>
<li><code>receive</code> will only return datagrams sent from the provided <code>remote-address</code>.</li>
</ul>
<p>This method may be called multiple times on the same socket to change its association, but
only the most recently returned pair of streams will be operational. Implementations may trap if
the streams returned by a previous invocation haven't been dropped yet before calling <code>stream</code> again.</p>
<p>The POSIX equivalent in pseudo-code is:</p>
<pre><code class="language-text">if (was previously connected) {
  connect(s, AF_UNSPEC)
}
if (remote_address is Some) {
  connect(s, remote_address)
}
</code></pre>
<p>Unlike in POSIX, the socket must already be explicitly bound.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:          The <code>remote-address</code> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-argument</code>:          The IP address in <code>remote-address</code> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-argument</code>:          The port in <code>remote-address</code> is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-state</code>:             The socket is not bound.</li>
<li><code>address-in-use</code>:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)</li>
<li><code>remote-unreachable</code>:        The remote address is not reachable. (ECONNRESET, ENETRESET, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
<li><code>connection-refused</code>:        The connection was refused. (ECONNREFUSED)</li>
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
<li><a id="method_udp_socket_stream.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_stream.remote_address"></a><code>remote-address</code>: option&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_stream.0"></a> result&lt;(own&lt;<a href="#incoming_datagram_stream"><a href="#incoming_datagram_stream"><code>incoming-datagram-stream</code></a></a>&gt;, own&lt;<a href="#outgoing_datagram_stream"><a href="#outgoing_datagram_stream"><code>outgoing-datagram-stream</code></a></a>&gt;), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_local_address"></a><code>[method]udp-socket.local-address: func</code></h4>
<p>Get the current bound address.</p>
<p>POSIX mentions:</p>
<blockquote>
<p>If the socket has not been bound to a local name, the value
stored in the object pointed to by <code>address</code> is unspecified.</p>
</blockquote>
<p>WASI is stricter and requires <code>local-address</code> to return <code>invalid-state</code> when the socket hasn't been bound yet.</p>
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
<li><a id="method_udp_socket_local_address.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_remote_address"></a><code>[method]udp-socket.remote-address: func</code></h4>
<p>Get the address the socket is currently streaming to.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>: The socket is not streaming to a specific remote address. (ENOTCONN)</li>
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
<li><a id="method_udp_socket_remote_address.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_address_family"></a><code>[method]udp-socket.address-family: func</code></h4>
<p>Whether this is a IPv4 or IPv6 socket.</p>
<p>Equivalent to the SO_DOMAIN socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_address_family.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_address_family.0"></a> <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h4><a id="method_udp_socket_unicast_hop_limit"></a><code>[method]udp-socket.unicast-hop-limit: func</code></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The TTL value must be 1 or higher.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_unicast_hop_limit.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_unicast_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_udp_socket_receive_buffer_size"></a><code>[method]udp-socket.receive-buffer-size: func</code></h4>
<p>The kernel buffer space reserved for sends/receives on this socket.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
Any other value will never cause an error, but it might be silently clamped and/or rounded.
I.e. after setting a value, reading the same setting back may return a different value.</p>
<p>Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_receive_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_udp_socket_send_buffer_size"></a><code>[method]udp-socket.send-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_send_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_udp_socket_subscribe"></a><code>[method]udp-socket.subscribe: func</code></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the socket is ready for I/O.</p>
<p>Note: this function is here for WASI 0.2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_subscribe.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_incoming_datagram_stream_receive"></a><code>[method]incoming-datagram-stream.receive: func</code></h4>
<p>Receive messages on the socket.</p>
<p>This function attempts to receive up to <code>max-results</code> datagrams on the socket without blocking.
The returned list may contain fewer elements than requested, but never more.</p>
<p>This function returns successfully with an empty list when either:</p>
<ul>
<li><code>max-results</code> is 0, or:</li>
<li><code>max-results</code> is greater than 0, but no results are immediately available.
This function never returns <code>error(would-block)</code>.</li>
</ul>
<h1>Typical errors</h1>
<ul>
<li><code>remote-unreachable</code>: The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
<li><code>connection-refused</code>: The connection was refused. (ECONNREFUSED)</li>
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
<li><a id="method_incoming_datagram_stream_receive.self"></a><code>self</code>: borrow&lt;<a href="#incoming_datagram_stream"><a href="#incoming_datagram_stream"><code>incoming-datagram-stream</code></a></a>&gt;</li>
<li><a id="method_incoming_datagram_stream_receive.max_results"></a><code>max-results</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_datagram_stream_receive.0"></a> result&lt;list&lt;<a href="#incoming_datagram"><a href="#incoming_datagram"><code>incoming-datagram</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_incoming_datagram_stream_subscribe"></a><code>[method]incoming-datagram-stream.subscribe: func</code></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the stream is ready to receive again.</p>
<p>Note: this function is here for WASI 0.2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_datagram_stream_subscribe.self"></a><code>self</code>: borrow&lt;<a href="#incoming_datagram_stream"><a href="#incoming_datagram_stream"><code>incoming-datagram-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_datagram_stream_subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_outgoing_datagram_stream_check_send"></a><code>[method]outgoing-datagram-stream.check-send: func</code></h4>
<p>Check readiness for sending. This function never blocks.</p>
<p>Returns the number of datagrams permitted for the next call to <code>send</code>,
or an error. Calling <code>send</code> with more datagrams than this function has
permitted will trap.</p>
<p>When this function returns ok(0), the <code>subscribe</code> pollable will
become ready when this function will report at least ok(1), or an
error.</p>
<p>Never returns <code>would-block</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_datagram_stream_check_send.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_datagram_stream"><a href="#outgoing_datagram_stream"><code>outgoing-datagram-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_datagram_stream_check_send.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_outgoing_datagram_stream_send"></a><code>[method]outgoing-datagram-stream.send: func</code></h4>
<p>Send messages on the socket.</p>
<p>This function attempts to send all provided <code>datagrams</code> on the socket without blocking and
returns how many messages were actually sent (or queued for sending). This function never
returns <code>error(would-block)</code>. If none of the datagrams were able to be sent, <code>ok(0)</code> is returned.</p>
<p>This function semantically behaves the same as iterating the <code>datagrams</code> list and sequentially
sending each individual datagram until either the end of the list has been reached or the first error occurred.
If at least one datagram has been sent successfully, this function never returns an error.</p>
<p>If the input list is empty, the function returns <code>ok(0)</code>.</p>
<p>Each call to <code>send</code> must be permitted by a preceding <code>check-send</code>. Implementations must trap if
either <code>check-send</code> was not called or <code>datagrams</code> contains more items than <code>check-send</code> permitted.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:        The <code>remote-address</code> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-argument</code>:        The IP address in <code>remote-address</code> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-argument</code>:        The port in <code>remote-address</code> is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-argument</code>:        The socket is in &quot;connected&quot; mode and <code>remote-address</code> is <code>some</code> value that does not match the address passed to <code>stream</code>. (EISCONN)</li>
<li><code>invalid-argument</code>:        The socket is not &quot;connected&quot; and no value for <code>remote-address</code> was provided. (EDESTADDRREQ)</li>
<li><code>remote-unreachable</code>:      The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
<li><code>connection-refused</code>:      The connection was refused. (ECONNREFUSED)</li>
<li><code>datagram-too-large</code>:      The datagram is too large. (EMSGSIZE)</li>
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
<li><a id="method_outgoing_datagram_stream_send.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_datagram_stream"><a href="#outgoing_datagram_stream"><code>outgoing-datagram-stream</code></a></a>&gt;</li>
<li><a id="method_outgoing_datagram_stream_send.datagrams"></a><code>datagrams</code>: list&lt;<a href="#outgoing_datagram"><a href="#outgoing_datagram"><code>outgoing-datagram</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_datagram_stream_send.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_outgoing_datagram_stream_subscribe"></a><code>[method]outgoing-datagram-stream.subscribe: func</code></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the stream is ready to send again.</p>
<p>Note: this function is here for WASI 0.2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_datagram_stream_subscribe.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_datagram_stream"><a href="#outgoing_datagram_stream"><code>outgoing-datagram-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_datagram_stream_subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_sockets_udp_create_socket_0_2_9"></a>Import interface wasi:sockets/udp-create-socket@0.2.9</h2>
<hr />
<h3>Types</h3>
<h4><a id="network"></a><code>type network</code></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<h4><a id="ip_address_family"></a><code>type ip-address-family</code></h4>
<p><a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></p>
<p>
<h4><a id="udp_socket"></a><code>type udp-socket</code></h4>
<p><a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="create_udp_socket"></a><code>create-udp-socket: func</code></h4>
<p>Create a new UDP socket.</p>
<p>Similar to <code>socket(AF_INET or AF_INET6, SOCK_DGRAM, IPPROTO_UDP)</code> in POSIX.
On IPv6 sockets, IPV6_V6ONLY is enabled by default and can't be configured otherwise.</p>
<p>This function does not require a network capability handle. This is considered to be safe because
at time of creation, the socket is not bound to any <a href="#network"><code>network</code></a> yet. Up to the moment <code>bind</code> is called,
the socket is effectively an in-memory configuration object, unable to communicate with the outside world.</p>
<p>All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-supported</code>:     The specified <code>address-family</code> is not supported. (EAFNOSUPPORT)</li>
<li><code>new-socket-limit</code>:  The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)</li>
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
<li><a id="create_udp_socket.address_family"></a><code>address-family</code>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="create_udp_socket.0"></a> result&lt;own&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_io_streams_0_2_9"></a>Import interface wasi:io/streams@0.2.9</h2>
<p>WASI I/O is an I/O abstraction API which is currently focused on providing
stream types.</p>
<p>In the future, the component model is expected to add built-in stream types;
when it does, they are expected to subsume this API.</p>
<hr />
<h3>Types</h3>
<h4><a id="error"></a><code>type error</code></h4>
<p><a href="#error"><a href="#error"><code>error</code></a></a></p>
<p>
<h4><a id="pollable"></a><code>type pollable</code></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
<h4><a id="stream_error"></a><code>variant stream-error</code></h4>
<p>An error for input-stream and output-stream operations.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="stream_error.last_operation_failed"></a><code>last-operation-failed</code>: own&lt;<a href="#error"><a href="#error"><code>error</code></a></a>&gt;</p>
<p>The last operation (a write or flush) failed before completion.
<p>More information is available in the <a href="#error"><code>error</code></a> payload.</p>
<p>After this, the stream will be closed. All future operations return
<a href="#stream_error.closed"><code>stream-error::closed</code></a>.</p>
</li>
<li>
<p><a id="stream_error.closed"></a><code>closed</code></p>
<p>The stream is closed: no more input will be accepted by the
stream. A closed output-stream will return this error on all
future operations.
</li>
</ul>
<h4><a id="input_stream"></a><code>resource input-stream</code></h4>
<p>An input bytestream.</p>
<p><a href="#input_stream"><code>input-stream</code></a>s are <em>non-blocking</em> to the extent practical on underlying
platforms. I/O operations always return promptly; if fewer bytes are
promptly available than requested, they return the number of bytes promptly
available, which could even be zero. To wait for data to be available,
use the <code>subscribe</code> function to obtain a <a href="#pollable"><code>pollable</code></a> which can be polled
for using <code>wasi:io/poll</code>.</p>
<h4><a id="output_stream"></a><code>resource output-stream</code></h4>
<p>An output bytestream.</p>
<p><a href="#output_stream"><code>output-stream</code></a>s are <em>non-blocking</em> to the extent practical on
underlying platforms. Except where specified otherwise, I/O operations also
always return promptly, after the number of bytes that can be written
promptly, which could even be zero. To wait for the stream to be ready to
accept data, the <code>subscribe</code> function to obtain a <a href="#pollable"><code>pollable</code></a> which can be
polled for using <code>wasi:io/poll</code>.</p>
<h2>Dropping an <a href="#output_stream"><code>output-stream</code></a> while there's still an active write in
progress may result in the data being lost. Before dropping the stream,
be sure to fully flush your writes.</h2>
<h3>Functions</h3>
<h4><a id="method_input_stream_read"></a><code>[method]input-stream.read: func</code></h4>
<p>Perform a non-blocking read from the stream.</p>
<p>When the source of a <code>read</code> is binary data, the bytes from the source
are returned verbatim. When the source of a <code>read</code> is known to the
implementation to be text, bytes containing the UTF-8 encoding of the
text are returned.</p>
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
<li><a id="method_input_stream_read.self"></a><code>self</code>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a id="method_input_stream_read.len"></a><code>len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_input_stream_read.0"></a> result&lt;list&lt;<code>u8</code>&gt;, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_input_stream_blocking_read"></a><code>[method]input-stream.blocking-read: func</code></h4>
<p>Read bytes from a stream, after blocking until at least one byte can
be read. Except for blocking, behavior is identical to <code>read</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_input_stream_blocking_read.self"></a><code>self</code>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a id="method_input_stream_blocking_read.len"></a><code>len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_input_stream_blocking_read.0"></a> result&lt;list&lt;<code>u8</code>&gt;, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_input_stream_skip"></a><code>[method]input-stream.skip: func</code></h4>
<p>Skip bytes from a stream. Returns number of bytes skipped.</p>
<p>Behaves identical to <code>read</code>, except instead of returning a list
of bytes, returns the number of bytes consumed from the stream.</p>
<h5>Params</h5>
<ul>
<li><a id="method_input_stream_skip.self"></a><code>self</code>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a id="method_input_stream_skip.len"></a><code>len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_input_stream_skip.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_input_stream_blocking_skip"></a><code>[method]input-stream.blocking-skip: func</code></h4>
<p>Skip bytes from a stream, after blocking until at least one byte
can be skipped. Except for blocking behavior, identical to <code>skip</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_input_stream_blocking_skip.self"></a><code>self</code>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a id="method_input_stream_blocking_skip.len"></a><code>len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_input_stream_blocking_skip.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_input_stream_subscribe"></a><code>[method]input-stream.subscribe: func</code></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once either the specified stream
has bytes available to read or the other end of the stream has been
closed.
The created <a href="#pollable"><code>pollable</code></a> is a child resource of the <a href="#input_stream"><code>input-stream</code></a>.
Implementations may trap if the <a href="#input_stream"><code>input-stream</code></a> is dropped before
all derived <a href="#pollable"><code>pollable</code></a>s created with this function are dropped.</p>
<h5>Params</h5>
<ul>
<li><a id="method_input_stream_subscribe.self"></a><code>self</code>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_input_stream_subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_output_stream_check_write"></a><code>[method]output-stream.check-write: func</code></h4>
<p>Check readiness for writing. This function never blocks.</p>
<p>Returns the number of bytes permitted for the next call to <code>write</code>,
or an error. Calling <code>write</code> with more bytes than this function has
permitted will trap.</p>
<p>When this function returns 0 bytes, the <code>subscribe</code> pollable will
become ready when this function will report at least 1 byte, or an
error.</p>
<h5>Params</h5>
<ul>
<li><a id="method_output_stream_check_write.self"></a><code>self</code>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_output_stream_check_write.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_output_stream_write"></a><code>[method]output-stream.write: func</code></h4>
<p>Perform a write. This function never blocks.</p>
<p>When the destination of a <code>write</code> is binary data, the bytes from
<code>contents</code> are written verbatim. When the destination of a <code>write</code> is
known to the implementation to be text, the bytes of <code>contents</code> are
transcoded from UTF-8 into the encoding of the destination and then
written.</p>
<p>Precondition: check-write gave permit of Ok(n) and contents has a
length of less than or equal to n. Otherwise, this function will trap.</p>
<p>returns Err(closed) without writing if the stream has closed since
the last call to check-write provided a permit.</p>
<h5>Params</h5>
<ul>
<li><a id="method_output_stream_write.self"></a><code>self</code>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a id="method_output_stream_write.contents"></a><code>contents</code>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_output_stream_write.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_output_stream_blocking_write_and_flush"></a><code>[method]output-stream.blocking-write-and-flush: func</code></h4>
<p>Perform a write of up to 4096 bytes, and then flush the stream. Block
until all of these operations are complete, or an error occurs.</p>
<p>Returns success when all of the contents written are successfully
flushed to output. If an error occurs at any point before all
contents are successfully flushed, that error is returned as soon as
possible. If writing and flushing the complete contents causes the
stream to become closed, this call should return success, and
subsequent calls to check-write or other interfaces should return
stream-error::closed.</p>
<h5>Params</h5>
<ul>
<li><a id="method_output_stream_blocking_write_and_flush.self"></a><code>self</code>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a id="method_output_stream_blocking_write_and_flush.contents"></a><code>contents</code>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_output_stream_blocking_write_and_flush.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_output_stream_flush"></a><code>[method]output-stream.flush: func</code></h4>
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
<li><a id="method_output_stream_flush.self"></a><code>self</code>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_output_stream_flush.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_output_stream_blocking_flush"></a><code>[method]output-stream.blocking-flush: func</code></h4>
<p>Request to flush buffered output, and block until flush completes
and stream is ready for writing again.</p>
<h5>Params</h5>
<ul>
<li><a id="method_output_stream_blocking_flush.self"></a><code>self</code>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_output_stream_blocking_flush.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_output_stream_subscribe"></a><code>[method]output-stream.subscribe: func</code></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the output-stream
is ready for more writing, or an error has occurred. When this
pollable is ready, <code>check-write</code> will return <code>ok(n)</code> with n&gt;0, or an
error.</p>
<p>If the stream is closed, this pollable is always ready immediately.</p>
<p>The created <a href="#pollable"><code>pollable</code></a> is a child resource of the <a href="#output_stream"><code>output-stream</code></a>.
Implementations may trap if the <a href="#output_stream"><code>output-stream</code></a> is dropped before
all derived <a href="#pollable"><code>pollable</code></a>s created with this function are dropped.</p>
<h5>Params</h5>
<ul>
<li><a id="method_output_stream_subscribe.self"></a><code>self</code>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_output_stream_subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_output_stream_write_zeroes"></a><code>[method]output-stream.write-zeroes: func</code></h4>
<p>Write zeroes to a stream.</p>
<p>This should be used precisely like <code>write</code> with the exact same
preconditions (must use check-write first), but instead of
passing a list of bytes, you simply pass the number of zero-bytes
that should be written.</p>
<h5>Params</h5>
<ul>
<li><a id="method_output_stream_write_zeroes.self"></a><code>self</code>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a id="method_output_stream_write_zeroes.len"></a><code>len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_output_stream_write_zeroes.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_output_stream_blocking_write_zeroes_and_flush"></a><code>[method]output-stream.blocking-write-zeroes-and-flush: func</code></h4>
<p>Perform a write of up to 4096 zeroes, and then flush the stream.
Block until all of these operations are complete, or an error
occurs.</p>
<p>Functionality is equivelant to <code>blocking-write-and-flush</code> with
contents given as a list of len containing only zeroes.</p>
<h5>Params</h5>
<ul>
<li><a id="method_output_stream_blocking_write_zeroes_and_flush.self"></a><code>self</code>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a id="method_output_stream_blocking_write_zeroes_and_flush.len"></a><code>len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_output_stream_blocking_write_zeroes_and_flush.0"></a> result&lt;_, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_output_stream_splice"></a><code>[method]output-stream.splice: func</code></h4>
<p>Read from one stream and write to another.</p>
<p>The behavior of splice is equivalent to:</p>
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
<li><a id="method_output_stream_splice.self"></a><code>self</code>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a id="method_output_stream_splice.src"></a><code>src</code>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a id="method_output_stream_splice.len"></a><code>len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_output_stream_splice.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_output_stream_blocking_splice"></a><code>[method]output-stream.blocking-splice: func</code></h4>
<p>Read from one stream and write to another, with blocking.</p>
<p>This is similar to <code>splice</code>, except that it blocks until the
<a href="#output_stream"><code>output-stream</code></a> is ready for writing, and the <a href="#input_stream"><code>input-stream</code></a>
is ready for reading, before performing the <code>splice</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_output_stream_blocking_splice.self"></a><code>self</code>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a id="method_output_stream_blocking_splice.src"></a><code>src</code>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a id="method_output_stream_blocking_splice.len"></a><code>len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_output_stream_blocking_splice.0"></a> result&lt;<code>u64</code>, <a href="#stream_error"><a href="#stream_error"><code>stream-error</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_clocks_monotonic_clock_0_2_9"></a>Import interface wasi:clocks/monotonic-clock@0.2.9</h2>
<p>WASI Monotonic Clock is a clock API intended to let users measure elapsed
time.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<p>A monotonic clock is a clock which has an unspecified initial value, and
successive reads of the clock will produce non-decreasing values.</p>
<hr />
<h3>Types</h3>
<h4><a id="pollable"></a><code>type pollable</code></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
<h4><a id="instant"></a><code>type instant</code></h4>
<p><code>u64</code></p>
<p>An instant in time, in nanoseconds. An instant is relative to an
unspecified initial value, and can only be compared to instances from
the same monotonic-clock.
<h4><a id="duration"></a><code>type duration</code></h4>
<p><code>u64</code></p>
<p>A duration of time, in nanoseconds.
<hr />
<h3>Functions</h3>
<h4><a id="now"></a><code>now: func</code></h4>
<p>Read the current value of the clock.</p>
<p>The clock is monotonic, therefore calling this function repeatedly will
produce a sequence of non-decreasing values.</p>
<p>For completeness, this function traps if it's not possible to represent
the value of the clock in an <a href="#instant"><code>instant</code></a>. Consequently, implementations
should ensure that the starting time is low enough to avoid the
possibility of overflow in practice.</p>
<h5>Return values</h5>
<ul>
<li><a id="now.0"></a> <a href="#instant"><a href="#instant"><code>instant</code></a></a></li>
</ul>
<h4><a id="resolution"></a><code>resolution: func</code></h4>
<p>Query the resolution of the clock. Returns the duration of time
corresponding to a clock tick.</p>
<h5>Return values</h5>
<ul>
<li><a id="resolution.0"></a> <a href="#duration"><a href="#duration"><code>duration</code></a></a></li>
</ul>
<h4><a id="subscribe_instant"></a><code>subscribe-instant: func</code></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the specified instant
has occurred.</p>
<h5>Params</h5>
<ul>
<li><a id="subscribe_instant.when"></a><code>when</code>: <a href="#instant"><a href="#instant"><code>instant</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="subscribe_instant.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="subscribe_duration"></a><code>subscribe-duration: func</code></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> that will resolve after the specified duration has
elapsed from the time this function is invoked.</p>
<h5>Params</h5>
<ul>
<li><a id="subscribe_duration.when"></a><code>when</code>: <a href="#duration"><a href="#duration"><code>duration</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="subscribe_duration.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_sockets_tcp_0_2_9"></a>Import interface wasi:sockets/tcp@0.2.9</h2>
<hr />
<h3>Types</h3>
<h4><a id="input_stream"></a><code>type input-stream</code></h4>
<p><a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></p>
<p>
<h4><a id="output_stream"></a><code>type output-stream</code></h4>
<p><a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></p>
<p>
<h4><a id="pollable"></a><code>type pollable</code></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><a href="#duration"><a href="#duration"><code>duration</code></a></a></p>
<p>
<h4><a id="network"></a><code>type network</code></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<h4><a id="ip_socket_address"></a><code>type ip-socket-address</code></h4>
<p><a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></p>
<p>
<h4><a id="ip_address_family"></a><code>type ip-address-family</code></h4>
<p><a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></p>
<p>
<h4><a id="shutdown_type"></a><code>enum shutdown-type</code></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="shutdown_type.receive"></a><code>receive</code></p>
<p>Similar to `SHUT_RD` in POSIX.
</li>
<li>
<p><a id="shutdown_type.send"></a><code>send</code></p>
<p>Similar to `SHUT_WR` in POSIX.
</li>
<li>
<p><a id="shutdown_type.both"></a><code>both</code></p>
<p>Similar to `SHUT_RDWR` in POSIX.
</li>
</ul>
<h4><a id="tcp_socket"></a><code>resource tcp-socket</code></h4>
<p>A TCP socket resource.</p>
<p>The socket can be in one of the following states:</p>
<ul>
<li><code>unbound</code></li>
<li><code>bind-in-progress</code></li>
<li><code>bound</code> (See note below)</li>
<li><code>listen-in-progress</code></li>
<li><code>listening</code></li>
<li><code>connect-in-progress</code></li>
<li><code>connected</code></li>
<li><code>closed</code>
See <a href="https://github.com/WebAssembly/wasi-sockets/blob/main/TcpSocketOperationalSemantics.md">https://github.com/WebAssembly/wasi-sockets/blob/main/TcpSocketOperationalSemantics.md</a>
for more information.</li>
</ul>
<p>Note: Except where explicitly mentioned, whenever this documentation uses
the term &quot;bound&quot; without backticks it actually means: in the <code>bound</code> state <em>or higher</em>.
(i.e. <code>bound</code>, <code>listen-in-progress</code>, <code>listening</code>, <code>connect-in-progress</code> or <code>connected</code>)</p>
<h2>In addition to the general error codes documented on the
<code>network::error-code</code> type, TCP socket methods may always return
<code>error(invalid-state)</code> when in the <code>closed</code> state.</h2>
<h3>Functions</h3>
<h4><a id="method_tcp_socket_start_bind"></a><code>[method]tcp-socket.start-bind: func</code></h4>
<p>Bind the socket to a specific network on the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is left to the implementation to decide which
network interface(s) to bind to.
If the TCP/UDP port is zero, the socket will be bound to a random free port.</p>
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
<li><code>address-not-bindable</code>:      <code>local-address</code> is not an address that the <a href="#network"><code>network</code></a> can bind to. (EADDRNOTAVAIL)</li>
<li><code>not-in-progress</code>:           A <code>bind</code> operation is not in progress.</li>
<li><code>would-block</code>:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)</li>
</ul>
<h1>Implementors note</h1>
<p>When binding to a non-zero port, this bind operation shouldn't be affected by the TIME_WAIT
state of a recently closed socket on the same local address. In practice this means that the SO_REUSEADDR
socket option should be set implicitly on all platforms, except on Windows where this is the default behavior
and SO_REUSEADDR performs something different entirely.</p>
<p>Unlike in POSIX, in WASI the bind operation is async. This enables
interactive WASI hosts to inject permission prompts. Runtimes that
don't want to make use of this ability can simply call the native
<code>bind</code> as part of either <code>start-bind</code> or <code>finish-bind</code>.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/bind.2.html">https://man7.org/linux/man-pages/man2/bind.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=bind&amp;sektion=2&amp;format=html">https://man.freebsd.org/cgi/man.cgi?query=bind&amp;sektion=2&amp;format=html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_start_bind.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_start_bind.network"></a><a href="#network"><code>network</code></a>: borrow&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_start_bind.local_address"></a><code>local-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_start_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_finish_bind"></a><code>[method]tcp-socket.finish-bind: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_finish_bind.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_finish_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_start_connect"></a><code>[method]tcp-socket.start-connect: func</code></h4>
<p>Connect to a remote endpoint.</p>
<p>On success:</p>
<ul>
<li>the socket is transitioned into the <code>connected</code> state.</li>
<li>a pair of streams is returned that can be used to read &amp; write to the connection</li>
</ul>
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
<li><code>invalid-argument</code>:          The socket is already attached to a different network. The <a href="#network"><code>network</code></a> passed to <code>connect</code> must be identical to the one passed to <code>bind</code>.</li>
<li><code>invalid-state</code>:             The socket is already in the <code>connected</code> state. (EISCONN)</li>
<li><code>invalid-state</code>:             The socket is already in the <code>listening</code> state. (EOPNOTSUPP, EINVAL on Windows)</li>
<li><code>timeout</code>:                   Connection timed out. (ETIMEDOUT)</li>
<li><code>connection-refused</code>:        The connection was forcefully rejected. (ECONNREFUSED)</li>
<li><code>connection-reset</code>:          The connection was reset. (ECONNRESET)</li>
<li><code>connection-aborted</code>:        The connection was aborted. (ECONNABORTED)</li>
<li><code>remote-unreachable</code>:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
<li><code>address-in-use</code>:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)</li>
<li><code>not-in-progress</code>:           A connect operation is not in progress.</li>
<li><code>would-block</code>:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)</li>
</ul>
<h1>Implementors note</h1>
<p>The POSIX equivalent of <code>start-connect</code> is the regular <code>connect</code> syscall.
Because all WASI sockets are non-blocking this is expected to return
EINPROGRESS, which should be translated to <code>ok()</code> in WASI.</p>
<p>The POSIX equivalent of <code>finish-connect</code> is a <a href="#poll"><code>poll</code></a> for event <code>POLLOUT</code>
with a timeout of 0 on the socket descriptor. Followed by a check for
the <code>SO_ERROR</code> socket option, in case the poll signaled readiness.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/connect.2.html">https://man7.org/linux/man-pages/man2/connect.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?connect">https://man.freebsd.org/cgi/man.cgi?connect</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_start_connect.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_start_connect.network"></a><a href="#network"><code>network</code></a>: borrow&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_start_connect.remote_address"></a><code>remote-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_start_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_finish_connect"></a><code>[method]tcp-socket.finish-connect: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_finish_connect.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_finish_connect.0"></a> result&lt;(own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;, own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_start_listen"></a><code>[method]tcp-socket.start-listen: func</code></h4>
<p>Start listening for new connections.</p>
<p>Transitions the socket into the <code>listening</code> state.</p>
<p>Unlike POSIX, the socket must already be explicitly bound.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:             The socket is not bound to any local address. (EDESTADDRREQ)</li>
<li><code>invalid-state</code>:             The socket is already in the <code>connected</code> state. (EISCONN, EINVAL on BSD)</li>
<li><code>invalid-state</code>:             The socket is already in the <code>listening</code> state.</li>
<li><code>address-in-use</code>:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)</li>
<li><code>not-in-progress</code>:           A listen operation is not in progress.</li>
<li><code>would-block</code>:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)</li>
</ul>
<h1>Implementors note</h1>
<p>Unlike in POSIX, in WASI the listen operation is async. This enables
interactive WASI hosts to inject permission prompts. Runtimes that
don't want to make use of this ability can simply call the native
<code>listen</code> as part of either <code>start-listen</code> or <code>finish-listen</code>.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/listen.2.html">https://man7.org/linux/man-pages/man2/listen.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=listen&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=listen&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_start_listen.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_start_listen.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_finish_listen"></a><code>[method]tcp-socket.finish-listen: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_finish_listen.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_finish_listen.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_accept"></a><code>[method]tcp-socket.accept: func</code></h4>
<p>Accept a new client socket.</p>
<p>The returned socket is bound and in the <code>connected</code> state. The following properties are inherited from the listener socket:</p>
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
<p>On success, this function returns the newly accepted client socket along with
a pair of streams that can be used to read &amp; write to the connection.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:      Socket is not in the <code>listening</code> state. (EINVAL)</li>
<li><code>would-block</code>:        No pending connections at the moment. (EWOULDBLOCK, EAGAIN)</li>
<li><code>connection-aborted</code>: An incoming connection was pending, but was terminated by the client before this listener could accept it. (ECONNABORTED)</li>
<li><code>new-socket-limit</code>:   The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/accept.2.html">https://man7.org/linux/man-pages/man2/accept.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=accept&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=accept&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_accept.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_accept.0"></a> result&lt;(own&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;, own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;, own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_local_address"></a><code>[method]tcp-socket.local-address: func</code></h4>
<p>Get the bound local address.</p>
<p>POSIX mentions:</p>
<blockquote>
<p>If the socket has not been bound to a local name, the value
stored in the object pointed to by <code>address</code> is unspecified.</p>
</blockquote>
<p>WASI is stricter and requires <code>local-address</code> to return <code>invalid-state</code> when the socket hasn't been bound yet.</p>
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
<li><a id="method_tcp_socket_local_address.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_remote_address"></a><code>[method]tcp-socket.remote-address: func</code></h4>
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
<li><a id="method_tcp_socket_remote_address.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_is_listening"></a><code>[method]tcp-socket.is-listening: func</code></h4>
<p>Whether the socket is in the <code>listening</code> state.</p>
<p>Equivalent to the SO_ACCEPTCONN socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_is_listening.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_is_listening.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_tcp_socket_address_family"></a><code>[method]tcp-socket.address-family: func</code></h4>
<p>Whether this is a IPv4 or IPv6 socket.</p>
<p>Equivalent to the SO_DOMAIN socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_address_family.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_address_family.0"></a> <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h4><a id="method_tcp_socket_set_listen_backlog_size"></a><code>[method]tcp-socket.set-listen-backlog-size: func</code></h4>
<p>Hints the desired listen queue size. Implementations are free to ignore this.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
Any other value will never cause an error, but it might be silently clamped and/or rounded.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-supported</code>:        (set) The platform does not support changing the backlog size after the initial listen.</li>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
<li><code>invalid-state</code>:        (set) The socket is in the <code>connect-in-progress</code> or <code>connected</code> state.</li>
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
<h4><a id="method_tcp_socket_keep_alive_enabled"></a><code>[method]tcp-socket.keep-alive-enabled: func</code></h4>
<p>Enables or disables keepalive.</p>
<p>The keepalive behavior can be adjusted using:</p>
<ul>
<li><code>keep-alive-idle-time</code></li>
<li><code>keep-alive-interval</code></li>
<li><code>keep-alive-count</code>
These properties can be configured while <code>keep-alive-enabled</code> is false, but only come into effect when <code>keep-alive-enabled</code> is true.</li>
</ul>
<p>Equivalent to the SO_KEEPALIVE socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_keep_alive_enabled.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_keep_alive_enabled.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_keep_alive_idle_time"></a><code>[method]tcp-socket.keep-alive-idle-time: func</code></h4>
<p>Amount of time the connection has to be idle before TCP starts sending keepalive packets.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
Any other value will never cause an error, but it might be silently clamped and/or rounded.
I.e. after setting a value, reading the same setting back may return a different value.</p>
<p>Equivalent to the TCP_KEEPIDLE socket option. (TCP_KEEPALIVE on MacOS)</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_keep_alive_idle_time.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_keep_alive_idle_time.0"></a> result&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_keep_alive_interval"></a><code>[method]tcp-socket.keep-alive-interval: func</code></h4>
<p>The time between keepalive packets.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
Any other value will never cause an error, but it might be silently clamped and/or rounded.
I.e. after setting a value, reading the same setting back may return a different value.</p>
<p>Equivalent to the TCP_KEEPINTVL socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_keep_alive_interval.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_keep_alive_interval.0"></a> result&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_keep_alive_count"></a><code>[method]tcp-socket.keep-alive-count: func</code></h4>
<p>The maximum amount of keepalive packets TCP should send before aborting the connection.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
Any other value will never cause an error, but it might be silently clamped and/or rounded.
I.e. after setting a value, reading the same setting back may return a different value.</p>
<p>Equivalent to the TCP_KEEPCNT socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_keep_alive_count.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_keep_alive_count.0"></a> result&lt;<code>u32</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_hop_limit"></a><code>[method]tcp-socket.hop-limit: func</code></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The TTL value must be 1 or higher.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_hop_limit.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_receive_buffer_size"></a><code>[method]tcp-socket.receive-buffer-size: func</code></h4>
<p>The kernel buffer space reserved for sends/receives on this socket.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
Any other value will never cause an error, but it might be silently clamped and/or rounded.
I.e. after setting a value, reading the same setting back may return a different value.</p>
<p>Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_receive_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_send_buffer_size"></a><code>[method]tcp-socket.send-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_send_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_subscribe"></a><code>[method]tcp-socket.subscribe: func</code></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which can be used to poll for, or block on,
completion of any of the asynchronous operations of this socket.</p>
<p>When <code>finish-bind</code>, <code>finish-listen</code>, <code>finish-connect</code> or <code>accept</code>
return <code>error(would-block)</code>, this pollable can be used to wait for
their success or failure, after which the method can be retried.</p>
<p>The pollable is not limited to the async operation that happens to be
in progress at the time of calling <code>subscribe</code> (if any). Theoretically,
<code>subscribe</code> only has to be called once per socket and can then be
(re)used for the remainder of the socket's lifetime.</p>
<p>See <a href="https://github.com/WebAssembly/wasi-sockets/blob/main/TcpSocketOperationalSemantics.md#pollable-readiness">https://github.com/WebAssembly/wasi-sockets/blob/main/TcpSocketOperationalSemantics.md#pollable-readiness</a>
for more information.</p>
<p>Note: this function is here for WASI 0.2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_subscribe.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_shutdown"></a><code>[method]tcp-socket.shutdown: func</code></h4>
<p>Initiate a graceful shutdown.</p>
<ul>
<li><code>receive</code>: The socket is not expecting to receive any data from
the peer. The <a href="#input_stream"><code>input-stream</code></a> associated with this socket will be
closed. Any data still in the receive queue at time of calling
this method will be discarded.</li>
<li><code>send</code>: The socket has no more data to send to the peer. The <a href="#output_stream"><code>output-stream</code></a>
associated with this socket will be closed and a FIN packet will be sent.</li>
<li><code>both</code>: Same effect as <code>receive</code> &amp; <code>send</code> combined.</li>
</ul>
<p>This function is idempotent; shutting down a direction more than once
has no effect and returns <code>ok</code>.</p>
<p>The shutdown function does not close (drop) the socket.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>: The socket is not in the <code>connected</code> state. (ENOTCONN)</li>
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
<li><a id="method_tcp_socket_shutdown.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_shutdown.shutdown_type"></a><a href="#shutdown_type"><code>shutdown-type</code></a>: <a href="#shutdown_type"><a href="#shutdown_type"><code>shutdown-type</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_shutdown.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_sockets_tcp_create_socket_0_2_9"></a>Import interface wasi:sockets/tcp-create-socket@0.2.9</h2>
<hr />
<h3>Types</h3>
<h4><a id="network"></a><code>type network</code></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<h4><a id="ip_address_family"></a><code>type ip-address-family</code></h4>
<p><a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></p>
<p>
<h4><a id="tcp_socket"></a><code>type tcp-socket</code></h4>
<p><a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="create_tcp_socket"></a><code>create-tcp-socket: func</code></h4>
<p>Create a new TCP socket.</p>
<p>Similar to <code>socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)</code> in POSIX.
On IPv6 sockets, IPV6_V6ONLY is enabled by default and can't be configured otherwise.</p>
<p>This function does not require a network capability handle. This is considered to be safe because
at time of creation, the socket is not bound to any <a href="#network"><code>network</code></a> yet. Up to the moment <code>bind</code>/<code>connect</code>
is called, the socket is effectively an in-memory configuration object, unable to communicate with the outside world.</p>
<p>All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-supported</code>:     The specified <code>address-family</code> is not supported. (EAFNOSUPPORT)</li>
<li><code>new-socket-limit</code>:  The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)</li>
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
<li><a id="create_tcp_socket.address_family"></a><code>address-family</code>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="create_tcp_socket.0"></a> result&lt;own&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_sockets_ip_name_lookup_0_2_9"></a>Import interface wasi:sockets/ip-name-lookup@0.2.9</h2>
<hr />
<h3>Types</h3>
<h4><a id="pollable"></a><code>type pollable</code></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
<h4><a id="network"></a><code>type network</code></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<h4><a id="ip_address"></a><code>type ip-address</code></h4>
<p><a href="#ip_address"><a href="#ip_address"><code>ip-address</code></a></a></p>
<p>
<h4><a id="resolve_address_stream"></a><code>resource resolve-address-stream</code></h4>
<hr />
<h3>Functions</h3>
<h4><a id="resolve_addresses"></a><code>resolve-addresses: func</code></h4>
<p>Resolve an internet host name to a list of IP addresses.</p>
<p>Unicode domain names are automatically converted to ASCII using IDNA encoding.
If the input is an IP address string, the address is parsed and returned
as-is without making any external requests.</p>
<p>See the wasi-socket proposal README.md for a comparison with getaddrinfo.</p>
<p>This function never blocks. It either immediately fails or immediately
returns successfully with a <a href="#resolve_address_stream"><code>resolve-address-stream</code></a> that can be used
to (asynchronously) fetch the results.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>: <code>name</code> is a syntactically invalid domain name or IP address.</li>
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
<li><a id="resolve_addresses.network"></a><a href="#network"><code>network</code></a>: borrow&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
<li><a id="resolve_addresses.name"></a><code>name</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="resolve_addresses.0"></a> result&lt;own&lt;<a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_resolve_address_stream_resolve_next_address"></a><code>[method]resolve-address-stream.resolve-next-address: func</code></h4>
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
<li><a id="method_resolve_address_stream_resolve_next_address.self"></a><code>self</code>: borrow&lt;<a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_resolve_address_stream_resolve_next_address.0"></a> result&lt;option&lt;<a href="#ip_address"><a href="#ip_address"><code>ip-address</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_resolve_address_stream_subscribe"></a><code>[method]resolve-address-stream.subscribe: func</code></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the stream is ready for I/O.</p>
<p>Note: this function is here for WASI 0.2 only.
It's planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a id="method_resolve_address_stream_subscribe.self"></a><code>self</code>: borrow&lt;<a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_resolve_address_stream_subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
