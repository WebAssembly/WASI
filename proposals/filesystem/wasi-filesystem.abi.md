# Types

## <a href="#filesize" name="filesize"></a> `filesize`: `u64`

  Non-negative file size or length of a region within a file.

Size: 8, Alignment: 8

## <a href="#filedelta" name="filedelta"></a> `filedelta`: `s64`

  Relative offset within a file.

Size: 8, Alignment: 8

## <a href="#datetime" name="datetime"></a> `datetime`: record

  Timestamp in seconds and nanoseconds.

Size: 16, Alignment: 8

### Record Fields

- <a href="datetime.seconds" name="datetime.seconds"></a> [`seconds`](#datetime.seconds): `u64`


- <a href="datetime.nanoseconds" name="datetime.nanoseconds"></a> [`nanoseconds`](#datetime.nanoseconds): `u32`


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

## <a href="#descriptor_flags" name="descriptor_flags"></a> `descriptor-flags`: flags

  Descriptor flags.
  
  Note: This was called `fdflags` in earlier versions of WASI.

Size: 1, Alignment: 1

### Flags Fields

- <a href="descriptor_flags.read" name="descriptor_flags.read"></a> [`read`](#descriptor_flags.read)

  Read mode: Data can be read.
Bit: 0

- <a href="descriptor_flags.write" name="descriptor_flags.write"></a> [`write`](#descriptor_flags.write)

  Write mode: Data can be written to.
Bit: 1

- <a href="descriptor_flags.append" name="descriptor_flags.append"></a> [`append`](#descriptor_flags.append)

  Append mode: Data written to the file is always appended to the file's
  end.
Bit: 2

- <a href="descriptor_flags.dsync" name="descriptor_flags.dsync"></a> [`dsync`](#descriptor_flags.dsync)

  Write according to synchronized I/O data integrity completion. Only the
  data stored in the file is synchronized.
Bit: 3

- <a href="descriptor_flags.nonblock" name="descriptor_flags.nonblock"></a> [`nonblock`](#descriptor_flags.nonblock)

  Non-blocking mode.
Bit: 4

- <a href="descriptor_flags.rsync" name="descriptor_flags.rsync"></a> [`rsync`](#descriptor_flags.rsync)

  Synchronized read I/O operations.
Bit: 5

- <a href="descriptor_flags.sync" name="descriptor_flags.sync"></a> [`sync`](#descriptor_flags.sync)

  Write according to synchronized I/O file integrity completion. In
  addition to synchronizing the data stored in the file, the
  implementation may also synchronously update the file's metadata.
Bit: 6

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

## <a href="#at_flags" name="at_flags"></a> `at-flags`: flags

  Flags determining the method of how paths are resolved.

Size: 1, Alignment: 1

### Flags Fields

- <a href="at_flags.symlink_follow" name="at_flags.symlink_follow"></a> [`symlink-follow`](#at_flags.symlink_follow)

  As long as the resolved path corresponds to a symbolic link, it is expanded.
Bit: 0

## <a href="#o_flags" name="o_flags"></a> `o-flags`: flags

  Open flags used by `open-at`.

Size: 1, Alignment: 1

### Flags Fields

- <a href="o_flags.create" name="o_flags.create"></a> [`create`](#o_flags.create)

  Create file if it does not exist.
Bit: 0

- <a href="o_flags.directory" name="o_flags.directory"></a> [`directory`](#o_flags.directory)

  Fail if not a directory.
Bit: 1

- <a href="o_flags.excl" name="o_flags.excl"></a> [`excl`](#o_flags.excl)

  Fail if file already exists.
Bit: 2

- <a href="o_flags.trunc" name="o_flags.trunc"></a> [`trunc`](#o_flags.trunc)

  Truncate file to size 0.
Bit: 3

## <a href="#mode" name="mode"></a> `mode`: flags

  Permissions mode used by `open-at`, `change-file-permissions-at`, and
  similar.

Size: 1, Alignment: 1

### Flags Fields

- <a href="mode.readable" name="mode.readable"></a> [`readable`](#mode.readable)

  True if the resource is considered readable by the containing
  filesystem.
Bit: 0

- <a href="mode.writeable" name="mode.writeable"></a> [`writeable`](#mode.writeable)

  True if the resource is considered writeable by the containing
  filesystem.
Bit: 1

- <a href="mode.executable" name="mode.executable"></a> [`executable`](#mode.executable)

  True if the resource is considered executable by the containing
  filesystem. This does not apply to directories.
Bit: 2

## <a href="#linkcount" name="linkcount"></a> `linkcount`: `u64`

  Number of hard links to an inode.

Size: 8, Alignment: 8

## <a href="#device" name="device"></a> `device`: `u64`

  Identifier for a device containing a file system. Can be used in combination
  with `inode` to uniquely identify a file or directory in the filesystem.

Size: 8, Alignment: 8

## <a href="#inode" name="inode"></a> `inode`: `u64`

  Filesystem object serial number that is unique within its file system.

Size: 8, Alignment: 8

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

- <a href="errno.child" name="errno.child"></a> [`child`](#errno.child)

  No child processes.

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

## <a href="#seek_from" name="seek_from"></a> `seek-from`: variant

  The position relative to which to set the offset of the descriptor.

Size: 16, Alignment: 8

### Variant Cases

- <a href="seek_from.set" name="seek_from.set"></a> [`set`](#seek_from.set): [`filesize`](#filesize)

  Seek relative to start-of-file.

- <a href="seek_from.cur" name="seek_from.cur"></a> [`cur`](#seek_from.cur): [`filedelta`](#filedelta)

  Seek relative to current position.

- <a href="seek_from.end" name="seek_from.end"></a> [`end`](#seek_from.end): [`filesize`](#filesize)

  Seek relative to end-of-file.

# Functions

----

#### <a href="#descriptor_fadvise" name="descriptor_fadvise"></a> `descriptor::fadvise` 

  Provide file advisory information on a descriptor.
  
  This is similar to `posix_fadvise` in POSIX.
##### Params

- <a href="#descriptor_fadvise.self" name="descriptor_fadvise.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_fadvise.offset" name="descriptor_fadvise.offset"></a> `offset`: [`filesize`](#filesize)
- <a href="#descriptor_fadvise.len" name="descriptor_fadvise.len"></a> `len`: [`filesize`](#filesize)
- <a href="#descriptor_fadvise.advice" name="descriptor_fadvise.advice"></a> `advice`: [`advice`](#advice)
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_datasync" name="descriptor_datasync"></a> `descriptor::datasync` 

  Synchronize the data of a file to disk.
  
  Note: This is similar to `fdatasync` in POSIX.
##### Params

- <a href="#descriptor_datasync.self" name="descriptor_datasync.self"></a> `self`: handle<descriptor>
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_flags" name="descriptor_flags"></a> `descriptor::flags` 

  Get flags associated with a descriptor.
  
  Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX.
  
  Note: This returns the value that was the `fs_flags` value returned
  from `fdstat_get` in earlier versions of WASI.
##### Params

- <a href="#descriptor_flags.self" name="descriptor_flags.self"></a> `self`: handle<descriptor>
##### Results

- result<[`descriptor-flags`](#descriptor_flags), [`errno`](#errno)>

----

#### <a href="#descriptor_type" name="descriptor_type"></a> `descriptor::type` 

  Get the dynamic type of a descriptor.
  
  Note: This returns the same value as the `type` field of the `fd-stat`
  returned by `stat`, `stat-at` and similar.
  
  Note: This returns similar flags to the `st_mode & S_IFMT` value provided
  by `fstat` in POSIX.
  
  Note: This returns the value that was the `fs_filetype` value returned
  from `fdstat_get` in earlier versions of WASI.
##### Params

- <a href="#descriptor_type.self" name="descriptor_type.self"></a> `self`: handle<descriptor>
##### Results

- result<[`descriptor-type`](#descriptor_type), [`errno`](#errno)>

----

#### <a href="#descriptor_set_flags" name="descriptor_set_flags"></a> `descriptor::set-flags` 

  Set flags associated with a descriptor.
  
  Note: This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX.
  
  Note: This was called `fd_fdstat_set_flags` in earlier versions of WASI.
##### Params

- <a href="#descriptor_set_flags.self" name="descriptor_set_flags.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_set_flags.flags" name="descriptor_set_flags.flags"></a> `flags`: [`descriptor-flags`](#descriptor_flags)
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_set_size" name="descriptor_set_size"></a> `descriptor::set-size` 

  Adjust the size of an open file. If this increases the file's size, the
  extra bytes are filled with zeros.
  
  Note: This was called `fd_filestat_set_size` in earlier versions of WASI.
##### Params

- <a href="#descriptor_set_size.self" name="descriptor_set_size.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_set_size.size" name="descriptor_set_size.size"></a> `size`: [`filesize`](#filesize)
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_set_times" name="descriptor_set_times"></a> `descriptor::set-times` 

  Adjust the timestamps of an open file or directory.
  
  Note: This is similar to `futimens` in POSIX.
  
  Note: This was called `fd_filestat_set_times` in earlier versions of WASI.
##### Params

- <a href="#descriptor_set_times.self" name="descriptor_set_times.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_set_times.atim" name="descriptor_set_times.atim"></a> `atim`: [`new-timestamp`](#new_timestamp)
- <a href="#descriptor_set_times.mtim" name="descriptor_set_times.mtim"></a> `mtim`: [`new-timestamp`](#new_timestamp)
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_pread" name="descriptor_pread"></a> `descriptor::pread` 

  Read from a descriptor, without using and updating the descriptor's offset.
  
  Note: This is similar to `pread` in POSIX.
##### Params

- <a href="#descriptor_pread.self" name="descriptor_pread.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_pread.len" name="descriptor_pread.len"></a> `len`: [`filesize`](#filesize)
- <a href="#descriptor_pread.offset" name="descriptor_pread.offset"></a> `offset`: [`filesize`](#filesize)
##### Results

- stream<`u8`, [`errno`](#errno)>

----

#### <a href="#descriptor_pwrite" name="descriptor_pwrite"></a> `descriptor::pwrite` 

  Write to a descriptor, without using and updating the descriptor's offset.
  
  It is valid to write past the end of a file; the file is extended to the
  extent of the write, with bytes between the previous end and the start of
  the write set to zero.
  
  Note: This is similar to `pwrite` in POSIX.
##### Params

- <a href="#descriptor_pwrite.self" name="descriptor_pwrite.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_pwrite.buf" name="descriptor_pwrite.buf"></a> `buf`: stream<`u8`>
- <a href="#descriptor_pwrite.offset" name="descriptor_pwrite.offset"></a> `offset`: [`filesize`](#filesize)
##### Results

- future<result<[`filesize`](#filesize), [`errno`](#errno)>>

----

#### <a href="#descriptor_readdir" name="descriptor_readdir"></a> `descriptor::readdir` 

  Read directory entries from a directory.
  
  This always returns a new stream which starts at the beginning of the
  directory.
##### Params

- <a href="#descriptor_readdir.self" name="descriptor_readdir.self"></a> `self`: handle<descriptor>
##### Results

- stream<[`dir-entry`](#dir_entry), [`errno`](#errno)>

----

#### <a href="#descriptor_seek" name="descriptor_seek"></a> `descriptor::seek` 

  Move the offset of a file descriptor.
  
  If the descriptor refers to a directory, this function fails with
  `errno::spipe`.
  
  Returns new offset of the descriptor, relative to the start of the file.
  
  It is valid to seek past the end of a file. The file size is not modified
  until a write is performed, at which time the file is extended to the
  extent of the write, with bytes between the previous end and the start of
  the write set to zero.
  
  Note: This is similar to `lseek` in POSIX.
##### Params

- <a href="#descriptor_seek.self" name="descriptor_seek.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_seek.from" name="descriptor_seek.from"></a> `from`: [`seek-from`](#seek_from)
##### Results

- result<[`filesize`](#filesize), [`errno`](#errno)>

----

#### <a href="#descriptor_sync" name="descriptor_sync"></a> `descriptor::sync` 

  Synchronize the data and metadata of a file to disk.
  
  Note: This is similar to `fsync` in POSIX.
##### Params

- <a href="#descriptor_sync.self" name="descriptor_sync.self"></a> `self`: handle<descriptor>
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_tell" name="descriptor_tell"></a> `descriptor::tell` 

  Return the current offset of a descriptor.
  
  If the descriptor refers to a directory, this function fails with
  `errno::spipe`.
  
  Returns the current offset of the descriptor, relative to the start of the file.
  
  Note: This is similar to `lseek(fd, 0, SEEK_CUR)` in POSIX.
##### Params

- <a href="#descriptor_tell.self" name="descriptor_tell.self"></a> `self`: handle<descriptor>
##### Results

- result<[`filesize`](#filesize), [`errno`](#errno)>

----

#### <a href="#descriptor_create_directory_at" name="descriptor_create_directory_at"></a> `descriptor::create-directory-at` 

  Create a directory.
  
  Note: This is similar to `mkdirat` in POSIX.
##### Params

- <a href="#descriptor_create_directory_at.self" name="descriptor_create_directory_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_create_directory_at.path" name="descriptor_create_directory_at.path"></a> `path`: `string`
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_stat" name="descriptor_stat"></a> `descriptor::stat` 

  Return the attributes of an open file or directory.
  
  Note: This is similar to `fstat` in POSIX.
  
  Note: This was called `fd_filestat_get` in earlier versions of WASI.
##### Params

- <a href="#descriptor_stat.self" name="descriptor_stat.self"></a> `self`: handle<descriptor>
##### Results

- result<[`descriptor-stat`](#descriptor_stat), [`errno`](#errno)>

----

#### <a href="#descriptor_stat_at" name="descriptor_stat_at"></a> `descriptor::stat-at` 

  Return the attributes of a file or directory.
  
  Note: This is similar to `fstatat` in POSIX.
  
  Note: This was called `path_filestat_get` in earlier versions of WASI.
##### Params

- <a href="#descriptor_stat_at.self" name="descriptor_stat_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_stat_at.at_flags" name="descriptor_stat_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#descriptor_stat_at.path" name="descriptor_stat_at.path"></a> `path`: `string`
##### Results

- result<[`descriptor-stat`](#descriptor_stat), [`errno`](#errno)>

----

#### <a href="#descriptor_set_times_at" name="descriptor_set_times_at"></a> `descriptor::set-times-at` 

  Adjust the timestamps of a file or directory.
  
  Note: This is similar to `utimensat` in POSIX.
  
  Note: This was called `path_filestat_set_times` in earlier versions of WASI.
##### Params

- <a href="#descriptor_set_times_at.self" name="descriptor_set_times_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_set_times_at.at_flags" name="descriptor_set_times_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#descriptor_set_times_at.path" name="descriptor_set_times_at.path"></a> `path`: `string`
- <a href="#descriptor_set_times_at.atim" name="descriptor_set_times_at.atim"></a> `atim`: [`new-timestamp`](#new_timestamp)
- <a href="#descriptor_set_times_at.mtim" name="descriptor_set_times_at.mtim"></a> `mtim`: [`new-timestamp`](#new_timestamp)
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_link_at" name="descriptor_link_at"></a> `descriptor::link-at` 

  Create a hard link.
  
  Note: This is similar to `linkat` in POSIX.
##### Params

- <a href="#descriptor_link_at.self" name="descriptor_link_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_link_at.old_at_flags" name="descriptor_link_at.old_at_flags"></a> `old-at-flags`: [`at-flags`](#at_flags)
- <a href="#descriptor_link_at.old_path" name="descriptor_link_at.old_path"></a> `old-path`: `string`
- <a href="#descriptor_link_at.new_descriptor" name="descriptor_link_at.new_descriptor"></a> `new-descriptor`: handle<descriptor>
- <a href="#descriptor_link_at.new_path" name="descriptor_link_at.new_path"></a> `new-path`: `string`
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_open_at" name="descriptor_open_at"></a> `descriptor::open-at` 

  Open a file or directory.
  
  The returned descriptor is not guaranteed to be the lowest-numbered
  descriptor not currently open/ it is randomized to prevent applications
  from depending on making assumptions about indexes, since this is
  error-prone in multi-threaded contexts. The returned descriptor is
  guaranteed to be less than 2**31.
  
  Note: This is similar to `openat` in POSIX.
##### Params

- <a href="#descriptor_open_at.self" name="descriptor_open_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_open_at.at_flags" name="descriptor_open_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#descriptor_open_at.path" name="descriptor_open_at.path"></a> `path`: `string`
- <a href="#descriptor_open_at.o_flags" name="descriptor_open_at.o_flags"></a> `o-flags`: [`o-flags`](#o_flags)
- <a href="#descriptor_open_at.flags" name="descriptor_open_at.flags"></a> `flags`: [`descriptor-flags`](#descriptor_flags)
- <a href="#descriptor_open_at.mode" name="descriptor_open_at.mode"></a> `mode`: [`mode`](#mode)
##### Results

- result<handle<descriptor>, [`errno`](#errno)>

----

#### <a href="#descriptor_readlink_at" name="descriptor_readlink_at"></a> `descriptor::readlink-at` 

  Read the contents of a symbolic link.
  
  Note: This is similar to `readlinkat` in POSIX.
##### Params

- <a href="#descriptor_readlink_at.self" name="descriptor_readlink_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_readlink_at.path" name="descriptor_readlink_at.path"></a> `path`: `string`
##### Results

- result<`string`, [`errno`](#errno)>

----

#### <a href="#descriptor_remove_directory_at" name="descriptor_remove_directory_at"></a> `descriptor::remove-directory-at` 

  Remove a directory.
  
  Return `errno::notempty` if the directory is not empty.
  
  Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
##### Params

- <a href="#descriptor_remove_directory_at.self" name="descriptor_remove_directory_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_remove_directory_at.path" name="descriptor_remove_directory_at.path"></a> `path`: `string`
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_rename_at" name="descriptor_rename_at"></a> `descriptor::rename-at` 

  Rename a filesystem object.
  
  Note: This is similar to `renameat` in POSIX.
##### Params

- <a href="#descriptor_rename_at.self" name="descriptor_rename_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_rename_at.old_path" name="descriptor_rename_at.old_path"></a> `old-path`: `string`
- <a href="#descriptor_rename_at.new_descriptor" name="descriptor_rename_at.new_descriptor"></a> `new-descriptor`: handle<descriptor>
- <a href="#descriptor_rename_at.new_path" name="descriptor_rename_at.new_path"></a> `new-path`: `string`
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_symlink_at" name="descriptor_symlink_at"></a> `descriptor::symlink-at` 

  Create a symbolic link.
  
  Note: This is similar to `symlinkat` in POSIX.
##### Params

- <a href="#descriptor_symlink_at.self" name="descriptor_symlink_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_symlink_at.old_path" name="descriptor_symlink_at.old_path"></a> `old-path`: `string`
- <a href="#descriptor_symlink_at.new_path" name="descriptor_symlink_at.new_path"></a> `new-path`: `string`
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_unlink_file_at" name="descriptor_unlink_file_at"></a> `descriptor::unlink-file-at` 

  Unlink a filesystem object that is not a directory.
  
  Return `errno::isdir` if the path refers to a directory.
  Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.
##### Params

- <a href="#descriptor_unlink_file_at.self" name="descriptor_unlink_file_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_unlink_file_at.path" name="descriptor_unlink_file_at.path"></a> `path`: `string`
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_change_file_permissions_at" name="descriptor_change_file_permissions_at"></a> `descriptor::change-file-permissions-at` 

  Change the permissions of a filesystem object that is not a directory.
  
  Note that the ultimate meanings of these permissions is
  filesystem-specific.
  
  Note: This is similar to `fchmodat` in POSIX.
##### Params

- <a href="#descriptor_change_file_permissions_at.self" name="descriptor_change_file_permissions_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_change_file_permissions_at.at_flags" name="descriptor_change_file_permissions_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#descriptor_change_file_permissions_at.path" name="descriptor_change_file_permissions_at.path"></a> `path`: `string`
- <a href="#descriptor_change_file_permissions_at.mode" name="descriptor_change_file_permissions_at.mode"></a> `mode`: [`mode`](#mode)
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_change_directory_permissions_at" name="descriptor_change_directory_permissions_at"></a> `descriptor::change-directory-permissions-at` 

  Change the permissions of a directory.
  
  Note that the ultimate meanings of these permissions is
  filesystem-specific.
  
  Unlike in POSIX, the `executable` flag is not reinterpreted as a "search"
  flag. `read` on a directory implies readability and searchability, and
  `execute` is not valid for directories.
  
  Note: This is similar to `fchmodat` in POSIX.
##### Params

- <a href="#descriptor_change_directory_permissions_at.self" name="descriptor_change_directory_permissions_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_change_directory_permissions_at.at_flags" name="descriptor_change_directory_permissions_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#descriptor_change_directory_permissions_at.path" name="descriptor_change_directory_permissions_at.path"></a> `path`: `string`
- <a href="#descriptor_change_directory_permissions_at.mode" name="descriptor_change_directory_permissions_at.mode"></a> `mode`: [`mode`](#mode)
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_lock_shared" name="descriptor_lock_shared"></a> `descriptor::lock-shared` 

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

- <a href="#descriptor_lock_shared.self" name="descriptor_lock_shared.self"></a> `self`: handle<descriptor>
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_lock_exclusive" name="descriptor_lock_exclusive"></a> `descriptor::lock-exclusive` 

  Request an exclusive advisory lock for an open file.
  
  This requests an *exclusive* lock; no other locks may be held for the
  file while an exclusive lock is held.
  
  If the open file has a shared lock and there are no exclusive locks held
  for the fhile, this function upgrades the lock to an exclusive lock. If the
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

- <a href="#descriptor_lock_exclusive.self" name="descriptor_lock_exclusive.self"></a> `self`: handle<descriptor>
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_try_lock_shared" name="descriptor_try_lock_shared"></a> `descriptor::try-lock-shared` 

  Request a shared advisory lock for an open file.
  
  This requests a *shared* lock; more than one shared lock can be held for
  a file at the same time.
  
  If the open file has an exclusive lock, this function downgrades the lock
  to a shared lock. If it has a shared lock, this function has no effect.
  
  This requests an *advisory* lock, meaning that the file could be accessed
  by other programs that don't hold the lock.
  
  It is unspecified how shared locks interact with locks acquired by
  non-WASI programs.
  
  This function returns `errno::wouldblock` if the lock cannot be acquired.
  
  Not all filesystems support locking; on filesystems which don't support
  locking, this function returns `errno::notsup`.
  
  Note: This is similar to `flock(fd, LOCK_SH | LOCK_NB)` in Unix.
##### Params

- <a href="#descriptor_try_lock_shared.self" name="descriptor_try_lock_shared.self"></a> `self`: handle<descriptor>
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_try_lock_exclusive" name="descriptor_try_lock_exclusive"></a> `descriptor::try-lock-exclusive` 

  Request an exclusive advisory lock for an open file.
  
  This requests an *exclusive* lock; no other locks may be held for the
  file while an exclusive lock is held.
  
  If the open file has a shared lock and there are no exclusive locks held
  for the fhile, this function upgrades the lock to an exclusive lock. If the
  open file already has an exclusive lock, this function has no effect.
  
  This requests an *advisory* lock, meaning that the file could be accessed
  by other programs that don't hold the lock.
  
  It is unspecified whether this function succeeds if the file descriptor
  is not opened for writing. It is unspecified how exclusive locks interact
  with locks acquired by non-WASI programs.
  
  This function returns `errno::wouldblock` if the lock cannot be acquired.
  
  Not all filesystems support locking; on filesystems which don't support
  locking, this function returns `errno::notsup`.
  
  Note: This is similar to `flock(fd, LOCK_EX | LOCK_NB)` in Unix.
##### Params

- <a href="#descriptor_try_lock_exclusive.self" name="descriptor_try_lock_exclusive.self"></a> `self`: handle<descriptor>
##### Results

- result<_, [`errno`](#errno)>

----

#### <a href="#descriptor_unlock" name="descriptor_unlock"></a> `descriptor::unlock` 

  Release a shared or exclusive lock on an open file.
  
  Note: This is similar to `flock(fd, LOCK_UN)` in Unix.
##### Params

- <a href="#descriptor_unlock.self" name="descriptor_unlock.self"></a> `self`: handle<descriptor>
##### Results

- result<_, [`errno`](#errno)>

