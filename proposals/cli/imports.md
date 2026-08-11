<h1><a id="imports"></a>World imports</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_cli_environment_0_3_1"><code>wasi:cli/environment@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_exit_0_3_1"><code>wasi:cli/exit@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_types_0_3_1"><code>wasi:cli/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_stdin_0_3_1"><code>wasi:cli/stdin@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_stdout_0_3_1"><code>wasi:cli/stdout@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_stderr_0_3_1"><code>wasi:cli/stderr@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_terminal_input_0_3_1"><code>wasi:cli/terminal-input@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_terminal_output_0_3_1"><code>wasi:cli/terminal-output@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_terminal_stdin_0_3_1"><code>wasi:cli/terminal-stdin@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_terminal_stdout_0_3_1"><code>wasi:cli/terminal-stdout@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_terminal_stderr_0_3_1"><code>wasi:cli/terminal-stderr@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_types_0_3_1"><code>wasi:clocks/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_monotonic_clock_0_3_1"><code>wasi:clocks/monotonic-clock@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_system_clock_0_3_1"><code>wasi:clocks/system-clock@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_timezone_0_3_1"><code>wasi:clocks/timezone@0.3.1</code></a></li>
<li>interface <a href="#wasi_filesystem_types_0_3_1"><code>wasi:filesystem/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_filesystem_preopens_0_3_1"><code>wasi:filesystem/preopens@0.3.1</code></a></li>
<li>interface <a href="#wasi_sockets_types_0_3_1"><code>wasi:sockets/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_sockets_ip_name_lookup_0_3_1"><code>wasi:sockets/ip-name-lookup@0.3.1</code></a></li>
<li>interface <a href="#wasi_random_random_0_3_1"><code>wasi:random/random@0.3.1</code></a></li>
<li>interface <a href="#wasi_random_insecure_0_3_1"><code>wasi:random/insecure@0.3.1</code></a></li>
<li>interface <a href="#wasi_random_insecure_seed_0_3_1"><code>wasi:random/insecure-seed@0.3.1</code></a></li>
</ul>
</li>
</ul>
<h2><a id="wasi_cli_environment_0_3_1"></a>Import interface wasi:cli/environment@0.3.1</h2>
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
<h4><a id="get_initial_cwd"></a><code>get-initial-cwd: func</code></h4>
<p>Return a path that programs should use as their initial current working
directory, interpreting <code>.</code> as shorthand for this.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_initial_cwd.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h2><a id="wasi_cli_exit_0_3_1"></a>Import interface wasi:cli/exit@0.3.1</h2>
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
<h2><a id="wasi_cli_types_0_3_1"></a>Import interface wasi:cli/types@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="error_code"></a><code>enum error-code</code></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="error_code.io"></a><code>io</code></p>
<p>Input/output error
</li>
<li>
<p><a id="error_code.illegal_byte_sequence"></a><code>illegal-byte-sequence</code></p>
<p>Invalid or incomplete multibyte or wide character
</li>
<li>
<p><a id="error_code.pipe"></a><code>pipe</code></p>
<p>Broken pipe
</li>
</ul>
<h2><a id="wasi_cli_stdin_0_3_1"></a>Import interface wasi:cli/stdin@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="read_via_stream"></a><code>read-via-stream: func</code></h4>
<p>Return a stream for reading from stdin.</p>
<p>This function returns a stream which provides data read from stdin,
and a future to signal read results.</p>
<p>If the stream's readable end is dropped the future will resolve to success.</p>
<p>If the stream's writable end is dropped the future will either resolve to
success if stdin was closed by the writer or to an error-code if reading
failed for some other reason.</p>
<p>Multiple streams may be active at the same time. The behavior of concurrent
reads is implementation-specific.</p>
<h5>Return values</h5>
<ul>
<li><a id="read_via_stream.0"></a> (stream&lt;<code>u8</code>&gt;, future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;)</li>
</ul>
<h2><a id="wasi_cli_stdout_0_3_1"></a>Import interface wasi:cli/stdout@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="write_via_stream"></a><code>write-via-stream: func</code></h4>
<p>Write the given stream to stdout.</p>
<p>If the stream's writable end is dropped this function will either return
success once the entire contents of the stream have been written or an
error-code representing a failure.</p>
<p>Otherwise if there is an error the readable end of the stream will be
dropped and this function will return an error-code.</p>
<h5>Params</h5>
<ul>
<li><a id="write_via_stream.data"></a><code>data</code>: stream&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="write_via_stream.0"></a> future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
</ul>
<h2><a id="wasi_cli_stderr_0_3_1"></a>Import interface wasi:cli/stderr@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="write_via_stream"></a><code>write-via-stream: func</code></h4>
<p>Write the given stream to stderr.</p>
<p>If the stream's writable end is dropped this function will either return
success once the entire contents of the stream have been written or an
error-code representing a failure.</p>
<p>Otherwise if there is an error the readable end of the stream will be
dropped and this function will return an error-code.</p>
<h5>Params</h5>
<ul>
<li><a id="write_via_stream.data"></a><code>data</code>: stream&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="write_via_stream.0"></a> future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
</ul>
<h2><a id="wasi_cli_terminal_input_0_3_1"></a>Import interface wasi:cli/terminal-input@0.3.1</h2>
<p>Terminal input.</p>
<p>In the future, this may include functions for disabling echoing,
disabling input buffering so that keyboard events are sent through
immediately, querying supported features, and so on.</p>
<hr />
<h3>Types</h3>
<h4><a id="terminal_input"></a><code>resource terminal-input</code></h4>
<p>The input side of a terminal.</p>
<h2><a id="wasi_cli_terminal_output_0_3_1"></a>Import interface wasi:cli/terminal-output@0.3.1</h2>
<p>Terminal output.</p>
<p>In the future, this may include functions for querying the terminal
size, being notified of terminal size changes, querying supported
features, and so on.</p>
<hr />
<h3>Types</h3>
<h4><a id="terminal_output"></a><code>resource terminal-output</code></h4>
<p>The output side of a terminal.</p>
<h2><a id="wasi_cli_terminal_stdin_0_3_1"></a>Import interface wasi:cli/terminal-stdin@0.3.1</h2>
<p>An interface providing an optional <a href="#terminal_input"><code>terminal-input</code></a> for stdin as a
link-time authority.</p>
<hr />
<h3>Types</h3>
<h4><a id="terminal_input"></a><code>type terminal-input</code></h4>
<p><a href="#terminal_input"><a href="#terminal_input"><code>terminal-input</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="get_terminal_stdin"></a><code>get-terminal-stdin: func</code></h4>
<p>If stdin is connected to a terminal, return a <a href="#terminal_input"><code>terminal-input</code></a> handle
allowing further interaction with it.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_terminal_stdin.0"></a> option&lt;own&lt;<a href="#terminal_input"><a href="#terminal_input"><code>terminal-input</code></a></a>&gt;&gt;</li>
</ul>
<h2><a id="wasi_cli_terminal_stdout_0_3_1"></a>Import interface wasi:cli/terminal-stdout@0.3.1</h2>
<p>An interface providing an optional <a href="#terminal_output"><code>terminal-output</code></a> for stdout as a
link-time authority.</p>
<hr />
<h3>Types</h3>
<h4><a id="terminal_output"></a><code>type terminal-output</code></h4>
<p><a href="#terminal_output"><a href="#terminal_output"><code>terminal-output</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="get_terminal_stdout"></a><code>get-terminal-stdout: func</code></h4>
<p>If stdout is connected to a terminal, return a <a href="#terminal_output"><code>terminal-output</code></a> handle
allowing further interaction with it.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_terminal_stdout.0"></a> option&lt;own&lt;<a href="#terminal_output"><a href="#terminal_output"><code>terminal-output</code></a></a>&gt;&gt;</li>
</ul>
<h2><a id="wasi_cli_terminal_stderr_0_3_1"></a>Import interface wasi:cli/terminal-stderr@0.3.1</h2>
<p>An interface providing an optional <a href="#terminal_output"><code>terminal-output</code></a> for stderr as a
link-time authority.</p>
<hr />
<h3>Types</h3>
<h4><a id="terminal_output"></a><code>type terminal-output</code></h4>
<p><a href="#terminal_output"><a href="#terminal_output"><code>terminal-output</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="get_terminal_stderr"></a><code>get-terminal-stderr: func</code></h4>
<p>If stderr is connected to a terminal, return a <a href="#terminal_output"><code>terminal-output</code></a> handle
allowing further interaction with it.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_terminal_stderr.0"></a> option&lt;own&lt;<a href="#terminal_output"><a href="#terminal_output"><code>terminal-output</code></a></a>&gt;&gt;</li>
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
<h2><a id="wasi_filesystem_types_0_3_1"></a>Import interface wasi:filesystem/types@0.3.1</h2>
<p>WASI filesystem is a filesystem API primarily intended to let users run WASI
programs that access their files on their existing filesystems, without
significant overhead.</p>
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
<p>Though this package presents a portable interface modelled on POSIX, it
prioritizes compatibility over portability: allowing users to access their
files on their machine is more important than exposing a single semantics
across all platforms.  Notably, depending on the underlying operating system
and file system:</p>
<ul>
<li>Paths may be case-folded or not.</li>
<li>Deleting (unlinking) a file may fail if there are other file descriptors
open.</li>
<li>Durability and atomicity of changes to underlying files when there are
concurrent writers.</li>
</ul>
<p>Users that need well-defined, portable semantics should use a key-value
store or a database instead.</p>
<hr />
<h3>Types</h3>
<h4><a id="instant"></a><code>type instant</code></h4>
<p><a href="#instant"><a href="#instant"><code>instant</code></a></a></p>
<p>
<h4><a id="filesize"></a><code>type filesize</code></h4>
<p><code>u64</code></p>
<p>File size or length of a region within a file.
<h4><a id="descriptor_type"></a><code>variant descriptor-type</code></h4>
<p>The type of a filesystem object referenced by a descriptor.</p>
<p>Note: This was called <code>filetype</code> in earlier versions of WASI.</p>
<h5>Variant Cases</h5>
<ul>
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
<li>
<p><a id="descriptor_type.other"></a><code>other</code>: option&lt;<code>string</code>&gt;</p>
<p>The type of the descriptor or file is different from any of the
other types specified.
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
<p><a id="descriptor_stat.data_access_timestamp"></a><code>data-access-timestamp</code>: option&lt;<a href="#instant"><a href="#instant"><code>instant</code></a></a>&gt;</p>
<p>Last data access timestamp.
<p>If the <code>option</code> is none, the platform doesn't maintain an access
timestamp for this file.</p>
</li>
<li>
<p><a id="descriptor_stat.data_modification_timestamp"></a><code>data-modification-timestamp</code>: option&lt;<a href="#instant"><a href="#instant"><code>instant</code></a></a>&gt;</p>
<p>Last data modification timestamp.
<p>If the <code>option</code> is none, the platform doesn't maintain a
modification timestamp for this file.</p>
</li>
<li>
<p><a id="descriptor_stat.status_change_timestamp"></a><code>status-change-timestamp</code>: option&lt;<a href="#instant"><a href="#instant"><code>instant</code></a></a>&gt;</p>
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
<p><a id="new_timestamp.timestamp"></a><code>timestamp</code>: <a href="#instant"><a href="#instant"><code>instant</code></a></a></p>
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
<h4><a id="error_code"></a><code>variant error-code</code></h4>
<p>Error codes returned by functions, similar to <code>errno</code> in POSIX.
Not all of these error codes are returned by the functions provided by this
API; some are used in higher-level library layers, and others are provided
merely for alignment with POSIX.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="error_code.access"></a><code>access</code></p>
<p>Permission denied, similar to `EACCES` in POSIX.
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
<li>
<p><a id="error_code.other"></a><code>other</code>: option&lt;<code>string</code>&gt;</p>
<p>A catch-all for errors not captured by the existing variants.
Implementations can use this to extend the error type without
breaking existing code.
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
<h2>A descriptor is a reference to a filesystem object, which may be a file,
directory, named pipe, special file, or other object on which filesystem
calls may be made.</h2>
<h3>Functions</h3>
<h4><a id="method_descriptor_read_via_stream"></a><code>[method]descriptor.read-via-stream: func</code></h4>
<p>Return a stream for reading from a file.</p>
<p>Multiple read, write, and append streams may be active on the same open
file and they do not interfere with each other.</p>
<p>This function returns a <code>stream</code> which provides the data received from the
file, and a <code>future</code> providing additional error information in case an
error is encountered.</p>
<p>If no error is encountered, <code>stream.read</code> on the <code>stream</code> will return
<code>read-status::closed</code> with no <code>error-context</code> and the future resolves to
the value <code>ok</code>. If an error is encountered, <code>stream.read</code> on the
<code>stream</code> returns <code>read-status::closed</code> with an <code>error-context</code> and the future
resolves to <code>err</code> with an <a href="#error_code"><code>error-code</code></a>.</p>
<p>Note: This is similar to <code>pread</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_read_via_stream.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_read_via_stream.offset"></a><code>offset</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_read_via_stream.0"></a> (stream&lt;<code>u8</code>&gt;, future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;)</li>
</ul>
<h4><a id="method_descriptor_write_via_stream"></a><code>[method]descriptor.write-via-stream: func</code></h4>
<p>Return a stream for writing to a file, if available.</p>
<p>May fail with an error-code describing why the file cannot be written.</p>
<p>It is valid to write past the end of a file; the file is extended to the
extent of the write, with bytes between the previous end and the start of
the write set to zero.</p>
<p>This function returns once either full contents of the stream are
written or an error is encountered.</p>
<p>Note: This is similar to <code>pwrite</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_write_via_stream.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_write_via_stream.data"></a><code>data</code>: stream&lt;<code>u8</code>&gt;</li>
<li><a id="method_descriptor_write_via_stream.offset"></a><code>offset</code>: <a href="#filesize"><a href="#filesize"><code>filesize</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_write_via_stream.0"></a> future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_descriptor_append_via_stream"></a><code>[method]descriptor.append-via-stream: func</code></h4>
<p>Return a stream for appending to a file, if available.</p>
<p>May fail with an error-code describing why the file cannot be appended.</p>
<p>This function returns once either full contents of the stream are
written or an error is encountered.</p>
<p>Note: This is similar to <code>write</code> with <code>O_APPEND</code> in POSIX.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_append_via_stream.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
<li><a id="method_descriptor_append_via_stream.data"></a><code>data</code>: stream&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_append_via_stream.0"></a> future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
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
<h4><a id="method_descriptor_read_directory"></a><code>[method]descriptor.read-directory: func</code></h4>
<p>Read directory entries from a directory.</p>
<p>On filesystems where directories contain entries referring to themselves
and their parents, often named <code>.</code> and <code>..</code> respectively, these entries
are omitted.</p>
<p>This always returns a new stream which starts at the beginning of the
directory. Multiple streams may be active on the same directory, and they
do not interfere with each other.</p>
<p>This function returns a future, which will resolve to an error code if
reading full contents of the directory fails.</p>
<h5>Params</h5>
<ul>
<li><a id="method_descriptor_read_directory.self"></a><code>self</code>: borrow&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_descriptor_read_directory.0"></a> (stream&lt;<a href="#directory_entry"><a href="#directory_entry"><code>directory-entry</code></a></a>&gt;, future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;)</li>
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
<p>Fails with <a href="#error_code.no_entry"><code>error-code::no-entry</code></a> if the old path does not exist,
with <a href="#error_code.exist"><code>error-code::exist</code></a> if the new path already exists, and
<a href="#error_code.not_permitted"><code>error-code::not-permitted</code></a> if the old path is not a file.</p>
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
<p>This is similar to <code>unlinkat(fd, path, 0)</code> in POSIX.</p>
<p>Error returns are as specified by POSIX.</p>
<p>If the filesystem object is a directory, <a href="#error_code.access"><code>error-code::access</code></a> or
<a href="#error_code.is_directory"><code>error-code::is-directory</code></a> may be returned instead of the
POSIX-specified <a href="#error_code.not_permitted"><code>error-code::not-permitted</code></a>.</p>
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
<p>Implementations are encouraged to provide the following properties:</p>
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
<h2><a id="wasi_filesystem_preopens_0_3_1"></a>Import interface wasi:filesystem/preopens@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="descriptor"></a><code>type descriptor</code></h4>
<p><a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="get_directories"></a><code>get-directories: func</code></h4>
<p>Return the set of preopened directories, and their paths.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_directories.0"></a> list&lt;(own&lt;<a href="#descriptor"><a href="#descriptor"><code>descriptor</code></a></a>&gt;, <code>string</code>)&gt;</li>
</ul>
<h2><a id="wasi_sockets_types_0_3_1"></a>Import interface wasi:sockets/types@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><a href="#duration"><a href="#duration"><code>duration</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>variant error-code</code></h4>
<p>Error codes.</p>
<p>In theory, every API can return any error code.
In practice, API's typically only return the errors documented per API
combined with a couple of errors that are always possible:</p>
<ul>
<li><code>other</code></li>
<li><code>access-denied</code></li>
<li><code>not-supported</code></li>
<li><code>out-of-memory</code></li>
</ul>
<p>See each individual API for what the POSIX equivalents are. They sometimes differ per API.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="error_code.access_denied"></a><code>access-denied</code></p>
<p>Access denied.
<p>POSIX equivalent: EACCES, EPERM</p>
</li>
<li>
<p><a id="error_code.not_supported"></a><code>not-supported</code></p>
<p>The operation is not supported.
<p>POSIX equivalent: EOPNOTSUPP, ENOPROTOOPT, EPFNOSUPPORT, EPROTONOSUPPORT, ESOCKTNOSUPPORT</p>
</li>
<li>
<p><a id="error_code.invalid_argument"></a><code>invalid-argument</code></p>
<p>One of the arguments is invalid.
<p>POSIX equivalent: EINVAL, EDESTADDRREQ, EAFNOSUPPORT</p>
</li>
<li>
<p><a id="error_code.out_of_memory"></a><code>out-of-memory</code></p>
<p>Not enough memory to complete the operation.
<p>POSIX equivalent: ENOMEM, ENOBUFS</p>
</li>
<li>
<p><a id="error_code.timeout"></a><code>timeout</code></p>
<p>The operation timed out before it could finish completely.
<p>POSIX equivalent: ETIMEDOUT</p>
</li>
<li>
<p><a id="error_code.invalid_state"></a><code>invalid-state</code></p>
<p>The operation is not valid in the socket's current state.
</li>
<li>
<p><a id="error_code.address_not_bindable"></a><code>address-not-bindable</code></p>
<p>The local address is not available.
<p>POSIX equivalent: EADDRNOTAVAIL</p>
</li>
<li>
<p><a id="error_code.address_in_use"></a><code>address-in-use</code></p>
<p>A bind operation failed because the provided address is already in
use or because there are no ephemeral ports available.
<p>POSIX equivalent: EADDRINUSE</p>
</li>
<li>
<p><a id="error_code.remote_unreachable"></a><code>remote-unreachable</code></p>
<p>The remote address is not reachable.
<p>POSIX equivalent: EHOSTUNREACH, EHOSTDOWN, ENETDOWN, ENETUNREACH, ENONET</p>
</li>
<li>
<p><a id="error_code.connection_refused"></a><code>connection-refused</code></p>
<p>The connection was forcefully rejected.
<p>POSIX equivalent: ECONNREFUSED</p>
</li>
<li>
<p><a id="error_code.connection_broken"></a><code>connection-broken</code></p>
<p>A write failed because the connection was broken.
<p>POSIX equivalent: EPIPE</p>
</li>
<li>
<p><a id="error_code.connection_reset"></a><code>connection-reset</code></p>
<p>The connection was reset.
<p>POSIX equivalent: ECONNRESET</p>
</li>
<li>
<p><a id="error_code.connection_aborted"></a><code>connection-aborted</code></p>
<p>The connection was aborted.
<p>POSIX equivalent: ECONNABORTED</p>
</li>
<li>
<p><a id="error_code.datagram_too_large"></a><code>datagram-too-large</code></p>
<p>The size of a datagram sent to a UDP socket exceeded the maximum
supported size.
<p>POSIX equivalent: EMSGSIZE</p>
</li>
<li>
<p><a id="error_code.other"></a><code>other</code>: option&lt;<code>string</code>&gt;</p>
<p>A catch-all for errors not captured by the existing variants.
Implementations can use this to extend the error type without
breaking existing code.
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
<h4><a id="tcp_socket"></a><code>resource tcp-socket</code></h4>
<p>A TCP socket resource.</p>
<p>The socket can be in one of the following states:</p>
<ul>
<li><code>unbound</code></li>
<li><code>bound</code> (See note below)</li>
<li><code>listening</code></li>
<li><code>connecting</code></li>
<li><code>connected</code></li>
<li><code>closed</code>
See <a href="https://github.com/WebAssembly/WASI/blob/main/proposals/sockets/TcpSocketOperationalSemantics-0.3.0.md">https://github.com/WebAssembly/WASI/blob/main/proposals/sockets/TcpSocketOperationalSemantics-0.3.0.md</a>
for more information.</li>
</ul>
<p>Note: Except where explicitly mentioned, whenever this documentation uses
the term &quot;bound&quot; without backticks it actually means: in the <code>bound</code> state <em>or higher</em>.
(i.e. <code>bound</code>, <code>listening</code>, <code>connecting</code> or <code>connected</code>)</p>
<p>WASI uses shared ownership semantics: the <a href="#tcp_socket"><code>tcp-socket</code></a> handle and all
derived <code>stream</code> and <code>future</code> values reference a single underlying OS
socket:</p>
<ul>
<li>Send/receive streams remain functional after the original <a href="#tcp_socket"><code>tcp-socket</code></a>
handle is dropped.</li>
<li>The stream returned by <code>listen</code> behaves similarly.</li>
<li>Client sockets returned by <code>tcp-socket::listen</code> are independent and do
not keep the listening socket alive.</li>
</ul>
<p>The OS socket is closed only after the last handle is dropped. This
model has observable effects; for example, it affects when the local
port binding is released.</p>
<p>In addition to the general error codes documented on the
<code>types::error-code</code> type, TCP socket methods may always return
<code>error(invalid-state)</code> when in the <code>closed</code> state.</p>
<h4><a id="udp_socket"></a><code>resource udp-socket</code></h4>
<h2>A UDP socket handle.</h2>
<h3>Functions</h3>
<h4><a id="static_tcp_socket_create"></a><code>[static]tcp-socket.create: func</code></h4>
<p>Create a new TCP socket.</p>
<p>Similar to <code>socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)</code>
in POSIX. On IPv6 sockets, IPV6_V6ONLY is enabled by default and
can't be configured otherwise.</p>
<p>Unlike POSIX, WASI sockets have no notion of a socket-level
<code>O_NONBLOCK</code> flag. Instead they fully rely on the Component Model's
async support.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-supported</code>: The <code>address-family</code> is not supported. (EAFNOSUPPORT)</li>
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
<li><a id="static_tcp_socket_create.address_family"></a><code>address-family</code>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_tcp_socket_create.0"></a> result&lt;own&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_bind"></a><code>[method]tcp-socket.bind: func</code></h4>
<p>Bind the socket to the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is
left to the implementation to decide which network interface(s) to
bind to. If the TCP/UDP port is zero, the socket will be bound to a
random free port.</p>
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
<li><code>address-not-bindable</code>:      <code>local-address</code> is not an address that can be bound to. (EADDRNOTAVAIL)</li>
</ul>
<h1>Implementors note</h1>
<p>The bind operation shouldn't be affected by the TIME_WAIT state of a
recently closed socket on the same local address. In practice this
means that the SO_REUSEADDR socket option should be set implicitly
on all platforms, except on Windows where this is the default
behavior and SO_REUSEADDR performs something different.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/bind.2.html">https://man7.org/linux/man-pages/man2/bind.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=bind&amp;sektion=2&amp;format=html">https://man.freebsd.org/cgi/man.cgi?query=bind&amp;sektion=2&amp;format=html</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_bind.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_bind.local_address"></a><code>local-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_connect"></a><code>[method]tcp-socket.connect: func</code></h4>
<p>Connect to a remote endpoint.</p>
<p>On success, the socket is transitioned into the <code>connected</code> state
and the <code>remote-address</code> of the socket is updated.
The <code>local-address</code> may be updated as well, based on the best network
path to <code>remote-address</code>. If the socket was not already explicitly
bound, this function will implicitly bind the socket to a random
free port.</p>
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
<li><code>invalid-state</code>:             The socket is already in the <code>connecting</code> state. (EALREADY)</li>
<li><code>invalid-state</code>:             The socket is already in the <code>connected</code> state. (EISCONN)</li>
<li><code>invalid-state</code>:             The socket is already in the <code>listening</code> state. (EOPNOTSUPP, EINVAL on Windows)</li>
<li><code>timeout</code>:                   Connection timed out. (ETIMEDOUT)</li>
<li><code>connection-refused</code>:        The connection was forcefully rejected. (ECONNREFUSED)</li>
<li><code>connection-reset</code>:          The connection was reset. (ECONNRESET)</li>
<li><code>connection-aborted</code>:        The connection was aborted. (ECONNABORTED)</li>
<li><code>remote-unreachable</code>:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
<li><code>address-in-use</code>:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)</li>
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
<li><a id="method_tcp_socket_connect.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_connect.remote_address"></a><code>remote-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_listen"></a><code>[method]tcp-socket.listen: func</code></h4>
<p>Start listening and return a stream of new inbound connections.</p>
<p>Transitions the socket into the <code>listening</code> state. This can be called
at most once per socket.</p>
<p>If the socket is not already explicitly bound, this function will
implicitly bind the socket to a random free port.</p>
<p>Normally, the returned sockets are bound, in the <code>connected</code> state
and immediately ready for I/O. Though, depending on exact timing and
circumstances, a newly accepted connection may already be <code>closed</code>
by the time the server attempts to perform its first I/O on it. This
is true regardless of whether the WASI implementation uses
&quot;synthesized&quot; sockets or not (see Implementors Notes below).</p>
<p>The following properties are inherited from the listener socket:</p>
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
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:             The socket is already in the <code>connected</code> state. (EISCONN, EINVAL on BSD)</li>
<li><code>invalid-state</code>:             The socket is already in the <code>listening</code> state.</li>
<li><code>address-in-use</code>:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)</li>
</ul>
<h1>Implementors note</h1>
<p>This method returns a single perpetual stream that should only close
on fatal errors (if any). Yet, the POSIX' <code>accept</code> function may also
return transient errors (e.g. ECONNABORTED). The exact details differ
per operation system. For example, the Linux manual mentions:</p>
<blockquote>
<p>Linux accept() passes already-pending network errors on the new
socket as an error code from accept(). This behavior differs from
other BSD socket implementations. For reliable operation the
application should detect the network errors defined for the
protocol after accept() and treat them like EAGAIN by retrying.
In the case of TCP/IP, these are ENETDOWN, EPROTO, ENOPROTOOPT,
EHOSTDOWN, ENONET, EHOSTUNREACH, EOPNOTSUPP, and ENETUNREACH.
Source: https://man7.org/linux/man-pages/man2/accept.2.html</p>
</blockquote>
<p>WASI implementations have two options to handle this:</p>
<ul>
<li>Optionally log it and then skip over non-fatal errors returned by
<code>accept</code>. Guest code never gets to see these failures. Or:</li>
<li>Synthesize a <a href="#tcp_socket"><code>tcp-socket</code></a> resource that exposes the error when
attempting to send or receive on it. Guest code then sees these
failures as regular I/O errors.</li>
</ul>
<p>In either case, the stream returned by this <code>listen</code> method remains
operational.</p>
<p>WASI requires <code>listen</code> to perform an implicit bind if the socket
has not already been bound. Not all platforms (notably Windows)
exhibit this behavior out of the box. On platforms that require it,
the WASI implementation can emulate this behavior by performing
the bind itself if the guest hasn't already done so.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html</a></li>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/listen.2.html">https://man7.org/linux/man-pages/man2/listen.2.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/accept.2.html">https://man7.org/linux/man-pages/man2/accept.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=listen&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=listen&amp;sektion=2</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=accept&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=accept&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_listen.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_listen.0"></a> result&lt;stream&lt;own&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_send"></a><code>[method]tcp-socket.send: func</code></h4>
<p>Transmit data to peer.</p>
<p>The caller should close the stream when it has no more data to send
to the peer. Under normal circumstances this will cause a FIN packet
to be sent out. Closing the stream is equivalent to calling
<code>shutdown(SHUT_WR)</code> in POSIX.</p>
<p>This function may be called at most once and returns once the full
contents of the stream are transmitted or an error is encountered.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:             The socket is not in the <code>connected</code> state. (ENOTCONN)</li>
<li><code>invalid-state</code>:             <code>send</code> has already been called on this socket.</li>
<li><code>connection-broken</code>:         The connection is not writable anymore. (EPIPE, ECONNABORTED on Windows)</li>
<li><code>connection-reset</code>:          The connection was reset. (ECONNRESET)</li>
<li><code>remote-unreachable</code>:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/send.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/send.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/send.2.html">https://man7.org/linux/man-pages/man2/send.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=send&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=send&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_send.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
<li><a id="method_tcp_socket_send.data"></a><code>data</code>: stream&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_send.0"></a> future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_tcp_socket_receive"></a><code>[method]tcp-socket.receive: func</code></h4>
<p>Read data from peer.</p>
<p>Returns a <code>stream</code> of data sent by the peer. The implementation
drops the stream once no more data is available. At that point, the
returned <code>future</code> resolves to:</p>
<ul>
<li><code>ok</code> after a graceful shutdown from the peer (i.e. a FIN packet), or</li>
<li><code>err</code> if the socket was closed abnormally.</li>
</ul>
<p><code>receive</code> may be called only once per socket. Subsequent calls return
a closed stream and a future resolved to <code>err(invalid-state)</code>.</p>
<p>If the caller is not expecting to receive any more data from the peer,
they should drop the stream. Any data still in the receive queue
will be discarded. This is equivalent to calling <code>shutdown(SHUT_RD)</code>
in POSIX.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:             The socket is not in the <code>connected</code> state. (ENOTCONN)</li>
<li><code>invalid-state</code>:             <code>receive</code> has already been called on this socket.</li>
<li><code>connection-reset</code>:          The connection was reset. (ECONNRESET)</li>
<li><code>remote-unreachable</code>:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/recv.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/recv.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/recv.2.html">https://man7.org/linux/man-pages/man2/recv.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recv">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recv</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=recv&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=recv&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_receive.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_receive.0"></a> (stream&lt;<code>u8</code>&gt;, future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;)</li>
</ul>
<h4><a id="method_tcp_socket_get_local_address"></a><code>[method]tcp-socket.get-local-address: func</code></h4>
<p>Get the bound local address.</p>
<p>POSIX mentions:</p>
<blockquote>
<p>If the socket has not been bound to a local name, the value
stored in the object pointed to by <code>address</code> is unspecified.</p>
</blockquote>
<p>WASI is stricter and requires <code>get-local-address</code> to return
<code>invalid-state</code> when the socket hasn't been bound yet.</p>
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
<li><a id="method_tcp_socket_get_local_address.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_remote_address"></a><code>[method]tcp-socket.get-remote-address: func</code></h4>
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
<li><a id="method_tcp_socket_get_remote_address.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_tcp_socket_get_is_listening"></a><code>[method]tcp-socket.get-is-listening: func</code></h4>
<p>Whether the socket is in the <code>listening</code> state.</p>
<p>Equivalent to the SO_ACCEPTCONN socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_is_listening.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_is_listening.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_tcp_socket_get_address_family"></a><code>[method]tcp-socket.get-address-family: func</code></h4>
<p>Whether this is a IPv4 or IPv6 socket.</p>
<p>This is the value passed to the constructor.</p>
<p>Equivalent to the SO_DOMAIN socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_address_family.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_address_family.0"></a> <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h4><a id="method_tcp_socket_set_listen_backlog_size"></a><code>[method]tcp-socket.set-listen-backlog-size: func</code></h4>
<p>Hints the desired listen queue size. Implementations are free to
ignore this.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
Any other value will never cause an error, but it might be silently
clamped and/or rounded.</p>
<h1>Typical errors</h1>
<ul>
<li><code>not-supported</code>:        (set) The platform does not support changing the backlog size after the initial listen.</li>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
<li><code>invalid-state</code>:        (set) The socket is in the <code>connecting</code> or <code>connected</code> state.</li>
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
<h4><a id="method_tcp_socket_get_keep_alive_enabled"></a><code>[method]tcp-socket.get-keep-alive-enabled: func</code></h4>
<p>Enables or disables keepalive.</p>
<p>The keepalive behavior can be adjusted using:</p>
<ul>
<li><code>keep-alive-idle-time</code></li>
<li><code>keep-alive-interval</code></li>
<li><code>keep-alive-count</code>
These properties can be configured while <code>keep-alive-enabled</code> is
false, but only come into effect when <code>keep-alive-enabled</code> is true.</li>
</ul>
<p>Equivalent to the SO_KEEPALIVE socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_enabled.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_enabled.0"></a> result&lt;<code>bool</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_get_keep_alive_idle_time"></a><code>[method]tcp-socket.get-keep-alive-idle-time: func</code></h4>
<p>Amount of time the connection has to be idle before TCP starts
sending keepalive packets.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
All other values are accepted without error, but may be
clamped or rounded. As a result, the value read back from
this setting may differ from the value that was set.</p>
<p>Equivalent to the TCP_KEEPIDLE socket option. (TCP_KEEPALIVE on MacOS)</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_idle_time.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_idle_time.0"></a> result&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_get_keep_alive_interval"></a><code>[method]tcp-socket.get-keep-alive-interval: func</code></h4>
<p>The time between keepalive packets.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
All other values are accepted without error, but may be
clamped or rounded. As a result, the value read back from
this setting may differ from the value that was set.</p>
<p>Equivalent to the TCP_KEEPINTVL socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_interval.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_interval.0"></a> result&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_get_keep_alive_count"></a><code>[method]tcp-socket.get-keep-alive-count: func</code></h4>
<p>The maximum amount of keepalive packets TCP should send before
aborting the connection.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
All other values are accepted without error, but may be
clamped or rounded. As a result, the value read back from
this setting may differ from the value that was set.</p>
<p>Equivalent to the TCP_KEEPCNT socket option.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_count.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_keep_alive_count.0"></a> result&lt;<code>u32</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_get_hop_limit"></a><code>[method]tcp-socket.get-hop-limit: func</code></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The TTL value must be 1 or higher.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_hop_limit.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_get_receive_buffer_size"></a><code>[method]tcp-socket.get-receive-buffer-size: func</code></h4>
<p>Kernel buffer space reserved for sending/receiving on this socket.
Implementations usually treat this as a cap the buffer can grow to,
rather than allocating the full amount immediately.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
All other values are accepted without error, but may be
clamped or rounded. As a result, the value read back from
this setting may differ from the value that was set.</p>
<p>This is only a performance hint. The implementation may ignore it or
tweak it based on real traffic patterns.
Linux and macOS appear to behave differently depending on whether a
buffer size was explicitly set. When set, they tend to honor it; when
not set, they dynamically adjust the buffer size as the connection
progresses. This is especially noticeable when comparing the values
from before and after connection establishment.</p>
<p>Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_receive_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_tcp_socket_get_send_buffer_size"></a><code>[method]tcp-socket.get-send-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_tcp_socket_get_send_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#tcp_socket"><a href="#tcp_socket"><code>tcp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_tcp_socket_get_send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="static_udp_socket_create"></a><code>[static]udp-socket.create: func</code></h4>
<p>Create a new UDP socket.</p>
<p>Similar to <code>socket(AF_INET or AF_INET6, SOCK_DGRAM, IPPROTO_UDP)</code>
in POSIX. On IPv6 sockets, IPV6_V6ONLY is enabled by default and
can't be configured otherwise.</p>
<p>Unlike POSIX, WASI sockets have no notion of a socket-level
<code>O_NONBLOCK</code> flag. Instead they fully rely on the Component Model's
async support.</p>
<h1>References:</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/socket.2.html">https://man7.org/linux/man-pages/man2/socket.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=socket&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=socket&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="static_udp_socket_create.address_family"></a><code>address-family</code>: <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_udp_socket_create.0"></a> result&lt;own&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_bind"></a><code>[method]udp-socket.bind: func</code></h4>
<p>Bind the socket to the provided IP address and port.</p>
<p>If the IP address is zero (<code>0.0.0.0</code> in IPv4, <code>::</code> in IPv6), it is
left to the implementation to decide which network interface(s) to
bind to. If the port is zero, the socket will be bound to a random
free port.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:          The <code>local-address</code> has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)</li>
<li><code>invalid-state</code>:             The socket is already bound. (EINVAL)</li>
<li><code>address-in-use</code>:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)</li>
<li><code>address-in-use</code>:            Address is already in use. (EADDRINUSE)</li>
<li><code>address-not-bindable</code>:      <code>local-address</code> is not an address that can be bound to. (EADDRNOTAVAIL)</li>
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
<li><a id="method_udp_socket_bind.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_bind.local_address"></a><code>local-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_bind.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_connect"></a><code>[method]udp-socket.connect: func</code></h4>
<p>Associate this socket with a specific peer address.</p>
<p>On success, the <code>remote-address</code> of the socket is updated.
The <code>local-address</code> may be updated as well, based on the best network
path to <code>remote-address</code>. If the socket was not already explicitly
bound, this function will implicitly bind the socket to a random
free port.</p>
<p>When a UDP socket is &quot;connected&quot;, the <code>send</code> and <code>receive</code> methods
are limited to communicating with that peer only:</p>
<ul>
<li><code>send</code> can only be used to send to this destination.</li>
<li><code>receive</code> will only return datagrams sent from the provided <code>remote-address</code>.</li>
</ul>
<p>The name &quot;connect&quot; was kept to align with the existing POSIX
terminology. Other than that, this function only changes the local
socket configuration and does not generate any network traffic.
The peer is not aware of this &quot;connection&quot;.</p>
<p>This method may be called multiple times on the same socket to change
its association, but only the most recent one will be effective.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:          The <code>remote-address</code> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-argument</code>:          The IP address in <code>remote-address</code> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-argument</code>:          The port in <code>remote-address</code> is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>address-in-use</code>:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)</li>
</ul>
<h1>Implementors note</h1>
<p>If the socket is already connected, some platforms (e.g. Linux)
require a disconnect before connecting to a different peer address.</p>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/connect.2.html">https://man7.org/linux/man-pages/man2/connect.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect">https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?connect">https://man.freebsd.org/cgi/man.cgi?connect</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_connect.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_connect.remote_address"></a><code>remote-address</code>: <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_connect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_disconnect"></a><code>[method]udp-socket.disconnect: func</code></h4>
<p>Dissociate this socket from its peer address.</p>
<p>After calling this method, <code>send</code> &amp; <code>receive</code> are free to communicate
with any remote address again.</p>
<p>The POSIX equivalent of this is calling <code>connect</code> with an <code>AF_UNSPEC</code> address.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:           The socket is not connected.</li>
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
<li><a id="method_udp_socket_disconnect.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_disconnect.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_send"></a><code>[method]udp-socket.send: func</code></h4>
<p>Send a message on the socket to a particular peer.</p>
<p>If the socket is connected, the peer address may be left empty. In
that case this is equivalent to <code>send</code> in POSIX. Otherwise it is
equivalent to <code>sendto</code>.</p>
<p>Additionally, if the socket is connected, a <code>remote-address</code> argument
<em>may</em> be provided but then it must be identical to the address
passed to <code>connect</code>.</p>
<p>If the socket has not been explicitly bound, it will be
implicitly bound to a random free port.</p>
<p>Implementations may trap if the <code>data</code> length exceeds 64 KiB.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:        The <code>remote-address</code> has the wrong address family. (EAFNOSUPPORT)</li>
<li><code>invalid-argument</code>:        The IP address in <code>remote-address</code> is set to INADDR_ANY (<code>0.0.0.0</code> / <code>::</code>). (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-argument</code>:        The port in <code>remote-address</code> is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)</li>
<li><code>invalid-argument</code>:        The socket is in &quot;connected&quot; mode and <code>remote-address</code> is <code>some</code> value that does not match the address passed to <code>connect</code>. (EISCONN)</li>
<li><code>invalid-argument</code>:        The socket is not &quot;connected&quot; and no value for <code>remote-address</code> was provided. (EDESTADDRREQ)</li>
<li><code>remote-unreachable</code>:      The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
<li><code>connection-refused</code>:      The connection was refused. (ECONNREFUSED)</li>
<li><code>datagram-too-large</code>:      The datagram is too large. (EMSGSIZE)</li>
<li><code>address-in-use</code>:          Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)</li>
</ul>
<h1>Implementors note</h1>
<p>WASI requires <code>send</code> to perform an implicit bind if the socket
has not been bound. Not all platforms (notably Windows) exhibit
this behavior natively. On such platforms, the WASI implementation
should emulate it by performing the bind if the guest has not
already done so.</p>
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
<li><a id="method_udp_socket_send.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
<li><a id="method_udp_socket_send.data"></a><code>data</code>: list&lt;<code>u8</code>&gt;</li>
<li><a id="method_udp_socket_send.remote_address"></a><code>remote-address</code>: option&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_send.0"></a> result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_receive"></a><code>[method]udp-socket.receive: func</code></h4>
<p>Receive a message on the socket.</p>
<p>On success, the return value contains a tuple of the received data
and the address of the sender. Theoretical maximum length of the
data is 64 KiB. Though in practice, it will typically be less than
1500 bytes.</p>
<p>If the socket is connected, the sender address is guaranteed to
match the remote address passed to <code>connect</code>.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>:        The socket has not been bound yet.</li>
<li><code>remote-unreachable</code>:   The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)</li>
<li><code>connection-refused</code>:   The connection was refused. (ECONNREFUSED)</li>
</ul>
<h1>References</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html</a></li>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/recv.2.html">https://man7.org/linux/man-pages/man2/recv.2.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man2/recvmmsg.2.html">https://man7.org/linux/man-pages/man2/recvmmsg.2.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recvfrom">https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recvfrom</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/mswsock/nc-mswsock-lpfn_wsarecvmsg">https://learn.microsoft.com/en-us/windows/win32/api/mswsock/nc-mswsock-lpfn_wsarecvmsg</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=recv&amp;sektion=2">https://man.freebsd.org/cgi/man.cgi?query=recv&amp;sektion=2</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_receive.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_receive.0"></a> result&lt;(list&lt;<code>u8</code>&gt;, <a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>), <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_get_local_address"></a><code>[method]udp-socket.get-local-address: func</code></h4>
<p>Get the current bound address.</p>
<p>POSIX mentions:</p>
<blockquote>
<p>If the socket has not been bound to a local name, the value
stored in the object pointed to by <code>address</code> is unspecified.</p>
</blockquote>
<p>WASI is stricter and requires <code>get-local-address</code> to return
<code>invalid-state</code> when the socket hasn't been bound yet.</p>
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
<li><a id="method_udp_socket_get_local_address.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_local_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_get_remote_address"></a><code>[method]udp-socket.get-remote-address: func</code></h4>
<p>Get the address the socket is currently &quot;connected&quot; to.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-state</code>: The socket is not &quot;connected&quot; to a specific remote address. (ENOTCONN)</li>
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
<li><a id="method_udp_socket_get_remote_address.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_remote_address.0"></a> result&lt;<a href="#ip_socket_address"><a href="#ip_socket_address"><code>ip-socket-address</code></a></a>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h4><a id="method_udp_socket_get_address_family"></a><code>[method]udp-socket.get-address-family: func</code></h4>
<p>Whether this is a IPv4 or IPv6 socket.</p>
<p>This is the value passed to the constructor.</p>
<p>Equivalent to the SO_DOMAIN socket option.</p>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_get_address_family.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_address_family.0"></a> <a href="#ip_address_family"><a href="#ip_address_family"><code>ip-address-family</code></a></a></li>
</ul>
<h4><a id="method_udp_socket_get_unicast_hop_limit"></a><code>[method]udp-socket.get-unicast-hop-limit: func</code></h4>
<p>Equivalent to the IP_TTL &amp; IPV6_UNICAST_HOPS socket options.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The TTL value must be 1 or higher.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_get_unicast_hop_limit.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_unicast_hop_limit.0"></a> result&lt;<code>u8</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_udp_socket_get_receive_buffer_size"></a><code>[method]udp-socket.get-receive-buffer-size: func</code></h4>
<p>Kernel buffer space reserved for sending/receiving on this socket.
Implementations usually treat this as a cap the buffer can grow to,
rather than allocating the full amount immediately.</p>
<p>If the provided value is 0, an <code>invalid-argument</code> error is returned.
All other values are accepted without error, but may be
clamped or rounded. As a result, the value read back from
this setting may differ from the value that was set.</p>
<p>Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.</p>
<h1>Typical errors</h1>
<ul>
<li><code>invalid-argument</code>:     (set) The provided value was 0.</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_get_receive_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_receive_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h4><a id="method_udp_socket_get_send_buffer_size"></a><code>[method]udp-socket.get-send-buffer-size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_udp_socket_get_send_buffer_size.self"></a><code>self</code>: borrow&lt;<a href="#udp_socket"><a href="#udp_socket"><code>udp-socket</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_udp_socket_get_send_buffer_size.0"></a> result&lt;<code>u64</code>, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
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
<h2><a id="wasi_sockets_ip_name_lookup_0_3_1"></a>Import interface wasi:sockets/ip-name-lookup@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="ip_address"></a><code>type ip-address</code></h4>
<p><a href="#ip_address"><a href="#ip_address"><code>ip-address</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>variant error-code</code></h4>
<p>Lookup error codes.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="error_code.access_denied"></a><code>access-denied</code></p>
<p>Access denied.
<p>POSIX equivalent: EACCES, EPERM</p>
</li>
<li>
<p><a id="error_code.invalid_argument"></a><code>invalid-argument</code></p>
<p>`name` is a syntactically invalid domain name or IP address.
<p>POSIX equivalent: EINVAL</p>
</li>
<li>
<p><a id="error_code.name_unresolvable"></a><code>name-unresolvable</code></p>
<p>Name does not exist or has no suitable associated IP addresses.
<p>POSIX equivalent: EAI_NONAME, EAI_NODATA, EAI_ADDRFAMILY</p>
</li>
<li>
<p><a id="error_code.temporary_resolver_failure"></a><code>temporary-resolver-failure</code></p>
<p>A temporary failure in name resolution occurred.
<p>POSIX equivalent: EAI_AGAIN</p>
</li>
<li>
<p><a id="error_code.permanent_resolver_failure"></a><code>permanent-resolver-failure</code></p>
<p>A permanent failure in name resolution occurred.
<p>POSIX equivalent: EAI_FAIL</p>
</li>
<li>
<p><a id="error_code.other"></a><code>other</code>: option&lt;<code>string</code>&gt;</p>
<p>A catch-all for errors not captured by the existing variants.
Implementations can use this to extend the error type without
breaking existing code.
</li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a id="resolve_addresses"></a><code>resolve-addresses: func</code></h4>
<p>Resolve an internet host name to a list of IP addresses.</p>
<p>Unicode domain names are automatically converted to ASCII using IDNA
encoding. If the input is an IP address string, the address is parsed
and returned as-is without making any external requests.</p>
<p>See the wasi-socket proposal README.md for a comparison with getaddrinfo.</p>
<p>The results are returned in connection order preference.</p>
<p>This function never succeeds with 0 results. It either fails or succeeds
with at least one address. Additionally, this function never returns
IPv4-mapped IPv6 addresses.</p>
<h1>References:</h1>
<ul>
<li><a href="https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html">https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html</a></li>
<li><a href="https://man7.org/linux/man-pages/man3/getaddrinfo.3.html">https://man7.org/linux/man-pages/man3/getaddrinfo.3.html</a></li>
<li><a href="https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo">https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo</a></li>
<li><a href="https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&amp;sektion=3">https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&amp;sektion=3</a></li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="resolve_addresses.name"></a><code>name</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="resolve_addresses.0"></a> result&lt;list&lt;<a href="#ip_address"><a href="#ip_address"><code>ip-address</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_random_random_0_3_1"></a>Import interface wasi:random/random@0.3.1</h2>
<p>WASI Random is a random data API.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a id="get_random_bytes"></a><code>get-random-bytes: func</code></h4>
<p>Return up to <code>max-len</code> cryptographically-secure random or pseudo-random
bytes.</p>
<p>This function must produce data at least as cryptographically secure and
fast as an adequately seeded cryptographically-secure pseudo-random
number generator (CSPRNG). It must not block, from the perspective of
the calling program, under any circumstances, including on the first
request and on requests for numbers of bytes. The returned data must
always be unpredictable.</p>
<p>Implementations MAY return fewer bytes than requested (a short read).
Callers that require exactly <code>max-len</code> bytes MUST call this function in
a loop until the desired number of bytes has been accumulated.
Implementations MUST return at least 1 byte when <code>max-len</code> is greater
than zero. When <code>max-len</code> is zero, implementations MUST return an empty
list without trapping.</p>
<p>This function must always return fresh data. Deterministic environments
must omit this function, rather than implementing it with deterministic
data.</p>
<h5>Params</h5>
<ul>
<li><a id="get_random_bytes.max_len"></a><code>max-len</code>: <code>u64</code></li>
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
<h2><a id="wasi_random_insecure_0_3_1"></a>Import interface wasi:random/insecure@0.3.1</h2>
<p>The insecure interface for insecure pseudo-random numbers.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a id="get_insecure_random_bytes"></a><code>get-insecure-random-bytes: func</code></h4>
<p>Return up to <code>max-len</code> insecure pseudo-random bytes.</p>
<p>This function is not cryptographically secure. Do not use it for
anything related to security.</p>
<p>There are no requirements on the values of the returned bytes, however
implementations are encouraged to return evenly distributed values with
a long period.</p>
<p>Implementations MAY return fewer bytes than requested (a short read).
Callers that require exactly <code>max-len</code> bytes MUST call this function in
a loop until the desired number of bytes has been accumulated.
Implementations MUST return at least 1 byte when <code>max-len</code> is greater
than zero. When <code>max-len</code> is zero, implementations MUST return an empty
list without trapping.</p>
<h5>Params</h5>
<ul>
<li><a id="get_insecure_random_bytes.max_len"></a><code>max-len</code>: <code>u64</code></li>
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
<h2><a id="wasi_random_insecure_seed_0_3_1"></a>Import interface wasi:random/insecure-seed@0.3.1</h2>
<p>The insecure-seed interface for seeding hash-map DoS resistance.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a id="get_insecure_seed"></a><code>get-insecure-seed: func</code></h4>
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
<li><a id="get_insecure_seed.0"></a> (<code>u64</code>, <code>u64</code>)</li>
</ul>
