<h1><a id="imports"></a>World imports</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_clocks_types_0_3_1"><code>wasi:clocks/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_monotonic_clock_0_3_1"><code>wasi:clocks/monotonic-clock@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_system_clock_0_3_1"><code>wasi:clocks/system-clock@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_timezone_0_3_1"><code>wasi:clocks/timezone@0.3.1</code></a></li>
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
<h2><a id="wasi_clocks_monotonic_clock_0_3_1"></a>Import interface wasi:clocks/monotonic-clock@0.3.1</h2>
<p>WASI Monotonic Clock is a clock API intended to let users measure elapsed
time.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<p>A monotonic clock is a clock which has an unspecified initial value, and
successive reads of the clock will produce non-decreasing values.</p>
<hr />
<h3>Types</h3>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><a href="#duration"><a href="#duration"><code>duration</code></a></a></p>
<p>
<h4><a id="mark"></a><code>type mark</code></h4>
<p><code>u64</code></p>
<p>A mark on a monotonic clock is a number of nanoseconds since an
unspecified initial value, and can only be compared to instances from
the same monotonic-clock.
<hr />
<h3>Functions</h3>
<h4><a id="now"></a><code>now: func</code></h4>
<p>Read the current value of the clock.</p>
<p>The clock is monotonic, therefore calling this function repeatedly will
produce a sequence of non-decreasing values.</p>
<p>For completeness, this function traps if it's not possible to represent
the value of the clock in a <a href="#mark"><code>mark</code></a>. Consequently, implementations
should ensure that the starting time is low enough to avoid the
possibility of overflow in practice.</p>
<h5>Return values</h5>
<ul>
<li><a id="now.0"></a> <a href="#mark"><a href="#mark"><code>mark</code></a></a></li>
</ul>
<h4><a id="get_resolution"></a><code>get-resolution: func</code></h4>
<p>Query the resolution of the clock. Returns the duration of time
corresponding to a clock tick.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_resolution.0"></a> <a href="#duration"><a href="#duration"><code>duration</code></a></a></li>
</ul>
<h4><a id="wait_until"></a><code>wait-until: func</code></h4>
<p>Wait until the specified mark has occurred.</p>
<h5>Params</h5>
<ul>
<li><a id="wait_until.when"></a><code>when</code>: <a href="#mark"><a href="#mark"><code>mark</code></a></a></li>
</ul>
<h4><a id="wait_for"></a><code>wait-for: func</code></h4>
<p>Wait for the specified duration to elapse.</p>
<h5>Params</h5>
<ul>
<li><a id="wait_for.how_long"></a><code>how-long</code>: <a href="#duration"><a href="#duration"><code>duration</code></a></a></li>
</ul>
<h2><a id="wasi_clocks_system_clock_0_3_1"></a>Import interface wasi:clocks/system-clock@0.3.1</h2>
<p>WASI System Clock is a clock API intended to let users query the current
time. The clock is not necessarily monotonic as it may be reset.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<p>External references may be reset, so this clock is not necessarily
monotonic, making it unsuitable for measuring elapsed time.</p>
<p>It is intended for reporting the current date and time for humans.</p>
<hr />
<h3>Types</h3>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><a href="#duration"><a href="#duration"><code>duration</code></a></a></p>
<p>
<h4><a id="instant"></a><code>record instant</code></h4>
<p>An &quot;instant&quot;, or &quot;exact time&quot;, is a point in time without regard to any
time zone: just the time since a particular external reference point,
often called an &quot;epoch&quot;.</p>
<p>Here, the epoch is 1970-01-01T00:00:00Z, also known as
<a href="https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16">POSIX's Seconds Since the Epoch</a>, also known as <a href="https://en.wikipedia.org/wiki/Unix_time">Unix Time</a>.</p>
<p>Note that even if the seconds field is negative, incrementing
nanoseconds always represents moving forwards in time.
For example, <code>{ -1 seconds, 999999999 nanoseconds }</code> represents the
instant one nanosecond before the epoch.
For more on various different ways to represent time, see
https://tc39.es/proposal-temporal/docs/timezone.html</p>
<h5>Record Fields</h5>
<ul>
<li><a id="instant.seconds"></a><code>seconds</code>: <code>s64</code></li>
<li><a id="instant.nanoseconds"></a><code>nanoseconds</code>: <code>u32</code></li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a id="now"></a><code>now: func</code></h4>
<p>Read the current value of the clock.</p>
<p>This clock is not monotonic, therefore calling this function repeatedly
will not necessarily produce a sequence of non-decreasing values.</p>
<p>The nanoseconds field of the output is always less than 1000000000.</p>
<h5>Return values</h5>
<ul>
<li><a id="now.0"></a> <a href="#instant"><a href="#instant"><code>instant</code></a></a></li>
</ul>
<h4><a id="get_resolution"></a><code>get-resolution: func</code></h4>
<p>Query the resolution of the clock. Returns the smallest duration of time
that the implementation permits distinguishing.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_resolution.0"></a> <a href="#duration"><a href="#duration"><code>duration</code></a></a></li>
</ul>
<h2><a id="wasi_clocks_timezone_0_3_1"></a>Import interface wasi:clocks/timezone@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="instant"></a><code>type instant</code></h4>
<p><a href="#instant"><a href="#instant"><code>instant</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="iana_id"></a><code>iana-id: func</code></h4>
<p>Return the IANA identifier of the currently configured timezone. This
should be an identifier from the IANA Time Zone Database.</p>
<p>For displaying to a user, the identifier should be converted into a
localized name by means of an internationalization API.</p>
<p>If the implementation does not expose an actual timezone, or is unable
to provide mappings from times to deltas between the configured timezone
and UTC, or determining the current timezone fails, or the timezone does
not have an IANA identifier, this returns nothing.</p>
<h5>Return values</h5>
<ul>
<li><a id="iana_id.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="utc_offset"></a><code>utc-offset: func</code></h4>
<p>The number of nanoseconds difference between UTC time and the local
time of the currently configured timezone, at the exact time of
<a href="#instant"><code>instant</code></a>.</p>
<p>The magnitude of the returned value will always be less than
86,400,000,000,000 which is the number of nanoseconds in a day
(24<em>60</em>60*1e9).</p>
<p>If the implementation does not expose an actual timezone, or is unable
to provide mappings from times to deltas between the configured timezone
and UTC, or determining the current timezone fails, this returns
nothing.</p>
<h5>Params</h5>
<ul>
<li><a id="utc_offset.when"></a><code>when</code>: <a href="#instant"><a href="#instant"><code>instant</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="utc_offset.0"></a> option&lt;<code>s64</code>&gt;</li>
</ul>
<h4><a id="to_debug_string"></a><code>to-debug-string: func</code></h4>
<p>Returns a string that is suitable to assist humans in debugging whether
any timezone is available, and if so, which. This may be the same string
as <a href="#iana_id"><code>iana-id</code></a>, or a formatted representation of the UTC offset such as
<code>-04:00</code>, or something else.</p>
<p>WARNING: The returned string should not be consumed mechanically! It may
change across platforms, hosts, or other implementation details. Parsing
this string is a major platform-compatibility hazard.</p>
<h5>Return values</h5>
<ul>
<li><a id="to_debug_string.0"></a> <code>string</code></li>
</ul>
