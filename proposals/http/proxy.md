<h1><a id="proxy"></a>World proxy</h1>
<p>The <code>wasi:http/proxy</code> world captures a widely-implementable intersection of
hosts that includes HTTP forward and reverse proxies. Components targeting
this world may concurrently stream in and out any number of incoming and
outgoing HTTP requests.</p>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_io_poll_0_2_3"><code>wasi:io/poll@0.2.3</code></a></li>
<li>interface <a href="#wasi_clocks_monotonic_clock_0_2_3"><code>wasi:clocks/monotonic-clock@0.2.3</code></a></li>
<li>interface <a href="#wasi_clocks_wall_clock_0_2_3"><code>wasi:clocks/wall-clock@0.2.3</code></a></li>
<li>interface <a href="#wasi_random_random_0_2_3"><code>wasi:random/random@0.2.3</code></a></li>
<li>interface <a href="#wasi_io_error_0_2_3"><code>wasi:io/error@0.2.3</code></a></li>
<li>interface <a href="#wasi_io_streams_0_2_3"><code>wasi:io/streams@0.2.3</code></a></li>
<li>interface <a href="#wasi_cli_stdout_0_2_3"><code>wasi:cli/stdout@0.2.3</code></a></li>
<li>interface <a href="#wasi_cli_stderr_0_2_3"><code>wasi:cli/stderr@0.2.3</code></a></li>
<li>interface <a href="#wasi_cli_stdin_0_2_3"><code>wasi:cli/stdin@0.2.3</code></a></li>
<li>interface <a href="#wasi_http_types_0_2_3"><code>wasi:http/types@0.2.3</code></a></li>
<li>interface <a href="#wasi_http_outgoing_handler_0_2_3"><code>wasi:http/outgoing-handler@0.2.3</code></a></li>
</ul>
</li>
<li>Exports:
<ul>
<li>interface <a href="#wasi_http_incoming_handler_0_2_3"><code>wasi:http/incoming-handler@0.2.3</code></a></li>
</ul>
</li>
</ul>
<h2><a id="wasi_io_poll_0_2_3"></a>Import interface wasi:io/poll@0.2.3</h2>
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
<h2><a id="wasi_clocks_monotonic_clock_0_2_3"></a>Import interface wasi:clocks/monotonic-clock@0.2.3</h2>
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
<h2><a id="wasi_clocks_wall_clock_0_2_3"></a>Import interface wasi:clocks/wall-clock@0.2.3</h2>
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
<h2><a id="wasi_random_random_0_2_3"></a>Import interface wasi:random/random@0.2.3</h2>
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
<h2><a id="wasi_io_error_0_2_3"></a>Import interface wasi:io/error@0.2.3</h2>
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
<h2><a id="wasi_io_streams_0_2_3"></a>Import interface wasi:io/streams@0.2.3</h2>
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
<h2><a id="wasi_cli_stdout_0_2_3"></a>Import interface wasi:cli/stdout@0.2.3</h2>
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
<h2><a id="wasi_cli_stderr_0_2_3"></a>Import interface wasi:cli/stderr@0.2.3</h2>
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
<h2><a id="wasi_cli_stdin_0_2_3"></a>Import interface wasi:cli/stdin@0.2.3</h2>
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
<h2><a id="wasi_http_types_0_2_3"></a>Import interface wasi:http/types@0.2.3</h2>
<p>This interface defines all of the types and methods for implementing
HTTP Requests and Responses, both incoming and outgoing, as well as
their headers, trailers, and bodies.</p>
<hr />
<h3>Types</h3>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><a href="#duration"><a href="#duration"><code>duration</code></a></a></p>
<p>
#### <a id="input_stream"></a>`type input-stream`
[`input-stream`](#input_stream)
<p>
#### <a id="output_stream"></a>`type output-stream`
[`output-stream`](#output_stream)
<p>
#### <a id="io_error"></a>`type io-error`
[`error`](#error)
<p>
#### <a id="pollable"></a>`type pollable`
[`pollable`](#pollable)
<p>
#### <a id="method"></a>`variant method`
<p>This type corresponds to HTTP standard Methods.</p>
<h5>Variant Cases</h5>
<ul>
<li><a id="method.get"></a><code>get</code></li>
<li><a id="method.head"></a><code>head</code></li>
<li><a id="method.post"></a><code>post</code></li>
<li><a id="method.put"></a><code>put</code></li>
<li><a id="method.delete"></a><code>delete</code></li>
<li><a id="method.connect"></a><code>connect</code></li>
<li><a id="method.options"></a><code>options</code></li>
<li><a id="method.trace"></a><code>trace</code></li>
<li><a id="method.patch"></a><code>patch</code></li>
<li><a id="method.other"></a><code>other</code>: <code>string</code></li>
</ul>
<h4><a id="scheme"></a><code>variant scheme</code></h4>
<p>This type corresponds to HTTP standard Related Schemes.</p>
<h5>Variant Cases</h5>
<ul>
<li><a id="scheme.http"></a><code>HTTP</code></li>
<li><a id="scheme.https"></a><code>HTTPS</code></li>
<li><a id="scheme.other"></a><code>other</code>: <code>string</code></li>
</ul>
<h4><a id="dns_error_payload"></a><code>record DNS-error-payload</code></h4>
<p>Defines the case payload type for <code>DNS-error</code> above:</p>
<h5>Record Fields</h5>
<ul>
<li><a id="dns_error_payload.rcode"></a><code>rcode</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="dns_error_payload.info_code"></a><code>info-code</code>: option&lt;<code>u16</code>&gt;</li>
</ul>
<h4><a id="tls_alert_received_payload"></a><code>record TLS-alert-received-payload</code></h4>
<p>Defines the case payload type for <code>TLS-alert-received</code> above:</p>
<h5>Record Fields</h5>
<ul>
<li><a id="tls_alert_received_payload.alert_id"></a><code>alert-id</code>: option&lt;<code>u8</code>&gt;</li>
<li><a id="tls_alert_received_payload.alert_message"></a><code>alert-message</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="field_size_payload"></a><code>record field-size-payload</code></h4>
<p>Defines the case payload type for <code>HTTP-response-{header,trailer}-size</code> above:</p>
<h5>Record Fields</h5>
<ul>
<li><a id="field_size_payload.field_name"></a><a href="#field_name"><code>field-name</code></a>: option&lt;<code>string</code>&gt;</li>
<li><a id="field_size_payload.field_size"></a><code>field-size</code>: option&lt;<code>u32</code>&gt;</li>
</ul>
<h4><a id="error_code"></a><code>variant error-code</code></h4>
<p>These cases are inspired by the IANA HTTP Proxy Error Types:
<a href="https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types">https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types</a></p>
<h5>Variant Cases</h5>
<ul>
<li><a id="error_code.dns_timeout"></a><code>DNS-timeout</code></li>
<li><a id="error_code.dns_error"></a><code>DNS-error</code>: <a href="#dns_error_payload"><a href="#dns_error_payload"><code>DNS-error-payload</code></a></a></li>
<li><a id="error_code.destination_not_found"></a><code>destination-not-found</code></li>
<li><a id="error_code.destination_unavailable"></a><code>destination-unavailable</code></li>
<li><a id="error_code.destination_ip_prohibited"></a><code>destination-IP-prohibited</code></li>
<li><a id="error_code.destination_ip_unroutable"></a><code>destination-IP-unroutable</code></li>
<li><a id="error_code.connection_refused"></a><code>connection-refused</code></li>
<li><a id="error_code.connection_terminated"></a><code>connection-terminated</code></li>
<li><a id="error_code.connection_timeout"></a><code>connection-timeout</code></li>
<li><a id="error_code.connection_read_timeout"></a><code>connection-read-timeout</code></li>
<li><a id="error_code.connection_write_timeout"></a><code>connection-write-timeout</code></li>
<li><a id="error_code.connection_limit_reached"></a><code>connection-limit-reached</code></li>
<li><a id="error_code.tls_protocol_error"></a><code>TLS-protocol-error</code></li>
<li><a id="error_code.tls_certificate_error"></a><code>TLS-certificate-error</code></li>
<li><a id="error_code.tls_alert_received"></a><code>TLS-alert-received</code>: <a href="#tls_alert_received_payload"><a href="#tls_alert_received_payload"><code>TLS-alert-received-payload</code></a></a></li>
<li><a id="error_code.http_request_denied"></a><code>HTTP-request-denied</code></li>
<li><a id="error_code.http_request_length_required"></a><code>HTTP-request-length-required</code></li>
<li><a id="error_code.http_request_body_size"></a><code>HTTP-request-body-size</code>: option&lt;<code>u64</code>&gt;</li>
<li><a id="error_code.http_request_method_invalid"></a><code>HTTP-request-method-invalid</code></li>
<li><a id="error_code.http_request_uri_invalid"></a><code>HTTP-request-URI-invalid</code></li>
<li><a id="error_code.http_request_uri_too_long"></a><code>HTTP-request-URI-too-long</code></li>
<li><a id="error_code.http_request_header_section_size"></a><code>HTTP-request-header-section-size</code>: option&lt;<code>u32</code>&gt;</li>
<li><a id="error_code.http_request_header_size"></a><code>HTTP-request-header-size</code>: option&lt;<a href="#field_size_payload"><a href="#field_size_payload"><code>field-size-payload</code></a></a>&gt;</li>
<li><a id="error_code.http_request_trailer_section_size"></a><code>HTTP-request-trailer-section-size</code>: option&lt;<code>u32</code>&gt;</li>
<li><a id="error_code.http_request_trailer_size"></a><code>HTTP-request-trailer-size</code>: <a href="#field_size_payload"><a href="#field_size_payload"><code>field-size-payload</code></a></a></li>
<li><a id="error_code.http_response_incomplete"></a><code>HTTP-response-incomplete</code></li>
<li><a id="error_code.http_response_header_section_size"></a><code>HTTP-response-header-section-size</code>: option&lt;<code>u32</code>&gt;</li>
<li><a id="error_code.http_response_header_size"></a><code>HTTP-response-header-size</code>: <a href="#field_size_payload"><a href="#field_size_payload"><code>field-size-payload</code></a></a></li>
<li><a id="error_code.http_response_body_size"></a><code>HTTP-response-body-size</code>: option&lt;<code>u64</code>&gt;</li>
<li><a id="error_code.http_response_trailer_section_size"></a><code>HTTP-response-trailer-section-size</code>: option&lt;<code>u32</code>&gt;</li>
<li><a id="error_code.http_response_trailer_size"></a><code>HTTP-response-trailer-size</code>: <a href="#field_size_payload"><a href="#field_size_payload"><code>field-size-payload</code></a></a></li>
<li><a id="error_code.http_response_transfer_coding"></a><code>HTTP-response-transfer-coding</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="error_code.http_response_content_coding"></a><code>HTTP-response-content-coding</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="error_code.http_response_timeout"></a><code>HTTP-response-timeout</code></li>
<li><a id="error_code.http_upgrade_failed"></a><code>HTTP-upgrade-failed</code></li>
<li><a id="error_code.http_protocol_error"></a><code>HTTP-protocol-error</code></li>
<li><a id="error_code.loop_detected"></a><code>loop-detected</code></li>
<li><a id="error_code.configuration_error"></a><code>configuration-error</code></li>
<li><a id="error_code.internal_error"></a><code>internal-error</code>: option&lt;<code>string</code>&gt;<p>This is a catch-all error for anything that doesn't fit cleanly into a
more specific case. It also includes an optional string for an
unstructured description of the error. Users should not depend on the
string for diagnosing errors, as it's not required to be consistent
between implementations.
</li>
</ul>
<h4><a id="header_error"></a><code>variant header-error</code></h4>
<p>This type enumerates the different kinds of errors that may occur when
setting or appending to a <a href="#fields"><code>fields</code></a> resource.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="header_error.invalid_syntax"></a><code>invalid-syntax</code></p>
<p>This error indicates that a `field-name` or `field-value` was
syntactically invalid when used with an operation that sets headers in a
`fields`.
</li>
<li>
<p><a id="header_error.forbidden"></a><code>forbidden</code></p>
<p>This error indicates that a forbidden `field-name` was used when trying
to set a header in a `fields`.
</li>
<li>
<p><a id="header_error.immutable"></a><code>immutable</code></p>
<p>This error indicates that the operation on the `fields` was not
permitted because the fields are immutable.
</li>
</ul>
<h4><a id="field_key"></a><code>type field-key</code></h4>
<p><code>string</code></p>
<p>Field keys are always strings.
<p>Field keys should always be treated as case insensitive by the <a href="#fields"><code>fields</code></a>
resource for the purposes of equality checking.</p>
<h1>Deprecation</h1>
<p>This type has been deprecated in favor of the <a href="#field_name"><code>field-name</code></a> type.</p>
<h4><a id="field_name"></a><code>type field-name</code></h4>
<p><a href="#field_key"><a href="#field_key"><code>field-key</code></a></a></p>
<p>Field names are always strings.
<p>Field names should always be treated as case insensitive by the <a href="#fields"><code>fields</code></a>
resource for the purposes of equality checking.</p>
<h4><a id="field_value"></a><code>type field-value</code></h4>
<p><a href="#field_value"><a href="#field_value"><code>field-value</code></a></a></p>
<p>Field values should always be ASCII strings. However, in
reality, HTTP implementations often have to interpret malformed values,
so they are provided as a list of bytes.
<h4><a id="fields"></a><code>resource fields</code></h4>
<p>This following block defines the <a href="#fields"><code>fields</code></a> resource which corresponds to
HTTP standard Fields. Fields are a common representation used for both
Headers and Trailers.</p>
<p>A <a href="#fields"><code>fields</code></a> may be mutable or immutable. A <a href="#fields"><code>fields</code></a> created using the
constructor, <code>from-list</code>, or <code>clone</code> will be mutable, but a <a href="#fields"><code>fields</code></a>
resource given by other means (including, but not limited to,
<code>incoming-request.headers</code>, <code>outgoing-request.headers</code>) might be
immutable. In an immutable fields, the <code>set</code>, <code>append</code>, and <code>delete</code>
operations will fail with <code>header-error.immutable</code>.</p>
<h4><a id="headers"></a><code>type headers</code></h4>
<p><a href="#fields"><a href="#fields"><code>fields</code></a></a></p>
<p>Headers is an alias for Fields.
<h4><a id="trailers"></a><code>type trailers</code></h4>
<p><a href="#fields"><a href="#fields"><code>fields</code></a></a></p>
<p>Trailers is an alias for Fields.
<h4><a id="incoming_request"></a><code>resource incoming-request</code></h4>
<p>Represents an incoming HTTP Request.</p>
<h4><a id="outgoing_request"></a><code>resource outgoing-request</code></h4>
<p>Represents an outgoing HTTP Request.</p>
<h4><a id="request_options"></a><code>resource request-options</code></h4>
<p>Parameters for making an HTTP Request. Each of these parameters is
currently an optional timeout applicable to the transport layer of the
HTTP protocol.</p>
<p>These timeouts are separate from any the user may use to bound a
blocking call to <code>wasi:io/poll.poll</code>.</p>
<h4><a id="response_outparam"></a><code>resource response-outparam</code></h4>
<p>Represents the ability to send an HTTP Response.</p>
<p>This resource is used by the <code>wasi:http/incoming-handler</code> interface to
allow a Response to be sent corresponding to the Request provided as the
other argument to <code>incoming-handler.handle</code>.</p>
<h4><a id="status_code"></a><code>type status-code</code></h4>
<p><code>u16</code></p>
<p>This type corresponds to the HTTP standard Status Code.
<h4><a id="incoming_response"></a><code>resource incoming-response</code></h4>
<p>Represents an incoming HTTP Response.</p>
<h4><a id="incoming_body"></a><code>resource incoming-body</code></h4>
<p>Represents an incoming HTTP Request or Response's Body.</p>
<p>A body has both its contents - a stream of bytes - and a (possibly
empty) set of trailers, indicating that the full contents of the
body have been received. This resource represents the contents as
an <a href="#input_stream"><code>input-stream</code></a> and the delivery of trailers as a <a href="#future_trailers"><code>future-trailers</code></a>,
and ensures that the user of this interface may only be consuming either
the body contents or waiting on trailers at any given time.</p>
<h4><a id="future_trailers"></a><code>resource future-trailers</code></h4>
<p>Represents a future which may eventually return trailers, or an error.</p>
<p>In the case that the incoming HTTP Request or Response did not have any
trailers, this future will resolve to the empty set of trailers once the
complete Request or Response body has been received.</p>
<h4><a id="outgoing_response"></a><code>resource outgoing-response</code></h4>
<p>Represents an outgoing HTTP Response.</p>
<h4><a id="outgoing_body"></a><code>resource outgoing-body</code></h4>
<p>Represents an outgoing HTTP Request or Response's Body.</p>
<p>A body has both its contents - a stream of bytes - and a (possibly
empty) set of trailers, inducating the full contents of the body
have been sent. This resource represents the contents as an
<a href="#output_stream"><code>output-stream</code></a> child resource, and the completion of the body (with
optional trailers) with a static function that consumes the
<a href="#outgoing_body"><code>outgoing-body</code></a> resource, and ensures that the user of this interface
may not write to the body contents after the body has been finished.</p>
<p>If the user code drops this resource, as opposed to calling the static
method <code>finish</code>, the implementation should treat the body as incomplete,
and that an error has occurred. The implementation should propagate this
error to the HTTP protocol by whatever means it has available,
including: corrupting the body on the wire, aborting the associated
Request, or sending a late status code for the Response.</p>
<h4><a id="future_incoming_response"></a><code>resource future-incoming-response</code></h4>
<p>Represents a future which may eventually return an incoming HTTP
Response, or an error.</p>
<h2>This resource is returned by the <code>wasi:http/outgoing-handler</code> interface to
provide the HTTP Response corresponding to the sent Request.</h2>
<h3>Functions</h3>
<h4><a id="http_error_code"></a><code>http-error-code: func</code></h4>
<p>Attempts to extract a http-related <a href="#error"><code>error</code></a> from the wasi:io <a href="#error"><code>error</code></a>
provided.</p>
<p>Stream operations which return
<code>wasi:io/stream/stream-error::last-operation-failed</code> have a payload of
type <code>wasi:io/error/error</code> with more information about the operation
that failed. This payload can be passed through to this function to see
if there's http-related information about the error to return.</p>
<p>Note that this function is fallible because not all io-errors are
http-related errors.</p>
<h5>Params</h5>
<ul>
<li><a id="http_error_code.err"></a><code>err</code>: borrow&lt;<a href="#io_error"><a href="#io_error"><code>io-error</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="http_error_code.0"></a> option&lt;<a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="constructor_fields"></a><code>[constructor]fields: func</code></h4>
<p>Construct an empty HTTP Fields.</p>
<p>The resulting <a href="#fields"><code>fields</code></a> is mutable.</p>
<h5>Return values</h5>
<ul>
<li><a id="constructor_fields.0"></a> own&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
</ul>
<h4><a id="static_fields_from_list"></a><code>[static]fields.from-list: func</code></h4>
<p>Construct an HTTP Fields.</p>
<p>The resulting <a href="#fields"><code>fields</code></a> is mutable.</p>
<p>The list represents each name-value pair in the Fields. Names
which have multiple values are represented by multiple entries in this
list with the same name.</p>
<p>The tuple is a pair of the field name, represented as a string, and
Value, represented as a list of bytes.</p>
<p>An error result will be returned if any <a href="#field_name"><code>field-name</code></a> or <a href="#field_value"><code>field-value</code></a> is
syntactically invalid, or if a field is forbidden.</p>
<h5>Params</h5>
<ul>
<li><a id="static_fields_from_list.entries"></a><code>entries</code>: list&lt;(<a href="#field_name"><a href="#field_name"><code>field-name</code></a></a>, <a href="#field_value"><a href="#field_value"><code>field-value</code></a></a>)&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_fields_from_list.0"></a> result&lt;own&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;, <a href="#header_error"><a href="#header_error"><code>header-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_get"></a><code>[method]fields.get: func</code></h4>
<p>Get all of the values corresponding to a name. If the name is not present
in this <a href="#fields"><code>fields</code></a> or is syntactically invalid, an empty list is returned.
However, if the name is present but empty, this is represented by a list
with one or more empty field-values present.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_get.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_get.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_get.0"></a> list&lt;<a href="#field_value"><a href="#field_value"><code>field-value</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_has"></a><code>[method]fields.has: func</code></h4>
<p>Returns <code>true</code> when the name is present in this <a href="#fields"><code>fields</code></a>. If the name is
syntactically invalid, <code>false</code> is returned.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_has.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_has.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_has.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_fields_set"></a><code>[method]fields.set: func</code></h4>
<p>Set all of the values for a name. Clears any existing values for that
name, if they have been set.</p>
<p>Fails with <code>header-error.immutable</code> if the <a href="#fields"><code>fields</code></a> are immutable.</p>
<p>Fails with <code>header-error.invalid-syntax</code> if the <a href="#field_name"><code>field-name</code></a> or any of
the <a href="#field_value"><code>field-value</code></a>s are syntactically invalid.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_set.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_set.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
<li><a id="method_fields_set.value"></a><code>value</code>: list&lt;<a href="#field_value"><a href="#field_value"><code>field-value</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_set.0"></a> result&lt;_, <a href="#header_error"><a href="#header_error"><code>header-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_delete"></a><code>[method]fields.delete: func</code></h4>
<p>Delete all values for a name. Does nothing if no values for the name
exist.</p>
<p>Fails with <code>header-error.immutable</code> if the <a href="#fields"><code>fields</code></a> are immutable.</p>
<p>Fails with <code>header-error.invalid-syntax</code> if the <a href="#field_name"><code>field-name</code></a> is
syntactically invalid.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_delete.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_delete.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_delete.0"></a> result&lt;_, <a href="#header_error"><a href="#header_error"><code>header-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_append"></a><code>[method]fields.append: func</code></h4>
<p>Append a value for a name. Does not change or delete any existing
values for that name.</p>
<p>Fails with <code>header-error.immutable</code> if the <a href="#fields"><code>fields</code></a> are immutable.</p>
<p>Fails with <code>header-error.invalid-syntax</code> if the <a href="#field_name"><code>field-name</code></a> or
<a href="#field_value"><code>field-value</code></a> are syntactically invalid.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_append.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_append.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
<li><a id="method_fields_append.value"></a><code>value</code>: <a href="#field_value"><a href="#field_value"><code>field-value</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_append.0"></a> result&lt;_, <a href="#header_error"><a href="#header_error"><code>header-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_entries"></a><code>[method]fields.entries: func</code></h4>
<p>Retrieve the full set of names and values in the Fields. Like the
constructor, the list represents each name-value pair.</p>
<p>The outer list represents each name-value pair in the Fields. Names
which have multiple values are represented by multiple entries in this
list with the same name.</p>
<p>The names and values are always returned in the original casing and in
the order in which they will be serialized for transport.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_entries.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_entries.0"></a> list&lt;(<a href="#field_name"><a href="#field_name"><code>field-name</code></a></a>, <a href="#field_value"><a href="#field_value"><code>field-value</code></a></a>)&gt;</li>
</ul>
<h4><a id="method_fields_clone"></a><code>[method]fields.clone: func</code></h4>
<p>Make a deep copy of the Fields. Equivalent in behavior to calling the
<a href="#fields"><code>fields</code></a> constructor on the return value of <code>entries</code>. The resulting
<a href="#fields"><code>fields</code></a> is mutable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_clone.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_clone.0"></a> own&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
</ul>
<h4><a id="method_incoming_request_method"></a><code>[method]incoming-request.method: func</code></h4>
<p>Returns the method of the incoming request.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_request_method.self"></a><code>self</code>: borrow&lt;<a href="#incoming_request"><a href="#incoming_request"><code>incoming-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_request_method.0"></a> <a href="#method"><a href="#method"><code>method</code></a></a></li>
</ul>
<h4><a id="method_incoming_request_path_with_query"></a><code>[method]incoming-request.path-with-query: func</code></h4>
<p>Returns the path with query parameters from the request, as a string.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_request_path_with_query.self"></a><code>self</code>: borrow&lt;<a href="#incoming_request"><a href="#incoming_request"><code>incoming-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_request_path_with_query.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="method_incoming_request_scheme"></a><code>[method]incoming-request.scheme: func</code></h4>
<p>Returns the protocol scheme from the request.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_request_scheme.self"></a><code>self</code>: borrow&lt;<a href="#incoming_request"><a href="#incoming_request"><code>incoming-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_request_scheme.0"></a> option&lt;<a href="#scheme"><a href="#scheme"><code>scheme</code></a></a>&gt;</li>
</ul>
<h4><a id="method_incoming_request_authority"></a><code>[method]incoming-request.authority: func</code></h4>
<p>Returns the authority of the Request's target URI, if present.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_request_authority.self"></a><code>self</code>: borrow&lt;<a href="#incoming_request"><a href="#incoming_request"><code>incoming-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_request_authority.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="method_incoming_request_headers"></a><code>[method]incoming-request.headers: func</code></h4>
<p>Get the <a href="#headers"><code>headers</code></a> associated with the request.</p>
<p>The returned <a href="#headers"><code>headers</code></a> resource is immutable: <code>set</code>, <code>append</code>, and
<code>delete</code> operations will fail with <code>header-error.immutable</code>.</p>
<p>The <a href="#headers"><code>headers</code></a> returned are a child resource: it must be dropped before
the parent <a href="#incoming_request"><code>incoming-request</code></a> is dropped. Dropping this
<a href="#incoming_request"><code>incoming-request</code></a> before all children are dropped will trap.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_request_headers.self"></a><code>self</code>: borrow&lt;<a href="#incoming_request"><a href="#incoming_request"><code>incoming-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_request_headers.0"></a> own&lt;<a href="#headers"><a href="#headers"><code>headers</code></a></a>&gt;</li>
</ul>
<h4><a id="method_incoming_request_consume"></a><code>[method]incoming-request.consume: func</code></h4>
<p>Gives the <a href="#incoming_body"><code>incoming-body</code></a> associated with this request. Will only
return success at most once, and subsequent calls will return error.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_request_consume.self"></a><code>self</code>: borrow&lt;<a href="#incoming_request"><a href="#incoming_request"><code>incoming-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_request_consume.0"></a> result&lt;own&lt;<a href="#incoming_body"><a href="#incoming_body"><code>incoming-body</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="constructor_outgoing_request"></a><code>[constructor]outgoing-request: func</code></h4>
<p>Construct a new <a href="#outgoing_request"><code>outgoing-request</code></a> with a default <a href="#method"><code>method</code></a> of <code>GET</code>, and
<code>none</code> values for <code>path-with-query</code>, <a href="#scheme"><code>scheme</code></a>, and <code>authority</code>.</p>
<ul>
<li><a href="#headers"><code>headers</code></a> is the HTTP Headers for the Request.</li>
</ul>
<p>It is possible to construct, or manipulate with the accessor functions
below, an <a href="#outgoing_request"><code>outgoing-request</code></a> with an invalid combination of <a href="#scheme"><code>scheme</code></a>
and <code>authority</code>, or <a href="#headers"><code>headers</code></a> which are not permitted to be sent.
It is the obligation of the <code>outgoing-handler.handle</code> implementation
to reject invalid constructions of <a href="#outgoing_request"><code>outgoing-request</code></a>.</p>
<h5>Params</h5>
<ul>
<li><a id="constructor_outgoing_request.headers"></a><a href="#headers"><code>headers</code></a>: own&lt;<a href="#headers"><a href="#headers"><code>headers</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="constructor_outgoing_request.0"></a> own&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
</ul>
<h4><a id="method_outgoing_request_body"></a><code>[method]outgoing-request.body: func</code></h4>
<p>Returns the resource corresponding to the outgoing Body for this
Request.</p>
<p>Returns success on the first call: the <a href="#outgoing_body"><code>outgoing-body</code></a> resource for
this <a href="#outgoing_request"><code>outgoing-request</code></a> can be retrieved at most once. Subsequent
calls will return error.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_request_body.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_request_body.0"></a> result&lt;own&lt;<a href="#outgoing_body"><a href="#outgoing_body"><code>outgoing-body</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_outgoing_request_method"></a><code>[method]outgoing-request.method: func</code></h4>
<p>Get the Method for the Request.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_request_method.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_request_method.0"></a> <a href="#method"><a href="#method"><code>method</code></a></a></li>
</ul>
<h4><a id="method_outgoing_request_set_method"></a><code>[method]outgoing-request.set-method: func</code></h4>
<p>Set the Method for the Request. Fails if the string present in a
<code>method.other</code> argument is not a syntactically valid method.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_request_set_method.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
<li><a id="method_outgoing_request_set_method.method"></a><a href="#method"><code>method</code></a>: <a href="#method"><a href="#method"><code>method</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_request_set_method.0"></a> result</li>
</ul>
<h4><a id="method_outgoing_request_path_with_query"></a><code>[method]outgoing-request.path-with-query: func</code></h4>
<p>Get the combination of the HTTP Path and Query for the Request.
When <code>none</code>, this represents an empty Path and empty Query.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_request_path_with_query.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_request_path_with_query.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="method_outgoing_request_set_path_with_query"></a><code>[method]outgoing-request.set-path-with-query: func</code></h4>
<p>Set the combination of the HTTP Path and Query for the Request.
When <code>none</code>, this represents an empty Path and empty Query. Fails is the
string given is not a syntactically valid path and query uri component.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_request_set_path_with_query.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
<li><a id="method_outgoing_request_set_path_with_query.path_with_query"></a><code>path-with-query</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_request_set_path_with_query.0"></a> result</li>
</ul>
<h4><a id="method_outgoing_request_scheme"></a><code>[method]outgoing-request.scheme: func</code></h4>
<p>Get the HTTP Related Scheme for the Request. When <code>none</code>, the
implementation may choose an appropriate default scheme.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_request_scheme.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_request_scheme.0"></a> option&lt;<a href="#scheme"><a href="#scheme"><code>scheme</code></a></a>&gt;</li>
</ul>
<h4><a id="method_outgoing_request_set_scheme"></a><code>[method]outgoing-request.set-scheme: func</code></h4>
<p>Set the HTTP Related Scheme for the Request. When <code>none</code>, the
implementation may choose an appropriate default scheme. Fails if the
string given is not a syntactically valid uri scheme.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_request_set_scheme.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
<li><a id="method_outgoing_request_set_scheme.scheme"></a><a href="#scheme"><code>scheme</code></a>: option&lt;<a href="#scheme"><a href="#scheme"><code>scheme</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_request_set_scheme.0"></a> result</li>
</ul>
<h4><a id="method_outgoing_request_authority"></a><code>[method]outgoing-request.authority: func</code></h4>
<p>Get the authority of the Request's target URI. A value of <code>none</code> may be used
with Related Schemes which do not require an authority. The HTTP and
HTTPS schemes always require an authority.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_request_authority.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_request_authority.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="method_outgoing_request_set_authority"></a><code>[method]outgoing-request.set-authority: func</code></h4>
<p>Set the authority of the Request's target URI. A value of <code>none</code> may be used
with Related Schemes which do not require an authority. The HTTP and
HTTPS schemes always require an authority. Fails if the string given is
not a syntactically valid URI authority.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_request_set_authority.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
<li><a id="method_outgoing_request_set_authority.authority"></a><code>authority</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_request_set_authority.0"></a> result</li>
</ul>
<h4><a id="method_outgoing_request_headers"></a><code>[method]outgoing-request.headers: func</code></h4>
<p>Get the headers associated with the Request.</p>
<p>The returned <a href="#headers"><code>headers</code></a> resource is immutable: <code>set</code>, <code>append</code>, and
<code>delete</code> operations will fail with <code>header-error.immutable</code>.</p>
<p>This headers resource is a child: it must be dropped before the parent
<a href="#outgoing_request"><code>outgoing-request</code></a> is dropped, or its ownership is transferred to
another component by e.g. <code>outgoing-handler.handle</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_request_headers.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_request_headers.0"></a> own&lt;<a href="#headers"><a href="#headers"><code>headers</code></a></a>&gt;</li>
</ul>
<h4><a id="constructor_request_options"></a><code>[constructor]request-options: func</code></h4>
<p>Construct a default <a href="#request_options"><code>request-options</code></a> value.</p>
<h5>Return values</h5>
<ul>
<li><a id="constructor_request_options.0"></a> own&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_connect_timeout"></a><code>[method]request-options.connect-timeout: func</code></h4>
<p>The timeout for the initial connect to the HTTP Server.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_connect_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_connect_timeout.0"></a> option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_set_connect_timeout"></a><code>[method]request-options.set-connect-timeout: func</code></h4>
<p>Set the timeout for the initial connect to the HTTP Server. An error
return value indicates that this timeout is not supported.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_set_connect_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
<li><a id="method_request_options_set_connect_timeout.duration"></a><a href="#duration"><code>duration</code></a>: option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_set_connect_timeout.0"></a> result</li>
</ul>
<h4><a id="method_request_options_first_byte_timeout"></a><code>[method]request-options.first-byte-timeout: func</code></h4>
<p>The timeout for receiving the first byte of the Response body.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_first_byte_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_first_byte_timeout.0"></a> option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_set_first_byte_timeout"></a><code>[method]request-options.set-first-byte-timeout: func</code></h4>
<p>Set the timeout for receiving the first byte of the Response body. An
error return value indicates that this timeout is not supported.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_set_first_byte_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
<li><a id="method_request_options_set_first_byte_timeout.duration"></a><a href="#duration"><code>duration</code></a>: option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_set_first_byte_timeout.0"></a> result</li>
</ul>
<h4><a id="method_request_options_between_bytes_timeout"></a><code>[method]request-options.between-bytes-timeout: func</code></h4>
<p>The timeout for receiving subsequent chunks of bytes in the Response
body stream.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_between_bytes_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_between_bytes_timeout.0"></a> option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_set_between_bytes_timeout"></a><code>[method]request-options.set-between-bytes-timeout: func</code></h4>
<p>Set the timeout for receiving subsequent chunks of bytes in the Response
body stream. An error return value indicates that this timeout is not
supported.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_set_between_bytes_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
<li><a id="method_request_options_set_between_bytes_timeout.duration"></a><a href="#duration"><code>duration</code></a>: option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_set_between_bytes_timeout.0"></a> result</li>
</ul>
<h4><a id="static_response_outparam_set"></a><code>[static]response-outparam.set: func</code></h4>
<p>Set the value of the <a href="#response_outparam"><code>response-outparam</code></a> to either send a response,
or indicate an error.</p>
<p>This method consumes the <a href="#response_outparam"><code>response-outparam</code></a> to ensure that it is
called at most once. If it is never called, the implementation
will respond with an error.</p>
<p>The user may provide an <a href="#error"><code>error</code></a> to <code>response</code> to allow the
implementation determine how to respond with an HTTP error response.</p>
<h5>Params</h5>
<ul>
<li><a id="static_response_outparam_set.param"></a><code>param</code>: own&lt;<a href="#response_outparam"><a href="#response_outparam"><code>response-outparam</code></a></a>&gt;</li>
<li><a id="static_response_outparam_set.response"></a><code>response</code>: result&lt;own&lt;<a href="#outgoing_response"><a href="#outgoing_response"><code>outgoing-response</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_incoming_response_status"></a><code>[method]incoming-response.status: func</code></h4>
<p>Returns the status code from the incoming response.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_response_status.self"></a><code>self</code>: borrow&lt;<a href="#incoming_response"><a href="#incoming_response"><code>incoming-response</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_response_status.0"></a> <a href="#status_code"><a href="#status_code"><code>status-code</code></a></a></li>
</ul>
<h4><a id="method_incoming_response_headers"></a><code>[method]incoming-response.headers: func</code></h4>
<p>Returns the headers from the incoming response.</p>
<p>The returned <a href="#headers"><code>headers</code></a> resource is immutable: <code>set</code>, <code>append</code>, and
<code>delete</code> operations will fail with <code>header-error.immutable</code>.</p>
<p>This headers resource is a child: it must be dropped before the parent
<a href="#incoming_response"><code>incoming-response</code></a> is dropped.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_response_headers.self"></a><code>self</code>: borrow&lt;<a href="#incoming_response"><a href="#incoming_response"><code>incoming-response</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_response_headers.0"></a> own&lt;<a href="#headers"><a href="#headers"><code>headers</code></a></a>&gt;</li>
</ul>
<h4><a id="method_incoming_response_consume"></a><code>[method]incoming-response.consume: func</code></h4>
<p>Returns the incoming body. May be called at most once. Returns error
if called additional times.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_response_consume.self"></a><code>self</code>: borrow&lt;<a href="#incoming_response"><a href="#incoming_response"><code>incoming-response</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_response_consume.0"></a> result&lt;own&lt;<a href="#incoming_body"><a href="#incoming_body"><code>incoming-body</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_incoming_body_stream"></a><code>[method]incoming-body.stream: func</code></h4>
<p>Returns the contents of the body, as a stream of bytes.</p>
<p>Returns success on first call: the stream representing the contents
can be retrieved at most once. Subsequent calls will return error.</p>
<p>The returned <a href="#input_stream"><code>input-stream</code></a> resource is a child: it must be dropped
before the parent <a href="#incoming_body"><code>incoming-body</code></a> is dropped, or consumed by
<code>incoming-body.finish</code>.</p>
<p>This invariant ensures that the implementation can determine whether
the user is consuming the contents of the body, waiting on the
<a href="#future_trailers"><code>future-trailers</code></a> to be ready, or neither. This allows for network
backpressure is to be applied when the user is consuming the body,
and for that backpressure to not inhibit delivery of the trailers if
the user does not read the entire body.</p>
<h5>Params</h5>
<ul>
<li><a id="method_incoming_body_stream.self"></a><code>self</code>: borrow&lt;<a href="#incoming_body"><a href="#incoming_body"><code>incoming-body</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_incoming_body_stream.0"></a> result&lt;own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="static_incoming_body_finish"></a><code>[static]incoming-body.finish: func</code></h4>
<p>Takes ownership of <a href="#incoming_body"><code>incoming-body</code></a>, and returns a <a href="#future_trailers"><code>future-trailers</code></a>.
This function will trap if the <a href="#input_stream"><code>input-stream</code></a> child is still alive.</p>
<h5>Params</h5>
<ul>
<li><a id="static_incoming_body_finish.this"></a><code>this</code>: own&lt;<a href="#incoming_body"><a href="#incoming_body"><code>incoming-body</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_incoming_body_finish.0"></a> own&lt;<a href="#future_trailers"><a href="#future_trailers"><code>future-trailers</code></a></a>&gt;</li>
</ul>
<h4><a id="method_future_trailers_subscribe"></a><code>[method]future-trailers.subscribe: func</code></h4>
<p>Returns a pollable which becomes ready when either the trailers have
been received, or an error has occurred. When this pollable is ready,
the <code>get</code> method will return <code>some</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_future_trailers_subscribe.self"></a><code>self</code>: borrow&lt;<a href="#future_trailers"><a href="#future_trailers"><code>future-trailers</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_future_trailers_subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_future_trailers_get"></a><code>[method]future-trailers.get: func</code></h4>
<p>Returns the contents of the trailers, or an error which occurred,
once the future is ready.</p>
<p>The outer <code>option</code> represents future readiness. Users can wait on this
<code>option</code> to become <code>some</code> using the <code>subscribe</code> method.</p>
<p>The outer <code>result</code> is used to retrieve the trailers or error at most
once. It will be success on the first call in which the outer option
is <code>some</code>, and error on subsequent calls.</p>
<p>The inner <code>result</code> represents that either the HTTP Request or Response
body, as well as any trailers, were received successfully, or that an
error occurred receiving them. The optional <a href="#trailers"><code>trailers</code></a> indicates whether
or not trailers were present in the body.</p>
<p>When some <a href="#trailers"><code>trailers</code></a> are returned by this method, the <a href="#trailers"><code>trailers</code></a>
resource is immutable, and a child. Use of the <code>set</code>, <code>append</code>, or
<code>delete</code> methods will return an error, and the resource must be
dropped before the parent <a href="#future_trailers"><code>future-trailers</code></a> is dropped.</p>
<h5>Params</h5>
<ul>
<li><a id="method_future_trailers_get.self"></a><code>self</code>: borrow&lt;<a href="#future_trailers"><a href="#future_trailers"><code>future-trailers</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_future_trailers_get.0"></a> option&lt;result&lt;result&lt;option&lt;own&lt;<a href="#trailers"><a href="#trailers"><code>trailers</code></a></a>&gt;&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;&gt;</li>
</ul>
<h4><a id="constructor_outgoing_response"></a><code>[constructor]outgoing-response: func</code></h4>
<p>Construct an <a href="#outgoing_response"><code>outgoing-response</code></a>, with a default <a href="#status_code"><code>status-code</code></a> of <code>200</code>.
If a different <a href="#status_code"><code>status-code</code></a> is needed, it must be set via the
<code>set-status-code</code> method.</p>
<ul>
<li><a href="#headers"><code>headers</code></a> is the HTTP Headers for the Response.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="constructor_outgoing_response.headers"></a><a href="#headers"><code>headers</code></a>: own&lt;<a href="#headers"><a href="#headers"><code>headers</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="constructor_outgoing_response.0"></a> own&lt;<a href="#outgoing_response"><a href="#outgoing_response"><code>outgoing-response</code></a></a>&gt;</li>
</ul>
<h4><a id="method_outgoing_response_status_code"></a><code>[method]outgoing-response.status-code: func</code></h4>
<p>Get the HTTP Status Code for the Response.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_response_status_code.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_response"><a href="#outgoing_response"><code>outgoing-response</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_response_status_code.0"></a> <a href="#status_code"><a href="#status_code"><code>status-code</code></a></a></li>
</ul>
<h4><a id="method_outgoing_response_set_status_code"></a><code>[method]outgoing-response.set-status-code: func</code></h4>
<p>Set the HTTP Status Code for the Response. Fails if the status-code
given is not a valid http status code.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_response_set_status_code.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_response"><a href="#outgoing_response"><code>outgoing-response</code></a></a>&gt;</li>
<li><a id="method_outgoing_response_set_status_code.status_code"></a><a href="#status_code"><code>status-code</code></a>: <a href="#status_code"><a href="#status_code"><code>status-code</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_response_set_status_code.0"></a> result</li>
</ul>
<h4><a id="method_outgoing_response_headers"></a><code>[method]outgoing-response.headers: func</code></h4>
<p>Get the headers associated with the Request.</p>
<p>The returned <a href="#headers"><code>headers</code></a> resource is immutable: <code>set</code>, <code>append</code>, and
<code>delete</code> operations will fail with <code>header-error.immutable</code>.</p>
<p>This headers resource is a child: it must be dropped before the parent
<a href="#outgoing_request"><code>outgoing-request</code></a> is dropped, or its ownership is transferred to
another component by e.g. <code>outgoing-handler.handle</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_response_headers.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_response"><a href="#outgoing_response"><code>outgoing-response</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_response_headers.0"></a> own&lt;<a href="#headers"><a href="#headers"><code>headers</code></a></a>&gt;</li>
</ul>
<h4><a id="method_outgoing_response_body"></a><code>[method]outgoing-response.body: func</code></h4>
<p>Returns the resource corresponding to the outgoing Body for this Response.</p>
<p>Returns success on the first call: the <a href="#outgoing_body"><code>outgoing-body</code></a> resource for
this <a href="#outgoing_response"><code>outgoing-response</code></a> can be retrieved at most once. Subsequent
calls will return error.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_response_body.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_response"><a href="#outgoing_response"><code>outgoing-response</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_response_body.0"></a> result&lt;own&lt;<a href="#outgoing_body"><a href="#outgoing_body"><code>outgoing-body</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_outgoing_body_write"></a><code>[method]outgoing-body.write: func</code></h4>
<p>Returns a stream for writing the body contents.</p>
<p>The returned <a href="#output_stream"><code>output-stream</code></a> is a child resource: it must be dropped
before the parent <a href="#outgoing_body"><code>outgoing-body</code></a> resource is dropped (or finished),
otherwise the <a href="#outgoing_body"><code>outgoing-body</code></a> drop or <code>finish</code> will trap.</p>
<p>Returns success on the first call: the <a href="#output_stream"><code>output-stream</code></a> resource for
this <a href="#outgoing_body"><code>outgoing-body</code></a> may be retrieved at most once. Subsequent calls
will return error.</p>
<h5>Params</h5>
<ul>
<li><a id="method_outgoing_body_write.self"></a><code>self</code>: borrow&lt;<a href="#outgoing_body"><a href="#outgoing_body"><code>outgoing-body</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_outgoing_body_write.0"></a> result&lt;own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="static_outgoing_body_finish"></a><code>[static]outgoing-body.finish: func</code></h4>
<p>Finalize an outgoing body, optionally providing trailers. This must be
called to signal that the response is complete. If the <a href="#outgoing_body"><code>outgoing-body</code></a>
is dropped without calling <code>outgoing-body.finalize</code>, the implementation
should treat the body as corrupted.</p>
<p>Fails if the body's <a href="#outgoing_request"><code>outgoing-request</code></a> or <a href="#outgoing_response"><code>outgoing-response</code></a> was
constructed with a Content-Length header, and the contents written
to the body (via <code>write</code>) does not match the value given in the
Content-Length.</p>
<h5>Params</h5>
<ul>
<li><a id="static_outgoing_body_finish.this"></a><code>this</code>: own&lt;<a href="#outgoing_body"><a href="#outgoing_body"><code>outgoing-body</code></a></a>&gt;</li>
<li><a id="static_outgoing_body_finish.trailers"></a><a href="#trailers"><code>trailers</code></a>: option&lt;own&lt;<a href="#trailers"><a href="#trailers"><code>trailers</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_outgoing_body_finish.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_future_incoming_response_subscribe"></a><code>[method]future-incoming-response.subscribe: func</code></h4>
<p>Returns a pollable which becomes ready when either the Response has
been received, or an error has occurred. When this pollable is ready,
the <code>get</code> method will return <code>some</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_future_incoming_response_subscribe.self"></a><code>self</code>: borrow&lt;<a href="#future_incoming_response"><a href="#future_incoming_response"><code>future-incoming-response</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_future_incoming_response_subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_future_incoming_response_get"></a><code>[method]future-incoming-response.get: func</code></h4>
<p>Returns the incoming HTTP Response, or an error, once one is ready.</p>
<p>The outer <code>option</code> represents future readiness. Users can wait on this
<code>option</code> to become <code>some</code> using the <code>subscribe</code> method.</p>
<p>The outer <code>result</code> is used to retrieve the response or error at most
once. It will be success on the first call in which the outer option
is <code>some</code>, and error on subsequent calls.</p>
<p>The inner <code>result</code> represents that either the incoming HTTP Response
status and headers have received successfully, or that an error
occurred. Errors may also occur while consuming the response body,
but those will be reported by the <a href="#incoming_body"><code>incoming-body</code></a> and its
<a href="#output_stream"><code>output-stream</code></a> child.</p>
<h5>Params</h5>
<ul>
<li><a id="method_future_incoming_response_get.self"></a><code>self</code>: borrow&lt;<a href="#future_incoming_response"><a href="#future_incoming_response"><code>future-incoming-response</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_future_incoming_response_get.0"></a> option&lt;result&lt;result&lt;own&lt;<a href="#incoming_response"><a href="#incoming_response"><code>incoming-response</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;&gt;</li>
</ul>
<h2><a id="wasi_http_outgoing_handler_0_2_3"></a>Import interface wasi:http/outgoing-handler@0.2.3</h2>
<p>This interface defines a handler of outgoing HTTP Requests. It should be
imported by components which wish to make HTTP Requests.</p>
<hr />
<h3>Types</h3>
<h4><a id="outgoing_request"></a><code>type outgoing-request</code></h4>
<p><a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a></p>
<p>
#### <a id="request_options"></a>`type request-options`
[`request-options`](#request_options)
<p>
#### <a id="future_incoming_response"></a>`type future-incoming-response`
[`future-incoming-response`](#future_incoming_response)
<p>
#### <a id="error_code"></a>`type error-code`
[`error-code`](#error_code)
<p>
----
<h3>Functions</h3>
<h4><a id="handle"></a><code>handle: func</code></h4>
<p>This function is invoked with an outgoing HTTP Request, and it returns
a resource <a href="#future_incoming_response"><code>future-incoming-response</code></a> which represents an HTTP Response
which may arrive in the future.</p>
<p>The <code>options</code> argument accepts optional parameters for the HTTP
protocol's transport layer.</p>
<p>This function may return an error if the <a href="#outgoing_request"><code>outgoing-request</code></a> is invalid
or not allowed to be made. Otherwise, protocol errors are reported
through the <a href="#future_incoming_response"><code>future-incoming-response</code></a>.</p>
<h5>Params</h5>
<ul>
<li><a id="handle.request"></a><code>request</code>: own&lt;<a href="#outgoing_request"><a href="#outgoing_request"><code>outgoing-request</code></a></a>&gt;</li>
<li><a id="handle.options"></a><code>options</code>: option&lt;own&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="handle.0"></a> result&lt;own&lt;<a href="#future_incoming_response"><a href="#future_incoming_response"><code>future-incoming-response</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_http_incoming_handler_0_2_3"></a>Export interface wasi:http/incoming-handler@0.2.3</h2>
<hr />
<h3>Types</h3>
<h4><a id="incoming_request"></a><code>type incoming-request</code></h4>
<p><a href="#incoming_request"><a href="#incoming_request"><code>incoming-request</code></a></a></p>
<p>
#### <a id="response_outparam"></a>`type response-outparam`
[`response-outparam`](#response_outparam)
<p>
----
<h3>Functions</h3>
<h4><a id="handle"></a><code>handle: func</code></h4>
<p>This function is invoked with an incoming HTTP Request, and a resource
<a href="#response_outparam"><code>response-outparam</code></a> which provides the capability to reply with an HTTP
Response. The response is sent by calling the <code>response-outparam.set</code>
method, which allows execution to continue after the response has been
sent. This enables both streaming to the response body, and performing other
work.</p>
<p>The implementor of this function must write a response to the
<a href="#response_outparam"><code>response-outparam</code></a> before returning, or else the caller will respond
with an error on its behalf.</p>
<h5>Params</h5>
<ul>
<li><a id="handle.request"></a><code>request</code>: own&lt;<a href="#incoming_request"><a href="#incoming_request"><code>incoming-request</code></a></a>&gt;</li>
<li><a id="handle.response_out"></a><code>response-out</code>: own&lt;<a href="#response_outparam"><a href="#response_outparam"><code>response-outparam</code></a></a>&gt;</li>
</ul>
