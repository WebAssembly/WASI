<h1><a name="imports">World imports</a></h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi:io_poll"><code>wasi:io/poll</code></a></li>
<li>interface <a href="#wasi:io_streams"><code>wasi:io/streams</code></a></li>
<li>interface <a href="#wasi:clocks_wall_clock"><code>wasi:clocks/wall-clock</code></a></li>
<li>interface <a href="#wasi:filesystem_types"><code>wasi:filesystem/types</code></a></li>
<li>interface <a href="#wasi:filesystem_preopens"><code>wasi:filesystem/preopens</code></a></li>
</ul>
</li>
</ul>
<h2><a name="wasi:io_poll">Import interface wasi:io/poll</a></h2>
<p>A poll API intended to let users wait for I/O events on multiple handles
at once.</p>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>resource pollable</code></a></h4>
<hr />
<h3>Functions</h3>
<h4><a name="poll_list"><code>poll-list: func</code></a></h4>
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
<li><a name="poll_list.in"><code>in</code></a>: list&lt;borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="poll_list.0"></a> list&lt;<code>u32</code>&gt;</li>
</ul>
<h4><a name="poll_one"><code>poll-one: func</code></a></h4>
<p>Poll for completion on a single pollable.</p>
<p>This function is similar to <a href="#poll_list"><code>poll-list</code></a>, but operates on only a single
pollable. When it returns, the handle is ready for I/O.</p>
<h5>Params</h5>
<ul>
<li><a name="poll_one.in"><code>in</code></a>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
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
#### <a name="stream_status">`enum stream-status`</a>
<p>Streams provide a sequence of data and then end; once they end, they
no longer provide any further data.</p>
<p>For example, a stream reading from a file ends when the stream reaches
the end of the file. For another example, a stream reading from a
socket ends when the socket is closed.</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="stream_status.open"><code>open</code></a></p>
<p>The stream is open and may produce further data.
</li>
<li>
<p><a name="stream_status.ended"><code>ended</code></a></p>
<p>When reading, this indicates that the stream will not produce
further data.
When writing, this indicates that the stream will no longer be read.
Further writes are still permitted.
</li>
</ul>
<h4><a name="input_stream"><code>resource input-stream</code></a></h4>
<h4><a name="write_error"><code>enum write-error</code></a></h4>
<p>An error for output-stream operations.</p>
<p>Contrary to input-streams, a closed output-stream is reported using
an error.</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="write_error.last_operation_failed"><code>last-operation-failed</code></a></p>
<p>The last operation (a write or flush) failed before completion.
</li>
<li>
<p><a name="write_error.closed"><code>closed</code></a></p>
<p>The stream is closed: no more input will be accepted by the
stream. A closed output-stream will return this error on all
future operations.
</li>
</ul>
<h4><a name="output_stream"><code>resource output-stream</code></a></h4>
<hr />
<h3>Functions</h3>
<h4><a name="method_input_stream.read"><code>[method]input-stream.read: func</code></a></h4>
<p>Perform a non-blocking read from the stream.</p>
<p>This function returns a list of bytes containing the data that was
read, along with a <a href="#stream_status"><code>stream-status</code></a> which, indicates whether further
reads are expected to produce data. The returned list will contain up to
<code>len</code> bytes; it may return fewer than requested, but not more. An
empty list and <code>stream-status:open</code> indicates no more data is
available at this time, and that the pollable given by <code>subscribe</code>
will be ready when more data is available.</p>
<p>Once a stream has reached the end, subsequent calls to <code>read</code> or
<code>skip</code> will always report <code>stream-status:ended</code> rather than producing more
data.</p>
<p>When the caller gives a <code>len</code> of 0, it represents a request to read 0
bytes. This read should  always succeed and return an empty list and
the current <a href="#stream_status"><code>stream-status</code></a>.</p>
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
<li><a name="method_input_stream.read.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h4><a name="method_input_stream.blocking_read"><code>[method]input-stream.blocking-read: func</code></a></h4>
<p>Read bytes from a stream, after blocking until at least one byte can
be read. Except for blocking, identical to <code>read</code>.</p>
<h5>Params</h5>
<ul>
<li><a name="method_input_stream.blocking_read.self"><code>self</code></a>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a name="method_input_stream.blocking_read.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_input_stream.blocking_read.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h4><a name="method_input_stream.skip"><code>[method]input-stream.skip: func</code></a></h4>
<p>Skip bytes from a stream.</p>
<p>This is similar to the <code>read</code> function, but avoids copying the
bytes into the instance.</p>
<p>Once a stream has reached the end, subsequent calls to read or
<code>skip</code> will always report end-of-stream rather than producing more
data.</p>
<p>This function returns the number of bytes skipped, along with a
<a href="#stream_status"><code>stream-status</code></a> indicating whether the end of the stream was
reached. The returned value will be at most <code>len</code>; it may be less.</p>
<h5>Params</h5>
<ul>
<li><a name="method_input_stream.skip.self"><code>self</code></a>: borrow&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a name="method_input_stream.skip.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_input_stream.skip.0"></a> result&lt;(<code>u64</code>, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
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
<li><a name="method_input_stream.blocking_skip.0"></a> result&lt;(<code>u64</code>, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
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
<li><a name="method_output_stream.check_write.0"></a> result&lt;<code>u64</code>, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
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
<li><a name="method_output_stream.write.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
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
<li><a name="method_output_stream.blocking_write_and_flush.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
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
<li><a name="method_output_stream.flush.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
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
<li><a name="method_output_stream.blocking_flush.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
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
<li><a name="method_output_stream.write_zeroes.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
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
<li><a name="method_output_stream.blocking_write_zeroes_and_flush.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_output_stream.splice"><code>[method]output-stream.splice: func</code></a></h4>
<p>Read from one stream and write to another.</p>
<p>This function returns the number of bytes transferred; it may be less
than <code>len</code>.</p>
<p>Unlike other I/O functions, this function blocks until all the data
read from the input stream has been written to the output stream.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.splice.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.splice.src"><code>src</code></a>: own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.splice.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.splice.0"></a> result&lt;(<code>u64</code>, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h4><a name="method_output_stream.blocking_splice"><code>[method]output-stream.blocking-splice: func</code></a></h4>
<p>Read from one stream and write to another, with blocking.</p>
<p>This is similar to <code>splice</code>, except that it blocks until at least
one byte can be read.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.blocking_splice.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.blocking_splice.src"><code>src</code></a>: own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.blocking_splice.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.blocking_splice.0"></a> result&lt;(<code>u64</code>, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h4><a name="method_output_stream.forward"><code>[method]output-stream.forward: func</code></a></h4>
<p>Forward the entire contents of an input stream to an output stream.</p>
<p>This function repeatedly reads from the input stream and writes
the data to the output stream, until the end of the input stream
is reached, or an error is encountered.</p>
<p>Unlike other I/O functions, this function blocks until the end
of the input stream is seen and all the data has been written to
the output stream.</p>
<p>This function returns the number of bytes transferred, and the status of
the output stream.</p>
<h5>Params</h5>
<ul>
<li><a name="method_output_stream.forward.self"><code>self</code></a>: borrow&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;</li>
<li><a name="method_output_stream.forward.src"><code>src</code></a>: own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_output_stream.forward.0"></a> result&lt;(<code>u64</code>, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h2><a name="wasi:clocks_wall_clock">Import interface wasi:clocks/wall-clock</a></h2>
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
<h4><a name="datetime"><code>record datetime</code></a></h4>
<p>A time and date in seconds plus nanoseconds.</p>
<h5>Record Fields</h5>
<ul>
<li><a name="datetime.seconds"><code>seconds</code></a>: <code>u64</code></li>
<li><a name="datetime.nanoseconds"><code>nanoseconds</code></a>: <code>u32</code></li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a name="now"><code>now: func</code></a></h4>
<p>Read the current value of the clock.</p>
<p>This clock is not monotonic, therefore calling this function repeatedly
will not necessarily produce a sequence of non-decreasing values.</p>
<p>The returned timestamps represent the number of seconds since
1970-01-01T00:00:00Z, also known as <a href="https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16">POSIX's Seconds Since the Epoch</a>,
also known as <a href="https://en.wikipedia.org/wiki/Unix_time">Unix Time</a>.</p>
<p>The nanoseconds field of the output is always less than 1000000000.</p>
<h5>Return values</h5>
<ul>
<li><a name="now.0"></a> <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></li>
</ul>
<h4><a name="resolution"><code>resolution: func</code></a></h4>
<p>Query the resolution of the clock.</p>
<p>The nanoseconds field of the output is always less than 1000000000.</p>
<h5>Return values</h5>
<ul>
<li><a name="resolution.0"></a> <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></li>
</ul>
<h2><a name="wasi:filesystem_types">Import interface wasi:filesystem/types</a></h2>
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
<h4><a name="input_stream"><code>type input-stream</code></a></h4>
<p><a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></p>
<p>
#### <a name="output_stream">`type output-stream`</a>
[`output-stream`](#output_stream)
<p>
#### <a name="datetime">`type datetime`</a>
[`datetime`](#datetime)
<p>
#### <a name="filesize">`type filesize`</a>
`u64`
<p>File size or length of a region within a file.
<h4><a name="descriptor_type"><code>enum descriptor-type</code></a></h4>
<p>The type of a filesystem object referenced by a descriptor.</p>
<p>Note: This was called <code>filetype</code> in earlier versions of WASI.</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="descriptor_type.unknown"><code>unknown</code></a></p>
<p>The type of the descriptor or file is unknown or is different from
any of the other types specified.
</li>
<li>
<p><a name="descriptor_type.block_device"><code>block-device</code></a></p>
<p>The descriptor refers to a block device inode.
</li>
<li>
<p><a name="descriptor_type.character_device"><code>character-device</code></a></p>
<p>The descriptor refers to a character device inode.
</li>
<li>
<p><a name="descriptor_type.directory"><code>directory</code></a></p>
<p>The descriptor refers to a directory inode.
</li>
<li>
<p><a name="descriptor_type.fifo"><code>fifo</code></a></p>
<p>The descriptor refers to a named pipe.
</li>
<li>
<p><a name="descriptor_type.symbolic_link"><code>symbolic-link</code></a></p>
<p>The file refers to a symbolic link inode.
</li>
<li>
<p><a name="descriptor_type.regular_file"><code>regular-file</code></a></p>
<p>The descriptor refers to a regular file inode.
</li>
<li>
<p><a name="descriptor_type.socket"><code>socket</code></a></p>
<p>The descriptor refers to a socket.
</li>
</ul>
<h4><a name="descriptor_flags"><code>flags descriptor-flags</code></a></h4>
<p>Descriptor flags.</p>
<p>Note: This was called <code>fdflags</code> in earlier versions of WASI.</p>
<h5>Flags members</h5>
<ul>
<li>
<p><a name="descriptor_flags.read"><code>read</code></a>: </p>
<p>Read mode: Data can be read.
</li>
<li>
<p><a name="descriptor_flags.write"><code>write</code></a>: </p>
<p>Write mode: Data can be written to.
</li>
<li>
<p><a name="descriptor_flags.file_integrity_sync"><code>file-integrity-sync</code></a>: </p>
<p>Request that writes be performed according to synchronized I/O file
integrity completion. The data stored in the file and the file's
metadata are synchronized. This is similar to `O_SYNC` in POSIX.
<p>The precise semantics of this operation have not yet been defined for
WASI. At this time, it should be interpreted as a request, and not a
requirement.</p>
</li>
<li>
<p><a name="descriptor_flags.data_integrity_sync"><code>data-integrity-sync</code></a>: </p>
<p>Request that writes be performed according to synchronized I/O data
integrity completion. Only the data stored in the file is
synchronized. This is similar to `O_DSYNC` in POSIX.
<p>The precise semantics of this operation have not yet been defined for
WASI. At this time, it should be interpreted as a request, and not a
requirement.</p>
</li>
<li>
<p><a name="descriptor_flags.requested_write_sync"><code>requested-write-sync</code></a>: </p>
<p>Requests that reads be performed at the same level of integrety
requested for writes. This is similar to `O_RSYNC` in POSIX.
<p>The precise semantics of this operation have not yet been defined for
WASI. At this time, it should be interpreted as a request, and not a
requirement.</p>
</li>
<li>
<p><a name="descriptor_flags.mutate_directory"><code>mutate-directory</code></a>: </p>
<p>Mutating directories mode: Directory contents may be mutated.
<p>When this flag is unset on a descriptor, operations using the
descriptor which would create, rename, delete, modify the data or
metadata of filesystem objects, or obtain another handle which
would permit any of those, shall fail with <a href="#error_code.read_only"><code>error-code::read-only</code></a> if
they would otherwise succeed.</p>
<p>This may only be set on directories.</p>
</li>
</ul>
<h4><a name="path_flags"><code>flags path-flags</code></a></h4>
<p>Flags determining the method of how paths are resolved.</p>
<h5>Flags members</h5>
<ul>
<li><a name="path_flags.symlink_follow"><code>symlink-follow</code></a>: <p>As long as the resolved path corresponds to a symbolic link, it is
expanded.
</li>
</ul>
<h4><a name="open_flags"><code>flags open-flags</code></a></h4>
<p>Open flags used by <code>open-at</code>.</p>
<h5>Flags members</h5>
<ul>
<li>
<p><a name="open_flags.create"><code>create</code></a>: </p>
<p>Create file if it does not exist, similar to `O_CREAT` in POSIX.
</li>
<li>
<p><a name="open_flags.directory"><code>directory</code></a>: </p>
<p>Fail if not a directory, similar to `O_DIRECTORY` in POSIX.
</li>
<li>
<p><a name="open_flags.exclusive"><code>exclusive</code></a>: </p>
<p>Fail if file already exists, similar to `O_EXCL` in POSIX.
</li>
<li>
<p><a name="open_flags.truncate"><code>truncate</code></a>: </p>
<p>Truncate file to size 0, similar to `O_TRUNC` in POSIX.
</li>
</ul>
<h4><a name="modes"><code>flags modes</code></a></h4>
<p>Permissions mode used by <code>open-at</code>, <code>change-file-permissions-at</code>, and
similar.</p>
<h5>Flags members</h5>
<ul>
<li>
<p><a name="modes.readable"><code>readable</code></a>: </p>
<p>True if the resource is considered readable by the containing
filesystem.
</li>
<li>
<p><a name="modes.writable"><code>writable</code></a>: </p>
<p>True if the resource is considered writable by the containing
filesystem.
</li>
<li>
<p><a name="modes.executable"><code>executable</code></a>: </p>
<p>True if the resource is considered executable by the containing
filesystem. This does not apply to directories.
</li>
</ul>
<h4><a name="access_type"><code>variant access-type</code></a></h4>
<p>Access type used by <code>access-at</code>.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a name="access_type.access"><code>access</code></a>: <a href="#modes"><a href="#modes"><code>modes</code></a></a></p>
<p>Test for readability, writeability, or executability.
</li>
<li>
<p><a name="access_type.exists"><code>exists</code></a></p>
<p>Test whether the path exists.
</li>
</ul>
<h4><a name="link_count"><code>type link-count</code></a></h4>
<p><code>u64</code></p>
<p>Number of hard links to an inode.
<h4><a name="descriptor_stat"><code>record descriptor-stat</code></a></h4>
<p>File attributes.</p>
<p>Note: This was called <code>filestat</code> in earlier versions of WASI.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a name="descriptor_stat.type"><code>type</code></a>: <a href="#descriptor_type"><a href="#descriptor_type"><code>descriptor-type</code></a></a></p>
<p>File type.
</li>
<li>
<p><a name="descriptor_stat.link_count"><a href="#link_count"><code>link-count</code></a></a>: <a href="#link_count"><a href="#link_count"><code>link-count</code></a></a></p>
<p>Number of hard links to the file.
</li>
<li>
<p><a name="descriptor_stat.size"><code>size</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></p>
<p>For regular files, the file size in bytes. For symbolic links, the
length in bytes of the pathname contained in the symbolic link.
</li>
<li>
<p><a name="descriptor_stat.data_access_timestamp"><code>data-access-timestamp</code></a>: option&lt;<a href="#datetime"><a href="#datetime"><code>datetime</code></a></a>&gt;</p>
<p>Last data access timestamp.
<p>If the <code>option</code> is none, the platform doesn't maintain an access
timestamp for this file.</p>
</li>
<li>
<p><a name="descriptor_stat.data_modification_timestamp"><code>data-modification-timestamp</code></a>: option&lt;<a href="#datetime"><a href="#datetime"><code>datetime</code></a></a>&gt;</p>
<p>Last data modification timestamp.
<p>If the <code>option</code> is none, the platform doesn't maintain a
modification timestamp for this file.</p>
</li>
<li>
<p><a name="descriptor_stat.status_change_timestamp"><code>status-change-timestamp</code></a>: option&lt;<a href="#datetime"><a href="#datetime"><code>datetime</code></a></a>&gt;</p>
<p>Last file status-change timestamp.
<p>If the <code>option</code> is none, the platform doesn't maintain a
status-change timestamp for this file.</p>
</li>
</ul>
<h4><a name="new_timestamp"><code>variant new-timestamp</code></a></h4>
<p>When setting a timestamp, this gives the value to set it to.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a name="new_timestamp.no_change"><code>no-change</code></a></p>
<p>Leave the timestamp set to its previous value.
</li>
<li>
<p><a name="new_timestamp.now"><a href="#now"><code>now</code></a></a></p>
<p>Set the timestamp to the current time of the system clock associated
with the filesystem.
</li>
<li>
<p><a name="new_timestamp.timestamp"><code>timestamp</code></a>: <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></p>
<p>Set the timestamp to the given value.
</li>
</ul>
<h4><a name="directory_entry"><code>record directory-entry</code></a></h4>
<p>A directory entry.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a name="directory_entry.type"><code>type</code></a>: <a href="#descriptor_type"><a href="#descriptor_type"><code>descriptor-type</code></a></a></p>
<p>The type of the file referred to by this directory entry.
</li>
<li>
<p><a name="directory_entry.name"><code>name</code></a>: <code>string</code></p>
<p>The name of the object.
</li>
</ul>
<h4><a name="error_code"><code>enum error-code</code></a></h4>
<p>Error codes returned by functions, similar to <code>errno</code> in POSIX.
Not all of these error codes are returned by the functions provided by this
API; some are used in higher-level library layers, and others are provided
merely for alignment with POSIX.</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="error_code.access"><code>access</code></a></p>
<p>Permission denied, similar to `EACCES` in POSIX.
</li>
<li>
<p><a name="error_code.would_block"><code>would-block</code></a></p>
<p>Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX.
</li>
<li>
<p><a name="error_code.already"><code>already</code></a></p>
<p>Connection already in progress, similar to `EALREADY` in POSIX.
</li>
<li>
<p><a name="error_code.bad_descriptor"><code>bad-descriptor</code></a></p>
<p>Bad descriptor, similar to `EBADF` in POSIX.
</li>
<li>
<p><a name="error_code.busy"><code>busy</code></a></p>
<p>Device or resource busy, similar to `EBUSY` in POSIX.
</li>
<li>
<p><a name="error_code.deadlock"><code>deadlock</code></a></p>
<p>Resource deadlock would occur, similar to `EDEADLK` in POSIX.
</li>
<li>
<p><a name="error_code.quota"><code>quota</code></a></p>
<p>Storage quota exceeded, similar to `EDQUOT` in POSIX.
</li>
<li>
<p><a name="error_code.exist"><code>exist</code></a></p>
<p>File exists, similar to `EEXIST` in POSIX.
</li>
<li>
<p><a name="error_code.file_too_large"><code>file-too-large</code></a></p>
<p>File too large, similar to `EFBIG` in POSIX.
</li>
<li>
<p><a name="error_code.illegal_byte_sequence"><code>illegal-byte-sequence</code></a></p>
<p>Illegal byte sequence, similar to `EILSEQ` in POSIX.
</li>
<li>
<p><a name="error_code.in_progress"><code>in-progress</code></a></p>
<p>Operation in progress, similar to `EINPROGRESS` in POSIX.
</li>
<li>
<p><a name="error_code.interrupted"><code>interrupted</code></a></p>
<p>Interrupted function, similar to `EINTR` in POSIX.
</li>
<li>
<p><a name="error_code.invalid"><code>invalid</code></a></p>
<p>Invalid argument, similar to `EINVAL` in POSIX.
</li>
<li>
<p><a name="error_code.io"><code>io</code></a></p>
<p>I/O error, similar to `EIO` in POSIX.
</li>
<li>
<p><a name="error_code.is_directory"><code>is-directory</code></a></p>
<p>Is a directory, similar to `EISDIR` in POSIX.
</li>
<li>
<p><a name="error_code.loop"><code>loop</code></a></p>
<p>Too many levels of symbolic links, similar to `ELOOP` in POSIX.
</li>
<li>
<p><a name="error_code.too_many_links"><code>too-many-links</code></a></p>
<p>Too many links, similar to `EMLINK` in POSIX.
</li>
<li>
<p><a name="error_code.message_size"><code>message-size</code></a></p>
<p>Message too large, similar to `EMSGSIZE` in POSIX.
</li>
<li>
<p><a name="error_code.name_too_long"><code>name-too-long</code></a></p>
<p>Filename too long, similar to `ENAMETOOLONG` in POSIX.
</li>
<li>
<p><a name="error_code.no_device"><code>no-device</code></a></p>
<p>No such device, similar to `ENODEV` in POSIX.
</li>
<li>
<p><a name="error_code.no_entry"><code>no-entry</code></a></p>
<p>No such file or directory, similar to `ENOENT` in POSIX.
</li>
<li>
<p><a name="error_code.no_lock"><code>no-lock</code></a></p>
<p>No locks available, similar to `ENOLCK` in POSIX.
</li>
<li>
<p><a name="error_code.insufficient_memory"><code>insufficient-memory</code></a></p>
<p>Not enough space, similar to `ENOMEM` in POSIX.
</li>
<li>
<p><a name="error_code.insufficient_space"><code>insufficient-space</code></a></p>
<p>No space left on device, similar to `ENOSPC` in POSIX.
</li>
<li>
<p><a name="error_code.not_directory"><code>not-directory</code></a></p>
<p>Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX.
</li>
<li>
<p><a name="error_code.not_empty"><code>not-empty</code></a></p>
<p>Directory not empty, similar to `ENOTEMPTY` in POSIX.
</li>
<li>
<p><a name="error_code.not_recoverable"><code>not-recoverable</code></a></p>
<p>State not recoverable, similar to `ENOTRECOVERABLE` in POSIX.
</li>
<li>
<p><a name="error_code.unsupported"><code>unsupported</code></a></p>
<p>Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX.
</li>
<li>
<p><a name="error_code.no_tty"><code>no-tty</code></a></p>
<p>Inappropriate I/O control operation, similar to `ENOTTY` in POSIX.
</li>
<li>
<p><a name="error_code.no_such_device"><code>no-such-device</code></a></p>
<p>No such device or address, similar to `ENXIO` in POSIX.
</li>
<li>
<p><a name="error_code.overflow"><code>overflow</code></a></p>
<p>Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX.
</li>
<li>
<p><a name="error_code.not_permitted"><code>not-permitted</code></a></p>
<p>Operation not permitted, similar to `EPERM` in POSIX.
</li>
<li>
<p><a name="error_code.pipe"><code>pipe</code></a></p>
<p>Broken pipe, similar to `EPIPE` in POSIX.
</li>
<li>
<p><a name="error_code.read_only"><code>read-only</code></a></p>
<p>Read-only file system, similar to `EROFS` in POSIX.
</li>
<li>
<p><a name="error_code.invalid_seek"><code>invalid-seek</code></a></p>
<p>Invalid seek, similar to `ESPIPE` in POSIX.
</li>
<li>
<p><a name="error_code.text_file_busy"><code>text-file-busy</code></a></p>
<p>Text file busy, similar to `ETXTBSY` in POSIX.
</li>
<li>
<p><a name="error_code.cross_device"><code>cross-device</code></a></p>
<p>Cross-device link, similar to `EXDEV` in POSIX.
</li>
</ul>
<h4><a name="advice"><code>enum advice</code></a></h4>
<p>File or memory access pattern advisory information.</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="advice.normal"><code>normal</code></a></p>
<p>The application has no advice to give on its behavior with respect
to the specified data.
</li>
<li>
<p><a name="advice.sequential"><code>sequential</code></a></p>
<p>The application expects to access the specified data sequentially
from lower offsets to higher offsets.
</li>
<li>
<p><a name="advice.random"><code>random</code></a></p>
<p>The application expects to access the specified data in a random
order.
</li>
<li>
<p><a name="advice.will_need"><code>will-need</code></a></p>
<p>The application expects to access the specified data in the near
future.
</li>
<li>
<p><a name="advice.dont_need"><code>dont-need</code></a></p>
<p>The application expects that it will not access the specified data
in the near future.
</li>
<li>
<p><a name="advice.no_reuse"><code>no-reuse</code></a></p>
<p>The application expects to access the specified data once and then
not reuse it thereafter.
</li>
</ul>
<h4><a name="metadata_hash_value"><code>record metadata-hash-value</code></a></h4>
<p>A 128-bit hash value, split into parts because wasm doesn't have a
128-bit integer type.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a name="metadata_hash_value.lower"><code>lower</code></a>: <code>u64</code></p>
<p>64 bits of a 128-bit hash value.
</li>
<li>
<p><a name="metadata_hash_value.upper"><code>upper</code></a>: <code>u64</code></p>
<p>Another 64 bits of a 128-bit hash value.
</li>
</ul>
<h4><a name="descriptor"><code>resource descriptor</code></a></h4>
<h4><a name="directory_entry_stream"><code>resource directory-entry-stream</code></a></h4>
<hr />
<h3>Functions</h3>
<h4><a name="method_descriptor.read_via_stream"><code>[method]descriptor.read-via-stream: func</code></a></h4>
<p>Return a stream for reading from a file, if available.</p>
<p>May fail with an error-code describing why the file cannot be read.</p>
<p>Multiple read, write, and append streams may be active on the same open
file and they do not interfere with each other.</p>
<p>Note: This allows using <code>read-stream</code>, which is similar to <code>read</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.read_via_stream.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.read_via_stream.offset"><code>offset</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.read_via_stream.0"></a> result&lt;own&lt;<a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.write_via_stream"><code>[method]descriptor.write-via-stream: func</code></a></h4>
<p>Return a stream for writing to a file, if available.</p>
<p>May fail with an error-code describing why the file cannot be written.</p>
<p>Note: This allows using <code>write-stream</code>, which is similar to <code>write</code> in
POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.write_via_stream.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.write_via_stream.offset"><code>offset</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.write_via_stream.0"></a> result&lt;own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.append_via_stream"><code>[method]descriptor.append-via-stream: func</code></a></h4>
<p>Return a stream for appending to a file, if available.</p>
<p>May fail with an error-code describing why the file cannot be appended.</p>
<p>Note: This allows using <code>write-stream</code>, which is similar to <code>write</code> with
<code>O_APPEND</code> in in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.append_via_stream.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.append_via_stream.0"></a> result&lt;own&lt;<a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.advise"><code>[method]descriptor.advise: func</code></a></h4>
<p>Provide file advisory information on a descriptor.</p>
<p>This is similar to <code>posix_fadvise</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.advise.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.advise.offset"><code>offset</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
<li><a name="method_descriptor.advise.length"><code>length</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
<li><a name="method_descriptor.advise.advice"><a href="#advice"><code>advice</code></a></a>: <a href="#advice"><a href="#advice"><code>advice</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.advise.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.sync_data"><code>[method]descriptor.sync-data: func</code></a></h4>
<p>Synchronize the data of a file to disk.</p>
<p>This function succeeds with no effect if the file descriptor is not
opened for writing.</p>
<p>Note: This is similar to <code>fdatasync</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.sync_data.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.sync_data.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.get_flags"><code>[method]descriptor.get-flags: func</code></a></h4>
<p>Get flags associated with a descriptor.</p>
<p>Note: This returns similar flags to <code>fcntl(fd, F_GETFL)</code> in POSIX.</p>
<p>Note: This returns the value that was the <code>fs_flags</code> value returned
from <code>fdstat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.get_flags.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.get_flags.0"></a> result&lt;<a href="#descriptor_flags"><a href="#descriptor_flags"><code>descriptor-flags</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.get_type"><code>[method]descriptor.get-type: func</code></a></h4>
<p>Get the dynamic type of a descriptor.</p>
<p>Note: This returns the same value as the <code>type</code> field of the <code>fd-stat</code>
returned by <code>stat</code>, <code>stat-at</code> and similar.</p>
<p>Note: This returns similar flags to the <code>st_mode &amp; S_IFMT</code> value provided
by <code>fstat</code> in POSIX.</p>
<p>Note: This returns the value that was the <code>fs_filetype</code> value returned
from <code>fdstat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.get_type.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.get_type.0"></a> result&lt;<a href="#descriptor_type"><a href="#descriptor_type"><code>descriptor-type</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.set_size"><code>[method]descriptor.set-size: func</code></a></h4>
<p>Adjust the size of an open file. If this increases the file's size, the
extra bytes are filled with zeros.</p>
<p>Note: This was called <code>fd_filestat_set_size</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.set_size.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.set_size.size"><code>size</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.set_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.set_times"><code>[method]descriptor.set-times: func</code></a></h4>
<p>Adjust the timestamps of an open file or directory.</p>
<p>Note: This is similar to <code>futimens</code> in POSIX.</p>
<p>Note: This was called <code>fd_filestat_set_times</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.set_times.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.set_times.data_access_timestamp"><code>data-access-timestamp</code></a>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
<li><a name="method_descriptor.set_times.data_modification_timestamp"><code>data-modification-timestamp</code></a>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.set_times.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.read"><code>[method]descriptor.read: func</code></a></h4>
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
<li><a name="method_descriptor.read.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.read.length"><code>length</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
<li><a name="method_descriptor.read.offset"><code>offset</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.read.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <code>bool</code>), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.write"><code>[method]descriptor.write: func</code></a></h4>
<p>Write to a descriptor, without using and updating the descriptor's offset.</p>
<p>It is valid to write past the end of a file; the file is extended to the
extent of the write, with bytes between the previous end and the start of
the write set to zero.</p>
<p>In the future, this may change to take a <code>stream&lt;u8, error-code&gt;</code>.</p>
<p>Note: This is similar to <code>pwrite</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.write.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.write.buffer"><code>buffer</code></a>: list&lt;<code>u8</code>&gt;</li>
<li><a name="method_descriptor.write.offset"><code>offset</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.write.0"></a> result&lt;<a href="#filesize"><a href="#filesize"><code>filesize</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.read_directory"><code>[method]descriptor.read-directory: func</code></a></h4>
<p>Read directory entries from a directory.</p>
<p>On filesystems where directories contain entries referring to themselves
and their parents, often named <code>.</code> and <code>..</code> respectively, these entries
are omitted.</p>
<p>This always returns a new stream which starts at the beginning of the
directory. Multiple streams may be active on the same directory, and they
do not interfere with each other.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.read_directory.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.read_directory.0"></a> result&lt;own&lt;<a href="#directory_entry_stream"><a href="#directory_entry_stream"><code>directory-entry-stream</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.sync"><code>[method]descriptor.sync: func</code></a></h4>
<p>Synchronize the data and metadata of a file to disk.</p>
<p>This function succeeds with no effect if the file descriptor is not
opened for writing.</p>
<p>Note: This is similar to <code>fsync</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.sync.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.sync.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.create_directory_at"><code>[method]descriptor.create-directory-at: func</code></a></h4>
<p>Create a directory.</p>
<p>Note: This is similar to <code>mkdirat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.create_directory_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.create_directory_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.create_directory_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.stat"><code>[method]descriptor.stat: func</code></a></h4>
<p>Return the attributes of an open file or directory.</p>
<p>Note: This is similar to <code>fstat</code> in POSIX, except that it does not return
device and inode information. For testing whether two descriptors refer to
the same underlying filesystem object, use <code>is-same-object</code>. To obtain
additional data that can be used do determine whether a file has been
modified, use <code>metadata-hash</code>.</p>
<p>Note: This was called <code>fd_filestat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.stat.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.stat.0"></a> result&lt;<a href="#descriptor_stat"><a href="#descriptor_stat"><code>descriptor-stat</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.stat_at"><code>[method]descriptor.stat-at: func</code></a></h4>
<p>Return the attributes of a file or directory.</p>
<p>Note: This is similar to <code>fstatat</code> in POSIX, except that it does not
return device and inode information. See the <code>stat</code> description for a
discussion of alternatives.</p>
<p>Note: This was called <code>path_filestat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.stat_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.stat_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="method_descriptor.stat_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.stat_at.0"></a> result&lt;<a href="#descriptor_stat"><a href="#descriptor_stat"><code>descriptor-stat</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.set_times_at"><code>[method]descriptor.set-times-at: func</code></a></h4>
<p>Adjust the timestamps of a file or directory.</p>
<p>Note: This is similar to <code>utimensat</code> in POSIX.</p>
<p>Note: This was called <code>path_filestat_set_times</code> in earlier versions of
WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.set_times_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.set_times_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="method_descriptor.set_times_at.path"><code>path</code></a>: <code>string</code></li>
<li><a name="method_descriptor.set_times_at.data_access_timestamp"><code>data-access-timestamp</code></a>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
<li><a name="method_descriptor.set_times_at.data_modification_timestamp"><code>data-modification-timestamp</code></a>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.set_times_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.link_at"><code>[method]descriptor.link-at: func</code></a></h4>
<p>Create a hard link.</p>
<p>Note: This is similar to <code>linkat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.link_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.link_at.old_path_flags"><code>old-path-flags</code></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="method_descriptor.link_at.old_path"><code>old-path</code></a>: <code>string</code></li>
<li><a name="method_descriptor.link_at.new_descriptor"><code>new-descriptor</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.link_at.new_path"><code>new-path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.link_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.open_at"><code>[method]descriptor.open-at: func</code></a></h4>
<p>Open a file or directory.</p>
<p>The returned descriptor is not guaranteed to be the lowest-numbered
descriptor not currently open/ it is randomized to prevent applications
from depending on making assumptions about indexes, since this is
error-prone in multi-threaded contexts. The returned descriptor is
guaranteed to be less than 2**31.</p>
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
<li><a name="method_descriptor.open_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.open_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="method_descriptor.open_at.path"><code>path</code></a>: <code>string</code></li>
<li><a name="method_descriptor.open_at.open_flags"><a href="#open_flags"><code>open-flags</code></a></a>: <a href="#open_flags"><a href="#open_flags"><code>open-flags</code></a></a></li>
<li><a name="method_descriptor.open_at.flags"><code>flags</code></a>: <a href="#descriptor_flags"><a href="#descriptor_flags"><code>descriptor-flags</code></a></a></li>
<li><a name="method_descriptor.open_at.modes"><a href="#modes"><code>modes</code></a></a>: <a href="#modes"><a href="#modes"><code>modes</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.open_at.0"></a> result&lt;own&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.readlink_at"><code>[method]descriptor.readlink-at: func</code></a></h4>
<p>Read the contents of a symbolic link.</p>
<p>If the contents contain an absolute or rooted path in the underlying
filesystem, this function fails with <a href="#error_code.not_permitted"><code>error-code::not-permitted</code></a>.</p>
<p>Note: This is similar to <code>readlinkat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.readlink_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.readlink_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.readlink_at.0"></a> result&lt;<code>string</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.remove_directory_at"><code>[method]descriptor.remove-directory-at: func</code></a></h4>
<p>Remove a directory.</p>
<p>Return <a href="#error_code.not_empty"><code>error-code::not-empty</code></a> if the directory is not empty.</p>
<p>Note: This is similar to <code>unlinkat(fd, path, AT_REMOVEDIR)</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.remove_directory_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.remove_directory_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.remove_directory_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.rename_at"><code>[method]descriptor.rename-at: func</code></a></h4>
<p>Rename a filesystem object.</p>
<p>Note: This is similar to <code>renameat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.rename_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.rename_at.old_path"><code>old-path</code></a>: <code>string</code></li>
<li><a name="method_descriptor.rename_at.new_descriptor"><code>new-descriptor</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.rename_at.new_path"><code>new-path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.rename_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.symlink_at"><code>[method]descriptor.symlink-at: func</code></a></h4>
<p>Create a symbolic link (also known as a &quot;symlink&quot;).</p>
<p>If <code>old-path</code> starts with <code>/</code>, the function fails with
<a href="#error_code.not_permitted"><code>error-code::not-permitted</code></a>.</p>
<p>Note: This is similar to <code>symlinkat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.symlink_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.symlink_at.old_path"><code>old-path</code></a>: <code>string</code></li>
<li><a name="method_descriptor.symlink_at.new_path"><code>new-path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.symlink_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.access_at"><code>[method]descriptor.access-at: func</code></a></h4>
<p>Check accessibility of a filesystem path.</p>
<p>Check whether the given filesystem path names an object which is
readable, writable, or executable, or whether it exists.</p>
<p>This does not a guarantee that subsequent accesses will succeed, as
filesystem permissions may be modified asynchronously by external
entities.</p>
<p>Note: This is similar to <code>faccessat</code> with the <code>AT_EACCESS</code> flag in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.access_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.access_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="method_descriptor.access_at.path"><code>path</code></a>: <code>string</code></li>
<li><a name="method_descriptor.access_at.type"><code>type</code></a>: <a href="#access_type"><a href="#access_type"><code>access-type</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.access_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.unlink_file_at"><code>[method]descriptor.unlink-file-at: func</code></a></h4>
<p>Unlink a filesystem object that is not a directory.</p>
<p>Return <a href="#error_code.is_directory"><code>error-code::is-directory</code></a> if the path refers to a directory.
Note: This is similar to <code>unlinkat(fd, path, 0)</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.unlink_file_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.unlink_file_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.unlink_file_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.change_file_permissions_at"><code>[method]descriptor.change-file-permissions-at: func</code></a></h4>
<p>Change the permissions of a filesystem object that is not a directory.</p>
<p>Note that the ultimate meanings of these permissions is
filesystem-specific.</p>
<p>Note: This is similar to <code>fchmodat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.change_file_permissions_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.change_file_permissions_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="method_descriptor.change_file_permissions_at.path"><code>path</code></a>: <code>string</code></li>
<li><a name="method_descriptor.change_file_permissions_at.modes"><a href="#modes"><code>modes</code></a></a>: <a href="#modes"><a href="#modes"><code>modes</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.change_file_permissions_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.change_directory_permissions_at"><code>[method]descriptor.change-directory-permissions-at: func</code></a></h4>
<p>Change the permissions of a directory.</p>
<p>Note that the ultimate meanings of these permissions is
filesystem-specific.</p>
<p>Unlike in POSIX, the <code>executable</code> flag is not reinterpreted as a &quot;search&quot;
flag. <code>read</code> on a directory implies readability and searchability, and
<code>execute</code> is not valid for directories.</p>
<p>Note: This is similar to <code>fchmodat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.change_directory_permissions_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.change_directory_permissions_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="method_descriptor.change_directory_permissions_at.path"><code>path</code></a>: <code>string</code></li>
<li><a name="method_descriptor.change_directory_permissions_at.modes"><a href="#modes"><code>modes</code></a></a>: <a href="#modes"><a href="#modes"><code>modes</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.change_directory_permissions_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.lock_shared"><code>[method]descriptor.lock-shared: func</code></a></h4>
<p>Request a shared advisory lock for an open file.</p>
<p>This requests a <em>shared</em> lock; more than one shared lock can be held for
a file at the same time.</p>
<p>If the open file has an exclusive lock, this function downgrades the lock
to a shared lock. If it has a shared lock, this function has no effect.</p>
<p>This requests an <em>advisory</em> lock, meaning that the file could be accessed
by other programs that don't hold the lock.</p>
<p>It is unspecified how shared locks interact with locks acquired by
non-WASI programs.</p>
<p>This function blocks until the lock can be acquired.</p>
<p>Not all filesystems support locking; on filesystems which don't support
locking, this function returns <a href="#error_code.unsupported"><code>error-code::unsupported</code></a>.</p>
<p>Note: This is similar to <code>flock(fd, LOCK_SH)</code> in Unix.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.lock_shared.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.lock_shared.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.lock_exclusive"><code>[method]descriptor.lock-exclusive: func</code></a></h4>
<p>Request an exclusive advisory lock for an open file.</p>
<p>This requests an <em>exclusive</em> lock; no other locks may be held for the
file while an exclusive lock is held.</p>
<p>If the open file has a shared lock and there are no exclusive locks held
for the file, this function upgrades the lock to an exclusive lock. If the
open file already has an exclusive lock, this function has no effect.</p>
<p>This requests an <em>advisory</em> lock, meaning that the file could be accessed
by other programs that don't hold the lock.</p>
<p>It is unspecified whether this function succeeds if the file descriptor
is not opened for writing. It is unspecified how exclusive locks interact
with locks acquired by non-WASI programs.</p>
<p>This function blocks until the lock can be acquired.</p>
<p>Not all filesystems support locking; on filesystems which don't support
locking, this function returns <a href="#error_code.unsupported"><code>error-code::unsupported</code></a>.</p>
<p>Note: This is similar to <code>flock(fd, LOCK_EX)</code> in Unix.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.lock_exclusive.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.lock_exclusive.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.try_lock_shared"><code>[method]descriptor.try-lock-shared: func</code></a></h4>
<p>Request a shared advisory lock for an open file.</p>
<p>This requests a <em>shared</em> lock; more than one shared lock can be held for
a file at the same time.</p>
<p>If the open file has an exclusive lock, this function downgrades the lock
to a shared lock. If it has a shared lock, this function has no effect.</p>
<p>This requests an <em>advisory</em> lock, meaning that the file could be accessed
by other programs that don't hold the lock.</p>
<p>It is unspecified how shared locks interact with locks acquired by
non-WASI programs.</p>
<p>This function returns <a href="#error_code.would_block"><code>error-code::would-block</code></a> if the lock cannot be
acquired.</p>
<p>Not all filesystems support locking; on filesystems which don't support
locking, this function returns <a href="#error_code.unsupported"><code>error-code::unsupported</code></a>.</p>
<p>Note: This is similar to <code>flock(fd, LOCK_SH | LOCK_NB)</code> in Unix.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.try_lock_shared.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.try_lock_shared.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.try_lock_exclusive"><code>[method]descriptor.try-lock-exclusive: func</code></a></h4>
<p>Request an exclusive advisory lock for an open file.</p>
<p>This requests an <em>exclusive</em> lock; no other locks may be held for the
file while an exclusive lock is held.</p>
<p>If the open file has a shared lock and there are no exclusive locks held
for the file, this function upgrades the lock to an exclusive lock. If the
open file already has an exclusive lock, this function has no effect.</p>
<p>This requests an <em>advisory</em> lock, meaning that the file could be accessed
by other programs that don't hold the lock.</p>
<p>It is unspecified whether this function succeeds if the file descriptor
is not opened for writing. It is unspecified how exclusive locks interact
with locks acquired by non-WASI programs.</p>
<p>This function returns <a href="#error_code.would_block"><code>error-code::would-block</code></a> if the lock cannot be
acquired.</p>
<p>Not all filesystems support locking; on filesystems which don't support
locking, this function returns <a href="#error_code.unsupported"><code>error-code::unsupported</code></a>.</p>
<p>Note: This is similar to <code>flock(fd, LOCK_EX | LOCK_NB)</code> in Unix.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.try_lock_exclusive.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.try_lock_exclusive.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.unlock"><code>[method]descriptor.unlock: func</code></a></h4>
<p>Release a shared or exclusive lock on an open file.</p>
<p>Note: This is similar to <code>flock(fd, LOCK_UN)</code> in Unix.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.unlock.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.unlock.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.is_same_object"><code>[method]descriptor.is-same-object: func</code></a></h4>
<p>Test whether two descriptors refer to the same filesystem object.</p>
<p>In POSIX, this corresponds to testing whether the two descriptors have the
same device (<code>st_dev</code>) and inode (<code>st_ino</code> or <code>d_ino</code>) numbers.
wasi-filesystem does not expose device and inode numbers, so this function
may be used instead.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.is_same_object.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.is_same_object.other"><code>other</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.is_same_object.0"></a> <code>bool</code></li>
</ul>
<h4><a name="method_descriptor.metadata_hash"><code>[method]descriptor.metadata-hash: func</code></a></h4>
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
<li><a name="method_descriptor.metadata_hash.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.metadata_hash.0"></a> result&lt;<a href="#metadata_hash_value"><a href="#metadata_hash_value"><code>metadata-hash-value</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_descriptor.metadata_hash_at"><code>[method]descriptor.metadata-hash-at: func</code></a></h4>
<p>Return a hash of the metadata associated with a filesystem object referred
to by a directory descriptor and a relative path.</p>
<p>This performs the same hash computation as <code>metadata-hash</code>.</p>
<h5>Params</h5>
<ul>
<li><a name="method_descriptor.metadata_hash_at.self"><code>self</code></a>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a name="method_descriptor.metadata_hash_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="method_descriptor.metadata_hash_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_descriptor.metadata_hash_at.0"></a> result&lt;<a href="#metadata_hash_value"><a href="#metadata_hash_value"><code>metadata-hash-value</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="method_directory_entry_stream.read_directory_entry"><code>[method]directory-entry-stream.read-directory-entry: func</code></a></h4>
<p>Read a single directory entry from a <a href="#directory_entry_stream"><code>directory-entry-stream</code></a>.</p>
<h5>Params</h5>
<ul>
<li><a name="method_directory_entry_stream.read_directory_entry.self"><code>self</code></a>: borrow&lt;<a href="#directory_entry_stream"><a href="#directory_entry_stream"><code>directory-entry-stream</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_directory_entry_stream.read_directory_entry.0"></a> result&lt;option&lt;<a href="#directory_entry"><a href="#directory_entry"><code>directory-entry</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:filesystem_preopens">Import interface wasi:filesystem/preopens</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="descriptor"><code>type descriptor</code></a></h4>
<p><a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></p>
<p>
----
<h3>Functions</h3>
<h4><a name="get_directories"><code>get-directories: func</code></a></h4>
<p>Return the set of preopened directories, and their path.</p>
<h5>Return values</h5>
<ul>
<li><a name="get_directories.0"></a> list&lt;(own&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;, <code>string</code>)&gt;</li>
</ul>
