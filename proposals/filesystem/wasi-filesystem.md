# Import interface `wasi-poll`

## Types

## <a href="#pollable" name="pollable"></a> `pollable`: `u32`

A "pollable" handle.

This is conceptually represents a `stream<_, _>`, or in other words,
a stream that one can wait on, repeatedly, but which does not itself
produce any data. It's temporary scaffolding until component-model's
async features are ready.

And at present, it is a `u32` instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.

`pollable` lifetimes are not automatically managed. Users must ensure
that they do not outlive the resource they reference.

Size: 4, Alignment: 4

## Functions

----

#### <a href="#drop_pollable" name="drop_pollable"></a> `drop-pollable` 

Dispose of the specified `pollable`, after which it may no longer be used.
##### Params

- <a href="#drop_pollable.this" name="drop_pollable.this"></a> `this`: [`pollable`](#pollable)

----

#### <a href="#poll_oneoff" name="poll_oneoff"></a> `poll-oneoff` 

Poll for completion on a set of pollables.

The "oneoff" in the name refers to the fact that this function must do a
linear scan through the entire list of subscriptions, which may be
inefficient if the number is large and the same subscriptions are used
many times. In the future, this is expected to be obsoleted by the
component model async proposal, which will include a scalable waiting
facility.

Note that the return type would ideally be `list<bool>`, but that would
be more difficult to polyfill given the current state of `wit-bindgen`.
See <https://github.com/bytecodealliance/preview2-prototyping/pull/11#issuecomment-1329873061>
for details.  For now, we use zero to mean "not ready" and non-zero to
mean "ready".
##### Params

- <a href="#poll_oneoff.in" name="poll_oneoff.in"></a> `in`: list<[`pollable`](#pollable)>
##### Results

- <a href="#poll_oneoff.result0" name="poll_oneoff.result0"></a> `result0`: list<`u8`>

# Import interface `wasi-io`

## Types

## <a href="#pollable" name="pollable"></a> `pollable`: [`pollable`](#pollable)


Size: 4, Alignment: 4

## <a href="#stream_error" name="stream_error"></a> `stream-error`: record

An error type returned from a stream operation. Currently this
doesn't provide any additional information.

Size: 0, Alignment: 1

### Record Fields

## <a href="#output_stream" name="output_stream"></a> `output-stream`: `u32`

An output bytestream. In the future, this will be replaced by handle
types.

This conceptually represents a `stream<u8, _>`. It's temporary
scaffolding until component-model's async features are ready.

And at present, it is a `u32` instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.

Size: 4, Alignment: 4

## <a href="#input_stream" name="input_stream"></a> `input-stream`: `u32`

An input bytestream. In the future, this will be replaced by handle
types.

This conceptually represents a `stream<u8, _>`. It's temporary
scaffolding until component-model's async features are ready.

And at present, it is a `u32` instead of being an actual handle, until
the wit-bindgen implementation of handles and resources is ready.

Size: 4, Alignment: 4

## Functions

----

#### <a href="#read" name="read"></a> `read` 

Read bytes from a stream.

This function returns a list of bytes containing the data that was
read, along with a bool indicating whether the end of the stream
was reached. The returned list will contain up to `len` bytes; it
may return fewer than requested, but not more.

Once a stream has reached the end, subsequent calls to read or
`skip` will always report end-of-stream rather than producing more
data.

If `len` is 0, it represents a request to read 0 bytes, which should
always succeed, assuming the stream hasn't reached its end yet, and
return an empty list.

The len here is a `u64`, but some callees may not be able to allocate
a buffer as large as that would imply.
FIXME: describe what happens if allocation fails.
##### Params

- <a href="#read.this" name="read.this"></a> `this`: [`input-stream`](#input_stream)
- <a href="#read.len" name="read.len"></a> `len`: `u64`
##### Results

- <a href="#read.result0" name="read.result0"></a> `result0`: result<(list<`u8`>, `bool`), [`stream-error`](#stream_error)>

----

#### <a href="#skip" name="skip"></a> `skip` 

Skip bytes from a stream.

This is similar to the `read` function, but avoids copying the
bytes into the instance.

Once a stream has reached the end, subsequent calls to read or
`skip` will always report end-of-stream rather than producing more
data.

This function returns the number of bytes skipped, along with a bool
indicating whether the end of the stream was reached. The returned
value will be at most `len`; it may be less.
##### Params

- <a href="#skip.this" name="skip.this"></a> `this`: [`input-stream`](#input_stream)
- <a href="#skip.len" name="skip.len"></a> `len`: `u64`
##### Results

- <a href="#skip.result0" name="skip.result0"></a> `result0`: result<(`u64`, `bool`), [`stream-error`](#stream_error)>

----

#### <a href="#subscribe_to_input_stream" name="subscribe_to_input_stream"></a> `subscribe-to-input-stream` 

Create a `pollable` which will resolve once either the specified stream has bytes
available to read or the other end of the stream has been closed.
##### Params

- <a href="#subscribe_to_input_stream.this" name="subscribe_to_input_stream.this"></a> `this`: [`input-stream`](#input_stream)
##### Results

- <a href="#subscribe_to_input_stream.result0" name="subscribe_to_input_stream.result0"></a> `result0`: [`pollable`](#pollable)

----

#### <a href="#drop_input_stream" name="drop_input_stream"></a> `drop-input-stream` 

Dispose of the specified `input-stream`, after which it may no longer
be used.
##### Params

- <a href="#drop_input_stream.this" name="drop_input_stream.this"></a> `this`: [`input-stream`](#input_stream)

----

#### <a href="#write" name="write"></a> `write` 

Write bytes to a stream.

This function returns a `u64` indicating the number of bytes from
`buf` that were written; it may be less than the full list.
##### Params

- <a href="#write.this" name="write.this"></a> `this`: [`output-stream`](#output_stream)
- <a href="#write.buf" name="write.buf"></a> `buf`: list<`u8`>
##### Results

- <a href="#write.result0" name="write.result0"></a> `result0`: result<`u64`, [`stream-error`](#stream_error)>

----

#### <a href="#write_zeroes" name="write_zeroes"></a> `write-zeroes` 

Write multiple zero bytes to a stream.

This function returns a `u64` indicating the number of zero bytes
that were written; it may be less than `len`.
##### Params

- <a href="#write_zeroes.this" name="write_zeroes.this"></a> `this`: [`output-stream`](#output_stream)
- <a href="#write_zeroes.len" name="write_zeroes.len"></a> `len`: `u64`
##### Results

- <a href="#write_zeroes.result0" name="write_zeroes.result0"></a> `result0`: result<`u64`, [`stream-error`](#stream_error)>

----

#### <a href="#splice" name="splice"></a> `splice` 

Read from one stream and write to another.

This function returns the number of bytes transferred; it may be less
than `len`.
##### Params

- <a href="#splice.this" name="splice.this"></a> `this`: [`output-stream`](#output_stream)
- <a href="#splice.src" name="splice.src"></a> `src`: [`input-stream`](#input_stream)
- <a href="#splice.len" name="splice.len"></a> `len`: `u64`
##### Results

- <a href="#splice.result0" name="splice.result0"></a> `result0`: result<(`u64`, `bool`), [`stream-error`](#stream_error)>

----

#### <a href="#forward" name="forward"></a> `forward` 

Forward the entire contents of an input stream to an output stream.

This function repeatedly reads from the input stream and writes
the data to the output stream, until the end of the input stream
is reached, or an error is encountered.

This function returns the number of bytes transferred.
##### Params

- <a href="#forward.this" name="forward.this"></a> `this`: [`output-stream`](#output_stream)
- <a href="#forward.src" name="forward.src"></a> `src`: [`input-stream`](#input_stream)
##### Results

- <a href="#forward.result0" name="forward.result0"></a> `result0`: result<`u64`, [`stream-error`](#stream_error)>

----

#### <a href="#subscribe_to_output_stream" name="subscribe_to_output_stream"></a> `subscribe-to-output-stream` 

Create a `pollable` which will resolve once either the specified stream is ready
to accept bytes or the other end of the stream has been closed.
##### Params

- <a href="#subscribe_to_output_stream.this" name="subscribe_to_output_stream.this"></a> `this`: [`output-stream`](#output_stream)
##### Results

- <a href="#subscribe_to_output_stream.result0" name="subscribe_to_output_stream.result0"></a> `result0`: [`pollable`](#pollable)

----

#### <a href="#drop_output_stream" name="drop_output_stream"></a> `drop-output-stream` 

Dispose of the specified `output-stream`, after which it may no longer
be used.
##### Params

- <a href="#drop_output_stream.this" name="drop_output_stream.this"></a> `this`: [`output-stream`](#output_stream)

# Import interface `wasi-wall-clock`

## Types

## <a href="#wall_clock" name="wall_clock"></a> `wall-clock`: `u32`

A wall clock is a clock which measures the date and time according to some
external reference.

External references may be reset, so this clock is not necessarily
monotonic, making it unsuitable for measuring elapsed time.

It is intended for reporting the current date and time for humans.

Size: 4, Alignment: 4

## <a href="#datetime" name="datetime"></a> `datetime`: record

A time and date in seconds plus nanoseconds.

Size: 16, Alignment: 8

### Record Fields

- <a href="datetime.seconds" name="datetime.seconds"></a> [`seconds`](#datetime.seconds): `u64`
  
  
- <a href="datetime.nanoseconds" name="datetime.nanoseconds"></a> [`nanoseconds`](#datetime.nanoseconds): `u32`
  
  
## Functions

----

#### <a href="#now" name="now"></a> `now` 

Read the current value of the clock.

This clock is not monotonic, therefore calling this function repeatedly will
not necessarily produce a sequence of non-decreasing values.

The returned timestamps represent the number of seconds since
1970-01-01T00:00:00Z, also known as [POSIX's Seconds Since the Epoch], also
known as [Unix Time].

The nanoseconds field of the output is always less than 1000000000.

[POSIX's Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16
[Unix Time]: https://en.wikipedia.org/wiki/Unix_time
##### Params

- <a href="#now.this" name="now.this"></a> `this`: [`wall-clock`](#wall_clock)
##### Results

- <a href="#now.result0" name="now.result0"></a> `result0`: [`datetime`](#datetime)

----

#### <a href="#resolution" name="resolution"></a> `resolution` 

Query the resolution of the clock.

The nanoseconds field of the output is always less than 1000000000.
##### Params

- <a href="#resolution.this" name="resolution.this"></a> `this`: [`wall-clock`](#wall_clock)
##### Results

- <a href="#resolution.result0" name="resolution.result0"></a> `result0`: [`datetime`](#datetime)

----

#### <a href="#drop_wall_clock" name="drop_wall_clock"></a> `drop-wall-clock` 

Dispose of the specified `wall-clock`, after which it may no longer
be used.
##### Params

- <a href="#drop_wall_clock.this" name="drop_wall_clock.this"></a> `this`: [`wall-clock`](#wall_clock)

# Import interface `wasi-filesystem`

## Types

## <a href="#input_stream" name="input_stream"></a> `input-stream`: [`input-stream`](#input_stream)


Size: 4, Alignment: 4

## <a href="#output_stream" name="output_stream"></a> `output-stream`: [`output-stream`](#output_stream)


Size: 4, Alignment: 4

## <a href="#datetime" name="datetime"></a> `datetime`: [`datetime`](#datetime)


Size: 16, Alignment: 8

## <a href="#o_flags" name="o_flags"></a> `o-flags`: record

Open flags used by `open-at`.

Size: 1, Alignment: 1

### Record Fields

- <a href="o_flags.create" name="o_flags.create"></a> [`create`](#o_flags.create): 
  
  Create file if it does not exist.
  Bit: 0

- <a href="o_flags.directory" name="o_flags.directory"></a> [`directory`](#o_flags.directory): 
  
  Fail if not a directory.
  Bit: 1

- <a href="o_flags.excl" name="o_flags.excl"></a> [`excl`](#o_flags.excl): 
  
  Fail if file already exists.
  Bit: 2

- <a href="o_flags.trunc" name="o_flags.trunc"></a> [`trunc`](#o_flags.trunc): 
  
  Truncate file to size 0.
  Bit: 3

## <a href="#mode" name="mode"></a> `mode`: record

Permissions mode used by `open-at`, `change-file-permissions-at`, and
similar.

Size: 1, Alignment: 1

### Record Fields

- <a href="mode.readable" name="mode.readable"></a> [`readable`](#mode.readable): 
  
  True if the resource is considered readable by the containing
  filesystem.
  Bit: 0

- <a href="mode.writeable" name="mode.writeable"></a> [`writeable`](#mode.writeable): 
  
  True if the resource is considered writeable by the containing
  filesystem.
  Bit: 1

- <a href="mode.executable" name="mode.executable"></a> [`executable`](#mode.executable): 
  
  True if the resource is considered executable by the containing
  filesystem. This does not apply to directories.
  Bit: 2

## <a href="#linkcount" name="linkcount"></a> `linkcount`: `u64`

Number of hard links to an inode.

Size: 8, Alignment: 8

## <a href="#inode" name="inode"></a> `inode`: `u64`

Filesystem object serial number that is unique within its file system.

Size: 8, Alignment: 8

## <a href="#filesize" name="filesize"></a> `filesize`: `u64`

File size or length of a region within a file.

Size: 8, Alignment: 8

## <a href="#errno" name="errno"></a> `errno`: enum

Error codes returned by functions.
Not all of these error codes are returned by the functions provided by this
API; some are used in higher-level library layers, and others are provided
merely for alignment with POSIX.

Size: 1, Alignment: 1

### Enum Cases

- <a href="errno.access" name="errno.access"></a> [`access`](#errno.access)
  
  Permission denied.
  
- <a href="errno.again" name="errno.again"></a> [`again`](#errno.again)
  
  Resource unavailable, or operation would block.
  
- <a href="errno.already" name="errno.already"></a> [`already`](#errno.already)
  
  Connection already in progress.
  
- <a href="errno.badf" name="errno.badf"></a> [`badf`](#errno.badf)
  
  Bad descriptor.
  
- <a href="errno.busy" name="errno.busy"></a> [`busy`](#errno.busy)
  
  Device or resource busy.
  
- <a href="errno.deadlk" name="errno.deadlk"></a> [`deadlk`](#errno.deadlk)
  
  Resource deadlock would occur.
  
- <a href="errno.dquot" name="errno.dquot"></a> [`dquot`](#errno.dquot)
  
  Storage quota exceeded.
  
- <a href="errno.exist" name="errno.exist"></a> [`exist`](#errno.exist)
  
  File exists.
  
- <a href="errno.fbig" name="errno.fbig"></a> [`fbig`](#errno.fbig)
  
  File too large.
  
- <a href="errno.ilseq" name="errno.ilseq"></a> [`ilseq`](#errno.ilseq)
  
  Illegal byte sequence.
  
- <a href="errno.inprogress" name="errno.inprogress"></a> [`inprogress`](#errno.inprogress)
  
  Operation in progress.
  
- <a href="errno.intr" name="errno.intr"></a> [`intr`](#errno.intr)
  
  Interrupted function.
  
- <a href="errno.inval" name="errno.inval"></a> [`inval`](#errno.inval)
  
  Invalid argument.
  
- <a href="errno.io" name="errno.io"></a> [`io`](#errno.io)
  
  I/O error.
  
- <a href="errno.isdir" name="errno.isdir"></a> [`isdir`](#errno.isdir)
  
  Is a directory.
  
- <a href="errno.loop" name="errno.loop"></a> [`loop`](#errno.loop)
  
  Too many levels of symbolic links.
  
- <a href="errno.mlink" name="errno.mlink"></a> [`mlink`](#errno.mlink)
  
  Too many links.
  
- <a href="errno.msgsize" name="errno.msgsize"></a> [`msgsize`](#errno.msgsize)
  
  Message too large.
  
- <a href="errno.nametoolong" name="errno.nametoolong"></a> [`nametoolong`](#errno.nametoolong)
  
  Filename too long.
  
- <a href="errno.nodev" name="errno.nodev"></a> [`nodev`](#errno.nodev)
  
  No such device.
  
- <a href="errno.noent" name="errno.noent"></a> [`noent`](#errno.noent)
  
  No such file or directory.
  
- <a href="errno.nolck" name="errno.nolck"></a> [`nolck`](#errno.nolck)
  
  No locks available.
  
- <a href="errno.nomem" name="errno.nomem"></a> [`nomem`](#errno.nomem)
  
  Not enough space.
  
- <a href="errno.nospc" name="errno.nospc"></a> [`nospc`](#errno.nospc)
  
  No space left on device.
  
- <a href="errno.nosys" name="errno.nosys"></a> [`nosys`](#errno.nosys)
  
  Function not supported.
  
- <a href="errno.notdir" name="errno.notdir"></a> [`notdir`](#errno.notdir)
  
  Not a directory or a symbolic link to a directory.
  
- <a href="errno.notempty" name="errno.notempty"></a> [`notempty`](#errno.notempty)
  
  Directory not empty.
  
- <a href="errno.notrecoverable" name="errno.notrecoverable"></a> [`notrecoverable`](#errno.notrecoverable)
  
  State not recoverable.
  
- <a href="errno.notsup" name="errno.notsup"></a> [`notsup`](#errno.notsup)
  
  Not supported, or operation not supported on socket.
  
- <a href="errno.notty" name="errno.notty"></a> [`notty`](#errno.notty)
  
  Inappropriate I/O control operation.
  
- <a href="errno.nxio" name="errno.nxio"></a> [`nxio`](#errno.nxio)
  
  No such device or address.
  
- <a href="errno.overflow" name="errno.overflow"></a> [`overflow`](#errno.overflow)
  
  Value too large to be stored in data type.
  
- <a href="errno.perm" name="errno.perm"></a> [`perm`](#errno.perm)
  
  Operation not permitted.
  
- <a href="errno.pipe" name="errno.pipe"></a> [`pipe`](#errno.pipe)
  
  Broken pipe.
  
- <a href="errno.rofs" name="errno.rofs"></a> [`rofs`](#errno.rofs)
  
  Read-only file system.
  
- <a href="errno.spipe" name="errno.spipe"></a> [`spipe`](#errno.spipe)
  
  Invalid seek.
  
- <a href="errno.txtbsy" name="errno.txtbsy"></a> [`txtbsy`](#errno.txtbsy)
  
  Text file busy.
  
- <a href="errno.xdev" name="errno.xdev"></a> [`xdev`](#errno.xdev)
  
  Cross-device link.
  
## <a href="#dir_entry_stream" name="dir_entry_stream"></a> `dir-entry-stream`: `u32`

A stream of directory entries.

Size: 4, Alignment: 4

## <a href="#device" name="device"></a> `device`: `u64`

Identifier for a device containing a file system. Can be used in combination
with `inode` to uniquely identify a file or directory in the filesystem.

Size: 8, Alignment: 8

## <a href="#descriptor_type" name="descriptor_type"></a> `descriptor-type`: enum

The type of a filesystem object referenced by a descriptor.

Note: This was called `filetype` in earlier versions of WASI.

Size: 1, Alignment: 1

### Enum Cases

- <a href="descriptor_type.unknown" name="descriptor_type.unknown"></a> [`unknown`](#descriptor_type.unknown)
  
  The type of the descriptor or file is unknown or is different from
  any of the other types specified.
  
- <a href="descriptor_type.block_device" name="descriptor_type.block_device"></a> [`block-device`](#descriptor_type.block_device)
  
  The descriptor refers to a block device inode.
  
- <a href="descriptor_type.character_device" name="descriptor_type.character_device"></a> [`character-device`](#descriptor_type.character_device)
  
  The descriptor refers to a character device inode.
  
- <a href="descriptor_type.directory" name="descriptor_type.directory"></a> [`directory`](#descriptor_type.directory)
  
  The descriptor refers to a directory inode.
  
- <a href="descriptor_type.fifo" name="descriptor_type.fifo"></a> [`fifo`](#descriptor_type.fifo)
  
  The descriptor refers to a named pipe.
  
- <a href="descriptor_type.symbolic_link" name="descriptor_type.symbolic_link"></a> [`symbolic-link`](#descriptor_type.symbolic_link)
  
  The file refers to a symbolic link inode.
  
- <a href="descriptor_type.regular_file" name="descriptor_type.regular_file"></a> [`regular-file`](#descriptor_type.regular_file)
  
  The descriptor refers to a regular file inode.
  
- <a href="descriptor_type.socket" name="descriptor_type.socket"></a> [`socket`](#descriptor_type.socket)
  
  The descriptor refers to a socket.
  
## <a href="#dir_entry" name="dir_entry"></a> `dir-entry`: record

A directory entry.

Size: 32, Alignment: 8

### Record Fields

- <a href="dir_entry.ino" name="dir_entry.ino"></a> [`ino`](#dir_entry.ino): option<[`inode`](#inode)>
  
  The serial number of the object referred to by this directory entry.
  May be none if the inode value is not known.
  
  When this is none, libc implementations might do an extra `stat-at`
  call to retrieve the inode number to fill their `d_ino` fields, so
  implementations which can set this to a non-none value should do so.
  
- <a href="dir_entry.type" name="dir_entry.type"></a> [`type`](#dir_entry.type): [`descriptor-type`](#descriptor_type)
  
  The type of the file referred to by this directory entry.
  
- <a href="dir_entry.name" name="dir_entry.name"></a> [`name`](#dir_entry.name): `string`
  
  The name of the object.
  
## <a href="#descriptor_flags" name="descriptor_flags"></a> `descriptor-flags`: record

Descriptor flags.

Note: This was called `fdflags` in earlier versions of WASI.

Size: 1, Alignment: 1

### Record Fields

- <a href="descriptor_flags.read" name="descriptor_flags.read"></a> [`read`](#descriptor_flags.read): 
  
  Read mode: Data can be read.
  Bit: 0

- <a href="descriptor_flags.write" name="descriptor_flags.write"></a> [`write`](#descriptor_flags.write): 
  
  Write mode: Data can be written to.
  Bit: 1

- <a href="descriptor_flags.nonblock" name="descriptor_flags.nonblock"></a> [`nonblock`](#descriptor_flags.nonblock): 
  
  Requests non-blocking operation.
  
  When this flag is enabled, functions may return immediately with an
  `errno::again` error code in situations where they would otherwise
  block. However, this non-blocking behavior is not required.
  Implementations are permitted to ignore this flag and block.
  Bit: 2

- <a href="descriptor_flags.sync" name="descriptor_flags.sync"></a> [`sync`](#descriptor_flags.sync): 
  
  Request that writes be performed according to synchronized I/O file
  integrity completion. The data stored in the file and the file's
  metadata are synchronized.
  
  The precise semantics of this operation have not yet been defined for
  WASI. At this time, it should be interpreted as a request, and not a
  requirement.
  Bit: 3

- <a href="descriptor_flags.dsync" name="descriptor_flags.dsync"></a> [`dsync`](#descriptor_flags.dsync): 
  
  Request that writes be performed according to synchronized I/O data
  integrity completion. Only the data stored in the file is
  synchronized.
  
  The precise semantics of this operation have not yet been defined for
  WASI. At this time, it should be interpreted as a request, and not a
  requirement.
  Bit: 4

- <a href="descriptor_flags.rsync" name="descriptor_flags.rsync"></a> [`rsync`](#descriptor_flags.rsync): 
  
  Requests that reads be performed at the same level of integrety
  requested for writes.
  
  The precise semantics of this operation have not yet been defined for
  WASI. At this time, it should be interpreted as a request, and not a
  requirement.
  Bit: 5

- <a href="descriptor_flags.mutate_directory" name="descriptor_flags.mutate_directory"></a> [`mutate-directory`](#descriptor_flags.mutate_directory): 
  
  Mutating directories mode: Directory contents may be mutated.
  
  When this flag is unset on a descriptor, operations using the
  descriptor which would create, rename, delete, modify the data or
  metadata of filesystem objects, or obtain another handle which
  would permit any of those, shall fail with `errno::rofs` if
  they would otherwise succeed.
  
  This may only be set on directories.
  Bit: 6

## <a href="#descriptor" name="descriptor"></a> `descriptor`: `u32`

A descriptor is a reference to a filesystem object, which may be a file,
directory, named pipe, special file, or other object on which filesystem
calls may be made.

Size: 4, Alignment: 4

## <a href="#new_timestamp" name="new_timestamp"></a> `new-timestamp`: variant

When setting a timestamp, this gives the value to set it to.

Size: 24, Alignment: 8

### Variant Cases

- <a href="new_timestamp.no_change" name="new_timestamp.no_change"></a> [`no-change`](#new_timestamp.no_change)
  
  Leave the timestamp set to its previous value.
  
- <a href="new_timestamp.now" name="new_timestamp.now"></a> [`now`](#new_timestamp.now)
  
  Set the timestamp to the current time of the system clock associated
  with the filesystem.
  
- <a href="new_timestamp.timestamp" name="new_timestamp.timestamp"></a> [`timestamp`](#new_timestamp.timestamp): [`datetime`](#datetime)
  
  Set the timestamp to the given value.
  
## <a href="#descriptor_stat" name="descriptor_stat"></a> `descriptor-stat`: record

File attributes.

Note: This was called `filestat` in earlier versions of WASI.

Size: 88, Alignment: 8

### Record Fields

- <a href="descriptor_stat.dev" name="descriptor_stat.dev"></a> [`dev`](#descriptor_stat.dev): [`device`](#device)
  
  Device ID of device containing the file.
  
- <a href="descriptor_stat.ino" name="descriptor_stat.ino"></a> [`ino`](#descriptor_stat.ino): [`inode`](#inode)
  
  File serial number.
  
- <a href="descriptor_stat.type" name="descriptor_stat.type"></a> [`type`](#descriptor_stat.type): [`descriptor-type`](#descriptor_type)
  
  File type.
  
- <a href="descriptor_stat.nlink" name="descriptor_stat.nlink"></a> [`nlink`](#descriptor_stat.nlink): [`linkcount`](#linkcount)
  
  Number of hard links to the file.
  
- <a href="descriptor_stat.size" name="descriptor_stat.size"></a> [`size`](#descriptor_stat.size): [`filesize`](#filesize)
  
  For regular files, the file size in bytes. For symbolic links, the length
  in bytes of the pathname contained in the symbolic link.
  
- <a href="descriptor_stat.atim" name="descriptor_stat.atim"></a> [`atim`](#descriptor_stat.atim): [`datetime`](#datetime)
  
  Last data access timestamp.
  
- <a href="descriptor_stat.mtim" name="descriptor_stat.mtim"></a> [`mtim`](#descriptor_stat.mtim): [`datetime`](#datetime)
  
  Last data modification timestamp.
  
- <a href="descriptor_stat.ctim" name="descriptor_stat.ctim"></a> [`ctim`](#descriptor_stat.ctim): [`datetime`](#datetime)
  
  Last file status change timestamp.
  
## <a href="#at_flags" name="at_flags"></a> `at-flags`: record

Flags determining the method of how paths are resolved.

Size: 1, Alignment: 1

### Record Fields

- <a href="at_flags.symlink_follow" name="at_flags.symlink_follow"></a> [`symlink-follow`](#at_flags.symlink_follow): 
  
  As long as the resolved path corresponds to a symbolic link, it is expanded.
  Bit: 0

## <a href="#advice" name="advice"></a> `advice`: enum

File or memory access pattern advisory information.

Size: 1, Alignment: 1

### Enum Cases

- <a href="advice.normal" name="advice.normal"></a> [`normal`](#advice.normal)
  
  The application has no advice to give on its behavior with respect to the specified data.
  
- <a href="advice.sequential" name="advice.sequential"></a> [`sequential`](#advice.sequential)
  
  The application expects to access the specified data sequentially from lower offsets to higher offsets.
  
- <a href="advice.random" name="advice.random"></a> [`random`](#advice.random)
  
  The application expects to access the specified data in a random order.
  
- <a href="advice.will_need" name="advice.will_need"></a> [`will-need`](#advice.will_need)
  
  The application expects to access the specified data in the near future.
  
- <a href="advice.dont_need" name="advice.dont_need"></a> [`dont-need`](#advice.dont_need)
  
  The application expects that it will not access the specified data in the near future.
  
- <a href="advice.no_reuse" name="advice.no_reuse"></a> [`no-reuse`](#advice.no_reuse)
  
  The application expects to access the specified data once and then not reuse it thereafter.
  
## Functions

----

#### <a href="#read_via_stream" name="read_via_stream"></a> `read-via-stream` 

Return a stream for reading from a file.

Note: This allows using `read-stream`, which is similar to `read` in POSIX.
##### Params

- <a href="#read_via_stream.this" name="read_via_stream.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#read_via_stream.offset" name="read_via_stream.offset"></a> `offset`: [`filesize`](#filesize)
##### Results

- <a href="#read_via_stream.result0" name="read_via_stream.result0"></a> `result0`: result<[`input-stream`](#input_stream), [`errno`](#errno)>

----

#### <a href="#write_via_stream" name="write_via_stream"></a> `write-via-stream` 

Return a stream for writing to a file.

Note: This allows using `write-stream`, which is similar to `write` in POSIX.
##### Params

- <a href="#write_via_stream.this" name="write_via_stream.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#write_via_stream.offset" name="write_via_stream.offset"></a> `offset`: [`filesize`](#filesize)
##### Results

- <a href="#write_via_stream.result0" name="write_via_stream.result0"></a> `result0`: result<[`output-stream`](#output_stream), [`errno`](#errno)>

----

#### <a href="#append_via_stream" name="append_via_stream"></a> `append-via-stream` 

Return a stream for appending to a file.

Note: This allows using `write-stream`, which is similar to `write` with
`O_APPEND` in in POSIX.
##### Params

- <a href="#append_via_stream.this" name="append_via_stream.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#append_via_stream.fd" name="append_via_stream.fd"></a> `fd`: [`descriptor`](#descriptor)
##### Results

- <a href="#append_via_stream.result0" name="append_via_stream.result0"></a> `result0`: result<[`output-stream`](#output_stream), [`errno`](#errno)>

----

#### <a href="#fadvise" name="fadvise"></a> `fadvise` 

Provide file advisory information on a descriptor.

This is similar to `posix_fadvise` in POSIX.
##### Params

- <a href="#fadvise.this" name="fadvise.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#fadvise.offset" name="fadvise.offset"></a> `offset`: [`filesize`](#filesize)
- <a href="#fadvise.len" name="fadvise.len"></a> `len`: [`filesize`](#filesize)
- <a href="#fadvise.advice" name="fadvise.advice"></a> `advice`: [`advice`](#advice)
##### Results

- <a href="#fadvise.result0" name="fadvise.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#datasync" name="datasync"></a> `datasync` 

Synchronize the data of a file to disk.

This function succeeds with no effect if the file descriptor is not
opened for writing.

Note: This is similar to `fdatasync` in POSIX.
##### Params

- <a href="#datasync.this" name="datasync.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#datasync.result0" name="datasync.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#flags" name="flags"></a> `flags` 

Get flags associated with a descriptor.

Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX.

Note: This returns the value that was the `fs_flags` value returned
from `fdstat_get` in earlier versions of WASI.
##### Params

- <a href="#flags.this" name="flags.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#flags.result0" name="flags.result0"></a> `result0`: result<[`descriptor-flags`](#descriptor_flags), [`errno`](#errno)>

----

#### <a href="#type" name="type"></a> `type` 

Get the dynamic type of a descriptor.

Note: This returns the same value as the `type` field of the `fd-stat`
returned by `stat`, `stat-at` and similar.

Note: This returns similar flags to the `st_mode & S_IFMT` value provided
by `fstat` in POSIX.

Note: This returns the value that was the `fs_filetype` value returned
from `fdstat_get` in earlier versions of WASI.
##### Params

- <a href="#type.this" name="type.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#type.result0" name="type.result0"></a> `result0`: result<[`descriptor-type`](#descriptor_type), [`errno`](#errno)>

----

#### <a href="#set_flags" name="set_flags"></a> `set-flags` 

Set status flags associated with a descriptor.

This function may only change the `append` and `nonblock` flags.

Note: This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX.

Note: This was called `fd_fdstat_set_flags` in earlier versions of WASI.
##### Params

- <a href="#set_flags.this" name="set_flags.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#set_flags.flags" name="set_flags.flags"></a> `flags`: [`descriptor-flags`](#descriptor_flags)
##### Results

- <a href="#set_flags.result0" name="set_flags.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#set_size" name="set_size"></a> `set-size` 

Adjust the size of an open file. If this increases the file's size, the
extra bytes are filled with zeros.

Note: This was called `fd_filestat_set_size` in earlier versions of WASI.
##### Params

- <a href="#set_size.this" name="set_size.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#set_size.size" name="set_size.size"></a> `size`: [`filesize`](#filesize)
##### Results

- <a href="#set_size.result0" name="set_size.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#set_times" name="set_times"></a> `set-times` 

Adjust the timestamps of an open file or directory.

Note: This is similar to `futimens` in POSIX.

Note: This was called `fd_filestat_set_times` in earlier versions of WASI.
##### Params

- <a href="#set_times.this" name="set_times.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#set_times.atim" name="set_times.atim"></a> `atim`: [`new-timestamp`](#new_timestamp)
- <a href="#set_times.mtim" name="set_times.mtim"></a> `mtim`: [`new-timestamp`](#new_timestamp)
##### Results

- <a href="#set_times.result0" name="set_times.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#pread" name="pread"></a> `pread` 

Read from a descriptor, without using and updating the descriptor's offset.

This function returns a list of bytes containing the data that was
read, along with a bool which, when true, indicates that the end of the
file was reached. The returned list will contain up to `len` bytes; it
may return fewer than requested, if the end of the file is reached or
if the I/O operation is interrupted.

Note: This is similar to `pread` in POSIX.
##### Params

- <a href="#pread.this" name="pread.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#pread.len" name="pread.len"></a> `len`: [`filesize`](#filesize)
- <a href="#pread.offset" name="pread.offset"></a> `offset`: [`filesize`](#filesize)
##### Results

- <a href="#pread.result0" name="pread.result0"></a> `result0`: result<(list<`u8`>, `bool`), [`errno`](#errno)>

----

#### <a href="#pwrite" name="pwrite"></a> `pwrite` 

Write to a descriptor, without using and updating the descriptor's offset.

It is valid to write past the end of a file; the file is extended to the
extent of the write, with bytes between the previous end and the start of
the write set to zero.

Note: This is similar to `pwrite` in POSIX.
##### Params

- <a href="#pwrite.this" name="pwrite.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#pwrite.buf" name="pwrite.buf"></a> `buf`: list<`u8`>
- <a href="#pwrite.offset" name="pwrite.offset"></a> `offset`: [`filesize`](#filesize)
##### Results

- <a href="#pwrite.result0" name="pwrite.result0"></a> `result0`: result<[`filesize`](#filesize), [`errno`](#errno)>

----

#### <a href="#readdir" name="readdir"></a> `readdir` 

Read directory entries from a directory.

On filesystems where directories contain entries referring to themselves
and their parents, often named `.` and `..` respectively, these entries
are omitted.

This always returns a new stream which starts at the beginning of the
directory.
##### Params

- <a href="#readdir.this" name="readdir.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#readdir.result0" name="readdir.result0"></a> `result0`: result<[`dir-entry-stream`](#dir_entry_stream), [`errno`](#errno)>

----

#### <a href="#sync" name="sync"></a> `sync` 

Synchronize the data and metadata of a file to disk.

This function succeeds with no effect if the file descriptor is not
opened for writing.

Note: This is similar to `fsync` in POSIX.
##### Params

- <a href="#sync.this" name="sync.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#sync.result0" name="sync.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#create_directory_at" name="create_directory_at"></a> `create-directory-at` 

Create a directory.

Note: This is similar to `mkdirat` in POSIX.
##### Params

- <a href="#create_directory_at.this" name="create_directory_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#create_directory_at.path" name="create_directory_at.path"></a> `path`: `string`
##### Results

- <a href="#create_directory_at.result0" name="create_directory_at.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#stat" name="stat"></a> `stat` 

Return the attributes of an open file or directory.

Note: This is similar to `fstat` in POSIX.

Note: This was called `fd_filestat_get` in earlier versions of WASI.
##### Params

- <a href="#stat.this" name="stat.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#stat.result0" name="stat.result0"></a> `result0`: result<[`descriptor-stat`](#descriptor_stat), [`errno`](#errno)>

----

#### <a href="#stat_at" name="stat_at"></a> `stat-at` 

Return the attributes of a file or directory.

Note: This is similar to `fstatat` in POSIX.

Note: This was called `path_filestat_get` in earlier versions of WASI.
##### Params

- <a href="#stat_at.this" name="stat_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#stat_at.at_flags" name="stat_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#stat_at.path" name="stat_at.path"></a> `path`: `string`
##### Results

- <a href="#stat_at.result0" name="stat_at.result0"></a> `result0`: result<[`descriptor-stat`](#descriptor_stat), [`errno`](#errno)>

----

#### <a href="#set_times_at" name="set_times_at"></a> `set-times-at` 

Adjust the timestamps of a file or directory.

Note: This is similar to `utimensat` in POSIX.

Note: This was called `path_filestat_set_times` in earlier versions of WASI.
##### Params

- <a href="#set_times_at.this" name="set_times_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#set_times_at.at_flags" name="set_times_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#set_times_at.path" name="set_times_at.path"></a> `path`: `string`
- <a href="#set_times_at.atim" name="set_times_at.atim"></a> `atim`: [`new-timestamp`](#new_timestamp)
- <a href="#set_times_at.mtim" name="set_times_at.mtim"></a> `mtim`: [`new-timestamp`](#new_timestamp)
##### Results

- <a href="#set_times_at.result0" name="set_times_at.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#link_at" name="link_at"></a> `link-at` 

Create a hard link.

Note: This is similar to `linkat` in POSIX.
##### Params

- <a href="#link_at.this" name="link_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#link_at.old_at_flags" name="link_at.old_at_flags"></a> `old-at-flags`: [`at-flags`](#at_flags)
- <a href="#link_at.old_path" name="link_at.old_path"></a> `old-path`: `string`
- <a href="#link_at.new_descriptor" name="link_at.new_descriptor"></a> `new-descriptor`: [`descriptor`](#descriptor)
- <a href="#link_at.new_path" name="link_at.new_path"></a> `new-path`: `string`
##### Results

- <a href="#link_at.result0" name="link_at.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#open_at" name="open_at"></a> `open-at` 

Open a file or directory.

The returned descriptor is not guaranteed to be the lowest-numbered
descriptor not currently open/ it is randomized to prevent applications
from depending on making assumptions about indexes, since this is
error-prone in multi-threaded contexts. The returned descriptor is
guaranteed to be less than 2**31.

If `flags` contains `descriptor-flags::mutate-directory`, and the base
descriptor doesn't have `descriptor-flags::mutate-directory` set,
`open-at` fails with `errno::rofs`.

If `flags` contains `write` or `append`, or `o-flags` contains `trunc`
or `create`, and the base descriptor doesn't have
`descriptor-flags::mutate-directory` set, `open-at` fails with
`errno::rofs`.

Note: This is similar to `openat` in POSIX.
##### Params

- <a href="#open_at.this" name="open_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#open_at.at_flags" name="open_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#open_at.path" name="open_at.path"></a> `path`: `string`
- <a href="#open_at.o_flags" name="open_at.o_flags"></a> `o-flags`: [`o-flags`](#o_flags)
- <a href="#open_at.flags" name="open_at.flags"></a> `flags`: [`descriptor-flags`](#descriptor_flags)
- <a href="#open_at.mode" name="open_at.mode"></a> `mode`: [`mode`](#mode)
##### Results

- <a href="#open_at.result0" name="open_at.result0"></a> `result0`: result<[`descriptor`](#descriptor), [`errno`](#errno)>

----

#### <a href="#readlink_at" name="readlink_at"></a> `readlink-at` 

Read the contents of a symbolic link.

Note: This is similar to `readlinkat` in POSIX.
##### Params

- <a href="#readlink_at.this" name="readlink_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#readlink_at.path" name="readlink_at.path"></a> `path`: `string`
##### Results

- <a href="#readlink_at.result0" name="readlink_at.result0"></a> `result0`: result<`string`, [`errno`](#errno)>

----

#### <a href="#remove_directory_at" name="remove_directory_at"></a> `remove-directory-at` 

Remove a directory.

Return `errno::notempty` if the directory is not empty.

Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
##### Params

- <a href="#remove_directory_at.this" name="remove_directory_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#remove_directory_at.path" name="remove_directory_at.path"></a> `path`: `string`
##### Results

- <a href="#remove_directory_at.result0" name="remove_directory_at.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#rename_at" name="rename_at"></a> `rename-at` 

Rename a filesystem object.

Note: This is similar to `renameat` in POSIX.
##### Params

- <a href="#rename_at.this" name="rename_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#rename_at.old_path" name="rename_at.old_path"></a> `old-path`: `string`
- <a href="#rename_at.new_descriptor" name="rename_at.new_descriptor"></a> `new-descriptor`: [`descriptor`](#descriptor)
- <a href="#rename_at.new_path" name="rename_at.new_path"></a> `new-path`: `string`
##### Results

- <a href="#rename_at.result0" name="rename_at.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#symlink_at" name="symlink_at"></a> `symlink-at` 

Create a symbolic link.

Note: This is similar to `symlinkat` in POSIX.
##### Params

- <a href="#symlink_at.this" name="symlink_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#symlink_at.old_path" name="symlink_at.old_path"></a> `old-path`: `string`
- <a href="#symlink_at.new_path" name="symlink_at.new_path"></a> `new-path`: `string`
##### Results

- <a href="#symlink_at.result0" name="symlink_at.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#unlink_file_at" name="unlink_file_at"></a> `unlink-file-at` 

Unlink a filesystem object that is not a directory.

Return `errno::isdir` if the path refers to a directory.
Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.
##### Params

- <a href="#unlink_file_at.this" name="unlink_file_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#unlink_file_at.path" name="unlink_file_at.path"></a> `path`: `string`
##### Results

- <a href="#unlink_file_at.result0" name="unlink_file_at.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#change_file_permissions_at" name="change_file_permissions_at"></a> `change-file-permissions-at` 

Change the permissions of a filesystem object that is not a directory.

Note that the ultimate meanings of these permissions is
filesystem-specific.

Note: This is similar to `fchmodat` in POSIX.
##### Params

- <a href="#change_file_permissions_at.this" name="change_file_permissions_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#change_file_permissions_at.at_flags" name="change_file_permissions_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#change_file_permissions_at.path" name="change_file_permissions_at.path"></a> `path`: `string`
- <a href="#change_file_permissions_at.mode" name="change_file_permissions_at.mode"></a> `mode`: [`mode`](#mode)
##### Results

- <a href="#change_file_permissions_at.result0" name="change_file_permissions_at.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#change_directory_permissions_at" name="change_directory_permissions_at"></a> `change-directory-permissions-at` 

Change the permissions of a directory.

Note that the ultimate meanings of these permissions is
filesystem-specific.

Unlike in POSIX, the `executable` flag is not reinterpreted as a "search"
flag. `read` on a directory implies readability and searchability, and
`execute` is not valid for directories.

Note: This is similar to `fchmodat` in POSIX.
##### Params

- <a href="#change_directory_permissions_at.this" name="change_directory_permissions_at.this"></a> `this`: [`descriptor`](#descriptor)
- <a href="#change_directory_permissions_at.at_flags" name="change_directory_permissions_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#change_directory_permissions_at.path" name="change_directory_permissions_at.path"></a> `path`: `string`
- <a href="#change_directory_permissions_at.mode" name="change_directory_permissions_at.mode"></a> `mode`: [`mode`](#mode)
##### Results

- <a href="#change_directory_permissions_at.result0" name="change_directory_permissions_at.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#lock_shared" name="lock_shared"></a> `lock-shared` 

Request a shared advisory lock for an open file.

This requests a *shared* lock; more than one shared lock can be held for
a file at the same time.

If the open file has an exclusive lock, this function downgrades the lock
to a shared lock. If it has a shared lock, this function has no effect.

This requests an *advisory* lock, meaning that the file could be accessed
by other programs that don't hold the lock.

It is unspecified how shared locks interact with locks acquired by
non-WASI programs.

This function blocks until the lock can be acquired.

Not all filesystems support locking; on filesystems which don't support
locking, this function returns `errno::notsup`.

Note: This is similar to `flock(fd, LOCK_SH)` in Unix.
##### Params

- <a href="#lock_shared.this" name="lock_shared.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#lock_shared.result0" name="lock_shared.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#lock_exclusive" name="lock_exclusive"></a> `lock-exclusive` 

Request an exclusive advisory lock for an open file.

This requests an *exclusive* lock; no other locks may be held for the
file while an exclusive lock is held.

If the open file has a shared lock and there are no exclusive locks held
for the file, this function upgrades the lock to an exclusive lock. If the
open file already has an exclusive lock, this function has no effect.

This requests an *advisory* lock, meaning that the file could be accessed
by other programs that don't hold the lock.

It is unspecified whether this function succeeds if the file descriptor
is not opened for writing. It is unspecified how exclusive locks interact
with locks acquired by non-WASI programs.

This function blocks until the lock can be acquired.

Not all filesystems support locking; on filesystems which don't support
locking, this function returns `errno::notsup`.

Note: This is similar to `flock(fd, LOCK_EX)` in Unix.
##### Params

- <a href="#lock_exclusive.this" name="lock_exclusive.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#lock_exclusive.result0" name="lock_exclusive.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#try_lock_shared" name="try_lock_shared"></a> `try-lock-shared` 

Request a shared advisory lock for an open file.

This requests a *shared* lock; more than one shared lock can be held for
a file at the same time.

If the open file has an exclusive lock, this function downgrades the lock
to a shared lock. If it has a shared lock, this function has no effect.

This requests an *advisory* lock, meaning that the file could be accessed
by other programs that don't hold the lock.

It is unspecified how shared locks interact with locks acquired by
non-WASI programs.

This function returns `errno::again` if the lock cannot be acquired.

Not all filesystems support locking; on filesystems which don't support
locking, this function returns `errno::notsup`.

Note: This is similar to `flock(fd, LOCK_SH | LOCK_NB)` in Unix.
##### Params

- <a href="#try_lock_shared.this" name="try_lock_shared.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#try_lock_shared.result0" name="try_lock_shared.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#try_lock_exclusive" name="try_lock_exclusive"></a> `try-lock-exclusive` 

Request an exclusive advisory lock for an open file.

This requests an *exclusive* lock; no other locks may be held for the
file while an exclusive lock is held.

If the open file has a shared lock and there are no exclusive locks held
for the file, this function upgrades the lock to an exclusive lock. If the
open file already has an exclusive lock, this function has no effect.

This requests an *advisory* lock, meaning that the file could be accessed
by other programs that don't hold the lock.

It is unspecified whether this function succeeds if the file descriptor
is not opened for writing. It is unspecified how exclusive locks interact
with locks acquired by non-WASI programs.

This function returns `errno::again` if the lock cannot be acquired.

Not all filesystems support locking; on filesystems which don't support
locking, this function returns `errno::notsup`.

Note: This is similar to `flock(fd, LOCK_EX | LOCK_NB)` in Unix.
##### Params

- <a href="#try_lock_exclusive.this" name="try_lock_exclusive.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#try_lock_exclusive.result0" name="try_lock_exclusive.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#unlock" name="unlock"></a> `unlock` 

Release a shared or exclusive lock on an open file.

Note: This is similar to `flock(fd, LOCK_UN)` in Unix.
##### Params

- <a href="#unlock.this" name="unlock.this"></a> `this`: [`descriptor`](#descriptor)
##### Results

- <a href="#unlock.result0" name="unlock.result0"></a> `result0`: result<_, [`errno`](#errno)>

----

#### <a href="#drop_descriptor" name="drop_descriptor"></a> `drop-descriptor` 

Dispose of the specified `descriptor`, after which it may no longer
be used.
##### Params

- <a href="#drop_descriptor.this" name="drop_descriptor.this"></a> `this`: [`descriptor`](#descriptor)

----

#### <a href="#read_dir_entry" name="read_dir_entry"></a> `read-dir-entry` 

Read a single directory entry from a `dir-entry-stream`.
##### Params

- <a href="#read_dir_entry.this" name="read_dir_entry.this"></a> `this`: [`dir-entry-stream`](#dir_entry_stream)
##### Results

- <a href="#read_dir_entry.result0" name="read_dir_entry.result0"></a> `result0`: result<option<[`dir-entry`](#dir_entry)>, [`errno`](#errno)>

----

#### <a href="#drop_dir_entry_stream" name="drop_dir_entry_stream"></a> `drop-dir-entry-stream` 

Dispose of the specified `dir-entry-stream`, after which it may no longer
be used.
##### Params

- <a href="#drop_dir_entry_stream.this" name="drop_dir_entry_stream.this"></a> `this`: [`dir-entry-stream`](#dir_entry_stream)

