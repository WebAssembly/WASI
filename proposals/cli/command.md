<h1><a id="command"></a>World command</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_cli_environment_0_2_2"><code>wasi:cli/environment@0.2.2</code></a></li>
<li>interface <a href="#wasi_cli_exit_0_2_2"><code>wasi:cli/exit@0.2.2</code></a></li>
<li>interface <a href="#wasi_io_error_0_2_2"><code>wasi:io/error@0.2.2</code></a></li>
<li>interface <a href="#wasi_io_poll_0_2_2"><code>wasi:io/poll@0.2.2</code></a></li>
<li>interface <a href="#wasi_io_streams_0_2_2"><code>wasi:io/streams@0.2.2</code></a></li>
<li>interface <a href="#wasi_cli_stdin_0_2_2"><code>wasi:cli/stdin@0.2.2</code></a></li>
<li>interface <a href="#wasi_cli_stdout_0_2_2"><code>wasi:cli/stdout@0.2.2</code></a></li>
<li>interface <a href="#wasi_cli_stderr_0_2_2"><code>wasi:cli/stderr@0.2.2</code></a></li>
<li>interface <a href="#wasi_cli_terminal_input_0_2_2"><code>wasi:cli/terminal-input@0.2.2</code></a></li>
<li>interface <a href="#wasi_cli_terminal_output_0_2_2"><code>wasi:cli/terminal-output@0.2.2</code></a></li>
<li>interface <a href="#wasi_cli_terminal_stdin_0_2_2"><code>wasi:cli/terminal-stdin@0.2.2</code></a></li>
<li>interface <a href="#wasi_cli_terminal_stdout_0_2_2"><code>wasi:cli/terminal-stdout@0.2.2</code></a></li>
<li>interface <a href="#wasi_cli_terminal_stderr_0_2_2"><code>wasi:cli/terminal-stderr@0.2.2</code></a></li>
<li>interface <a href="#wasi_clocks_monotonic_clock_0_2_2"><code>wasi:clocks/monotonic-clock@0.2.2</code></a></li>
<li>interface <a href="#wasi_clocks_wall_clock_0_2_2"><code>wasi:clocks/wall-clock@0.2.2</code></a></li>
<li>interface <a href="#wasi_filesystem_types_0_2_2"><code>wasi:filesystem/types@0.2.2</code></a></li>
<li>interface <a href="#wasi_filesystem_preopens_0_2_2"><code>wasi:filesystem/preopens@0.2.2</code></a></li>
<li>interface <a href="#wasi_sockets_network_0_2_2"><code>wasi:sockets/network@0.2.2</code></a></li>
<li>interface <a href="#wasi_sockets_instance_network_0_2_2"><code>wasi:sockets/instance-network@0.2.2</code></a></li>
<li>interface <a href="#wasi_sockets_udp_0_2_2"><code>wasi:sockets/udp@0.2.2</code></a></li>
<li>interface <a href="#wasi_sockets_udp_create_socket_0_2_2"><code>wasi:sockets/udp-create-socket@0.2.2</code></a></li>
<li>interface <a href="#wasi_sockets_tcp_0_2_2"><code>wasi:sockets/tcp@0.2.2</code></a></li>
<li>interface <a href="#wasi_sockets_tcp_create_socket_0_2_2"><code>wasi:sockets/tcp-create-socket@0.2.2</code></a></li>
<li>interface <a href="#wasi_sockets_ip_name_lookup_0_2_2"><code>wasi:sockets/ip-name-lookup@0.2.2</code></a></li>
<li>interface <a href="#wasi_random_random_0_2_2"><code>wasi:random/random@0.2.2</code></a></li>
<li>interface <a href="#wasi_random_insecure_0_2_2"><code>wasi:random/insecure@0.2.2</code></a></li>
<li>interface <a href="#wasi_random_insecure_seed_0_2_2"><code>wasi:random/insecure-seed@0.2.2</code></a></li>
</ul>
</li>
<li>Exports:
<ul>
<li>interface <a href="#wasi_cli_run_0_2_2"><code>wasi:cli/run@0.2.2</code></a></li>
</ul>
</li>
</ul>
<h2><a id="wasi_cli_environment_0_2_2"></a>Import interface wasi:cli/environment@0.2.2</h2>
<hr />
<h3>Functions</h3>
<h4><a id="get_environment"></a><code>get-environment: func</code></h4>
<p>Get the POSIX-style environment variables.</p>
<p>Each environment variable is provided as a pair of string variable names
and string value.</p>
<p>Morally, these are a value import, but until value imports are available
in the component model, this import function should return the same
values each time it is called.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_environment.0"></a> list&lt;(<code>string</code>, <code>string</code>)&gt;</li>
</ul>
<h4><a id="get_arguments"></a><code>get-arguments: func</code></h4>
<p>Get the POSIX-style arguments to the program.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_arguments.0"></a> list&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="initial_cwd"></a><code>initial-cwd: func</code></h4>
<p>Return a path that programs should use as their initial current working
directory, interpreting <code>.</code> as shorthand for this.</p>
<h5>Return values</h5>
<ul>
<li><a id="initial_cwd.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h2><a id="wasi_cli_exit_0_2_2"></a>Import interface wasi:cli/exit@0.2.2</h2>
<hr />
<h3>Functions</h3>
<h4><a id="exit"></a><code>exit: func</code></h4>
<p>Exit the current instance and any linked instances.</p>
<h5>Params</h5>
<ul>
<li><a id="exit.status"></a><code>status</code>: result</li>
</ul>
<h4><a id="exit_with_code"></a><code>exit-with-code: func</code></h4>
<p>Exit the current instance and any linked instances, reporting the
specified status code to the host.</p>
<p>The meaning of the code depends on the context, with 0 usually meaning
&quot;success&quot;, and other values indicating various types of failure.</p>
<p>This function does not return; the effect is analogous to a trap, but
without the connotation that something bad has happened.</p>
<h5>Params</h5>
<ul>
<li><a id="exit_with_code.status_code"></a><code>status-code</code>: <code>u8</code></li>
</ul>
<h2><a id="wasi_io_error_0_2_2"></a>Import interface wasi:io/error@0.2.2</h2>
<hr />
<h3>Types</h3>
<h4><a id="error"></a><code>resource error</code></h4>
<p>A resource which represents some error information.</p>
<p>The only method provided by this resource is <code>to-debug-string</code>,
which provides some human-readable information about the error.</p>
<p>In the <code>wasi:io</code> package, this resource is returned through the
<code>wasi:io/streams/stream-error</code> type.</p>
<p>To provide more specific error information, other interfaces may
offer functions to &quot;downcast&quot; this error into more specific types. For example,
errors returned from streams derived from filesystem types can be described using
the filesystem's own error-code type. This is done using the function
<code>wasi:filesystem/types/filesystem-error-code</code>, which takes a <code>borrow&lt;error&gt;</code>
parameter and returns an <code>option&lt;wasi:filesystem/types/error-code&gt;</code>.</p>
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
<h2><a id="wasi_io_poll_0_2_2"></a>Import interface wasi:io/poll@0.2.2</h2>
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
<h2><a id="wasi_io_streams_0_2_2"></a>Import interface wasi:io/streams@0.2.2</h2>
<p>WASI I/O is an I/O abstraction API which is currently focused on providing
stream types.</p>
<p>In the future, the component model is expected to add built-in stream types;
when it does, they are expected to subsume this API.</p>
<hr />
<h3>Types</h3>
<h4><a id="error"></a><code>type error</code></h4>
<p><a href="#error"><a href="#error"><code>error</code></a></a></p>
<p>
#### <a id="pollable"></a>`type pollable`
[`pollable`](#pollable)
<p>
#### <a id="stream_error"></a>`variant stream-error`
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
<p>This is a convenience wrapper around the use of <code>check-write</code>,
<code>subscribe</code>, <code>write</code>, and <code>flush</code>, and is implemented with the
following pseudo-code:</p>
<pre><code class="language-text">let pollable = this.subscribe();
while !contents.is_empty() {
  // Wait for the stream to become writable
  pollable.block();
  let Ok(n) = this.check-write(); // eliding error handling
  let len = min(n, contents.len());
  let (chunk, rest) = contents.split_at(len);
  this.write(chunk  );            // eliding error handling
  contents = rest;
}
this.flush();
// Wait for completion of `flush`
pollable.block();
// Check for any errors that arose during `flush`
let _ = this.check-write();         // eliding error handling
</code></pre>
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
<p>This is a convenience wrapper around the use of <code>check-write</code>,
<code>subscribe</code>, <code>write-zeroes</code>, and <code>flush</code>, and is implemented with
the following pseudo-code:</p>
<pre><code class="language-text">let pollable = this.subscribe();
while num_zeroes != 0 {
  // Wait for the stream to become writable
  pollable.block();
  let Ok(n) = this.check-write(); // eliding error handling
  let len = min(n, num_zeroes);
  this.write-zeroes(len);         // eliding error handling
  num_zeroes -= len;
}
this.flush();
// Wait for completion of `flush`
pollable.block();
// Check for any errors that arose during `flush`
let _ = this.check-write();         // eliding error handling
</code></pre>
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
<h2><a id="wasi_cli_stdin_0_2_2"></a>Import interface wasi:cli/stdin@0.2.2</h2>
<hr />
<h3>Types</h3>
<h4><a id="input_stream"></a><code>type input-stream</code></h4>
<p><a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></p>
<p>
----
<h3>Functions</h3>
<h4><a id="get_stdin"></a><code>get-stdin: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="get_stdin.0"></a> own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_cli_stdout_0_2_2"></a>Import interface wasi:cli/stdout@0.2.2</h2>
<hr />
<h3>Types</h3>
<h4><a id="output_stream"></a><code>type output-stream</code></h4>
<p><a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></p>
<p>
----
<h3>Functions</h3>
<h4><a id="get_stdout"></a><code>get-stdout: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="get_stdout.0"></a> own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_cli_stderr_0_2_2"></a>Import interface wasi:cli/stderr@0.2.2</h2>
<hr />
<h3>Types</h3>
<h4><a id="output_stream"></a><code>type output-stream</code></h4>
<p><a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></p>
<p>
----
<h3>Functions</h3>
<h4><a id="get_stderr"></a><code>get-stderr: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="get_stderr.0"></a> own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_cli_terminal_input_0_2_2"></a>Import interface wasi:cli/terminal-input@0.2.2</h2>
<p>Terminal input.</p>
<p>In the future, this may include functions for disabling echoing,
disabling input buffering so that keyboard events are sent through
immediately, querying supported features, and so on.</p>
<hr />
<h3>Types</h3>
<h4><a id="terminal_input"></a><code>resource terminal-input</code></h4>
<p>The input side of a terminal.</p>
<h2><a id="wasi_cli_terminal_output_0_2_2"></a>Import interface wasi:cli/terminal-output@0.2.2</h2>
<p>Terminal output.</p>
<p>In the future, this may include functions for querying the terminal
size, being notified of terminal size changes, querying supported
features, and so on.</p>
<hr />
<h3>Types</h3>
<h4><a id="terminal_output"></a><code>resource terminal-output</code></h4>
<p>The output side of a terminal.</p>
<h2><a id="wasi_cli_terminal_stdin_0_2_2"></a>Import interface wasi:cli/terminal-stdin@0.2.2</h2>
<p>An interface providing an optional <a href="#terminal_input"><code>terminal-input</code></a> for stdin as a
link-time authority.</p>
<hr />
<h3>Types</h3>
<h4><a id="terminal_input"></a><code>type terminal-input</code></h4>
<p><a href="#terminal_input"><a href="#terminal_input"><code>terminal-input</code></a></a></p>
<p>
----
<h3>Functions</h3>
<h4><a id="get_terminal_stdin"></a><code>get-terminal-stdin: func</code></h4>
<p>If stdin is connected to a terminal, return a <a href="#terminal_input"><code>terminal-input</code></a> handle
allowing further interaction with it.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_terminal_stdin.0"></a> option&lt;own&lt;<a href="#terminal_input"><a href="#terminal_input"><code>terminal-input</code></a></a>&gt;&gt;</li>
</ul>
<h2><a id="wasi_cli_terminal_stdout_0_2_2"></a>Import interface wasi:cli/terminal-stdout@0.2.2</h2>
<p>An interface providing an optional <a href="#terminal_output"><code>terminal-output</code></a> for stdout as a
link-time authority.</p>
<hr />
<h3>Types</h3>
<h4><a id="terminal_output"></a><code>type terminal-output</code></h4>
<p><a href="#terminal_output"><a href="#terminal_output"><code>terminal-output</code></a></a></p>
<p>
----
<h3>Functions</h3>
<h4><a id="get_terminal_stdout"></a><code>get-terminal-stdout: func</code></h4>
<p>If stdout is connected to a terminal, return a <a href="#terminal_output"><code>terminal-output</code></a> handle
allowing further interaction with it.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_terminal_stdout.0"></a> option&lt;own&lt;<a href="#terminal_output"><a href="#terminal_output"><code>terminal-output</code></a></a>&gt;&gt;</li>
</ul>
<h2><a id="wasi_cli_terminal_stderr_0_2_2"></a>Import interface wasi:cli/terminal-stderr@0.2.2</h2>
<p>An interface providing an optional <a href="#terminal_output"><code>terminal-output</code></a> for stderr as a
link-time authority.</p>
<hr />
<h3>Types</h3>
<h4><a id="terminal_output"></a><code>type terminal-output</code></h4>
<p><a href="#terminal_output"><a href="#terminal_output"><code>terminal-output</code></a></a></p>
<p>
----
<h3>Functions</h3>
<h4><a id="get_terminal_stderr"></a><code>get-terminal-stderr: func</code></h4>
<p>If stderr is connected to a terminal, return a <a href="#terminal_output"><code>terminal-output</code></a> handle
allowing further interaction with it.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_terminal_stderr.0"></a> option&lt;own&lt;<a href="#terminal_output"><a href="#terminal_output"><code>terminal-output</code></a></a>&gt;&gt;</li>
</ul>
<h2><a id="wasi_clocks_monotonic_clock_0_2_2"></a>Import interface wasi:clocks/monotonic-clock@0.2.2</h2>
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
#### <a id="instant"></a>`type instant`
`u64`
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
<h2><a id="wasi_clocks_wall_clock_0_2_2"></a>Import interface wasi:clocks/wall-clock@0.2.2</h2>
<p>WASI Wall Clock is a clock API intended to let users query the current
time. The name &quot;wall&quot; makes an analogy to a &quot;clock on the wall&quot;, which
is not necessarily monotonic as it may be reset.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<p>A wall clock is a clock which measures the date and time according to
some external reference.</p>
<p>External references may be reset, so this clock is not necessarily
monotonic, making it unsuitable for measuring elapsed time.</p>
<p>It is intended for reporting the current date and time for humans.</p>
<hr />
<h3>Types</h3>
<h4><a id="datetime"></a><code>record datetime</code></h4>
<p>A time and date in seconds plus nanoseconds.</p>
<h5>Record Fields</h5>
<ul>
<li><a id="datetime.seconds"></a><code>seconds</code>: <code>u64</code></li>
<li><a id="datetime.nanoseconds"></a><code>nanoseconds</code>: <code>u32</code></li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a id="now"></a><code>now: func</code></h4>
<p>Read the current value of the clock.</p>
<p>This clock is not monotonic, therefore calling this function repeatedly
will not necessarily produce a sequence of non-decreasing values.</p>
<p>The returned timestamps represent the number of seconds since
1970-01-01T00:00:00Z, also known as <a href="https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16">POSIX's Seconds Since the Epoch</a>,
also known as <a href="https://en.wikipedia.org/wiki/Unix_time">Unix Time</a>.</p>
<p>The nanoseconds field of the output is always less than 1000000000.</p>
<h5>Return values</h5>
<ul>
<li><a id="now.0"></a> <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></li>
</ul>
<h4><a id="resolution"></a><code>resolution: func</code></h4>
<p>Query the resolution of the clock.</p>
<p>The nanoseconds field of the output is always less than 1000000000.</p>
<h5>Return values</h5>
<ul>
<li><a id="resolution.0"></a> <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></li>
</ul>
<h2><a id="wasi_filesystem_types_0_2_2"></a>Import interface wasi:filesystem/types@0.2.2</h2>
<p>WASI filesystem is a filesystem API primarily intended to let users run WASI
programs that access their files on their existing filesystems, without
significant overhead.</p>
<p>It is intended to be roughly portable between Unix-family platforms and
Windows, though it does not hide many of the major differences.</p>
<p>Paths are passed as interface-type <code>string</code>s, meaning they must consist of
a sequence of Unicode Scalar Values (USVs). Some filesystems may contain
paths which are not accessible by this API.</p>
<p>The directory separator in WASI is always the forward-slash (<code>/</code>).</p>
<p>All paths in WASI are relative paths, and are interpreted relative to a
<a href="#descriptor"><code>descriptor</code></a> referring to a base directory. If a <code>path</code> argument to any WASI
function starts with <code>/</code>, or if any step of resolving a <code>path</code>, including
<code>..</code> and symbolic link steps, reaches a directory outside of the base
directory, or reaches a symlink to an absolute or rooted path in the
underlying filesystem, the function fails with <a href="#error_code.not_permitted"><code>error-code::not-permitted</code></a>.</p>
<p>For more information about WASI path resolution and sandboxing, see
<a href="https://github.com/WebAssembly/wasi-filesystem/blob/main/path-resolution.md">WASI filesystem path resolution</a>.</p>
<hr />
<h3>Types</h3>
<h4><a id="input_stream"></a><code>type input-stream</code></h4>
<p><a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></p>
<p>
#### <a id="output_stream"></a>`type output-stream`
[`output-stream`](#output_stream)
<p>
#### <a id="error"></a>`type error`
[`error`](#error)
<p>
#### <a id="datetime"></a>`type datetime`
[`datetime`](#datetime)
<p>
#### <a id="filesize"></a>`type filesize`
`u64`
<p>File size or length of a region within a file.
<h4><a id="descriptor_type"></a><code>enum descriptor-type</code></h4>
<p>The type of a filesystem object referenced by a descriptor.</p>
<p>Note: This was called <code>filetype</code> in earlier versions of WASI.</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="descriptor_type.unknown"></a><code>unknown</code></p>
<p>The type of the descriptor or file is unknown or is different from
any of the other types specified.
</li>
<li>
<p><a id="descriptor_type.block_device"></a><code>block-device</code></p>
<p>The descriptor refers to a block device inode.
</li>
<li>
<p><a id="descriptor_type.character_device"></a><code>character-device</code></p>
<p>The descriptor refers to a character device inode.
</li>
<li>
<p><a id="descriptor_type.directory"></a><code>directory</code></p>
<p>The descriptor refers to a directory inode.
</li>
<li>
<p><a id="descriptor_type.fifo"></a><code>fifo</code></p>
<p>The descriptor refers to a named pipe.
</li>
<li>
<p><a id="descriptor_type.symbolic_link"></a><code>symbolic-link</code></p>
<p>The file refers to a symbolic link inode.
</li>
<li>
<p><a id="descriptor_type.regular_file"></a><code>regular-file</code></p>
<p>The descriptor refers to a regular file inode.
</li>
<li>
<p><a id="descriptor_type.socket"></a><code>socket</code></p>
<p>The descriptor refers to a socket.
</li>
</ul>
<h4><a id="descriptor_flags"></a><code>flags descriptor-flags</code></h4>
<p>Descriptor flags.</p>
<p>Note: This was called <code>fdflags</code> in earlier versions of WASI.</p>
<h5>Flags members</h5>
<ul>
<li>
<p><a id="descriptor_flags.read"></a><code>read</code>: </p>
<p>Read mode: Data can be read.
</li>
<li>
<p><a id="descriptor_flags.write"></a><code>write</code>: </p>
<p>Write mode: Data can be written to.
</li>
<li>
<p><a id="descriptor_flags.file_integrity_sync"></a><code>file-integrity-sync</code>: </p>
<p>Request that writes be performed according to synchronized I/O file
integrity completion. The data stored in the file and the file's
metadata are synchronized. This is similar to `O_SYNC` in POSIX.
<p>The precise semantics of this operation have not yet been defined for
WASI. At this time, it should be interpreted as a request, and not a
requirement.</p>
</li>
<li>
<p><a id="descriptor_flags.data_integrity_sync"></a><code>data-integrity-sync</code>: </p>
<p>Request that writes be performed according to synchronized I/O data
integrity completion. Only the data stored in the file is
synchronized. This is similar to `O_DSYNC` in POSIX.
<p>The precise semantics of this operation have not yet been defined for
WASI. At this time, it should be interpreted as a request, and not a
requirement.</p>
</li>
<li>
<p><a id="descriptor_flags.requested_write_sync"></a><code>requested-write-sync</code>: </p>
<p>Requests that reads be performed at the same level of integrity
requested for writes. This is similar to `O_RSYNC` in POSIX.
<p>The precise semantics of this operation have not yet been defined for
WASI. At this time, it should be interpreted as a request, and not a
requirement.</p>
</li>
<li>
<p><a id="descriptor_flags.mutate_directory"></a><code>mutate-directory</code>: </p>
<p>Mutating directories mode: Directory contents may be mutated.
<p>When this flag is unset on a descriptor, operations using the
descriptor which would create, rename, delete, modify the data or
metadata of filesystem objects, or obtain another handle which
would permit any of those, shall fail with <a href="#error_code.read_only"><code>error-code::read-only</code></a> if
they would otherwise succeed.</p>
<p>This may only be set on directories.</p>
</li>
</ul>
<h4><a id="path_flags"></a><code>flags path-flags</code></h4>
<p>Flags determining the method of how paths are resolved.</p>
<h5>Flags members</h5>
<ul>
<li><a id="path_flags.symlink_follow"></a><code>symlink-follow</code>: <p>As long as the resolved path corresponds to a symbolic link, it is
expanded.
</li>
</ul>
<h4><a id="open_flags"></a><code>flags open-flags</code></h4>
<p>Open flags used by <code>open-at</code>.</p>
<h5>Flags members</h5>
<ul>
<li>
<p><a id="open_flags.create"></a><code>create</code>: </p>
<p>Create file if it does not exist, similar to `O_CREAT` in POSIX.
</li>
<li>
<p><a id="open_flags.directory"></a><code>directory</code>: </p>
<p>Fail if not a directory, similar to `O_DIRECTORY` in POSIX.
</li>
<li>
<p><a id="open_flags.exclusive"></a><code>exclusive</code>: </p>
<p>Fail if file already exists, similar to `O_EXCL` in POSIX.
</li>
<li>
<p><a id="open_flags.truncate"></a><code>truncate</code>: </p>
<p>Truncate file to size 0, similar to `O_TRUNC` in POSIX.
</li>
</ul>
<h4><a id="link_count"></a><code>type link-count</code></h4>
<p><code>u64</code></p>
<p>Number of hard links to an inode.
<h4><a id="descriptor_stat"></a><code>record descriptor-stat</code></h4>
<p>File attributes.</p>
<p>Note: This was called <code>filestat</code> in earlier versions of WASI.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a id="descriptor_stat.type"></a><code>type</code>: <a href="#descriptor_type"><a href="#descriptor_type"><code>descriptor-type</code></a></a></p>
<p>File type.
</li>
<li>
<p><a id="descriptor_stat.link_count"></a><a href="#link_count"><code>link-count</code></a>: <a href="#link_count"><a href="#link_count"><code>link-count</code></a></a></p>
<p>Number of hard links to the file.
</li>
<li>
<p><a id="descriptor_stat.size"></a><code>size</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></p>
<p>For regular files, the file size in bytes. For symbolic links, the
length in bytes of the pathname contained in the symbolic link.
</li>
<li>
<p><a id="descriptor_stat.data_access_timestamp"></a><code>data-access-timestamp</code>: option&lt;<a href="#datetime"><a href="#datetime"><code>datetime</code></a></a>&gt;</p>
<p>Last data access timestamp.
<p>If the <code>option</code> is none, the platform doesn't maintain an access
timestamp for this file.</p>
</li>
<li>
<p><a id="descriptor_stat.data_modification_timestamp"></a><code>data-modification-timestamp</code>: option&lt;<a href="#datetime"><a href="#datetime"><code>datetime</code></a></a>&gt;</p>
<p>Last data modification timestamp.
<p>If the <code>option</code> is none, the platform doesn't maintain a
modification timestamp for this file.</p>
</li>
<li>
<p><a id="descriptor_stat.status_change_timestamp"></a><code>status-change-timestamp</code>: option&lt;<a href="#datetime"><a href="#datetime"><code>datetime</code></a></a>&gt;</p>
<p>Last file status-change timestamp.
<p>If the <code>option</code> is none, the platform doesn't maintain a
status-change timestamp for this file.</p>
</li>
</ul>
<h4><a id="new_timestamp"></a><code>variant new-timestamp</code></h4>
<p>When setting a timestamp, this gives the value to set it to.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="new_timestamp.no_change"></a><code>no-change</code></p>
<p>Leave the timestamp set to its previous value.
</li>
<li>
<p><a id="new_timestamp.now"></a><a href="#now"><code>now</code></a></p>
<p>Set the timestamp to the current time of the system clock associated
with the filesystem.
</li>
<li>
<p><a id="new_timestamp.timestamp"></a><code>timestamp</code>: <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></p>
<p>Set the timestamp to the given value.
</li>
</ul>
<h4><a id="directory_entry"></a><code>record directory-entry</code></h4>
<p>A directory entry.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a id="directory_entry.type"></a><code>type</code>: <a href="#descriptor_type"><a href="#descriptor_type"><code>descriptor-type</code></a></a></p>
<p>The type of the file referred to by this directory entry.
</li>
<li>
<p><a id="directory_entry.name"></a><code>name</code>: <code>string</code></p>
<p>The name of the object.
</li>
</ul>
<h4><a id="error_code"></a><code>enum error-code</code></h4>
<p>Error codes returned by functions, similar to <code>errno</code> in POSIX.
Not all of these error codes are returned by the functions provided by this
API; some are used in higher-level library layers, and others are provided
merely for alignment with POSIX.</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="error_code.access"></a><code>access</code></p>
<p>Permission denied, similar to `EACCES` in POSIX.
</li>
<li>
<p><a id="error_code.would_block"></a><code>would-block</code></p>
<p>Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX.
</li>
<li>
<p><a id="error_code.already"></a><code>already</code></p>
<p>Connection already in progress, similar to `EALREADY` in POSIX.
</li>
<li>
<p><a id="error_code.bad_descriptor"></a><code>bad-descriptor</code></p>
<p>Bad descriptor, similar to `EBADF` in POSIX.
</li>
<li>
<p><a id="error_code.busy"></a><code>busy</code></p>
<p>Device or resource busy, similar to `EBUSY` in POSIX.
</li>
<li>
<p><a id="error_code.deadlock"></a><code>deadlock</code></p>
<p>Resource deadlock would occur, similar to `EDEADLK` in POSIX.
</li>
<li>
<p><a id="error_code.quota"></a><code>quota</code></p>
<p>Storage quota exceeded, similar to `EDQUOT` in POSIX.
</li>
<li>
<p><a id="error_code.exist"></a><code>exist</code></p>
<p>File exists, similar to `EEXIST` in POSIX.
</li>
<li>
<p><a id="error_code.file_too_large"></a><code>file-too-large</code></p>
<p>File too large, similar to `EFBIG` in POSIX.
</li>
<li>
<p><a id="error_code.illegal_byte_sequence"></a><code>illegal-byte-sequence</code></p>
<p>Illegal byte sequence, similar to `EILSEQ` in POSIX.
</li>
<li>
<p><a id="error_code.in_progress"></a><code>in-progress</code></p>
<p>Operation in progress, similar to `EINPROGRESS` in POSIX.
</li>
<li>
<p><a id="error_code.interrupted"></a><code>interrupted</code></p>
<p>Interrupted function, similar to `EINTR` in POSIX.
</li>
<li>
<p><a id="error_code.invalid"></a><code>invalid</code></p>
<p>Invalid argument, similar to `EINVAL` in POSIX.
</li>
<li>
<p><a id="error_code.io"></a><code>io</code></p>
<p>I/O error, similar to `EIO` in POSIX.
</li>
<li>
<p><a id="error_code.is_directory"></a><code>is-directory</code></p>
<p>Is a directory, similar to `EISDIR` in POSIX.
</li>
<li>
<p><a id="error_code.loop"></a><code>loop</code></p>
<p>Too many levels of symbolic links, similar to `ELOOP` in POSIX.
</li>
<li>
<p><a id="error_code.too_many_links"></a><code>too-many-links</code></p>
<p>Too many links, similar to `EMLINK` in POSIX.
</li>
<li>
<p><a id="error_code.message_size"></a><code>message-size</code></p>
<p>Message too large, similar to `EMSGSIZE` in POSIX.
</li>
<li>
<p><a id="error_code.name_too_long"></a><code>name-too-long</code></p>
<p>Filename too long, similar to `ENAMETOOLONG` in POSIX.
</li>
<li>
<p><a id="error_code.no_device"></a><code>no-device</code></p>
<p>No such device, similar to `ENODEV` in POSIX.
</li>
<li>
<p><a id="error_code.no_entry"></a><code>no-entry</code></p>
<p>No such file or directory, similar to `ENOENT` in POSIX.
</li>
<li>
<p><a id="error_code.no_lock"></a><code>no-lock</code></p>
<p>No locks available, similar to `ENOLCK` in POSIX.
</li>
<li>
<p><a id="error_code.insufficient_memory"></a><code>insufficient-memory</code></p>
<p>Not enough space, similar to `ENOMEM` in POSIX.
</li>
<li>
<p><a id="error_code.insufficient_space"></a><code>insufficient-space</code></p>
<p>No space left on device, similar to `ENOSPC` in POSIX.
</li>
<li>
<p><a id="error_code.not_directory"></a><code>not-directory</code></p>
<p>Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX.
</li>
<li>
<p><a id="error_code.not_empty"></a><code>not-empty</code></p>
<p>Directory not empty, similar to `ENOTEMPTY` in POSIX.
</li>
<li>
<p><a id="error_code.not_recoverable"></a><code>not-recoverable</code></p>
<p>State not recoverable, similar to `ENOTRECOVERABLE` in POSIX.
</li>
<li>
<p><a id="error_code.unsupported"></a><code>unsupported</code></p>
<p>Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX.
</li>
<li>
<p><a id="error_code.no_tty"></a><code>no-tty</code></p>
<p>Inappropriate I/O control operation, similar to `ENOTTY` in POSIX.
</li>
<li>
<p><a id="error_code.no_such_device"></a><code>no-such-device</code></p>
<p>No such device or address, similar to `ENXIO` in POSIX.
</li>
<li>
<p><a id="error_code.overflow"></a><code>overflow</code></p>
<p>Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX.
</li>
<li>
<p><a id="error_code.not_permitted"></a><code>not-permitted</code></p>
<p>Operation not permitted, similar to `EPERM` in POSIX.
</li>
<li>
<p><a id="error_code.pipe"></a><code>pipe</code></p>
<p>Broken pipe, similar to `EPIPE` in POSIX.
</li>
<li>
<p><a id="error_code.read_only"></a><code>read-only</code></p>
<p>Read-only file system, similar to `EROFS` in POSIX.
</li>
<li>
<p><a id="error_code.invalid_seek"></a><code>invalid-seek</code></p>
<p>Invalid seek, similar to `ESPIPE` in POSIX.
</li>
<li>
<p><a id="error_code.text_file_busy"></a><code>text-file-busy</code></p>
<p>Text file busy, similar to `ETXTBSY` in POSIX.
</li>
<li>
<p><a id="error_code.cross_device"></a><code>cross-device</code></p>
<p>Cross-device link, similar to `EXDEV` in POSIX.
</li>
</ul>
<h4><a id="advice"></a><code>enum advice</code></h4>
<p>File or memory access pattern advisory information.</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="advice.normal"></a><code>normal</code></p>
<p>The application has no advice to give on its behavior with respect
to the specified data.
</li>
<li>
<p><a id="advice.sequential"></a><code>sequential</code></p>
<p>The application expects to access the specified data sequentially
from lower offsets to higher offsets.
</li>
<li>
<p><a id="advice.random"></a><code>random</code></p>
<p>The application expects to access the specified data in a random
order.
</li>
<li>
<p><a id="advice.will_need"></a><code>will-need</code></p>
<p>The application expects to access the specified data in the near
future.
</li>
<li>
<p><a id="advice.dont_need"></a><code>dont-need</code></p>
<p>The application expects that it will not access the specified data
in the near future.
</li>
<li>
<p><a id="advice.no_reuse"></a><code>no-reuse</code></p>
<p>The application expects to access the specified data once and then
not reuse it thereafter.
</li>
</ul>
<h4><a id="metadata_hash_value"></a><code>record metadata-hash-value</code></h4>
<p>A 128-bit hash value, split into parts because wasm doesn't have a
128-bit integer type.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a id="metadata_hash_value.lower"></a><code>lower</code>: <code>u64</code></p>
<p>64 bits of a 128-bit hash value.
</li>
<li>
<p><a id="metadata_hash_value.upper"></a><code>upper</code>: <code>u64</code></p>
<p>Another 64 bits of a 128-bit hash value.
</li>
</ul>
<h4><a id="descriptor"></a><code>resource descriptor</code></h4>
<p>A descriptor is a reference to a filesystem object, which may be a file,
directory, named pipe, special file, or other object on which filesystem
calls may be made.</p>
<h4><a id="directory_entry_stream"></a><code>resource directory-entry-stream</code></h4>
<h2>A stream of directory entries.</h2>
<h3>Functions</h3>
<h4><a id="method_descriptor_read_via_stream"></a><code>[method]descriptor.read-via-stream: func</code></h4>
<p>Return a stream for reading from a file, if available.</p>
<p>May fail with an error-code describing why the file cannot be read.</p>
<p>Multiple read, write, and append streams may be active on the same open
file and they do not interfere with each other.</p>
<p>Note: This allows using <code>read-stream</code>, which is similar to <code>read</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_read_via_stream.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_read_via_stream.offset"></a><code>offset</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_read_via_stream.0"></a> result&lt;own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_write_via_stream"></a><code>[method]descriptor.write-via-stream: func</code></h4>
<p>Return a stream for writing to a file, if available.</p>
<p>May fail with an error-code describing why the file cannot be written.</p>
<p>Note: This allows using <code>write-stream</code>, which is similar to <code>write</code> in
POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_write_via_stream.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_write_via_stream.offset"></a><code>offset</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_write_via_stream.0"></a> result&lt;own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_append_via_stream"></a><code>[method]descriptor.append-via-stream: func</code></h4>
<p>Return a stream for appending to a file, if available.</p>
<p>May fail with an error-code describing why the file cannot be appended.</p>
<p>Note: This allows using <code>write-stream</code>, which is similar to <code>write</code> with
<code>O_APPEND</code> in in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_append_via_stream.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_append_via_stream.0"></a> result&lt;own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_advise"></a><code>[method]descriptor.advise: func</code></h4>
<p>Provide file advisory information on a descriptor.</p>
<p>This is similar to <code>posix_fadvise</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_advise.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_advise.offset"></a><code>offset</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
<li><a id="method_descriptor_advise.length"></a><code>length</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
<li><a id="method_descriptor_advise.advice"></a><a href="#advice"><code>advice</code></a>: <a href="#advice"><a href="#advice"><code>advice</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_advise.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_sync_data"></a><code>[method]descriptor.sync-data: func</code></h4>
<p>Synchronize the data of a file to disk.</p>
<p>This function succeeds with no effect if the file descriptor is not
opened for writing.</p>
<p>Note: This is similar to <code>fdatasync</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_sync_data.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_sync_data.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_get_flags"></a><code>[method]descriptor.get-flags: func</code></h4>
<p>Get flags associated with a descriptor.</p>
<p>Note: This returns similar flags to <code>fcntl(fd, F_GETFL)</code> in POSIX.</p>
<p>Note: This returns the value that was the <code>fs_flags</code> value returned
from <code>fdstat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_get_flags.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_get_flags.0"></a> result&lt;<a href="#descriptor_flags"><a href="#descriptor_flags"><code>descriptor-flags</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_get_type"></a><code>[method]descriptor.get-type: func</code></h4>
<p>Get the dynamic type of a descriptor.</p>
<p>Note: This returns the same value as the <code>type</code> field of the <code>fd-stat</code>
returned by <code>stat</code>, <code>stat-at</code> and similar.</p>
<p>Note: This returns similar flags to the <code>st_mode &amp; S_IFMT</code> value provided
by <code>fstat</code> in POSIX.</p>
<p>Note: This returns the value that was the <code>fs_filetype</code> value returned
from <code>fdstat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_get_type.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_get_type.0"></a> result&lt;<a href="#descriptor_type"><a href="#descriptor_type"><code>descriptor-type</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_set_size"></a><code>[method]descriptor.set-size: func</code></h4>
<p>Adjust the size of an open file. If this increases the file's size, the
extra bytes are filled with zeros.</p>
<p>Note: This was called <code>fd_filestat_set_size</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_set_size.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_set_size.size"></a><code>size</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_set_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_set_times"></a><code>[method]descriptor.set-times: func</code></h4>
<p>Adjust the timestamps of an open file or directory.</p>
<p>Note: This is similar to <code>futimens</code> in POSIX.</p>
<p>Note: This was called <code>fd_filestat_set_times</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_set_times.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_set_times.data_access_timestamp"></a><code>data-access-timestamp</code>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
<li><a id="method_descriptor_set_times.data_modification_timestamp"></a><code>data-modification-timestamp</code>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_set_times.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_read"></a><code>[method]descriptor.read: func</code></h4>
<p>Read from a descriptor, without using and updating the descriptor's offset.</p>
<p>This function returns a list of bytes containing the data that was
read, along with a bool which, when true, indicates that the end of the
file was reached. The returned list will contain up to <code>length</code> bytes; it
may return fewer than requested, if the end of the file is reached or
if the I/O operation is interrupted.</p>
<p>In the future, this may change to return a <code>stream&lt;u8, error-code&gt;</code>.</p>
<p>Note: This is similar to <code>pread</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_read.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_read.length"></a><code>length</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
<li><a id="method_descriptor_read.offset"></a><code>offset</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_read.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <code>bool</code>), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_write"></a><code>[method]descriptor.write: func</code></h4>
<p>Write to a descriptor, without using and updating the descriptor's offset.</p>
<p>It is valid to write past the end of a file; the file is extended to the
extent of the write, with bytes between the previous end and the start of
the write set to zero.</p>
<p>In the future, this may change to take a <code>stream&lt;u8, error-code&gt;</code>.</p>
<p>Note: This is similar to <code>pwrite</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_write.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_write.buffer"></a><code>buffer</code>: list&lt;<code>u8</code>&gt;</li>
<li><a id="method_descriptor_write.offset"></a><code>offset</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_write.0"></a> result&lt;<a href="#filesize"><a href="#filesize"><code>filesize</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_read_directory"></a><code>[method]descriptor.read-directory: func</code></h4>
<p>Read directory entries from a directory.</p>
<p>On filesystems where directories contain entries referring to themselves
and their parents, often named <code>.</code> and <code>..</code> respectively, these entries
are omitted.</p>
<p>This always returns a new stream which starts at the beginning of the
directory. Multiple streams may be active on the same directory, and they
do not interfere with each other.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_read_directory.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_read_directory.0"></a> result&lt;own&lt;<a href="#directory_entry_stream"><a href="#directory_entry_stream"><code>directory-entry-stream</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_sync"></a><code>[method]descriptor.sync: func</code></h4>
<p>Synchronize the data and metadata of a file to disk.</p>
<p>This function succeeds with no effect if the file descriptor is not
opened for writing.</p>
<p>Note: This is similar to <code>fsync</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_sync.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_sync.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_create_directory_at"></a><code>[method]descriptor.create-directory-at: func</code></h4>
<p>Create a directory.</p>
<p>Note: This is similar to <code>mkdirat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_create_directory_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_create_directory_at.path"></a><code>path</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_create_directory_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_stat"></a><code>[method]descriptor.stat: func</code></h4>
<p>Return the attributes of an open file or directory.</p>
<p>Note: This is similar to <code>fstat</code> in POSIX, except that it does not return
device and inode information. For testing whether two descriptors refer to
the same underlying filesystem object, use <code>is-same-object</code>. To obtain
additional data that can be used do determine whether a file has been
modified, use <code>metadata-hash</code>.</p>
<p>Note: This was called <code>fd_filestat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_stat.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_stat.0"></a> result&lt;<a href="#descriptor_stat"><a href="#descriptor_stat"><code>descriptor-stat</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_stat_at"></a><code>[method]descriptor.stat-at: func</code></h4>
<p>Return the attributes of a file or directory.</p>
<p>Note: This is similar to <code>fstatat</code> in POSIX, except that it does not
return device and inode information. See the <code>stat</code> description for a
discussion of alternatives.</p>
<p>Note: This was called <code>path_filestat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_stat_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_stat_at.path_flags"></a><a href="#path_flags"><code>path-flags</code></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a id="method_descriptor_stat_at.path"></a><code>path</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_stat_at.0"></a> result&lt;<a href="#descriptor_stat"><a href="#descriptor_stat"><code>descriptor-stat</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_set_times_at"></a><code>[method]descriptor.set-times-at: func</code></h4>
<p>Adjust the timestamps of a file or directory.</p>
<p>Note: This is similar to <code>utimensat</code> in POSIX.</p>
<p>Note: This was called <code>path_filestat_set_times</code> in earlier versions of
WASI.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_set_times_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_set_times_at.path_flags"></a><a href="#path_flags"><code>path-flags</code></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a id="method_descriptor_set_times_at.path"></a><code>path</code>: <code>string</code></li>
<li><a id="method_descriptor_set_times_at.data_access_timestamp"></a><code>data-access-timestamp</code>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
<li><a id="method_descriptor_set_times_at.data_modification_timestamp"></a><code>data-modification-timestamp</code>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_set_times_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_link_at"></a><code>[method]descriptor.link-at: func</code></h4>
<p>Create a hard link.</p>
<p>Note: This is similar to <code>linkat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_link_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_link_at.old_path_flags"></a><code>old-path-flags</code>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a id="method_descriptor_link_at.old_path"></a><code>old-path</code>: <code>string</code></li>
<li><a id="method_descriptor_link_at.new_descriptor"></a><code>new-descriptor</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_link_at.new_path"></a><code>new-path</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_link_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_open_at"></a><code>[method]descriptor.open-at: func</code></h4>
<p>Open a file or directory.</p>
<p>If <code>flags</code> contains <a href="#descriptor_flags.mutate_directory"><code>descriptor-flags::mutate-directory</code></a>, and the base
descriptor doesn't have <a href="#descriptor_flags.mutate_directory"><code>descriptor-flags::mutate-directory</code></a> set,
<code>open-at</code> fails with <a href="#error_code.read_only"><code>error-code::read-only</code></a>.</p>
<p>If <code>flags</code> contains <code>write</code> or <code>mutate-directory</code>, or <a href="#open_flags"><code>open-flags</code></a>
contains <code>truncate</code> or <code>create</code>, and the base descriptor doesn't have
<a href="#descriptor_flags.mutate_directory"><code>descriptor-flags::mutate-directory</code></a> set, <code>open-at</code> fails with
<a href="#error_code.read_only"><code>error-code::read-only</code></a>.</p>
<p>Note: This is similar to <code>openat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_open_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_open_at.path_flags"></a><a href="#path_flags"><code>path-flags</code></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a id="method_descriptor_open_at.path"></a><code>path</code>: <code>string</code></li>
<li><a id="method_descriptor_open_at.open_flags"></a><a href="#open_flags"><code>open-flags</code></a>: <a href="#open_flags"><a href="#open_flags"><code>open-flags</code></a></a></li>
<li><a id="method_descriptor_open_at.flags"></a><code>flags</code>: <a href="#descriptor_flags"><a href="#descriptor_flags"><code>descriptor-flags</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_open_at.0"></a> result&lt;own&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_readlink_at"></a><code>[method]descriptor.readlink-at: func</code></h4>
<p>Read the contents of a symbolic link.</p>
<p>If the contents contain an absolute or rooted path in the underlying
filesystem, this function fails with <a href="#error_code.not_permitted"><code>error-code::not-permitted</code></a>.</p>
<p>Note: This is similar to <code>readlinkat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_readlink_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_readlink_at.path"></a><code>path</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_readlink_at.0"></a> result&lt;<code>string</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_remove_directory_at"></a><code>[method]descriptor.remove-directory-at: func</code></h4>
<p>Remove a directory.</p>
<p>Return <a href="#error_code.not_empty"><code>error-code::not-empty</code></a> if the directory is not empty.</p>
<p>Note: This is similar to <code>unlinkat(fd, path, AT_REMOVEDIR)</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_remove_directory_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_remove_directory_at.path"></a><code>path</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_remove_directory_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_rename_at"></a><code>[method]descriptor.rename-at: func</code></h4>
<p>Rename a filesystem object.</p>
<p>Note: This is similar to <code>renameat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_rename_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_rename_at.old_path"></a><code>old-path</code>: <code>string</code></li>
<li><a id="method_descriptor_rename_at.new_descriptor"></a><code>new-descriptor</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_rename_at.new_path"></a><code>new-path</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_rename_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_symlink_at"></a><code>[method]descriptor.symlink-at: func</code></h4>
<p>Create a symbolic link (also known as a &quot;symlink&quot;).</p>
<p>If <code>old-path</code> starts with <code>/</code>, the function fails with
<a href="#error_code.not_permitted"><code>error-code::not-permitted</code></a>.</p>
<p>Note: This is similar to <code>symlinkat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_symlink_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_symlink_at.old_path"></a><code>old-path</code>: <code>string</code></li>
<li><a id="method_descriptor_symlink_at.new_path"></a><code>new-path</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_symlink_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_unlink_file_at"></a><code>[method]descriptor.unlink-file-at: func</code></h4>
<p>Unlink a filesystem object that is not a directory.</p>
<p>Return <a href="#error_code.is_directory"><code>error-code::is-directory</code></a> if the path refers to a directory.
Note: This is similar to <code>unlinkat(fd, path, 0)</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_unlink_file_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_unlink_file_at.path"></a><code>path</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_unlink_file_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_is_same_object"></a><code>[method]descriptor.is-same-object: func</code></h4>
<p>Test whether two descriptors refer to the same filesystem object.</p>
<p>In POSIX, this corresponds to testing whether the two descriptors have the
same device (<code>st_dev</code>) and inode (<code>st_ino</code> or <code>d_ino</code>) numbers.
wasi-filesystem does not expose device and inode numbers, so this function
may be used instead.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_is_same_object.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_is_same_object.other"></a><code>other</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_is_same_object.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_descriptor_metadata_hash"></a><code>[method]descriptor.metadata-hash: func</code></h4>
<p>Return a hash of the metadata associated with a filesystem object referred
to by a descriptor.</p>
<p>This returns a hash of the last-modification timestamp and file size, and
may also include the inode number, device number, birth timestamp, and
other metadata fields that may change when the file is modified or
replaced. It may also include a secret value chosen by the
implementation and not otherwise exposed.</p>
<p>Implementations are encourated to provide the following properties:</p>
<ul>
<li>If the file is not modified or replaced, the computed hash value should
usually not change.</li>
<li>If the object is modified or replaced, the computed hash value should
usually change.</li>
<li>The inputs to the hash should not be easily computable from the
computed hash.</li>
</ul>
<p>However, none of these is required.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_metadata_hash.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_metadata_hash.0"></a> result&lt;<a href="#metadata_hash_value"><a href="#metadata_hash_value"><code>metadata-hash-value</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_descriptor_metadata_hash_at"></a><code>[method]descriptor.metadata-hash-at: func</code></h4>
<p>Return a hash of the metadata associated with a filesystem object referred
to by a directory descriptor and a relative path.</p>
<p>This performs the same hash computation as <code>metadata-hash</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_metadata_hash_at.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_metadata_hash_at.path_flags"></a><a href="#path_flags"><code>path-flags</code></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a id="method_descriptor_metadata_hash_at.path"></a><code>path</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_metadata_hash_at.0"></a> result&lt;<a href="#metadata_hash_value"><a href="#metadata_hash_value"><code>metadata-hash-value</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_directory_entry_stream_read_directory_entry"></a><code>[method]directory-entry-stream.read-directory-entry: func</code></h4>
<p>Read a single directory entry from a <a href="#directory_entry_stream"><code>directory-entry-stream</code></a>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_directory_entry_stream_read_directory_entry.self"></a><code>self</code>: borrow&lt;<a href="#directory_entry_stream"><a href="#directory_entry_stream"><code>directory-entry-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_directory_entry_stream_read_directory_entry.0"></a> result&lt;option&lt;<a href="#directory_entry"><a href="#directory_entry"><code>directory-entry</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="filesystem_error_code"></a><code>filesystem-error-code: func</code></h4>
<p>Attempts to extract a filesystem-related <a href="#error_code"><code>error-code</code></a> from the stream
<a href="#error"><code>error</code></a> provided.</p>
<p>Stream operations which return <a href="#stream_error.last_operation_failed"><code>stream-error::last-operation-failed</code></a>
have a payload with more information about the operation that failed.
This payload can be passed through to this function to see if there's
filesystem-related information about the error to return.</p>
<p>Note that this function is fallible because not all stream-related
errors are filesystem-related errors.</p>
<h5>Params</h5>
<ul>
<li><a id="filesystem_error_code.err"></a><code>err</code>: borrow&lt;<a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="filesystem_error_code.0"></a> option&lt;<a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_filesystem_preopens_0_2_2"></a>Import interface wasi:filesystem/preopens@0.2.2</h2>
<hr />
<h3>Types</h3>
<h4><a id="descriptor"></a><code>type descriptor</code></h4>
<p><a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></p>
<p>
----
<h3>Functions</h3>
<h4><a id="get_directories"></a><code>get-directories: func</code></h4>
<p>Return the set of preopened directories, and their path.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_directories.0"></a> list&lt;(own&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;, <code>string</code>)&gt;</li>
</ul>
<h2><a id="wasi_sockets_network_0_2_2"></a>Import interface wasi:sockets/network@0.2.2</h2>
<hr />
<h3>Types</h3>
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
<h2><a id="wasi_sockets_instance_network_0_2_2"></a>Import interface wasi:sockets/instance-network@0.2.2</h2>
<p>This interface provides a value-export of the default network handle..</p>
<hr />
<h3>Types</h3>
<h4><a id="network"></a><code>type network</code></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
----
<h3>Functions</h3>
<h4><a id="instance_network"></a><code>instance-network: func</code></h4>
<p>Get a handle to the default network.</p>
<h5>Return values</h5>
<ul>
<li><a id="instance_network.0"></a> own&lt;<a href="#network"><a href="#network"><code>network</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_sockets_udp_0_2_2"></a>Import interface wasi:sockets/udp@0.2.2</h2>
<hr />
<h3>Types</h3>
<h4><a id="pollable"></a><code>type pollable</code></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
#### <a id="network"></a>`type network`
[`network`](#network)
<p>
#### <a id="error_code"></a>`type error-code`
[`error-code`](#error_code)
<p>
#### <a id="ip_socket_address"></a>`type ip-socket-address`
[`ip-socket-address`](#ip_socket_address)
<p>
#### <a id="ip_address_family"></a>`type ip-address-family`
[`ip-address-family`](#ip_address_family)
<p>
#### <a id="incoming_datagram"></a>`record incoming-datagram`
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
<h2><a id="wasi_sockets_udp_create_socket_0_2_2"></a>Import interface wasi:sockets/udp-create-socket@0.2.2</h2>
<hr />
<h3>Types</h3>
<h4><a id="network"></a><code>type network</code></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
#### <a id="error_code"></a>`type error-code`
[`error-code`](#error_code)
<p>
#### <a id="ip_address_family"></a>`type ip-address-family`
[`ip-address-family`](#ip_address_family)
<p>
#### <a id="udp_socket"></a>`type udp-socket`
[`udp-socket`](#udp_socket)
<p>
----
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
<h2><a id="wasi_sockets_tcp_0_2_2"></a>Import interface wasi:sockets/tcp@0.2.2</h2>
<hr />
<h3>Types</h3>
<h4><a id="input_stream"></a><code>type input-stream</code></h4>
<p><a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></p>
<p>
#### <a id="output_stream"></a>`type output-stream`
[`output-stream`](#output_stream)
<p>
#### <a id="pollable"></a>`type pollable`
[`pollable`](#pollable)
<p>
#### <a id="duration"></a>`type duration`
[`duration`](#duration)
<p>
#### <a id="network"></a>`type network`
[`network`](#network)
<p>
#### <a id="error_code"></a>`type error-code`
[`error-code`](#error_code)
<p>
#### <a id="ip_socket_address"></a>`type ip-socket-address`
[`ip-socket-address`](#ip_socket_address)
<p>
#### <a id="ip_address_family"></a>`type ip-address-family`
[`ip-address-family`](#ip_address_family)
<p>
#### <a id="shutdown_type"></a>`enum shutdown-type`
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
<h2><a id="wasi_sockets_tcp_create_socket_0_2_2"></a>Import interface wasi:sockets/tcp-create-socket@0.2.2</h2>
<hr />
<h3>Types</h3>
<h4><a id="network"></a><code>type network</code></h4>
<p><a href="#network"><a href="#network"><code>network</code></a></a></p>
<p>
#### <a id="error_code"></a>`type error-code`
[`error-code`](#error_code)
<p>
#### <a id="ip_address_family"></a>`type ip-address-family`
[`ip-address-family`](#ip_address_family)
<p>
#### <a id="tcp_socket"></a>`type tcp-socket`
[`tcp-socket`](#tcp_socket)
<p>
----
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
<h2><a id="wasi_sockets_ip_name_lookup_0_2_2"></a>Import interface wasi:sockets/ip-name-lookup@0.2.2</h2>
<hr />
<h3>Types</h3>
<h4><a id="pollable"></a><code>type pollable</code></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
#### <a id="network"></a>`type network`
[`network`](#network)
<p>
#### <a id="error_code"></a>`type error-code`
[`error-code`](#error_code)
<p>
#### <a id="ip_address"></a>`type ip-address`
[`ip-address`](#ip_address)
<p>
#### <a id="resolve_address_stream"></a>`resource resolve-address-stream`
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
<h2><a id="wasi_random_random_0_2_2"></a>Import interface wasi:random/random@0.2.2</h2>
<p>WASI Random is a random data API.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a id="get_random_bytes"></a><code>get-random-bytes: func</code></h4>
<p>Return <code>len</code> cryptographically-secure random or pseudo-random bytes.</p>
<p>This function must produce data at least as cryptographically secure and
fast as an adequately seeded cryptographically-secure pseudo-random
number generator (CSPRNG). It must not block, from the perspective of
the calling program, under any circumstances, including on the first
request and on requests for numbers of bytes. The returned data must
always be unpredictable.</p>
<p>This function must always return fresh data. Deterministic environments
must omit this function, rather than implementing it with deterministic
data.</p>
<h5>Params</h5>
<ul>
<li><a id="get_random_bytes.len"></a><code>len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="get_random_bytes.0"></a> list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a id="get_random_u64"></a><code>get-random-u64: func</code></h4>
<p>Return a cryptographically-secure random or pseudo-random <code>u64</code> value.</p>
<p>This function returns the same type of data as <a href="#get_random_bytes"><code>get-random-bytes</code></a>,
represented as a <code>u64</code>.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_random_u64.0"></a> <code>u64</code></li>
</ul>
<h2><a id="wasi_random_insecure_0_2_2"></a>Import interface wasi:random/insecure@0.2.2</h2>
<p>The insecure interface for insecure pseudo-random numbers.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a id="get_insecure_random_bytes"></a><code>get-insecure-random-bytes: func</code></h4>
<p>Return <code>len</code> insecure pseudo-random bytes.</p>
<p>This function is not cryptographically secure. Do not use it for
anything related to security.</p>
<p>There are no requirements on the values of the returned bytes, however
implementations are encouraged to return evenly distributed values with
a long period.</p>
<h5>Params</h5>
<ul>
<li><a id="get_insecure_random_bytes.len"></a><code>len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="get_insecure_random_bytes.0"></a> list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a id="get_insecure_random_u64"></a><code>get-insecure-random-u64: func</code></h4>
<p>Return an insecure pseudo-random <code>u64</code> value.</p>
<p>This function returns the same type of pseudo-random data as
<a href="#get_insecure_random_bytes"><code>get-insecure-random-bytes</code></a>, represented as a <code>u64</code>.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_insecure_random_u64.0"></a> <code>u64</code></li>
</ul>
<h2><a id="wasi_random_insecure_seed_0_2_2"></a>Import interface wasi:random/insecure-seed@0.2.2</h2>
<p>The insecure-seed interface for seeding hash-map DoS resistance.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a id="insecure_seed"></a><code>insecure-seed: func</code></h4>
<p>Return a 128-bit value that may contain a pseudo-random value.</p>
<p>The returned value is not required to be computed from a CSPRNG, and may
even be entirely deterministic. Host implementations are encouraged to
provide pseudo-random values to any program exposed to
attacker-controlled content, to enable DoS protection built into many
languages' hash-map implementations.</p>
<p>This function is intended to only be called once, by a source language
to initialize Denial Of Service (DoS) protection in its hash-map
implementation.</p>
<h1>Expected future evolution</h1>
<p>This will likely be changed to a value import, to prevent it from being
called multiple times and potentially used for purposes other than DoS
protection.</p>
<h5>Return values</h5>
<ul>
<li><a id="insecure_seed.0"></a> (<code>u64</code>, <code>u64</code>)</li>
</ul>
<h2><a id="wasi_cli_run_0_2_2"></a>Export interface wasi:cli/run@0.2.2</h2>
<hr />
<h3>Functions</h3>
<h4><a id="run"></a><code>run: func</code></h4>
<p>Run the program.</p>
<h5>Return values</h5>
<ul>
<li><a id="run.0"></a> result</li>
</ul>
