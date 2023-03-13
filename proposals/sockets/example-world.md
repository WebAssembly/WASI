<h1><a name="example_world">World example-world</a></h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#network"><code>network</code></a></li>
<li>interface <a href="#instance_network"><code>instance-network</code></a></li>
<li>interface <a href="#poll"><code>poll</code></a></li>
<li>interface <a href="#udp"><code>udp</code></a></li>
<li>interface <a href="#udp_create_socket"><code>udp-create-socket</code></a></li>
<li>interface <a href="#streams"><code>streams</code></a></li>
<li>interface <a href="#tcp"><code>tcp</code></a></li>
<li>interface <a href="#tcp_create_socket"><code>tcp-create-socket</code></a></li>
<li>interface <a href="#ip_name_lookup"><code>ip-name-lookup</code></a></li>
</ul>
</li>
</ul>
<h2><a name="network">Import interface network</a></h2>
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
<h4><a name="error"><code>enum error</code></a></h4>
<h5>Enum Cases</h5>
<ul>
<li><a name="error.unknown"><code>unknown</code></a></li>
<li><a name="error.again"><code>again</code></a></li>
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
<h2><a name="instance_network">Import interface instance-network</a></h2>
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
<h2><a name="poll">Import interface poll</a></h2>
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
<h2><a name="udp">Import interface udp</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>type pollable</code></a></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
#### <a name="network">`type network`</a>
[`network`](#network)
<p>
#### <a name="error">`type error`</a>
[`error`](#error)
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
<h4><a name="bind"><code>bind: func</code></a></h4>
<p>Bind the socket to a specific network on the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is left to the implementation to decide which
network interface(s) to bind to.
If the TCP/UDP port is zero, the socket will be bound to a random free port.</p>
<p>When a socket is not explicitly bound, the first invocation to a connect, send or receive operation will
implicitly bind the socket.</p>
<p>Fails when:</p>
<ul>
<li>the socket is already bound.</li>
</ul>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/bind.2.html">https://man7.org/linux/man-pages/man2/bind.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="bind.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="bind.network"><a href="#network"><code>network</code></a></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
<li><a name="bind.local_address"><a href="#local_address"><code>local-address</code></a></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="bind.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="connect"><code>connect: func</code></a></h4>
<p>Set the destination address.</p>
<p>The local-address is updated based on the best network path to <a href="#remote_address"><code>remote-address</code></a>.</p>
<p>When a destination address is set:</p>
<ul>
<li>all receive operations will only return datagrams sent from the provided <a href="#remote_address"><code>remote-address</code></a>.</li>
<li>the <a href="#send"><code>send</code></a> function can only be used to send to this destination.</li>
</ul>
<p>Note that this function does not generate any network traffic and the peer is not aware of this &quot;connection&quot;.</p>
<p>Fails when:</p>
<ul>
<li>the socket is already bound to a different network.</li>
</ul>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/connect.2.html">https://man7.org/linux/man-pages/man2/connect.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="connect.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="connect.network"><a href="#network"><code>network</code></a></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
<li><a name="connect.remote_address"><a href="#remote_address"><code>remote-address</code></a></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="connect.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="receive"><code>receive: func</code></a></h4>
<p>Receive a message.</p>
<p>Returns:</p>
<ul>
<li>The sender address of the datagram</li>
<li>The number of bytes read.</li>
</ul>
<p>Fails when:</p>
<ul>
<li>the socket is not bound.</li>
</ul>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html</a></li>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/recv.2.html">https://man7.org/linux/man-pages/man2/recv.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="receive.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="receive.0"></a> result&lt;<a href="#datagram"><a href="#datagram"><code>datagram</code></a></a>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="send"><code>send: func</code></a></h4>
<p>Send a message to a specific destination address.</p>
<p>The remote address option is required. To send a message to the &quot;connected&quot; peer,
call <a href="#remote_address"><code>remote-address</code></a> to get their address.</p>
<p>Fails when:</p>
<ul>
<li>the socket is not bound. Unlike POSIX, this function does not perform an implicit bind.</li>
<li>the socket is in &quot;connected&quot; mode and the <code>datagram.remote-address</code> does not match the address passed to <a href="#connect"><code>connect</code></a>.</li>
</ul>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html</a></li>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/send.2.html">https://man7.org/linux/man-pages/man2/send.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="send.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="send.datagram"><a href="#datagram"><code>datagram</code></a></a>: <a href="#datagram"><a href="#datagram"><code>datagram</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="send.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="local_address"><code>local-address: func</code></a></h4>
<p>Get the current bound address.</p>
<p>Returns an error if the socket is not bound.</p>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/getsockname.2.html">https://man7.org/linux/man-pages/man2/getsockname.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="local_address.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="remote_address"><code>remote-address: func</code></a></h4>
<p>Get the address set with <a href="#connect"><code>connect</code></a>.</p>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/getpeername.2.html">https://man7.org/linux/man-pages/man2/getpeername.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="remote_address.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
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
<p>Whether IPv4 compatibility (dual-stack) mode is disabled or not.
Implementations are not required to support dual-stack mode, so calling <code>set-ipv6-only(false)</code> might fail.</p>
<p>Fails when called on an IPv4 socket.</p>
<p>Equivalent to the IPV6_V6ONLY socket option.</p>
<h5>Params</h5>
<ul>
<li><a name="ipv6_only.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="ipv6_only.0"></a> result&lt;<code>bool</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_ipv6_only"><code>set-ipv6-only: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_ipv6_only.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="set_ipv6_only.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_ipv6_only.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="unicast_hop_limit"><code>unicast-hop-limit: func</code></a></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<h5>Params</h5>
<ul>
<li><a name="unicast_hop_limit.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="unicast_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_unicast_hop_limit"><code>set-unicast-hop-limit: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_unicast_hop_limit.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="set_unicast_hop_limit.value"><code>value</code></a>: <code>u8</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_unicast_hop_limit.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="receive_buffer_size"><code>receive-buffer-size: func</code></a></h4>
<p>The kernel buffer space reserved for sends/receives on this socket.</p>
<p>Note #1: an implementation may choose to cap or round the buffer size when setting the value.
In other words, after setting a value, reading the same setting back may return a different value.</p>
<p>Note #2: there is not necessarily a direct relationship between the kernel buffer size and the bytes of
actual data to be sent/received by the application, because the kernel might also use the buffer space
for internal metadata structures.</p>
<p>Fails when this socket is in the Listening state.</p>
<p>Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.</p>
<h5>Params</h5>
<ul>
<li><a name="receive_buffer_size.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_receive_buffer_size"><code>set-receive-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_receive_buffer_size.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="set_receive_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_receive_buffer_size.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="send_buffer_size"><code>send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="send_buffer_size.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_send_buffer_size"><code>set-send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_send_buffer_size.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="set_send_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_send_buffer_size.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="non_blocking"><code>non-blocking: func</code></a></h4>
<p>Get/set the blocking mode of the socket.</p>
<p>By default a socket is in &quot;blocking&quot; mode, meaning that any function blocks and waits for its completion.
When switched to &quot;non-blocking&quot; mode, operations that would block return an <code>again</code> error. After which
the API consumer is expected to call <a href="#subscribe"><code>subscribe</code></a> and wait for completion using the wasi-poll module.</p>
<p>Note: these functions are here for WASI Preview2 only.
They're planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a name="non_blocking.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="non_blocking.0"></a> result&lt;<code>bool</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_non_blocking"><code>set-non-blocking: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_non_blocking.this"><code>this</code></a>: <a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a></li>
<li><a name="set_non_blocking.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_non_blocking.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
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
<h2><a name="udp_create_socket">Import interface udp-create-socket</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="network"><code>type network</code></a></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
#### <a name="error">`type error`</a>
[`error`](#error)
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
at time of creation, the socket is not bound to any <a href="#network"><code>network</code></a> yet. Up to the moment <a href="#bind"><code>bind</code></a>/<a href="#connect"><code>connect</code></a> is called,
the socket is effectively an in-memory configuration object, unable to communicate with the outside world.</p>
<p>References:</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/socket.2.html">https://man7.org/linux/man-pages/man2/socket.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="create_udp_socket.address_family"><a href="#address_family"><code>address-family</code></a></a>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="create_udp_socket.0"></a> result&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h2><a name="streams">Import interface streams</a></h2>
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
<h2><a name="tcp">Import interface tcp</a></h2>
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
#### <a name="error">`type error`</a>
[`error`](#error)
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
<h4><a name="bind"><code>bind: func</code></a></h4>
<p>Bind the socket to a specific network on the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is left to the implementation to decide which
network interface(s) to bind to.
If the TCP/UDP port is zero, the socket will be bound to a random free port.</p>
<p>When a socket is not explicitly bound, the first invocation to a listen or connect operation will
implicitly bind the socket.</p>
<p>Fails when:</p>
<ul>
<li>the socket is already bound.</li>
</ul>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/bind.2.html">https://man7.org/linux/man-pages/man2/bind.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="bind.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="bind.network"><a href="#network"><code>network</code></a></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
<li><a name="bind.local_address"><a href="#local_address"><code>local-address</code></a></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="bind.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="connect"><code>connect: func</code></a></h4>
<p>Connect to a remote endpoint.</p>
<p>On success:</p>
<ul>
<li>the socket is transitioned into the Connection state</li>
<li>a pair of streams is returned that can be used to read &amp; write to the connection</li>
</ul>
<p>Fails when:</p>
<ul>
<li>the socket is already bound to a different network.</li>
<li>the provided network does not allow connections to the specified endpoint.</li>
<li>the socket is already in the Connection or Listener state.</li>
<li>either the remote IP address or port is 0.</li>
</ul>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/connect.2.html">https://man7.org/linux/man-pages/man2/connect.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="connect.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="connect.network"><a href="#network"><code>network</code></a></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
<li><a name="connect.remote_address"><a href="#remote_address"><code>remote-address</code></a></a>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="connect.0"></a> result&lt;(<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>, <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>), <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="listen"><code>listen: func</code></a></h4>
<p>Start listening for new connections.</p>
<p>Transitions the socket into the Listener state.</p>
<p>Fails when:</p>
<ul>
<li>the socket is already bound to a different network.</li>
<li>the provided network does not allow listening on the specified address.</li>
<li>the socket is already in the Connection or Listener state.</li>
</ul>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/listen.2.html">https://man7.org/linux/man-pages/man2/listen.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="listen.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="listen.network"><a href="#network"><code>network</code></a></a>: <a href="#network"><a href="#network"><code>network</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="listen.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="accept"><code>accept: func</code></a></h4>
<p>Accept a new client socket.</p>
<p>The returned socket is bound and in the Connection state.</p>
<p>On success, this function returns the newly accepted client socket along with
a pair of streams that can be used to read &amp; write to the connection.</p>
<p>Fails when this socket is not in the Listening state.</p>
<p>References:</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/accept.2.html">https://man7.org/linux/man-pages/man2/accept.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="accept.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="accept.0"></a> result&lt;(<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>, <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>, <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>), <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="local_address"><code>local-address: func</code></a></h4>
<p>Get the bound local address.</p>
<p>Returns an error if the socket is not bound.</p>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/getsockname.2.html">https://man7.org/linux/man-pages/man2/getsockname.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="local_address.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="remote_address"><code>remote-address: func</code></a></h4>
<p>Get the bound remote address.</p>
<p>Fails when the socket is not in the Connection state.</p>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/getpeername.2.html">https://man7.org/linux/man-pages/man2/getpeername.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="remote_address.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
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
<p>Whether IPv4 compatibility (dual-stack) mode is disabled or not.
Implementations are not required to support dual-stack mode. Calling <code>set-ipv6-only(false)</code> might fail.</p>
<p>Fails when called on an IPv4 socket.</p>
<p>Equivalent to the IPV6_V6ONLY socket option.</p>
<h5>Params</h5>
<ul>
<li><a name="ipv6_only.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="ipv6_only.0"></a> result&lt;<code>bool</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_ipv6_only"><code>set-ipv6-only: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_ipv6_only.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_ipv6_only.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_ipv6_only.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_listen_backlog_size"><code>set-listen-backlog-size: func</code></a></h4>
<p>Hints the desired listen queue size. Implementations are free to ignore this.</p>
<h5>Params</h5>
<ul>
<li><a name="set_listen_backlog_size.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_listen_backlog_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_listen_backlog_size.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="keep_alive"><code>keep-alive: func</code></a></h4>
<p>Equivalent to the SO_KEEPALIVE socket option.</p>
<h5>Params</h5>
<ul>
<li><a name="keep_alive.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="keep_alive.0"></a> result&lt;<code>bool</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_keep_alive"><code>set-keep-alive: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_keep_alive.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_keep_alive.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_keep_alive.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="no_delay"><code>no-delay: func</code></a></h4>
<p>Equivalent to the TCP_NODELAY socket option.</p>
<h5>Params</h5>
<ul>
<li><a name="no_delay.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="no_delay.0"></a> result&lt;<code>bool</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_no_delay"><code>set-no-delay: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_no_delay.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_no_delay.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_no_delay.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="unicast_hop_limit"><code>unicast-hop-limit: func</code></a></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<h5>Params</h5>
<ul>
<li><a name="unicast_hop_limit.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="unicast_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_unicast_hop_limit"><code>set-unicast-hop-limit: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_unicast_hop_limit.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_unicast_hop_limit.value"><code>value</code></a>: <code>u8</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_unicast_hop_limit.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="receive_buffer_size"><code>receive-buffer-size: func</code></a></h4>
<p>The kernel buffer space reserved for sends/receives on this socket.</p>
<p>Note #1: an implementation may choose to cap or round the buffer size when setting the value.
In other words, after setting a value, reading the same setting back may return a different value.</p>
<p>Note #2: there is not necessarily a direct relationship between the kernel buffer size and the bytes of
actual data to be sent/received by the application, because the kernel might also use the buffer space
for internal metadata structures.</p>
<p>Fails when this socket is in the Listening state.</p>
<p>Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.</p>
<h5>Params</h5>
<ul>
<li><a name="receive_buffer_size.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_receive_buffer_size"><code>set-receive-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_receive_buffer_size.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_receive_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_receive_buffer_size.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="send_buffer_size"><code>send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="send_buffer_size.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_send_buffer_size"><code>set-send-buffer-size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_send_buffer_size.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_send_buffer_size.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_send_buffer_size.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="non_blocking"><code>non-blocking: func</code></a></h4>
<p>Get/set the blocking mode of the socket.</p>
<p>By default a socket is in &quot;blocking&quot; mode, meaning that any function blocks and waits for its completion.
When switched to &quot;non-blocking&quot; mode, operations that would block return an <code>again</code> error. After which
the API consumer is expected to call <a href="#subscribe"><code>subscribe</code></a> and wait for completion using the wasi-poll module.</p>
<p>Note: these functions are here for WASI Preview2 only.
They're planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a name="non_blocking.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="non_blocking.0"></a> result&lt;<code>bool</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_non_blocking"><code>set-non-blocking: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_non_blocking.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="set_non_blocking.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_non_blocking.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
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
<p>Gracefully shut down the connection.</p>
<ul>
<li>receive: the socket is not expecting to receive any more data from the peer. All subsequent read
operations on the <a href="#input_stream"><code>input-stream</code></a> associated with this socket will return an End Of Stream indication.
Any data still in the receive queue at time of calling <a href="#shutdown"><code>shutdown</code></a> will be discarded.</li>
<li>send: the socket is not expecting to send any more data to the peer. All subsequent write
operations on the <a href="#output_stream"><code>output-stream</code></a> associated with this socket will return an error.</li>
<li>both: same effect as receive &amp; send combined.</li>
</ul>
<p>The shutdown function does not close the socket.</p>
<p>Fails when the socket is not in the Connection state.</p>
<p>References</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/shutdown.2.html">https://man7.org/linux/man-pages/man2/shutdown.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="shutdown.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
<li><a name="shutdown.shutdown_type"><a href="#shutdown_type"><code>shutdown-type</code></a></a>: <a href="#shutdown_type"><a href="#shutdown_type"><code>shutdown-type</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="shutdown.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="drop_tcp_socket"><code>drop-tcp-socket: func</code></a></h4>
<p>Dispose of the specified <a href="#tcp_socket"><code>tcp-socket</code></a>, after which it may no longer be used.</p>
<p>Note: this function is scheduled to be removed when Resources are natively supported in Wit.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_tcp_socket.this"><code>this</code></a>: <a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a></li>
</ul>
<h2><a name="tcp_create_socket">Import interface tcp-create-socket</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="network"><code>type network</code></a></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
#### <a name="error">`type error`</a>
[`error`](#error)
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
at time of creation, the socket is not bound to any <a href="#network"><code>network</code></a> yet. Up to the moment <a href="#bind"><code>bind</code></a>/<a href="#listen"><code>listen</code></a>/<a href="#connect"><code>connect</code></a>
is called, the socket is effectively an in-memory configuration object, unable to communicate with the outside world.</p>
<p>References:</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/socket.2.html">https://man7.org/linux/man-pages/man2/socket.2.html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="create_tcp_socket.address_family"><a href="#address_family"><code>address-family</code></a></a>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="create_tcp_socket.0"></a> result&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h2><a name="ip_name_lookup">Import interface ip-name-lookup</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>type pollable</code></a></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
#### <a name="network">`type network`</a>
[`network`](#network)
<p>
#### <a name="error">`type error`</a>
[`error`](#error)
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
<p>Parameters:</p>
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
<p>This function never blocks. It either immediately returns successfully with a <a href="#resolve_address_stream"><code>resolve-address-stream</code></a>
that can be used to (asynchronously) fetch the results.
Or it immediately fails whenever <code>name</code> is:</p>
<ul>
<li>empty</li>
<li>an IP address</li>
<li>a syntactically invalid domain name in another way</li>
</ul>
<p>References:</p>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man3/getaddrinfo.3.html">https://man7.org/linux/man-pages/man3/getaddrinfo.3.html</a></li>
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
<li><a name="resolve_addresses.0"></a> result&lt;<a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="resolve_next_address"><code>resolve-next-address: func</code></a></h4>
<p>Returns the next address from the resolver.</p>
<p>This function should be called multiple times. On each call, it will
return the next address in connection order preference. If all
addresses have been exhausted, this function returns <code>none</code>.
After which, you should release the stream with <a href="#drop_resolve_address_stream"><code>drop-resolve-address-stream</code></a>.</p>
<p>This function never returns IPv4-mapped IPv6 addresses.</p>
<h5>Params</h5>
<ul>
<li><a name="resolve_next_address.this"><code>this</code></a>: <a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="resolve_next_address.0"></a> result&lt;option&lt;<a href="#ip_address"><a href="#ip_address"><code>ip-address</code></a></a>&gt;, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="drop_resolve_address_stream"><code>drop-resolve-address-stream: func</code></a></h4>
<p>Dispose of the specified <a href="#resolve_address_stream"><code>resolve-address-stream</code></a>, after which it may no longer be used.</p>
<p>Note: this function is scheduled to be removed when Resources are natively supported in Wit.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_resolve_address_stream.this"><code>this</code></a>: <a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a></li>
</ul>
<h4><a name="non_blocking"><code>non-blocking: func</code></a></h4>
<p>Get/set the blocking mode of the stream.</p>
<p>By default a stream is in &quot;blocking&quot; mode, meaning that any function blocks and waits for its completion.
When switched to &quot;non-blocking&quot; mode, operations that would block return an <code>again</code> error. After which
the API consumer is expected to call <a href="#subscribe"><code>subscribe</code></a> and wait for completion using the wasi-poll module.</p>
<p>Note: these functions are here for WASI Preview2 only.
They're planned to be removed when <code>future</code> is natively supported in Preview3.</p>
<h5>Params</h5>
<ul>
<li><a name="non_blocking.this"><code>this</code></a>: <a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="non_blocking.0"></a> result&lt;<code>bool</code>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h4><a name="set_non_blocking"><code>set-non-blocking: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="set_non_blocking.this"><code>this</code></a>: <a href="#resolve_address_stream"><a href="#resolve_address_stream"><code>resolve-address-stream</code></a></a></li>
<li><a name="set_non_blocking.value"><code>value</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_non_blocking.0"></a> result&lt;_, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
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
