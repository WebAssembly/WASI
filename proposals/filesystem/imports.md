<h1><a id="imports"></a>World imports</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_clocks_types_0_3_1"><code>wasi:clocks/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_system_clock_0_3_1"><code>wasi:clocks/system-clock@0.3.1</code></a></li>
<li>interface <a href="#wasi_filesystem_types_0_3_1"><code>wasi:filesystem/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_filesystem_preopens_0_3_1"><code>wasi:filesystem/preopens@0.3.1</code></a></li>
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
