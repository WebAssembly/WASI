<h1><a name="example_world">World example-world</a></h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi:poll_poll"><code>wasi:poll/poll</code></a></li>
<li>interface <a href="#wasi:io_streams"><code>wasi:io/streams</code></a></li>
</ul>
</li>
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
<p>This function takes a list of pollables, which identify I/O sources of
interest, and waits until one or more of the events is ready for I/O.</p>
<p>The result <code>list&lt;bool&gt;</code> is the same length as the argument
<code>list&lt;pollable&gt;</code>, and indicates the readiness of each corresponding
element in that list, with true indicating ready. A single call can
return multiple true elements.</p>
<p>A timeout can be implemented by adding a pollable from the
wasi-clocks API to the list.</p>
<p>This function does not return a <code>result</code>; polling in itself does not
do any I/O so it doesn't fail. If any of the I/O sources identified by
the pollables has an error, it is indicated by marking the source as
ready in the <code>list&lt;bool&gt;</code>.</p>
<p>The &quot;oneoff&quot; in the name refers to the fact that this function must do a
linear scan through the entire list of subscriptions, which may be
inefficient if the number is large and the same subscriptions are used
many times. In the future, this is expected to be obsoleted by the
component model async proposal, which will include a scalable waiting
facility.</p>
<h5>Params</h5>
<ul>
<li><a name="poll_oneoff.in"><code>in</code></a>: list&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="poll_oneoff.0"></a> list&lt;<code>bool</code>&gt;</li>
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
#### <a name="write_error">`enum write-error`</a>
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
<h4><a name="stream_status"><code>enum stream-status</code></a></h4>
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
<h4><a name="output_stream"><code>type output-stream</code></a></h4>
<p><code>u32</code></p>
<p>An output bytestream. In the future, this will be replaced by handle
types.
<p><a href="#output_stream"><code>output-stream</code></a>s are <em>non-blocking</em> to the extent practical on
underlying platforms. Except where specified otherwise, I/O operations also
always return promptly, after the number of bytes that can be written
promptly, which could even be zero. To wait for the stream to be ready to
accept data, the <a href="#subscribe_to_output_stream"><code>subscribe-to-output-stream</code></a> function to obtain a
<a href="#pollable"><code>pollable</code></a> which can be polled for using <code>wasi:poll</code>.</p>
<p>And at present, it is a <code>u32</code> instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.</p>
<p>This <a href="https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources">represents a resource</a>.</p>
<h4><a name="input_stream"><code>type input-stream</code></a></h4>
<p><code>u32</code></p>
<p>An input bytestream. In the future, this will be replaced by handle
types.
<p><a href="#input_stream"><code>input-stream</code></a>s are <em>non-blocking</em> to the extent practical on underlying
platforms. I/O operations always return promptly; if fewer bytes are
promptly available than requested, they return the number of bytes promptly
available, which could even be zero. To wait for data to be available,
use the <a href="#subscribe_to_input_stream"><code>subscribe-to-input-stream</code></a> function to obtain a <a href="#pollable"><code>pollable</code></a> which
can be polled for using <code>wasi:poll/poll.poll_oneoff</code>.</p>
<p>And at present, it is a <code>u32</code> instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.</p>
<p>This <a href="https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources">represents a resource</a>.</p>
<hr />
<h3>Functions</h3>
<h4><a name="read"><code>read: func</code></a></h4>
<p>Perform a non-blocking read from the stream.</p>
<p>This function returns a list of bytes containing the data that was
read, along with a <a href="#stream_status"><code>stream-status</code></a> which, indicates whether further
reads are expected to produce data. The returned list will contain up to
<code>len</code> bytes; it may return fewer than requested, but not more. An
empty list and <code>stream-status:open</code> indicates no more data is
available at this time, and that the pollable given by
<a href="#subscribe_to_input_stream"><code>subscribe-to-input-stream</code></a> will be ready when more data is available.</p>
<p>Once a stream has reached the end, subsequent calls to <a href="#read"><code>read</code></a> or
<a href="#skip"><code>skip</code></a> will always report <code>stream-status:ended</code> rather than producing more
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
<li><a name="read.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
<li><a name="read.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="read.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h4><a name="blocking_read"><code>blocking-read: func</code></a></h4>
<p>Read bytes from a stream, after blocking until at least one byte can
be read. Except for blocking, identical to <a href="#read"><code>read</code></a>.</p>
<h5>Params</h5>
<ul>
<li><a name="blocking_read.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
<li><a name="blocking_read.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="blocking_read.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h4><a name="skip"><code>skip: func</code></a></h4>
<p>Skip bytes from a stream.</p>
<p>This is similar to the <a href="#read"><code>read</code></a> function, but avoids copying the
bytes into the instance.</p>
<p>Once a stream has reached the end, subsequent calls to read or
<a href="#skip"><code>skip</code></a> will always report end-of-stream rather than producing more
data.</p>
<p>This function returns the number of bytes skipped, along with a
<a href="#stream_status"><code>stream-status</code></a> indicating whether the end of the stream was
reached. The returned value will be at most <code>len</code>; it may be less.</p>
<h5>Params</h5>
<ul>
<li><a name="skip.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
<li><a name="skip.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="skip.0"></a> result&lt;(<code>u64</code>, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h4><a name="blocking_skip"><code>blocking-skip: func</code></a></h4>
<p>Skip bytes from a stream, after blocking until at least one byte
can be skipped. Except for blocking behavior, identical to <a href="#skip"><code>skip</code></a>.</p>
<h5>Params</h5>
<ul>
<li><a name="blocking_skip.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
<li><a name="blocking_skip.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="blocking_skip.0"></a> result&lt;(<code>u64</code>, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h4><a name="subscribe_to_input_stream"><code>subscribe-to-input-stream: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once either the specified stream
has bytes available to read or the other end of the stream has been
closed.
The created <a href="#pollable"><code>pollable</code></a> is a child resource of the <a href="#input_stream"><code>input-stream</code></a>.
Implementations may trap if the <a href="#input_stream"><code>input-stream</code></a> is dropped before
all derived <a href="#pollable"><code>pollable</code></a>s created with this function are dropped.</p>
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
be used.
Implementations may trap if this <a href="#input_stream"><code>input-stream</code></a> is dropped while child
<a href="#pollable"><code>pollable</code></a> resources are still alive.
After this <a href="#input_stream"><code>input-stream</code></a> is dropped, implementations may report any
corresponding <a href="#output_stream"><code>output-stream</code></a> has <code>stream-state.closed</code>.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_input_stream.this"><code>this</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
</ul>
<h4><a name="check_write"><code>check-write: func</code></a></h4>
<p>Check readiness for writing. This function never blocks.</p>
<p>Returns the number of bytes permitted for the next call to <a href="#write"><code>write</code></a>,
or an error. Calling <a href="#write"><code>write</code></a> with more bytes than this function has
permitted will trap.</p>
<p>When this function returns 0 bytes, the <a href="#subscribe_to_output_stream"><code>subscribe-to-output-stream</code></a>
pollable will become ready when this function will report at least
1 byte, or an error.</p>
<h5>Params</h5>
<ul>
<li><a name="check_write.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="check_write.0"></a> result&lt;<code>u64</code>, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
</ul>
<h4><a name="write"><code>write: func</code></a></h4>
<p>Perform a write. This function never blocks.</p>
<p>Precondition: check-write gave permit of Ok(n) and contents has a
length of less than or equal to n. Otherwise, this function will trap.</p>
<p>returns Err(closed) without writing if the stream has closed since
the last call to check-write provided a permit.</p>
<h5>Params</h5>
<ul>
<li><a name="write.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="write.contents"><code>contents</code></a>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="write.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
</ul>
<h4><a name="blocking_write_and_flush"><code>blocking-write-and-flush: func</code></a></h4>
<p>Perform a write of up to 4096 bytes, and then flush the stream. Block
until all of these operations are complete, or an error occurs.</p>
<p>This is a convenience wrapper around the use of <a href="#check_write"><code>check-write</code></a>,
<a href="#subscribe_to_output_stream"><code>subscribe-to-output-stream</code></a>, <a href="#write"><code>write</code></a>, and <a href="#flush"><code>flush</code></a>, and is implemented
with the following pseudo-code:</p>
<pre><code class="language-text">let pollable = subscribe-to-output-stream(this);
while !contents.is_empty() {
  // Wait for the stream to become writable
  poll-oneoff(pollable);
  let Ok(n) = check-write(this); // eliding error handling
  let len = min(n, contents.len());
  let (chunk, rest) = contents.split_at(len);
  write(this, chunk);            // eliding error handling
  contents = rest;
}
flush(this);
// Wait for completion of `flush`
poll-oneoff(pollable);
// Check for any errors that arose during `flush`
let _ = check-write(this);       // eliding error handling
</code></pre>
<h5>Params</h5>
<ul>
<li><a name="blocking_write_and_flush.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="blocking_write_and_flush.contents"><code>contents</code></a>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="blocking_write_and_flush.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
</ul>
<h4><a name="flush"><code>flush: func</code></a></h4>
<p>Request to flush buffered output. This function never blocks.</p>
<p>This tells the output-stream that the caller intends any buffered
output to be flushed. the output which is expected to be flushed
is all that has been passed to <a href="#write"><code>write</code></a> prior to this call.</p>
<p>Upon calling this function, the <a href="#output_stream"><code>output-stream</code></a> will not accept any
writes (<a href="#check_write"><code>check-write</code></a> will return <code>ok(0)</code>) until the flush has
completed. The <a href="#subscribe_to_output_stream"><code>subscribe-to-output-stream</code></a> pollable will become ready
when the flush has completed and the stream can accept more writes.</p>
<h5>Params</h5>
<ul>
<li><a name="flush.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="flush.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
</ul>
<h4><a name="blocking_flush"><code>blocking-flush: func</code></a></h4>
<p>Request to flush buffered output, and block until flush completes
and stream is ready for writing again.</p>
<h5>Params</h5>
<ul>
<li><a name="blocking_flush.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="blocking_flush.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
</ul>
<h4><a name="subscribe_to_output_stream"><code>subscribe-to-output-stream: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the output-stream
is ready for more writing, or an error has occured. When this
pollable is ready, <a href="#check_write"><code>check-write</code></a> will return <code>ok(n)</code> with n&gt;0, or an
error.</p>
<p>If the stream is closed, this pollable is always ready immediately.</p>
<p>The created <a href="#pollable"><code>pollable</code></a> is a child resource of the <a href="#output_stream"><code>output-stream</code></a>.
Implementations may trap if the <a href="#output_stream"><code>output-stream</code></a> is dropped before
all derived <a href="#pollable"><code>pollable</code></a>s created with this function are dropped.</p>
<h5>Params</h5>
<ul>
<li><a name="subscribe_to_output_stream.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="subscribe_to_output_stream.0"></a> <a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></li>
</ul>
<h4><a name="write_zeroes"><code>write-zeroes: func</code></a></h4>
<p>Write zeroes to a stream.</p>
<p>this should be used precisely like <a href="#write"><code>write</code></a> with the exact same
preconditions (must use check-write first), but instead of
passing a list of bytes, you simply pass the number of zero-bytes
that should be written.</p>
<h5>Params</h5>
<ul>
<li><a name="write_zeroes.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="write_zeroes.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="write_zeroes.0"></a> result&lt;_, <a href="#write_error"><a href="#write_error"><code>write-error</code></a></a>&gt;</li>
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
<li><a name="splice.0"></a> result&lt;(<code>u64</code>, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
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
<li><a name="blocking_splice.0"></a> result&lt;(<code>u64</code>, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h4><a name="forward"><code>forward: func</code></a></h4>
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
<li><a name="forward.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
<li><a name="forward.src"><code>src</code></a>: <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="forward.0"></a> result&lt;(<code>u64</code>, <a href="#stream_status"><a href="#stream_status"><code>stream-status</code></a></a>)&gt;</li>
</ul>
<h4><a name="drop_output_stream"><code>drop-output-stream: func</code></a></h4>
<p>Dispose of the specified <a href="#output_stream"><code>output-stream</code></a>, after which it may no longer
be used.
Implementations may trap if this <a href="#output_stream"><code>output-stream</code></a> is dropped while
child <a href="#pollable"><code>pollable</code></a> resources are still alive.
After this <a href="#output_stream"><code>output-stream</code></a> is dropped, implementations may report any
corresponding <a href="#input_stream"><code>input-stream</code></a> has <code>stream-state.closed</code>.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_output_stream.this"><code>this</code></a>: <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
</ul>
