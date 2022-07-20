# Types

## <a href="#size" name="size"></a> `size`: `u32`

  Size of a range of bytes in memory.

Size: 4, Alignment: 4

## <a href="#filesize" name="filesize"></a> `filesize`: `u64`

  Non-negative file size or length of a region within a file.

Size: 8, Alignment: 8

## <a href="#filedelta" name="filedelta"></a> `filedelta`: `s64`

  Relative offset within a file.

Size: 8, Alignment: 8

## <a href="#timestamp" name="timestamp"></a> `timestamp`: `u64`

  Timestamp in nanoseconds.
  
  TODO: wasi-clocks is moving to seconds+nanoseconds.

Size: 8, Alignment: 8

## <a href="#info" name="info"></a> `info`: record

  Information associated with a descriptor.
  
  Note: This was called `fdstat` in earlier versions of WASI.

Size: 2, Alignment: 1

### Record Fields

- <a href="info.type" name="info.type"></a> [`type`](#info.type): [`type`](#type)

  The type of filesystem object referenced by a descriptor.

- <a href="info.flags" name="info.flags"></a> [`flags`](#info.flags): [`flags`](#flags)

  Flags associated with a descriptor.

## <a href="#type" name="type"></a> `type`: enum

  The type of a filesystem object referenced by a descriptor.
  
  Note: This was called `filetype` in earlier versions of WASI.

Size: 1, Alignment: 1

### Enum Cases

- <a href="type.unknown" name="type.unknown"></a> [`unknown`](#type.unknown)

  The type of the descriptor or file is unknown or is different from
  any of the other types specified.

- <a href="type.block_device" name="type.block_device"></a> [`block-device`](#type.block_device)

  The descriptor refers to a block device inode.

- <a href="type.character_device" name="type.character_device"></a> [`character-device`](#type.character_device)

  The descriptor refers to a character device inode.

- <a href="type.directory" name="type.directory"></a> [`directory`](#type.directory)

  The descriptor refers to a directory inode.

- <a href="type.fifo" name="type.fifo"></a> [`fifo`](#type.fifo)

  The descriptor refers to a named pipe.

- <a href="type.symbolic_link" name="type.symbolic_link"></a> [`symbolic-link`](#type.symbolic_link)

  The file refers to a symbolic link inode.

- <a href="type.regular_file" name="type.regular_file"></a> [`regular-file`](#type.regular_file)

  The descriptor refers to a regular file inode.

- <a href="type.socket" name="type.socket"></a> [`socket`](#type.socket)

  The descriptor refers to a socket.

## <a href="#flags" name="flags"></a> `flags`: flags

  Descriptor flags.
  
  Note: This was called `fd-flags` in earlier versions of WASI.

Size: 1, Alignment: 1

### Flags Fields

- <a href="flags.read" name="flags.read"></a> [`read`](#flags.read)

  Read mode: Data can be read.
Bit: 0

- <a href="flags.write" name="flags.write"></a> [`write`](#flags.write)

  Write mode: Data can be written to.
Bit: 1

- <a href="flags.append" name="flags.append"></a> [`append`](#flags.append)

  Append mode: Data written to the file is always appended to the file's
  end.
Bit: 2

- <a href="flags.dsync" name="flags.dsync"></a> [`dsync`](#flags.dsync)

  Write according to synchronized I/O data integrity completion. Only the
  data stored in the file is synchronized.
Bit: 3

- <a href="flags.nonblock" name="flags.nonblock"></a> [`nonblock`](#flags.nonblock)

  Non-blocking mode.
Bit: 4

- <a href="flags.rsync" name="flags.rsync"></a> [`rsync`](#flags.rsync)

  Synchronized read I/O operations.
Bit: 5

- <a href="flags.sync" name="flags.sync"></a> [`sync`](#flags.sync)

  Write according to synchronized I/O file integrity completion. In
  addition to synchronizing the data stored in the file, the
  implementation may also synchronously update the file's metadata.
Bit: 6

## <a href="#stat" name="stat"></a> `stat`: record

  File attributes.
  
  Note: This was called `filestat` in earlier versions of WASI.

Size: 64, Alignment: 8

### Record Fields

- <a href="stat.dev" name="stat.dev"></a> [`dev`](#stat.dev): [`device`](#device)

  Device ID of device containing the file.

- <a href="stat.ino" name="stat.ino"></a> [`ino`](#stat.ino): [`inode`](#inode)

  File serial number.

- <a href="stat.type" name="stat.type"></a> [`type`](#stat.type): [`type`](#type)

  File type.

- <a href="stat.nlink" name="stat.nlink"></a> [`nlink`](#stat.nlink): [`linkcount`](#linkcount)

  Number of hard links to the file.

- <a href="stat.size" name="stat.size"></a> [`size`](#stat.size): [`filesize`](#filesize)

  For regular files, the file size in bytes. For symbolic links, the length
  in bytes of the pathname contained in the symbolic link.

- <a href="stat.atim" name="stat.atim"></a> [`atim`](#stat.atim): [`timestamp`](#timestamp)

  Last data access timestamp.

- <a href="stat.mtim" name="stat.mtim"></a> [`mtim`](#stat.mtim): [`timestamp`](#timestamp)

  Last data modification timestamp.

- <a href="stat.ctim" name="stat.ctim"></a> [`ctim`](#stat.ctim): [`timestamp`](#timestamp)

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

  Permissions mode used by `open-at`, `change-permissions-at`, and similar.

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

Size: 16, Alignment: 8

### Variant Cases

- <a href="new_timestamp.no_change" name="new_timestamp.no_change"></a> [`no-change`](#new_timestamp.no_change): `unit`

  Leave the timestamp set to its previous value.

- <a href="new_timestamp.now" name="new_timestamp.now"></a> [`now`](#new_timestamp.now): `unit`

  Set the timestamp to the current time of the system clock associated
  with the filesystem.

- <a href="new_timestamp.timestamp" name="new_timestamp.timestamp"></a> [`timestamp`](#new_timestamp.timestamp): [`timestamp`](#timestamp)

  Set the timestamp to the given value.

## <a href="#dirent" name="dirent"></a> `dirent`: record

  A directory entry.

Size: 16, Alignment: 8

### Record Fields

- <a href="dirent.ino" name="dirent.ino"></a> [`ino`](#dirent.ino): [`inode`](#inode)

  The serial number of the file referred to by this directory entry.

- <a href="dirent.namelen" name="dirent.namelen"></a> [`namelen`](#dirent.namelen): [`size`](#size)

  The length of the name of the directory entry.

- <a href="dirent.type" name="dirent.type"></a> [`type`](#dirent.type): [`type`](#type)

  The type of the file referred to by this directory entry.

## <a href="#errno" name="errno"></a> `errno`: enum

  Error codes returned by functions.
  Not all of these error codes are returned by the functions provided by this
  API; some are used in higher-level library layers, and others are provided
  merely for alignment with POSIX.

Size: 1, Alignment: 1

### Enum Cases

- <a href="errno.success" name="errno.success"></a> [`success`](#errno.success)

  No error occurred. System call completed successfully.

- <a href="errno.toobig" name="errno.toobig"></a> [`toobig`](#errno.toobig)

  Argument list too long. This is similar to `E2BIG` in POSIX.

- <a href="errno.access" name="errno.access"></a> [`access`](#errno.access)

  Permission denied.

- <a href="errno.addrinuse" name="errno.addrinuse"></a> [`addrinuse`](#errno.addrinuse)

  Address in use.

- <a href="errno.addrnotavail" name="errno.addrnotavail"></a> [`addrnotavail`](#errno.addrnotavail)

  Address not available.

- <a href="errno.afnosupport" name="errno.afnosupport"></a> [`afnosupport`](#errno.afnosupport)

  Address family not supported.

- <a href="errno.again" name="errno.again"></a> [`again`](#errno.again)

  Resource unavailable, or operation would block.

- <a href="errno.already" name="errno.already"></a> [`already`](#errno.already)

  Connection already in progress.

- <a href="errno.badmsg" name="errno.badmsg"></a> [`badmsg`](#errno.badmsg)

  Bad message.

- <a href="errno.busy" name="errno.busy"></a> [`busy`](#errno.busy)

  Device or resource busy.

- <a href="errno.canceled" name="errno.canceled"></a> [`canceled`](#errno.canceled)

  Operation canceled.

- <a href="errno.child" name="errno.child"></a> [`child`](#errno.child)

  No child processes.

- <a href="errno.connaborted" name="errno.connaborted"></a> [`connaborted`](#errno.connaborted)

  Connection aborted.

- <a href="errno.connrefused" name="errno.connrefused"></a> [`connrefused`](#errno.connrefused)

  Connection refused.

- <a href="errno.connreset" name="errno.connreset"></a> [`connreset`](#errno.connreset)

  Connection reset.

- <a href="errno.deadlk" name="errno.deadlk"></a> [`deadlk`](#errno.deadlk)

  Resource deadlock would occur.

- <a href="errno.destaddrreq" name="errno.destaddrreq"></a> [`destaddrreq`](#errno.destaddrreq)

  Destination address required.

- <a href="errno.dom" name="errno.dom"></a> [`dom`](#errno.dom)

  Mathematics argument out of domain of function.

- <a href="errno.dquot" name="errno.dquot"></a> [`dquot`](#errno.dquot)

  Reserved.

- <a href="errno.exist" name="errno.exist"></a> [`exist`](#errno.exist)

  File exists.

- <a href="errno.fault" name="errno.fault"></a> [`fault`](#errno.fault)

  Bad address.

- <a href="errno.fbig" name="errno.fbig"></a> [`fbig`](#errno.fbig)

  File too large.

- <a href="errno.hostunreach" name="errno.hostunreach"></a> [`hostunreach`](#errno.hostunreach)

  Host is unreachable.

- <a href="errno.idrm" name="errno.idrm"></a> [`idrm`](#errno.idrm)

  Identifier removed.

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

- <a href="errno.isconn" name="errno.isconn"></a> [`isconn`](#errno.isconn)

  Socket is connected.

- <a href="errno.isdir" name="errno.isdir"></a> [`isdir`](#errno.isdir)

  Is a directory.

- <a href="errno.loop" name="errno.loop"></a> [`loop`](#errno.loop)

  Too many levels of symbolic links.

- <a href="errno.mfile" name="errno.mfile"></a> [`mfile`](#errno.mfile)

  File descriptor value too large.

- <a href="errno.mlink" name="errno.mlink"></a> [`mlink`](#errno.mlink)

  Too many links.

- <a href="errno.msgsize" name="errno.msgsize"></a> [`msgsize`](#errno.msgsize)

  Message too large.

- <a href="errno.multihop" name="errno.multihop"></a> [`multihop`](#errno.multihop)

  Reserved.

- <a href="errno.nametoolong" name="errno.nametoolong"></a> [`nametoolong`](#errno.nametoolong)

  Filename too long.

- <a href="errno.netdown" name="errno.netdown"></a> [`netdown`](#errno.netdown)

  Network is down.

- <a href="errno.netreset" name="errno.netreset"></a> [`netreset`](#errno.netreset)

  Connection aborted by network.

- <a href="errno.netunreach" name="errno.netunreach"></a> [`netunreach`](#errno.netunreach)

  Network unreachable.

- <a href="errno.nfile" name="errno.nfile"></a> [`nfile`](#errno.nfile)

  Too many files open in system.

- <a href="errno.nobufs" name="errno.nobufs"></a> [`nobufs`](#errno.nobufs)

  No buffer space available.

- <a href="errno.nodev" name="errno.nodev"></a> [`nodev`](#errno.nodev)

  No such device.

- <a href="errno.noent" name="errno.noent"></a> [`noent`](#errno.noent)

  No such file or directory.

- <a href="errno.noexec" name="errno.noexec"></a> [`noexec`](#errno.noexec)

  Executable file format error.

- <a href="errno.nolck" name="errno.nolck"></a> [`nolck`](#errno.nolck)

  No locks available.

- <a href="errno.nolink" name="errno.nolink"></a> [`nolink`](#errno.nolink)

  Reserved.

- <a href="errno.nomem" name="errno.nomem"></a> [`nomem`](#errno.nomem)

  Not enough space.

- <a href="errno.nomsg" name="errno.nomsg"></a> [`nomsg`](#errno.nomsg)

  No message of the desired type.

- <a href="errno.noprotoopt" name="errno.noprotoopt"></a> [`noprotoopt`](#errno.noprotoopt)

  Protocol not available.

- <a href="errno.nospc" name="errno.nospc"></a> [`nospc`](#errno.nospc)

  No space left on device.

- <a href="errno.nosys" name="errno.nosys"></a> [`nosys`](#errno.nosys)

  Function not supported.

- <a href="errno.notconn" name="errno.notconn"></a> [`notconn`](#errno.notconn)

  The socket is not connected.

- <a href="errno.notdir" name="errno.notdir"></a> [`notdir`](#errno.notdir)

  Not a directory or a symbolic link to a directory.

- <a href="errno.notempty" name="errno.notempty"></a> [`notempty`](#errno.notempty)

  Directory not empty.

- <a href="errno.notrecoverable" name="errno.notrecoverable"></a> [`notrecoverable`](#errno.notrecoverable)

  State not recoverable.

- <a href="errno.notsock" name="errno.notsock"></a> [`notsock`](#errno.notsock)

  Not a socket.

- <a href="errno.notsup" name="errno.notsup"></a> [`notsup`](#errno.notsup)

  Not supported, or operation not supported on socket.

- <a href="errno.notty" name="errno.notty"></a> [`notty`](#errno.notty)

  Inappropriate I/O control operation.

- <a href="errno.nxio" name="errno.nxio"></a> [`nxio`](#errno.nxio)

  No such device or address.

- <a href="errno.overflow" name="errno.overflow"></a> [`overflow`](#errno.overflow)

  Value too large to be stored in data type.

- <a href="errno.ownerdead" name="errno.ownerdead"></a> [`ownerdead`](#errno.ownerdead)

  Previous owner died.

- <a href="errno.perm" name="errno.perm"></a> [`perm`](#errno.perm)

  Operation not permitted.

- <a href="errno.pipe" name="errno.pipe"></a> [`pipe`](#errno.pipe)

  Broken pipe.

- <a href="errno.proto" name="errno.proto"></a> [`proto`](#errno.proto)

  Protocol error.

- <a href="errno.protonosupport" name="errno.protonosupport"></a> [`protonosupport`](#errno.protonosupport)

  Protocol not supported.

- <a href="errno.prototype" name="errno.prototype"></a> [`prototype`](#errno.prototype)

  Protocol wrong type for socket.

- <a href="errno.range" name="errno.range"></a> [`range`](#errno.range)

  Result too large.

- <a href="errno.rofs" name="errno.rofs"></a> [`rofs`](#errno.rofs)

  Read-only file system.

- <a href="errno.spipe" name="errno.spipe"></a> [`spipe`](#errno.spipe)

  Invalid seek.

- <a href="errno.srch" name="errno.srch"></a> [`srch`](#errno.srch)

  No such process.

- <a href="errno.stale" name="errno.stale"></a> [`stale`](#errno.stale)

  Reserved.

- <a href="errno.timedout" name="errno.timedout"></a> [`timedout`](#errno.timedout)

  Connection timed out.

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
- <a href="#descriptor_fadvise.offset" name="descriptor_fadvise.offset"></a> `offset`: `u64`
- <a href="#descriptor_fadvise.len" name="descriptor_fadvise.len"></a> `len`: `u64`
- <a href="#descriptor_fadvise.advice" name="descriptor_fadvise.advice"></a> `advice`: [`advice`](#advice)
##### Result

- expected<`unit`, [`errno`](#errno)>

----

#### <a href="#descriptor_fallocate" name="descriptor_fallocate"></a> `descriptor::fallocate` 

  Force the allocation of space in a file.
  
  Note: This is similar to `posix_fallocate` in POSIX.
##### Params

- <a href="#descriptor_fallocate.self" name="descriptor_fallocate.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_fallocate.offset" name="descriptor_fallocate.offset"></a> `offset`: [`filesize`](#filesize)
- <a href="#descriptor_fallocate.len" name="descriptor_fallocate.len"></a> `len`: [`filesize`](#filesize)
##### Result

- expected<`unit`, [`errno`](#errno)>

----

#### <a href="#descriptor_datasync" name="descriptor_datasync"></a> `descriptor::datasync` 

  Synchronize the data of a file to disk.
  
  Note: This is similar to `fdatasync` in POSIX.
##### Params

- <a href="#descriptor_datasync.self" name="descriptor_datasync.self"></a> `self`: handle<descriptor>
##### Result

- expected<`unit`, [`errno`](#errno)>

----

#### <a href="#descriptor_info" name="descriptor_info"></a> `descriptor::info` 

  Get information associated with a descriptor.
  
  Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX, as well
  as additional fields.
  
  Note: This was called `fdstat_get` in earlier versions of WASI.
##### Params

- <a href="#descriptor_info.self" name="descriptor_info.self"></a> `self`: handle<descriptor>
##### Result

- expected<[`info`](#info), [`errno`](#errno)>

----

#### <a href="#descriptor_set_size" name="descriptor_set_size"></a> `descriptor::set-size` 

  Adjust the size of an open file. If this increases the file's size, the
  extra bytes are filled with zeros.
  
  Note: This was called `fd_filestat_set_size` in earlier versions of WASI.
##### Params

- <a href="#descriptor_set_size.self" name="descriptor_set_size.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_set_size.size" name="descriptor_set_size.size"></a> `size`: [`filesize`](#filesize)
##### Result

- expected<`unit`, [`errno`](#errno)>

----

#### <a href="#descriptor_set_times" name="descriptor_set_times"></a> `descriptor::set-times` 

  Adjust the timestamps of an open file or directory.
  
  Note: This is similar to `futimens` in POSIX.
  
  Note: This was called `fd_filestat_set_times` in earlier versions of WASI.
##### Params

- <a href="#descriptor_set_times.self" name="descriptor_set_times.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_set_times.atim" name="descriptor_set_times.atim"></a> `atim`: [`new-timestamp`](#new_timestamp)
- <a href="#descriptor_set_times.mtim" name="descriptor_set_times.mtim"></a> `mtim`: [`new-timestamp`](#new_timestamp)
##### Result

- expected<`unit`, [`errno`](#errno)>

----

#### <a href="#descriptor_pread" name="descriptor_pread"></a> `descriptor::pread` 

  Read from a descriptor, without using and updating the descriptor's offset.
  
  Note: This is similar to `pread` in POSIX.
##### Params

- <a href="#descriptor_pread.self" name="descriptor_pread.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_pread.offset" name="descriptor_pread.offset"></a> `offset`: [`filesize`](#filesize)
##### Result

- stream<`u8`, [`errno`](#errno)>

----

#### <a href="#descriptor_pwrite" name="descriptor_pwrite"></a> `descriptor::pwrite` 

  Write to a descriptor, without using and updating the descriptor's offset.
  
  Note: This is similar to `pwrite` in POSIX.
##### Params

- <a href="#descriptor_pwrite.self" name="descriptor_pwrite.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_pwrite.buf" name="descriptor_pwrite.buf"></a> `buf`: stream<`u8`, `unit`>
- <a href="#descriptor_pwrite.offset" name="descriptor_pwrite.offset"></a> `offset`: [`filesize`](#filesize)
##### Result

- future<expected<`unit`, [`errno`](#errno)>>

----

#### <a href="#descriptor_read" name="descriptor_read"></a> `descriptor::read` 

  Read from a descriptor.
  
  The meaning of `read` on a directory is unspecified.
  
  Note: This is similar to `read` in POSIX.
##### Params

- <a href="#descriptor_read.self" name="descriptor_read.self"></a> `self`: handle<descriptor>
##### Result

- stream<`u8`, [`errno`](#errno)>

----

#### <a href="#descriptor_readdir" name="descriptor_readdir"></a> `descriptor::readdir` 

  Read directory entries from a directory.
  
  When successful, the contents of the output buffer consist of a sequence of
  directory entries. Each directory entry consists of a `dirent` object,
  followed by `dirent::d_namlen` bytes holding the name of the directory
  entry.
  
  This function fills the output buffer as much as possible, potentially
  truncating the last directory entry. This allows the caller to grow its
  read buffer size in case it's too small to fit a single large directory
  entry, or skip the oversized directory entry.
##### Params

- <a href="#descriptor_readdir.self" name="descriptor_readdir.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_readdir.rewind" name="descriptor_readdir.rewind"></a> `rewind`: `bool`
##### Result

- stream<`u8`, [`errno`](#errno)>

----

#### <a href="#descriptor_seek" name="descriptor_seek"></a> `descriptor::seek` 

  Move the offset of a descriptor.
  
  The meaning of `seek` on a directory is unspecified.
  
  Returns new offset of the descriptor, relative to the start of the file.
  
  Note: This is similar to `lseek` in POSIX.
##### Params

- <a href="#descriptor_seek.self" name="descriptor_seek.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_seek.from" name="descriptor_seek.from"></a> `from`: [`seek-from`](#seek_from)
##### Result

- expected<[`filesize`](#filesize), [`errno`](#errno)>

----

#### <a href="#descriptor_sync" name="descriptor_sync"></a> `descriptor::sync` 

  Synchronize the data and metadata of a file to disk.
  
  Note: This is similar to `fsync` in POSIX.
##### Params

- <a href="#descriptor_sync.self" name="descriptor_sync.self"></a> `self`: handle<descriptor>
##### Result

- expected<`unit`, [`errno`](#errno)>

----

#### <a href="#descriptor_tell" name="descriptor_tell"></a> `descriptor::tell` 

  Return the current offset of a descriptor.
  
  Returns the current offset of the descriptor, relative to the start of the file.
  
  Note: This is similar to `lseek(fd, 0, SEEK_CUR)` in POSIX.
##### Params

- <a href="#descriptor_tell.self" name="descriptor_tell.self"></a> `self`: handle<descriptor>
##### Result

- expected<[`filesize`](#filesize), [`errno`](#errno)>

----

#### <a href="#descriptor_write" name="descriptor_write"></a> `descriptor::write` 

  Write to a descriptor.
  
  Note: This is similar to `write` in POSIX.
##### Params

- <a href="#descriptor_write.self" name="descriptor_write.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_write.buf" name="descriptor_write.buf"></a> `buf`: stream<`u8`, `unit`>
##### Result

- future<expected<`unit`, [`errno`](#errno)>>

----

#### <a href="#descriptor_create_directory_at" name="descriptor_create_directory_at"></a> `descriptor::create-directory-at` 

  Create a directory.
  
  Note: This is similar to `mkdirat` in POSIX.
##### Params

- <a href="#descriptor_create_directory_at.self" name="descriptor_create_directory_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_create_directory_at.path" name="descriptor_create_directory_at.path"></a> `path`: `string`
##### Result

- expected<`unit`, [`errno`](#errno)>

----

#### <a href="#descriptor_stat_at" name="descriptor_stat_at"></a> `descriptor::stat-at` 

  Return the attributes of a file or directory.
  
  Note: This is similar to `fstatat` in POSIX.
  
  Note: This was called `fd_filestat_get` in earlier versions of WASI.
##### Params

- <a href="#descriptor_stat_at.self" name="descriptor_stat_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_stat_at.at_flags" name="descriptor_stat_at.at_flags"></a> `at-flags`: [`at-flags`](#at_flags)
- <a href="#descriptor_stat_at.path" name="descriptor_stat_at.path"></a> `path`: `string`
##### Result

- expected<[`stat`](#stat), [`errno`](#errno)>

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
##### Result

- expected<`unit`, [`errno`](#errno)>

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
##### Result

- expected<`unit`, [`errno`](#errno)>

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
- <a href="#descriptor_open_at.flags" name="descriptor_open_at.flags"></a> `flags`: [`flags`](#flags)
- <a href="#descriptor_open_at.mode" name="descriptor_open_at.mode"></a> `mode`: [`mode`](#mode)
##### Result

- expected<handle<descriptor>, [`errno`](#errno)>

----

#### <a href="#descriptor_readlink_at" name="descriptor_readlink_at"></a> `descriptor::readlink-at` 

  Read the contents of a symbolic link.
  
  Note: This is similar to `readlinkat` in POSIX.
##### Params

- <a href="#descriptor_readlink_at.self" name="descriptor_readlink_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_readlink_at.path" name="descriptor_readlink_at.path"></a> `path`: `string`
##### Result

- expected<`string`, [`errno`](#errno)>

----

#### <a href="#descriptor_remove_directory_at" name="descriptor_remove_directory_at"></a> `descriptor::remove-directory-at` 

  Remove a directory.
  
  Return `errno::notempty` if the directory is not empty.
  
  Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
##### Params

- <a href="#descriptor_remove_directory_at.self" name="descriptor_remove_directory_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_remove_directory_at.path" name="descriptor_remove_directory_at.path"></a> `path`: `string`
##### Result

- expected<`unit`, [`errno`](#errno)>

----

#### <a href="#descriptor_rename_at" name="descriptor_rename_at"></a> `descriptor::rename-at` 

  Rename a filesystem object.
  
  Note: This is similar to `renameat` in POSIX.
##### Params

- <a href="#descriptor_rename_at.self" name="descriptor_rename_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_rename_at.old_path" name="descriptor_rename_at.old_path"></a> `old-path`: `string`
- <a href="#descriptor_rename_at.new_descriptor" name="descriptor_rename_at.new_descriptor"></a> `new-descriptor`: handle<descriptor>
- <a href="#descriptor_rename_at.new_path" name="descriptor_rename_at.new_path"></a> `new-path`: `string`
##### Result

- expected<`unit`, [`errno`](#errno)>

----

#### <a href="#descriptor_symlink_at" name="descriptor_symlink_at"></a> `descriptor::symlink-at` 

  Create a symbolic link.
  
  Note: This is similar to `symlinkat` in POSIX.
##### Params

- <a href="#descriptor_symlink_at.self" name="descriptor_symlink_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_symlink_at.old_path" name="descriptor_symlink_at.old_path"></a> `old-path`: `string`
- <a href="#descriptor_symlink_at.new_path" name="descriptor_symlink_at.new_path"></a> `new-path`: `string`
##### Result

- expected<`unit`, [`errno`](#errno)>

----

#### <a href="#descriptor_unlink_file_at" name="descriptor_unlink_file_at"></a> `descriptor::unlink-file-at` 

  Unlink a filesystem object that is not a directory.
  
  Return `errno::isdir` if the path refers to a directory.
  Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.
##### Params

- <a href="#descriptor_unlink_file_at.self" name="descriptor_unlink_file_at.self"></a> `self`: handle<descriptor>
- <a href="#descriptor_unlink_file_at.path" name="descriptor_unlink_file_at.path"></a> `path`: `string`
##### Result

- expected<`unit`, [`errno`](#errno)>

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
##### Result

- expected<`unit`, [`errno`](#errno)>

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
##### Result

- expected<`unit`, [`errno`](#errno)>

