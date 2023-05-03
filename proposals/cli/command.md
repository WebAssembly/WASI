<h1><a name="command">World command</a></h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wall_clock"><code>wall-clock</code></a></li>
<li>interface <a href="#poll"><code>poll</code></a></li>
<li>interface <a href="#monotonic_clock"><code>monotonic-clock</code></a></li>
<li>interface <a href="#timezone"><code>timezone</code></a></li>
<li>interface <a href="#streams"><code>streams</code></a></li>
<li>interface <a href="#filesystem"><code>filesystem</code></a></li>
<li>interface <a href="#network"><code>network</code></a></li>
<li>interface <a href="#instance_network"><code>instance-network</code></a></li>
<li>interface <a href="#ip_name_lookup"><code>ip-name-lookup</code></a></li>
<li>interface <a href="#tcp"><code>tcp</code></a></li>
<li>interface <a href="#tcp_create_socket"><code>tcp-create-socket</code></a></li>
<li>interface <a href="#udp"><code>udp</code></a></li>
<li>interface <a href="#udp_create_socket"><code>udp-create-socket</code></a></li>
<li>interface <a href="#random"><code>random</code></a></li>
<li>interface <a href="#environment"><code>environment</code></a></li>
<li>interface <a href="#environment_preopens"><code>environment-preopens</code></a></li>
<li>interface <a href="#exit"><code>exit</code></a></li>
</ul>
</li>
<li>Exports:
<ul>
<li>interface <a href="#run"><code>run</code></a></li>
</ul>
</li>
</ul>
<h2><a name="wall_clock">Import interface wall-clock</a></h2>
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
<h2><a name="monotonic_clock">Import interface monotonic-clock</a></h2>
<p>WASI Monotonic Clock is a clock API intended to let users measure elapsed
time.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<p>A monotonic clock is a clock which has an unspecified initial value, and
successive reads of the clock will produce non-decreasing values.</p>
<p>It is intended for measuring elapsed time.</p>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>type pollable</code></a></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
#### <a name="instant">`type instant`</a>
`u64`
<p>A timestamp in nanoseconds.
<hr />
<h3>Functions</h3>
<h4><a name="now"><code>now: func</code></a></h4>
<p>Read the current value of the clock.</p>
<p>The clock is monotonic, therefore calling this function repeatedly will
produce a sequence of non-decreasing values.</p>
<h5>Return values</h5>
<ul>
<li><a name="now.0"></a> <a href="#instant"><a href="#instant"><code>instant</code></a></a></li>
</ul>
<h4><a name="resolution"><code>resolution: func</code></a></h4>
<p>Query the resolution of the clock.</p>
<h5>Return values</h5>
<ul>
<li><a name="resolution.0"></a> <a href="#instant"><a href="#instant"><code>instant</code></a></a></li>
</ul>
<h4><a name="subscribe"><code>subscribe: func</code></a></h4>
<p>Create a <a href="#pollable"><code>pollable</code></a> which will resolve once the specified time has been
reached.</p>
<h5>Params</h5>
<ul>
<li><a name="subscribe.when"><code>when</code></a>: <a href="#instant"><a href="#instant"><code>instant</code></a></a></li>
<li><a name="subscribe.absolute"><code>absolute</code></a>: <code>bool</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="subscribe.0"></a> <a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></li>
</ul>
<h2><a name="timezone">Import interface timezone</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="datetime"><code>type datetime</code></a></h4>
<p><a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></p>
<p>
#### <a name="timezone_display">`record timezone-display`</a>
<p>Information useful for displaying the timezone of a specific <a href="#datetime"><code>datetime</code></a>.</p>
<p>This information may vary within a single <a href="#timezone"><code>timezone</code></a> to reflect daylight
saving time adjustments.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a name="timezone_display.utc_offset"><a href="#utc_offset"><code>utc-offset</code></a></a>: <code>s32</code></p>
<p>The number of seconds difference between UTC time and the local
time of the timezone.
<p>The returned value will always be less than 86400 which is the
number of seconds in a day (24<em>60</em>60).</p>
<p>In implementations that do not expose an actual time zone, this
should return 0.</p>
</li>
<li>
<p><a name="timezone_display.name"><code>name</code></a>: <code>string</code></p>
<p>The abbreviated name of the timezone to display to a user. The name
`UTC` indicates Coordinated Universal Time. Otherwise, this should
reference local standards for the name of the time zone.
<p>In implementations that do not expose an actual time zone, this
should be the string <code>UTC</code>.</p>
<p>In time zones that do not have an applicable name, a formatted
representation of the UTC offset may be returned, such as <code>-04:00</code>.</p>
</li>
<li>
<p><a name="timezone_display.in_daylight_saving_time"><code>in-daylight-saving-time</code></a>: <code>bool</code></p>
<p>Whether daylight saving time is active.
<p>In implementations that do not expose an actual time zone, this
should return false.</p>
</li>
</ul>
<h4><a name="timezone"><code>type timezone</code></a></h4>
<p><code>u32</code></p>
<p>A timezone.
<p>In timezones that recognize daylight saving time, also known as daylight
time and summer time, the information returned from the functions varies
over time to reflect these adjustments.</p>
<p>This <a href="https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources">represents a resource</a>.</p>
<hr />
<h3>Functions</h3>
<h4><a name="display"><code>display: func</code></a></h4>
<p>Return information needed to display the given <a href="#datetime"><code>datetime</code></a>. This includes
the UTC offset, the time zone name, and a flag indicating whether
daylight saving time is active.</p>
<p>If the timezone cannot be determined for the given <a href="#datetime"><code>datetime</code></a>, return a
<a href="#timezone_display"><code>timezone-display</code></a> for <code>UTC</code> with a <a href="#utc_offset"><code>utc-offset</code></a> of 0 and no daylight
saving time.</p>
<h5>Params</h5>
<ul>
<li><a name="display.this"><code>this</code></a>: <a href="#timezone"><a href="#timezone"><code>timezone</code></a></a></li>
<li><a name="display.when"><code>when</code></a>: <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="display.0"></a> <a href="#timezone_display"><a href="#timezone_display"><code>timezone-display</code></a></a></li>
</ul>
<h4><a name="utc_offset"><code>utc-offset: func</code></a></h4>
<p>The same as <a href="#display"><code>display</code></a>, but only return the UTC offset.</p>
<h5>Params</h5>
<ul>
<li><a name="utc_offset.this"><code>this</code></a>: <a href="#timezone"><a href="#timezone"><code>timezone</code></a></a></li>
<li><a name="utc_offset.when"><code>when</code></a>: <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="utc_offset.0"></a> <code>s32</code></li>
</ul>
<h4><a name="drop_timezone"><code>drop-timezone: func</code></a></h4>
<p>Dispose of the specified input-stream, after which it may no longer
be used.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_timezone.this"><code>this</code></a>: <a href="#timezone"><a href="#timezone"><code>timezone</code></a></a></li>
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
<h2><a name="filesystem">Import interface filesystem</a></h2>
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
#### <a name="path_flags">`flags path-flags`</a>
<p>Flags determining the method of how paths are resolved.</p>
<h5>Flags members</h5>
<ul>
<li><a name="path_flags.symlink_follow"><code>symlink-follow</code></a>: <p>As long as the resolved path corresponds to a symbolic link, it is
expanded.
</li>
</ul>
<h4><a name="open_flags"><code>flags open-flags</code></a></h4>
<p>Open flags used by <a href="#open_at"><code>open-at</code></a>.</p>
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
<p>Permissions mode used by <a href="#open_at"><code>open-at</code></a>, <a href="#change_file_permissions_at"><code>change-file-permissions-at</code></a>, and
similar.</p>
<h5>Flags members</h5>
<ul>
<li>
<p><a name="modes.readable"><code>readable</code></a>: </p>
<p>True if the resource is considered readable by the containing
filesystem.
</li>
<li>
<p><a name="modes.writeable"><code>writeable</code></a>: </p>
<p>True if the resource is considered writeable by the containing
filesystem.
</li>
<li>
<p><a name="modes.executable"><code>executable</code></a>: </p>
<p>True if the resource is considered executable by the containing
filesystem. This does not apply to directories.
</li>
</ul>
<h4><a name="link_count"><code>type link-count</code></a></h4>
<p><code>u64</code></p>
<p>Number of hard links to an inode.
<h4><a name="inode"><code>type inode</code></a></h4>
<p><code>u64</code></p>
<p>Filesystem object serial number that is unique within its file system.
<h4><a name="filesize"><code>type filesize</code></a></h4>
<p><code>u64</code></p>
<p>File size or length of a region within a file.
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
<h4><a name="directory_entry_stream"><code>type directory-entry-stream</code></a></h4>
<p><code>u32</code></p>
<p>A stream of directory entries.
<p>This <a href="https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Streams">represents a stream of <code>dir-entry</code></a>.</p>
<h4><a name="device"><code>type device</code></a></h4>
<p><code>u64</code></p>
<p>Identifier for a device containing a file system. Can be used in
combination with `inode` to uniquely identify a file or directory in
the filesystem.
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
<h4><a name="directory_entry"><code>record directory-entry</code></a></h4>
<p>A directory entry.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a name="directory_entry.inode"><a href="#inode"><code>inode</code></a></a>: option&lt;<a href="#inode"><a href="#inode"><code>inode</code></a></a>&gt;</p>
<p>The serial number of the object referred to by this directory entry.
May be none if the inode value is not known.
<p>When this is none, libc implementations might do an extra <a href="#stat_at"><code>stat-at</code></a>
call to retrieve the inode number to fill their <code>d_ino</code> fields, so
implementations which can set this to a non-none value should do so.</p>
</li>
<li>
<p><a name="directory_entry.type"><code>type</code></a>: <a href="#descriptor_type"><a href="#descriptor_type"><code>descriptor-type</code></a></a></p>
<p>The type of the file referred to by this directory entry.
</li>
<li>
<p><a name="directory_entry.name"><code>name</code></a>: <code>string</code></p>
<p>The name of the object.
</li>
</ul>
<h4><a name="descriptor_flags"><code>flags descriptor-flags</code></a></h4>
<p>Descriptor flags.</p>
<p>Note: This was called <code>fdflags</code> in earlier versions of WASI.</p>
<h5>Flags members</h5>
<ul>
<li>
<p><a name="descriptor_flags.read"><a href="#read"><code>read</code></a></a>: </p>
<p>Read mode: Data can be read.
</li>
<li>
<p><a name="descriptor_flags.write"><a href="#write"><code>write</code></a></a>: </p>
<p>Write mode: Data can be written to.
</li>
<li>
<p><a name="descriptor_flags.non_blocking"><a href="#non_blocking"><code>non-blocking</code></a></a>: </p>
<p>Requests non-blocking operation.
<p>When this flag is enabled, functions may return immediately with an
<a href="#error_code.would_block"><code>error-code::would-block</code></a> error code in situations where they would
otherwise block. However, this non-blocking behavior is not
required. Implementations are permitted to ignore this flag and
block. This is similar to <code>O_NONBLOCK</code> in POSIX.</p>
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
<h4><a name="descriptor"><code>type descriptor</code></a></h4>
<p><code>u32</code></p>
<p>A descriptor is a reference to a filesystem object, which may be a file,
directory, named pipe, special file, or other object on which filesystem
calls may be made.
<p>This <a href="https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources">represents a resource</a>.</p>
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
<h4><a name="descriptor_stat"><code>record descriptor-stat</code></a></h4>
<p>File attributes.</p>
<p>Note: This was called <code>filestat</code> in earlier versions of WASI.</p>
<h5>Record Fields</h5>
<ul>
<li>
<p><a name="descriptor_stat.device"><a href="#device"><code>device</code></a></a>: <a href="#device"><a href="#device"><code>device</code></a></a></p>
<p>Device ID of device containing the file.
</li>
<li>
<p><a name="descriptor_stat.inode"><a href="#inode"><code>inode</code></a></a>: <a href="#inode"><a href="#inode"><code>inode</code></a></a></p>
<p>File serial number.
</li>
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
<p><a name="descriptor_stat.data_access_timestamp"><code>data-access-timestamp</code></a>: <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></p>
<p>Last data access timestamp.
</li>
<li>
<p><a name="descriptor_stat.data_modification_timestamp"><code>data-modification-timestamp</code></a>: <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></p>
<p>Last data modification timestamp.
</li>
<li>
<p><a name="descriptor_stat.status_change_timestamp"><code>status-change-timestamp</code></a>: <a href="#datetime"><a href="#datetime"><code>datetime</code></a></a></p>
<p>Last file status change timestamp.
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
<p><a name="advice.random"><a href="#random"><code>random</code></a></a></p>
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
<hr />
<h3>Functions</h3>
<h4><a name="read_via_stream"><code>read-via-stream: func</code></a></h4>
<p>Return a stream for reading from a file.</p>
<p>Multiple read, write, and append streams may be active on the same open
file and they do not interfere with each other.</p>
<p>Note: This allows using <code>read-stream</code>, which is similar to <a href="#read"><code>read</code></a> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="read_via_stream.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="read_via_stream.offset"><code>offset</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="read_via_stream.0"></a> <a href="#input_stream"><a href="#input_stream"><code>input-stream</code></a></a></li>
</ul>
<h4><a name="write_via_stream"><code>write-via-stream: func</code></a></h4>
<p>Return a stream for writing to a file.</p>
<p>Note: This allows using <code>write-stream</code>, which is similar to <a href="#write"><code>write</code></a> in
POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="write_via_stream.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="write_via_stream.offset"><code>offset</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="write_via_stream.0"></a> <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
</ul>
<h4><a name="append_via_stream"><code>append-via-stream: func</code></a></h4>
<p>Return a stream for appending to a file.</p>
<p>Note: This allows using <code>write-stream</code>, which is similar to <a href="#write"><code>write</code></a> with
<code>O_APPEND</code> in in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="append_via_stream.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="append_via_stream.0"></a> <a href="#output_stream"><a href="#output_stream"><code>output-stream</code></a></a></li>
</ul>
<h4><a name="advise"><code>advise: func</code></a></h4>
<p>Provide file advisory information on a descriptor.</p>
<p>This is similar to <code>posix_fadvise</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="advise.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="advise.offset"><code>offset</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
<li><a name="advise.length"><code>length</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
<li><a name="advise.advice"><a href="#advice"><code>advice</code></a></a>: <a href="#advice"><a href="#advice"><code>advice</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="advise.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="sync_data"><code>sync-data: func</code></a></h4>
<p>Synchronize the data of a file to disk.</p>
<p>This function succeeds with no effect if the file descriptor is not
opened for writing.</p>
<p>Note: This is similar to <code>fdatasync</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="sync_data.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="sync_data.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="get_flags"><code>get-flags: func</code></a></h4>
<p>Get flags associated with a descriptor.</p>
<p>Note: This returns similar flags to <code>fcntl(fd, F_GETFL)</code> in POSIX.</p>
<p>Note: This returns the value that was the <code>fs_flags</code> value returned
from <code>fdstat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="get_flags.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="get_flags.0"></a> result&lt;<a href="#descriptor_flags"><a href="#descriptor_flags"><code>descriptor-flags</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="get_type"><code>get-type: func</code></a></h4>
<p>Get the dynamic type of a descriptor.</p>
<p>Note: This returns the same value as the <code>type</code> field of the <code>fd-stat</code>
returned by <a href="#stat"><code>stat</code></a>, <a href="#stat_at"><code>stat-at</code></a> and similar.</p>
<p>Note: This returns similar flags to the <code>st_mode &amp; S_IFMT</code> value provided
by <code>fstat</code> in POSIX.</p>
<p>Note: This returns the value that was the <code>fs_filetype</code> value returned
from <code>fdstat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="get_type.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="get_type.0"></a> result&lt;<a href="#descriptor_type"><a href="#descriptor_type"><code>descriptor-type</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_flags"><code>set-flags: func</code></a></h4>
<p>Set status flags associated with a descriptor.</p>
<p>This function may only change the <a href="#non_blocking"><code>non-blocking</code></a> flag.</p>
<p>Note: This is similar to <code>fcntl(fd, F_SETFL, flags)</code> in POSIX.</p>
<p>Note: This was called <code>fd_fdstat_set_flags</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="set_flags.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="set_flags.flags"><code>flags</code></a>: <a href="#descriptor_flags"><a href="#descriptor_flags"><code>descriptor-flags</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_flags.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_size"><code>set-size: func</code></a></h4>
<p>Adjust the size of an open file. If this increases the file's size, the
extra bytes are filled with zeros.</p>
<p>Note: This was called <code>fd_filestat_set_size</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="set_size.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="set_size.size"><code>size</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_size.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_times"><code>set-times: func</code></a></h4>
<p>Adjust the timestamps of an open file or directory.</p>
<p>Note: This is similar to <code>futimens</code> in POSIX.</p>
<p>Note: This was called <code>fd_filestat_set_times</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="set_times.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="set_times.data_access_timestamp"><code>data-access-timestamp</code></a>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
<li><a name="set_times.data_modification_timestamp"><code>data-modification-timestamp</code></a>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_times.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="read"><code>read: func</code></a></h4>
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
<li><a name="read.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="read.length"><code>length</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
<li><a name="read.offset"><code>offset</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="read.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <code>bool</code>), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="write"><code>write: func</code></a></h4>
<p>Write to a descriptor, without using and updating the descriptor's offset.</p>
<p>It is valid to write past the end of a file; the file is extended to the
extent of the write, with bytes between the previous end and the start of
the write set to zero.</p>
<p>In the future, this may change to take a <code>stream&lt;u8, error-code&gt;</code>.</p>
<p>Note: This is similar to <code>pwrite</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="write.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="write.buffer"><code>buffer</code></a>: list&lt;<code>u8</code>&gt;</li>
<li><a name="write.offset"><code>offset</code></a>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="write.0"></a> result&lt;<a href="#filesize"><a href="#filesize"><code>filesize</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="read_directory"><code>read-directory: func</code></a></h4>
<p>Read directory entries from a directory.</p>
<p>On filesystems where directories contain entries referring to themselves
and their parents, often named <code>.</code> and <code>..</code> respectively, these entries
are omitted.</p>
<p>This always returns a new stream which starts at the beginning of the
directory. Multiple streams may be active on the same directory, and they
do not interfere with each other.</p>
<h5>Params</h5>
<ul>
<li><a name="read_directory.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="read_directory.0"></a> result&lt;<a href="#directory_entry_stream"><a href="#directory_entry_stream"><code>directory-entry-stream</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="sync"><code>sync: func</code></a></h4>
<p>Synchronize the data and metadata of a file to disk.</p>
<p>This function succeeds with no effect if the file descriptor is not
opened for writing.</p>
<p>Note: This is similar to <code>fsync</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="sync.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="sync.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="create_directory_at"><code>create-directory-at: func</code></a></h4>
<p>Create a directory.</p>
<p>Note: This is similar to <code>mkdirat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="create_directory_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="create_directory_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="create_directory_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="stat"><code>stat: func</code></a></h4>
<p>Return the attributes of an open file or directory.</p>
<p>Note: This is similar to <code>fstat</code> in POSIX.</p>
<p>Note: This was called <code>fd_filestat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="stat.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="stat.0"></a> result&lt;<a href="#descriptor_stat"><a href="#descriptor_stat"><code>descriptor-stat</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="stat_at"><code>stat-at: func</code></a></h4>
<p>Return the attributes of a file or directory.</p>
<p>Note: This is similar to <code>fstatat</code> in POSIX.</p>
<p>Note: This was called <code>path_filestat_get</code> in earlier versions of WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="stat_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="stat_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="stat_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="stat_at.0"></a> result&lt;<a href="#descriptor_stat"><a href="#descriptor_stat"><code>descriptor-stat</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="set_times_at"><code>set-times-at: func</code></a></h4>
<p>Adjust the timestamps of a file or directory.</p>
<p>Note: This is similar to <code>utimensat</code> in POSIX.</p>
<p>Note: This was called <code>path_filestat_set_times</code> in earlier versions of
WASI.</p>
<h5>Params</h5>
<ul>
<li><a name="set_times_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="set_times_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="set_times_at.path"><code>path</code></a>: <code>string</code></li>
<li><a name="set_times_at.data_access_timestamp"><code>data-access-timestamp</code></a>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
<li><a name="set_times_at.data_modification_timestamp"><code>data-modification-timestamp</code></a>: <a href="#new_timestamp"><a href="#new_timestamp"><code>new-timestamp</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="set_times_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="link_at"><code>link-at: func</code></a></h4>
<p>Create a hard link.</p>
<p>Note: This is similar to <code>linkat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="link_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="link_at.old_path_flags"><code>old-path-flags</code></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="link_at.old_path"><code>old-path</code></a>: <code>string</code></li>
<li><a name="link_at.new_descriptor"><code>new-descriptor</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="link_at.new_path"><code>new-path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="link_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="open_at"><code>open-at: func</code></a></h4>
<p>Open a file or directory.</p>
<p>The returned descriptor is not guaranteed to be the lowest-numbered
descriptor not currently open/ it is randomized to prevent applications
from depending on making assumptions about indexes, since this is
error-prone in multi-threaded contexts. The returned descriptor is
guaranteed to be less than 2**31.</p>
<p>If <code>flags</code> contains <a href="#descriptor_flags.mutate_directory"><code>descriptor-flags::mutate-directory</code></a>, and the base
descriptor doesn't have <a href="#descriptor_flags.mutate_directory"><code>descriptor-flags::mutate-directory</code></a> set,
<a href="#open_at"><code>open-at</code></a> fails with <a href="#error_code.read_only"><code>error-code::read-only</code></a>.</p>
<p>If <code>flags</code> contains <a href="#write"><code>write</code></a> or <code>mutate-directory</code>, or <a href="#open_flags"><code>open-flags</code></a>
contains <code>truncate</code> or <code>create</code>, and the base descriptor doesn't have
<a href="#descriptor_flags.mutate_directory"><code>descriptor-flags::mutate-directory</code></a> set, <a href="#open_at"><code>open-at</code></a> fails with
<a href="#error_code.read_only"><code>error-code::read-only</code></a>.</p>
<p>Note: This is similar to <code>openat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="open_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="open_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="open_at.path"><code>path</code></a>: <code>string</code></li>
<li><a name="open_at.open_flags"><a href="#open_flags"><code>open-flags</code></a></a>: <a href="#open_flags"><a href="#open_flags"><code>open-flags</code></a></a></li>
<li><a name="open_at.flags"><code>flags</code></a>: <a href="#descriptor_flags"><a href="#descriptor_flags"><code>descriptor-flags</code></a></a></li>
<li><a name="open_at.modes"><a href="#modes"><code>modes</code></a></a>: <a href="#modes"><a href="#modes"><code>modes</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="open_at.0"></a> result&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="readlink_at"><code>readlink-at: func</code></a></h4>
<p>Read the contents of a symbolic link.</p>
<p>If the contents contain an absolute or rooted path in the underlying
filesystem, this function fails with <a href="#error_code.not_permitted"><code>error-code::not-permitted</code></a>.</p>
<p>Note: This is similar to <code>readlinkat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="readlink_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="readlink_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="readlink_at.0"></a> result&lt;<code>string</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="remove_directory_at"><code>remove-directory-at: func</code></a></h4>
<p>Remove a directory.</p>
<p>Return <a href="#error_code.not_empty"><code>error-code::not-empty</code></a> if the directory is not empty.</p>
<p>Note: This is similar to <code>unlinkat(fd, path, AT_REMOVEDIR)</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="remove_directory_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="remove_directory_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="remove_directory_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="rename_at"><code>rename-at: func</code></a></h4>
<p>Rename a filesystem object.</p>
<p>Note: This is similar to <code>renameat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="rename_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="rename_at.old_path"><code>old-path</code></a>: <code>string</code></li>
<li><a name="rename_at.new_descriptor"><code>new-descriptor</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="rename_at.new_path"><code>new-path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="rename_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="symlink_at"><code>symlink-at: func</code></a></h4>
<p>Create a symbolic link (also known as a &quot;symlink&quot;).</p>
<p>If <code>old-path</code> starts with <code>/</code>, the function fails with
<a href="#error_code.not_permitted"><code>error-code::not-permitted</code></a>.</p>
<p>Note: This is similar to <code>symlinkat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="symlink_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="symlink_at.old_path"><code>old-path</code></a>: <code>string</code></li>
<li><a name="symlink_at.new_path"><code>new-path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="symlink_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="unlink_file_at"><code>unlink-file-at: func</code></a></h4>
<p>Unlink a filesystem object that is not a directory.</p>
<p>Return <a href="#error_code.is_directory"><code>error-code::is-directory</code></a> if the path refers to a directory.
Note: This is similar to <code>unlinkat(fd, path, 0)</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="unlink_file_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="unlink_file_at.path"><code>path</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="unlink_file_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="change_file_permissions_at"><code>change-file-permissions-at: func</code></a></h4>
<p>Change the permissions of a filesystem object that is not a directory.</p>
<p>Note that the ultimate meanings of these permissions is
filesystem-specific.</p>
<p>Note: This is similar to <code>fchmodat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="change_file_permissions_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="change_file_permissions_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="change_file_permissions_at.path"><code>path</code></a>: <code>string</code></li>
<li><a name="change_file_permissions_at.modes"><a href="#modes"><code>modes</code></a></a>: <a href="#modes"><a href="#modes"><code>modes</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="change_file_permissions_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="change_directory_permissions_at"><code>change-directory-permissions-at: func</code></a></h4>
<p>Change the permissions of a directory.</p>
<p>Note that the ultimate meanings of these permissions is
filesystem-specific.</p>
<p>Unlike in POSIX, the <code>executable</code> flag is not reinterpreted as a &quot;search&quot;
flag. <a href="#read"><code>read</code></a> on a directory implies readability and searchability, and
<code>execute</code> is not valid for directories.</p>
<p>Note: This is similar to <code>fchmodat</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a name="change_directory_permissions_at.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
<li><a name="change_directory_permissions_at.path_flags"><a href="#path_flags"><code>path-flags</code></a></a>: <a href="#path_flags"><a href="#path_flags"><code>path-flags</code></a></a></li>
<li><a name="change_directory_permissions_at.path"><code>path</code></a>: <code>string</code></li>
<li><a name="change_directory_permissions_at.modes"><a href="#modes"><code>modes</code></a></a>: <a href="#modes"><a href="#modes"><code>modes</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="change_directory_permissions_at.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="lock_shared"><code>lock-shared: func</code></a></h4>
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
<li><a name="lock_shared.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="lock_shared.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="lock_exclusive"><code>lock-exclusive: func</code></a></h4>
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
<li><a name="lock_exclusive.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="lock_exclusive.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="try_lock_shared"><code>try-lock-shared: func</code></a></h4>
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
<li><a name="try_lock_shared.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="try_lock_shared.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="try_lock_exclusive"><code>try-lock-exclusive: func</code></a></h4>
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
<li><a name="try_lock_exclusive.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="try_lock_exclusive.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="unlock"><code>unlock: func</code></a></h4>
<p>Release a shared or exclusive lock on an open file.</p>
<p>Note: This is similar to <code>flock(fd, LOCK_UN)</code> in Unix.</p>
<h5>Params</h5>
<ul>
<li><a name="unlock.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="unlock.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="drop_descriptor"><code>drop-descriptor: func</code></a></h4>
<p>Dispose of the specified <a href="#descriptor"><code>descriptor</code></a>, after which it may no longer
be used.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_descriptor.this"><code>this</code></a>: <a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></li>
</ul>
<h4><a name="read_directory_entry"><code>read-directory-entry: func</code></a></h4>
<p>Read a single directory entry from a <a href="#directory_entry_stream"><code>directory-entry-stream</code></a>.</p>
<h5>Params</h5>
<ul>
<li><a name="read_directory_entry.this"><code>this</code></a>: <a href="#directory_entry_stream"><a href="#directory_entry_stream"><code>directory-entry-stream</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="read_directory_entry.0"></a> result&lt;option&lt;<a href="#directory_entry"><a href="#directory_entry"><code>directory-entry</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a name="drop_directory_entry_stream"><code>drop-directory-entry-stream: func</code></a></h4>
<p>Dispose of the specified <a href="#directory_entry_stream"><code>directory-entry-stream</code></a>, after which it may no longer
be used.</p>
<h5>Params</h5>
<ul>
<li><a name="drop_directory_entry_stream.this"><code>this</code></a>: <a href="#directory_entry_stream"><a href="#directory_entry_stream"><code>directory-entry-stream</code></a></a></li>
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
<li><a name="address_family.0"></a> result&lt;<a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
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
<li><a name="address_family.0"></a> result&lt;<a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a>, <a href="#error"><a href="#error"><code>error</code></a></a>&gt;</li>
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
<h2><a name="random">Import interface random</a></h2>
<p>WASI Random is a random data API.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a name="get_random_bytes"><code>get-random-bytes: func</code></a></h4>
<p>Return <code>len</code> cryptographically-secure pseudo-random bytes.</p>
<p>This function must produce data from an adequately seeded
cryptographically-secure pseudo-random number generator (CSPRNG), so it
must not block, from the perspective of the calling program, and the
returned data is always unpredictable.</p>
<p>This function must always return fresh pseudo-random data. Deterministic
environments must omit this function, rather than implementing it with
deterministic data.</p>
<h5>Params</h5>
<ul>
<li><a name="get_random_bytes.len"><code>len</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="get_random_bytes.0"></a> list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a name="get_random_u64"><code>get-random-u64: func</code></a></h4>
<p>Return a cryptographically-secure pseudo-random <code>u64</code> value.</p>
<p>This function returns the same type of pseudo-random data as
<a href="#get_random_bytes"><code>get-random-bytes</code></a>, represented as a <code>u64</code>.</p>
<h5>Return values</h5>
<ul>
<li><a name="get_random_u64.0"></a> <code>u64</code></li>
</ul>
<h4><a name="insecure_random"><code>insecure-random: func</code></a></h4>
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
<li><a name="insecure_random.0"></a> (<code>u64</code>, <code>u64</code>)</li>
</ul>
<h2><a name="environment">Import interface environment</a></h2>
<hr />
<h3>Functions</h3>
<h4><a name="get_environment"><code>get-environment: func</code></a></h4>
<p>Get the POSIX-style environment variables.</p>
<p>Each environment variable is provided as a pair of string variable names
and string value.</p>
<p>Morally, these are a value import, but until value imports are available
in the component model, this import function should return the same
values each time it is called.</p>
<h5>Return values</h5>
<ul>
<li><a name="get_environment.0"></a> list&lt;(<code>string</code>, <code>string</code>)&gt;</li>
</ul>
<h4><a name="get_arguments"><code>get-arguments: func</code></a></h4>
<p>Get the POSIX-style arguments to the program.</p>
<h5>Return values</h5>
<ul>
<li><a name="get_arguments.0"></a> list&lt;<code>string</code>&gt;</li>
</ul>
<h2><a name="environment_preopens">Import interface environment-preopens</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="descriptor"><code>type descriptor</code></a></h4>
<p><a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></p>
<p>
#### <a name="input_stream">`type input-stream`</a>
[`input-stream`](#input_stream)
<p>
#### <a name="output_stream">`type output-stream`</a>
[`output-stream`](#output_stream)
<p>
----
<h3>Functions</h3>
<h4><a name="get_directories"><code>get-directories: func</code></a></h4>
<p>Return the set of of preopened directories, and their path.</p>
<h5>Return values</h5>
<ul>
<li><a name="get_directories.0"></a> list&lt;(<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>, <code>string</code>)&gt;</li>
</ul>
<h2><a name="exit">Import interface exit</a></h2>
<hr />
<h3>Functions</h3>
<h4><a name="exit"><code>exit: func</code></a></h4>
<p>Exit the curerent instance and any linked instances.</p>
<h5>Params</h5>
<ul>
<li><a name="exit.status"><code>status</code></a>: result</li>
</ul>
<h2><a name="run">Export interface run</a></h2>
<hr />
<h3>Functions</h3>
<h4><a name="run"><code>run: func</code></a></h4>
<p>Run the program.</p>
<h5>Return values</h5>
<ul>
<li><a name="run.0"></a> result</li>
</ul>
