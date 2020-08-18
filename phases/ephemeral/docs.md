# Types
## <a href="#size" name="size"></a> `size`: `usize`
An array size.

Note: This is similar to `size_t` in POSIX.

Size: 4

Alignment: 4

## <a href="#filesize" name="filesize"></a> `filesize`: `u64`
Non-negative file size or length of a region within a file.

Size: 8

Alignment: 8

## <a href="#timestamp" name="timestamp"></a> `timestamp`: `u64`
Timestamp in nanoseconds.

Size: 8

Alignment: 8

## <a href="#clockid" name="clockid"></a> `clockid`: Enum(`u32`)
Identifiers for clocks.

Size: 4

Alignment: 4

### Variants
- <a href="#clockid.realtime" name="clockid.realtime"></a> `realtime`
The clock measuring real time. Time value zero corresponds with
1970-01-01T00:00:00Z.

- <a href="#clockid.monotonic" name="clockid.monotonic"></a> `monotonic`
The store-wide monotonic clock, which is defined as a clock measuring
real time, whose value cannot be adjusted and which cannot have negative
clock jumps. The epoch of this clock is undefined. The absolute time
value of this clock therefore has no meaning.

## <a href="#errno" name="errno"></a> `errno`: Enum(`u16`)
Error codes returned by functions.
Not all of these error codes are returned by the functions provided by this
API; some are used in higher-level library layers, and others are provided
merely for alignment with POSIX.

Size: 2

Alignment: 2

### Variants
- <a href="#errno.success" name="errno.success"></a> `success`
No error occurred. System call completed successfully.

- <a href="#errno.2big" name="errno.2big"></a> `2big`
Argument list too long.

- <a href="#errno.access" name="errno.access"></a> `access`
Permission denied.

- <a href="#errno.addrinuse" name="errno.addrinuse"></a> `addrinuse`
Address in use.

- <a href="#errno.addrnotavail" name="errno.addrnotavail"></a> `addrnotavail`
Address not available.

- <a href="#errno.afnosupport" name="errno.afnosupport"></a> `afnosupport`
Address family not supported.

- <a href="#errno.again" name="errno.again"></a> `again`
Resource unavailable, or operation would block.

- <a href="#errno.already" name="errno.already"></a> `already`
Connection already in progress.

- <a href="#errno.badf" name="errno.badf"></a> `badf`
Bad file descriptor.

- <a href="#errno.badmsg" name="errno.badmsg"></a> `badmsg`
Bad message.

- <a href="#errno.busy" name="errno.busy"></a> `busy`
Device or resource busy.

- <a href="#errno.canceled" name="errno.canceled"></a> `canceled`
Operation canceled.

- <a href="#errno.child" name="errno.child"></a> `child`
No child processes.

- <a href="#errno.connaborted" name="errno.connaborted"></a> `connaborted`
Connection aborted.

- <a href="#errno.connrefused" name="errno.connrefused"></a> `connrefused`
Connection refused.

- <a href="#errno.connreset" name="errno.connreset"></a> `connreset`
Connection reset.

- <a href="#errno.deadlk" name="errno.deadlk"></a> `deadlk`
Resource deadlock would occur.

- <a href="#errno.destaddrreq" name="errno.destaddrreq"></a> `destaddrreq`
Destination address required.

- <a href="#errno.dom" name="errno.dom"></a> `dom`
Mathematics argument out of domain of function.

- <a href="#errno.dquot" name="errno.dquot"></a> `dquot`
Reserved.

- <a href="#errno.exist" name="errno.exist"></a> `exist`
File exists.

- <a href="#errno.fault" name="errno.fault"></a> `fault`
Bad address.

- <a href="#errno.fbig" name="errno.fbig"></a> `fbig`
File too large.

- <a href="#errno.hostunreach" name="errno.hostunreach"></a> `hostunreach`
Host is unreachable.

- <a href="#errno.idrm" name="errno.idrm"></a> `idrm`
Identifier removed.

- <a href="#errno.ilseq" name="errno.ilseq"></a> `ilseq`
Illegal byte sequence.

- <a href="#errno.inprogress" name="errno.inprogress"></a> `inprogress`
Operation in progress.

- <a href="#errno.intr" name="errno.intr"></a> `intr`
Interrupted function.

- <a href="#errno.inval" name="errno.inval"></a> `inval`
Invalid argument.

- <a href="#errno.io" name="errno.io"></a> `io`
I/O error.

- <a href="#errno.isconn" name="errno.isconn"></a> `isconn`
Socket is connected.

- <a href="#errno.isdir" name="errno.isdir"></a> `isdir`
Is a directory.

- <a href="#errno.loop" name="errno.loop"></a> `loop`
Too many levels of symbolic links.

- <a href="#errno.mfile" name="errno.mfile"></a> `mfile`
File descriptor value too large.

- <a href="#errno.mlink" name="errno.mlink"></a> `mlink`
Too many links.

- <a href="#errno.msgsize" name="errno.msgsize"></a> `msgsize`
Message too large.

- <a href="#errno.multihop" name="errno.multihop"></a> `multihop`
Reserved.

- <a href="#errno.nametoolong" name="errno.nametoolong"></a> `nametoolong`
Filename too long.

- <a href="#errno.netdown" name="errno.netdown"></a> `netdown`
Network is down.

- <a href="#errno.netreset" name="errno.netreset"></a> `netreset`
Connection aborted by network.

- <a href="#errno.netunreach" name="errno.netunreach"></a> `netunreach`
Network unreachable.

- <a href="#errno.nfile" name="errno.nfile"></a> `nfile`
Too many files open in system.

- <a href="#errno.nobufs" name="errno.nobufs"></a> `nobufs`
No buffer space available.

- <a href="#errno.nodev" name="errno.nodev"></a> `nodev`
No such device.

- <a href="#errno.noent" name="errno.noent"></a> `noent`
No such file or directory.

- <a href="#errno.noexec" name="errno.noexec"></a> `noexec`
Executable file format error.

- <a href="#errno.nolck" name="errno.nolck"></a> `nolck`
No locks available.

- <a href="#errno.nolink" name="errno.nolink"></a> `nolink`
Reserved.

- <a href="#errno.nomem" name="errno.nomem"></a> `nomem`
Not enough space.

- <a href="#errno.nomsg" name="errno.nomsg"></a> `nomsg`
No message of the desired type.

- <a href="#errno.noprotoopt" name="errno.noprotoopt"></a> `noprotoopt`
Protocol not available.

- <a href="#errno.nospc" name="errno.nospc"></a> `nospc`
No space left on device.

- <a href="#errno.nosys" name="errno.nosys"></a> `nosys`
Function not supported.

- <a href="#errno.notconn" name="errno.notconn"></a> `notconn`
The socket is not connected.

- <a href="#errno.notdir" name="errno.notdir"></a> `notdir`
Not a directory or a symbolic link to a directory.

- <a href="#errno.notempty" name="errno.notempty"></a> `notempty`
Directory not empty.

- <a href="#errno.notrecoverable" name="errno.notrecoverable"></a> `notrecoverable`
State not recoverable.

- <a href="#errno.notsock" name="errno.notsock"></a> `notsock`
Not a socket.

- <a href="#errno.notsup" name="errno.notsup"></a> `notsup`
Not supported, or operation not supported on socket.

- <a href="#errno.notty" name="errno.notty"></a> `notty`
Inappropriate I/O control operation.

- <a href="#errno.nxio" name="errno.nxio"></a> `nxio`
No such device or address.

- <a href="#errno.overflow" name="errno.overflow"></a> `overflow`
Value too large to be stored in data type.

- <a href="#errno.ownerdead" name="errno.ownerdead"></a> `ownerdead`
Previous owner died.

- <a href="#errno.perm" name="errno.perm"></a> `perm`
Operation not permitted.

- <a href="#errno.pipe" name="errno.pipe"></a> `pipe`
Broken pipe.

- <a href="#errno.proto" name="errno.proto"></a> `proto`
Protocol error.

- <a href="#errno.protonosupport" name="errno.protonosupport"></a> `protonosupport`
Protocol not supported.

- <a href="#errno.prototype" name="errno.prototype"></a> `prototype`
Protocol wrong type for socket.

- <a href="#errno.range" name="errno.range"></a> `range`
Result too large.

- <a href="#errno.rofs" name="errno.rofs"></a> `rofs`
Read-only file system.

- <a href="#errno.spipe" name="errno.spipe"></a> `spipe`
Invalid seek.

- <a href="#errno.srch" name="errno.srch"></a> `srch`
No such process.

- <a href="#errno.stale" name="errno.stale"></a> `stale`
Reserved.

- <a href="#errno.timedout" name="errno.timedout"></a> `timedout`
Connection timed out.

- <a href="#errno.txtbsy" name="errno.txtbsy"></a> `txtbsy`
Text file busy.

- <a href="#errno.xdev" name="errno.xdev"></a> `xdev`
Cross-device link.

- <a href="#errno.notcapable" name="errno.notcapable"></a> `notcapable`
Extension: Capabilities insufficient.

## <a href="#rights" name="rights"></a> `rights`: Flags(`u64`)
File descriptor rights, determining which actions may be performed.

Size: 8

Alignment: 8

### Flags
- <a href="#rights.fd_datasync" name="rights.fd_datasync"></a> `fd_datasync`
The right to invoke `fd_datasync`.
If `path_open` is set, includes the right to invoke
`path_open` with [`fdflags::dsync`](#fdflags.dsync).

- <a href="#rights.fd_read" name="rights.fd_read"></a> `fd_read`
The right to invoke `fd_read` and [`sock_recv`](#sock_recv).
If [`rights::fd_seek`](#rights.fd_seek) is set, includes the right to invoke `fd_pread`.

- <a href="#rights.fd_seek" name="rights.fd_seek"></a> `fd_seek`
The right to invoke `fd_seek`. This flag implies [`rights::fd_tell`](#rights.fd_tell).

- <a href="#rights.fd_fdstat_set_flags" name="rights.fd_fdstat_set_flags"></a> `fd_fdstat_set_flags`
The right to invoke `fd_fdstat_set_flags`.

- <a href="#rights.fd_sync" name="rights.fd_sync"></a> `fd_sync`
The right to invoke `fd_sync`.
If `path_open` is set, includes the right to invoke
`path_open` with [`fdflags::rsync`](#fdflags.rsync) and [`fdflags::dsync`](#fdflags.dsync).

- <a href="#rights.fd_tell" name="rights.fd_tell"></a> `fd_tell`
The right to invoke `fd_seek` in such a way that the file offset
remains unaltered (i.e., [`whence::cur`](#whence.cur) with offset zero), or to
invoke `fd_tell`.

- <a href="#rights.fd_write" name="rights.fd_write"></a> `fd_write`
The right to invoke `fd_write` and [`sock_send`](#sock_send).
If [`rights::fd_seek`](#rights.fd_seek) is set, includes the right to invoke `fd_pwrite`.

- <a href="#rights.fd_advise" name="rights.fd_advise"></a> `fd_advise`
The right to invoke `fd_advise`.

- <a href="#rights.fd_allocate" name="rights.fd_allocate"></a> `fd_allocate`
The right to invoke `fd_allocate`.

- <a href="#rights.path_create_directory" name="rights.path_create_directory"></a> `path_create_directory`
The right to invoke `path_create_directory`.

- <a href="#rights.path_create_file" name="rights.path_create_file"></a> `path_create_file`
If `path_open` is set, the right to invoke `path_open` with [`oflags::create`](#oflags.create).

- <a href="#rights.path_link_source" name="rights.path_link_source"></a> `path_link_source`
The right to invoke `path_link` with the file descriptor as the
source directory.

- <a href="#rights.path_link_target" name="rights.path_link_target"></a> `path_link_target`
The right to invoke `path_link` with the file descriptor as the
target directory.

- <a href="#rights.path_open" name="rights.path_open"></a> `path_open`
The right to invoke `path_open`.

- <a href="#rights.fd_readdir" name="rights.fd_readdir"></a> `fd_readdir`
The right to invoke `fd_readdir`.

- <a href="#rights.path_readlink" name="rights.path_readlink"></a> `path_readlink`
The right to invoke `path_readlink`.

- <a href="#rights.path_rename_source" name="rights.path_rename_source"></a> `path_rename_source`
The right to invoke `path_rename` with the file descriptor as the source directory.

- <a href="#rights.path_rename_target" name="rights.path_rename_target"></a> `path_rename_target`
The right to invoke `path_rename` with the file descriptor as the target directory.

- <a href="#rights.path_filestat_get" name="rights.path_filestat_get"></a> `path_filestat_get`
The right to invoke `path_filestat_get`.

- <a href="#rights.path_filestat_set_size" name="rights.path_filestat_set_size"></a> `path_filestat_set_size`
The right to change a file's size.
If `path_open` is set, includes the right to invoke `path_open` with [`oflags::trunc`](#oflags.trunc).
Note: there is no function named `path_filestat_set_size`. This follows POSIX design,
which only has `ftruncate` and does not provide `ftruncateat`.
While such function would be desirable from the API design perspective, there are virtually
no use cases for it since no code written for POSIX systems would use it.
Moreover, implementing it would require multiple syscalls, leading to inferior performance.

- <a href="#rights.path_filestat_set_times" name="rights.path_filestat_set_times"></a> `path_filestat_set_times`
The right to invoke `path_filestat_set_times`.

- <a href="#rights.path_permissions_set" name="rights.path_permissions_set"></a> `path_permissions_set`
The right to invoke `path_permissions_set`.

- <a href="#rights.fd_filestat_get" name="rights.fd_filestat_get"></a> `fd_filestat_get`
The right to invoke `fd_filestat_get`.

- <a href="#rights.fd_filestat_set_size" name="rights.fd_filestat_set_size"></a> `fd_filestat_set_size`
The right to invoke `fd_filestat_set_size`.

- <a href="#rights.fd_filestat_set_times" name="rights.fd_filestat_set_times"></a> `fd_filestat_set_times`
The right to invoke `fd_filestat_set_times`.

- <a href="#rights.fd_permissions_set" name="rights.fd_permissions_set"></a> `fd_permissions_set`
The right to invoke `fd_permissions_set`.

- <a href="#rights.path_symlink" name="rights.path_symlink"></a> `path_symlink`
The right to invoke `path_symlink`.

- <a href="#rights.path_remove_directory" name="rights.path_remove_directory"></a> `path_remove_directory`
The right to invoke `path_remove_directory`.

- <a href="#rights.path_unlink_file" name="rights.path_unlink_file"></a> `path_unlink_file`
The right to invoke `path_unlink_file`.

- <a href="#rights.poll_fd_readwrite" name="rights.poll_fd_readwrite"></a> `poll_fd_readwrite`
If [`rights::fd_read`](#rights.fd_read) is set, includes the right to invoke `poll_oneoff` to subscribe to [`eventtype::fd_read`](#eventtype.fd_read).
If [`rights::fd_write`](#rights.fd_write) is set, includes the right to invoke `poll_oneoff` to subscribe to [`eventtype::fd_write`](#eventtype.fd_write).

- <a href="#rights.sock_connect" name="rights.sock_connect"></a> `sock_connect`
Connect to an address

- <a href="#rights.sock_listen" name="rights.sock_listen"></a> `sock_listen`
Listen for incoming connection on an address

- <a href="#rights.sock_bind" name="rights.sock_bind"></a> `sock_bind`
Bind an address to a socket

- <a href="#rights.sock_accept" name="rights.sock_accept"></a> `sock_accept`
Accept incoming connection

- <a href="#rights.sock_recv" name="rights.sock_recv"></a> `sock_recv`
Receive data on a socket

- <a href="#rights.sock_send" name="rights.sock_send"></a> `sock_send`
Send data on a socket

- <a href="#rights.sock_addr_local" name="rights.sock_addr_local"></a> `sock_addr_local`
Retrieve locally bound address on a socket

- <a href="#rights.sock_addr_remote" name="rights.sock_addr_remote"></a> `sock_addr_remote`
Retrieve remote address on a socket

- <a href="#rights.sock_recv_from" name="rights.sock_recv_from"></a> `sock_recv_from`
Receive datagram on a socket

- <a href="#rights.sock_send_to" name="rights.sock_send_to"></a> `sock_send_to`
Send datagram on a socket

## <a href="#fd" name="fd"></a> `fd`
A file descriptor handle.

Size: 4

Alignment: 4

### Supertypes
## <a href="#iovec" name="iovec"></a> `iovec`: Struct
A region of memory for scatter/gather reads.

Size: 8

Alignment: 4

### Struct members
- <a href="#iovec.buf" name="iovec.buf"></a> `buf`: `Pointer<u8>`
The address of the buffer to be filled.

Offset: 0

- <a href="#iovec.buf_len" name="iovec.buf_len"></a> `buf_len`: [`size`](#size)
The length of the buffer to be filled.

Offset: 4

## <a href="#ciovec" name="ciovec"></a> `ciovec`: Struct
A region of memory for scatter/gather writes.

Size: 8

Alignment: 4

### Struct members
- <a href="#ciovec.buf" name="ciovec.buf"></a> `buf`: `ConstPointer<u8>`
The address of the buffer to be written.

Offset: 0

- <a href="#ciovec.buf_len" name="ciovec.buf_len"></a> `buf_len`: [`size`](#size)
The length of the buffer to be written.

Offset: 4

## <a href="#iovec_array" name="iovec_array"></a> `iovec_array`: `Array<iovec>`

Size: 8

Alignment: 4

## <a href="#ciovec_array" name="ciovec_array"></a> `ciovec_array`: `Array<ciovec>`

Size: 8

Alignment: 4

## <a href="#filedelta" name="filedelta"></a> `filedelta`: `s64`
Relative offset within a file.

Size: 8

Alignment: 8

## <a href="#whence" name="whence"></a> `whence`: Enum(`u8`)
The position relative to which to set the offset of the file descriptor.

Size: 1

Alignment: 1

### Variants
- <a href="#whence.set" name="whence.set"></a> `set`
Seek relative to start-of-file.

- <a href="#whence.cur" name="whence.cur"></a> `cur`
Seek relative to current position.

- <a href="#whence.end" name="whence.end"></a> `end`
Seek relative to end-of-file.

## <a href="#dircookie" name="dircookie"></a> `dircookie`: Int(`u64`)
A reference to the offset of a directory entry.

Size: 8

Alignment: 8

### Consts
- <a href="#dircookie.start" name="dircookie.start"></a> `start`
In an `fd_readdir` call, this value signifies the start of the directory.

## <a href="#dirnamlen" name="dirnamlen"></a> `dirnamlen`: `u32`
The type for the [`dirent::d_namlen`](#dirent.d_namlen) field of [`dirent`](#dirent).

Size: 4

Alignment: 4

## <a href="#inode" name="inode"></a> `inode`: `u64`
File serial number that is unique within its file system.

Size: 8

Alignment: 8

## <a href="#filetype" name="filetype"></a> `filetype`: Enum(`u8`)
The type of a file descriptor or file.

Size: 1

Alignment: 1

### Variants
- <a href="#filetype.unknown" name="filetype.unknown"></a> `unknown`
The type of the file descriptor or file is unknown or is different from any of the other types specified.

- <a href="#filetype.block_device" name="filetype.block_device"></a> `block_device`
The file descriptor or file refers to a block device inode.

- <a href="#filetype.character_device" name="filetype.character_device"></a> `character_device`
The file descriptor or file refers to a character device inode.

- <a href="#filetype.directory" name="filetype.directory"></a> `directory`
The file descriptor or file refers to a directory inode.

- <a href="#filetype.regular_file" name="filetype.regular_file"></a> `regular_file`
The file descriptor or file refers to a regular file inode.

- <a href="#filetype.socket_dgram" name="filetype.socket_dgram"></a> `socket_dgram`
The file descriptor or file refers to a datagram socket.

- <a href="#filetype.socket_stream" name="filetype.socket_stream"></a> `socket_stream`
The file descriptor or file refers to a byte-stream socket.

- <a href="#filetype.symbolic_link" name="filetype.symbolic_link"></a> `symbolic_link`
The file refers to a symbolic link inode.

- <a href="#filetype.fifo" name="filetype.fifo"></a> `fifo`
The file descriptor or file refers to a FIFO.

- <a href="#filetype.address_pool" name="filetype.address_pool"></a> `address_pool`
Address pool

## <a href="#dirent" name="dirent"></a> `dirent`: Struct
A directory entry.

Size: 24

Alignment: 8

### Struct members
- <a href="#dirent.d_next" name="dirent.d_next"></a> `d_next`: [`dircookie`](#dircookie)
The offset of the next directory entry stored in this directory.

Offset: 0

- <a href="#dirent.d_ino" name="dirent.d_ino"></a> `d_ino`: [`inode`](#inode)
The serial number of the file referred to by this directory entry.

Offset: 8

- <a href="#dirent.d_type" name="dirent.d_type"></a> `d_type`: [`filetype`](#filetype)
The type of the file referred to by this directory entry.

Offset: 16

- <a href="#dirent.d_namlen" name="dirent.d_namlen"></a> `d_namlen`: [`dirnamlen`](#dirnamlen)
The length of the name of the directory entry.

Offset: 20

## <a href="#advice" name="advice"></a> `advice`: Enum(`u8`)
File or memory access pattern advisory information.

Size: 1

Alignment: 1

### Variants
- <a href="#advice.normal" name="advice.normal"></a> `normal`
The application has no advice to give on its behavior with respect to the specified data.

- <a href="#advice.sequential" name="advice.sequential"></a> `sequential`
The application expects to access the specified data sequentially from lower offsets to higher offsets.

- <a href="#advice.random" name="advice.random"></a> `random`
The application expects to access the specified data in a random order.

- <a href="#advice.willneed" name="advice.willneed"></a> `willneed`
The application expects to access the specified data in the near future.

- <a href="#advice.dontneed" name="advice.dontneed"></a> `dontneed`
The application expects that it will not access the specified data in the near future.

- <a href="#advice.noreuse" name="advice.noreuse"></a> `noreuse`
The application expects to access the specified data once and then not reuse it thereafter.

## <a href="#fdflags" name="fdflags"></a> `fdflags`: Flags(`u16`)
File descriptor flags.

Size: 2

Alignment: 2

### Flags
- <a href="#fdflags.append" name="fdflags.append"></a> `append`
Append mode: Data written to the file is always appended to the file's end.

- <a href="#fdflags.dsync" name="fdflags.dsync"></a> `dsync`
Write according to synchronized I/O data integrity completion. Only the data stored in the file is synchronized.

- <a href="#fdflags.nonblock" name="fdflags.nonblock"></a> `nonblock`
Non-blocking mode.

- <a href="#fdflags.rsync" name="fdflags.rsync"></a> `rsync`
Synchronized read I/O operations.

- <a href="#fdflags.sync" name="fdflags.sync"></a> `sync`
Write according to synchronized I/O file integrity completion. In
addition to synchronizing the data stored in the file, the implementation
may also synchronously update the file's metadata.

## <a href="#fdstat" name="fdstat"></a> `fdstat`: Struct
File descriptor attributes.

Size: 24

Alignment: 8

### Struct members
- <a href="#fdstat.fs_filetype" name="fdstat.fs_filetype"></a> `fs_filetype`: [`filetype`](#filetype)
File type.

Offset: 0

- <a href="#fdstat.fs_flags" name="fdstat.fs_flags"></a> `fs_flags`: [`fdflags`](#fdflags)
File descriptor flags.

Offset: 2

- <a href="#fdstat.fs_rights_base" name="fdstat.fs_rights_base"></a> `fs_rights_base`: [`rights`](#rights)
Rights that apply to this file descriptor.

Offset: 8

- <a href="#fdstat.fs_rights_inheriting" name="fdstat.fs_rights_inheriting"></a> `fs_rights_inheriting`: [`rights`](#rights)
Maximum set of rights that may be installed on new file descriptors that
are created through this file descriptor, e.g., through `path_open`.

Offset: 16

## <a href="#device" name="device"></a> `device`: `u64`
Identifier for a device containing a file system. Can be used in combination
with [`inode`](#inode) to uniquely identify a file or directory in the filesystem.

Size: 8

Alignment: 8

## <a href="#fstflags" name="fstflags"></a> `fstflags`: Flags(`u16`)
Which file time attributes to adjust.

Size: 2

Alignment: 2

### Flags
- <a href="#fstflags.atim" name="fstflags.atim"></a> `atim`
Adjust the last data access timestamp to the value stored in [`filestat::atim`](#filestat.atim).

- <a href="#fstflags.atim_now" name="fstflags.atim_now"></a> `atim_now`
Adjust the last data access timestamp to the time of clock [`clockid::realtime`](#clockid.realtime).

- <a href="#fstflags.mtim" name="fstflags.mtim"></a> `mtim`
Adjust the last data modification timestamp to the value stored in [`filestat::mtim`](#filestat.mtim).

- <a href="#fstflags.mtim_now" name="fstflags.mtim_now"></a> `mtim_now`
Adjust the last data modification timestamp to the time of clock [`clockid::realtime`](#clockid.realtime).

## <a href="#lookupflags" name="lookupflags"></a> `lookupflags`: Flags(`u32`)
Flags determining the method of how paths are resolved.

Size: 4

Alignment: 4

### Flags
- <a href="#lookupflags.symlink_follow" name="lookupflags.symlink_follow"></a> `symlink_follow`
As long as the resolved path corresponds to a symbolic link, it is expanded.

## <a href="#oflags" name="oflags"></a> `oflags`: Flags(`u16`)
Open flags used by `path_open`.

Size: 2

Alignment: 2

### Flags
- <a href="#oflags.create" name="oflags.create"></a> `create`
Create file if it does not exist.

- <a href="#oflags.directory" name="oflags.directory"></a> `directory`
Fail if not a directory.

- <a href="#oflags.excl" name="oflags.excl"></a> `excl`
Fail if file already exists.

- <a href="#oflags.trunc" name="oflags.trunc"></a> `trunc`
Truncate file to size 0.

## <a href="#linkcount" name="linkcount"></a> `linkcount`: `u64`
Number of hard links to an inode.

Size: 8

Alignment: 8

## <a href="#permissions" name="permissions"></a> `permissions`: Flags(`u8`)
File permissions. This represents the permissions associated with a
file in a filesystem, and don't fully reflect all the conditions
which determine whether a given WASI program can access the file.

Size: 1

Alignment: 1

### Flags
- <a href="#permissions.read" name="permissions.read"></a> `read`
For files, permission to read the file.
For directories, permission to do [`readdir`](#readdir) and access files
within the directory.

Note: This is similar to the read bit being set on files, and the
read *and* execute bits being set on directories, in POSIX.

- <a href="#permissions.write" name="permissions.write"></a> `write`
For files, permission to mutate the file.
For directories, permission to create, remove, and rename items
within the directory.

- <a href="#permissions.execute" name="permissions.execute"></a> `execute`
For files, permission to "execute" the file, using whatever
concept of "executing" the host filesystem has.
This flag is not valid for directories.

- <a href="#permissions.private" name="permissions.private"></a> `private`
For filesystems which have a concept of multiple "users", this flag
indicates that the file is only accessible by the effective "user"
that the WASI store uses to access the filesystem, and inaccessible
to other "users".

## <a href="#filestat" name="filestat"></a> `filestat`: Struct
File attributes.

Size: 64

Alignment: 8

### Struct members
- <a href="#filestat.dev" name="filestat.dev"></a> `dev`: [`device`](#device)
Device ID of device containing the file.

Offset: 0

- <a href="#filestat.ino" name="filestat.ino"></a> `ino`: [`inode`](#inode)
File serial number.

Offset: 8

- <a href="#filestat.filetype" name="filestat.filetype"></a> `filetype`: [`filetype`](#filetype)
File type.

Offset: 16

- <a href="#filestat.permissions" name="filestat.permissions"></a> `permissions`: [`permissions`](#permissions)
File permissions.

Offset: 17

- <a href="#filestat.nlink" name="filestat.nlink"></a> `nlink`: [`linkcount`](#linkcount)
Number of hard links to the file.

Offset: 24

- <a href="#filestat.size" name="filestat.size"></a> `size`: [`filesize`](#filesize)
For regular files, the file size in bytes. For symbolic links, the length in bytes of the pathname contained in the symbolic link.

Offset: 32

- <a href="#filestat.atim" name="filestat.atim"></a> `atim`: [`timestamp`](#timestamp)
Last data access timestamp.

Offset: 40

- <a href="#filestat.mtim" name="filestat.mtim"></a> `mtim`: [`timestamp`](#timestamp)
Last data modification timestamp.

Offset: 48

- <a href="#filestat.ctim" name="filestat.ctim"></a> `ctim`: [`timestamp`](#timestamp)
Last file status change timestamp.

Offset: 56

## <a href="#userdata" name="userdata"></a> `userdata`: `u64`
User-provided value that may be attached to objects that is retained when
extracted from the implementation.

Size: 8

Alignment: 8

## <a href="#eventtype" name="eventtype"></a> `eventtype`: Enum(`u8`)
Type of a subscription to an event or its occurrence.

Size: 1

Alignment: 1

### Variants
- <a href="#eventtype.clock" name="eventtype.clock"></a> `clock`
The time value of clock [`subscription_clock::id`](#subscription_clock.id) has
reached timestamp [`subscription_clock::timeout`](#subscription_clock.timeout).

- <a href="#eventtype.fd_read" name="eventtype.fd_read"></a> `fd_read`
File descriptor [`subscription_fd_readwrite::fd`](#subscription_fd_readwrite.fd) has data
available for reading. This event always triggers for regular files.

- <a href="#eventtype.fd_write" name="eventtype.fd_write"></a> `fd_write`
File descriptor [`subscription_fd_readwrite::fd`](#subscription_fd_readwrite.fd) has capacity
available for writing. This event always triggers for regular files.

## <a href="#eventrwflags" name="eventrwflags"></a> `eventrwflags`: Flags(`u16`)
The state of the file descriptor subscribed to with
[`eventtype::fd_read`](#eventtype.fd_read) or [`eventtype::fd_write`](#eventtype.fd_write).

Size: 2

Alignment: 2

### Flags
- <a href="#eventrwflags.fd_readwrite_hangup" name="eventrwflags.fd_readwrite_hangup"></a> `fd_readwrite_hangup`
The peer of this socket has closed or disconnected.

## <a href="#event_fd_readwrite" name="event_fd_readwrite"></a> `event_fd_readwrite`: Struct
The contents of an [`event`](#event) when type is [`eventtype::fd_read`](#eventtype.fd_read) or
[`eventtype::fd_write`](#eventtype.fd_write).

Size: 16

Alignment: 8

### Struct members
- <a href="#event_fd_readwrite.nbytes" name="event_fd_readwrite.nbytes"></a> `nbytes`: [`filesize`](#filesize)
The number of bytes available for reading or writing.

Offset: 0

- <a href="#event_fd_readwrite.flags" name="event_fd_readwrite.flags"></a> `flags`: [`eventrwflags`](#eventrwflags)
The state of the file descriptor.

Offset: 8

## <a href="#event_u" name="event_u"></a> `event_u`: Union
The contents of an [`event`](#event).

Size: 24

Alignment: 8

### Union Layout
- tag_size: 1
- tag_align: 1
- contents_offset: 8
- contents_size: 16
- contents_align: 8
### Union variants
- <a href="#event_u.fd_read" name="event_u.fd_read"></a> `fd_read`: [`event_fd_readwrite`](#event_fd_readwrite)

- <a href="#event_u.fd_write" name="event_u.fd_write"></a> `fd_write`: [`event_fd_readwrite`](#event_fd_readwrite)

- <a href="#event_u.clock" name="event_u.clock"></a> `clock`

## <a href="#event" name="event"></a> `event`: Struct
An event that occurred.

Size: 40

Alignment: 8

### Struct members
- <a href="#event.userdata" name="event.userdata"></a> `userdata`: [`userdata`](#userdata)
User-provided value that got attached to [`subscription::userdata`](#subscription.userdata).

Offset: 0

- <a href="#event.error" name="event.error"></a> `error`: [`errno`](#errno)
If non-zero, an error that occurred while processing the subscription request.

Offset: 8

- <a href="#event.u" name="event.u"></a> `u`: [`event_u`](#event_u)
The type of the event that occurred, and the contents of the event

Offset: 16

## <a href="#subclockflags" name="subclockflags"></a> `subclockflags`: Flags(`u16`)
Flags determining how to interpret the timestamp provided in
[`subscription_clock::timeout`](#subscription_clock.timeout).

Size: 2

Alignment: 2

### Flags
- <a href="#subclockflags.subscription_clock_abstime" name="subclockflags.subscription_clock_abstime"></a> `subscription_clock_abstime`
If set, treat the timestamp provided in
[`subscription_clock::timeout`](#subscription_clock.timeout) as an absolute timestamp of clock
[`subscription_clock::id`](#subscription_clock.id). If clear, treat the timestamp
provided in [`subscription_clock::timeout`](#subscription_clock.timeout) relative to the
current time value of clock [`subscription_clock::id`](#subscription_clock.id).

## <a href="#subscription_clock" name="subscription_clock"></a> `subscription_clock`: Struct
The contents of a [`subscription`](#subscription) when type is [`eventtype::clock`](#eventtype.clock).

Size: 32

Alignment: 8

### Struct members
- <a href="#subscription_clock.id" name="subscription_clock.id"></a> `id`: [`clockid`](#clockid)
The clock against which to compare the timestamp.

Offset: 0

- <a href="#subscription_clock.timeout" name="subscription_clock.timeout"></a> `timeout`: [`timestamp`](#timestamp)
The absolute or relative timestamp.

Offset: 8

- <a href="#subscription_clock.precision" name="subscription_clock.precision"></a> `precision`: [`timestamp`](#timestamp)
The amount of time that the implementation may wait additionally
to coalesce with other events.

Offset: 16

- <a href="#subscription_clock.flags" name="subscription_clock.flags"></a> `flags`: [`subclockflags`](#subclockflags)
Flags specifying whether the timeout is absolute or relative

Offset: 24

## <a href="#subscription_fd_readwrite" name="subscription_fd_readwrite"></a> `subscription_fd_readwrite`: Struct
The contents of a [`subscription`](#subscription) when type is type is
[`eventtype::fd_read`](#eventtype.fd_read) or [`eventtype::fd_write`](#eventtype.fd_write).

Size: 4

Alignment: 4

### Struct members
- <a href="#subscription_fd_readwrite.fd" name="subscription_fd_readwrite.fd"></a> `fd`: [`fd`](#fd)
The file descriptor on which to wait for it to become ready for reading or writing.

Offset: 0

## <a href="#subscription_u" name="subscription_u"></a> `subscription_u`: Union
The contents of a [`subscription`](#subscription).

Size: 40

Alignment: 8

### Union Layout
- tag_size: 1
- tag_align: 1
- contents_offset: 8
- contents_size: 32
- contents_align: 8
### Union variants
- <a href="#subscription_u.clock" name="subscription_u.clock"></a> `clock`: [`subscription_clock`](#subscription_clock)

- <a href="#subscription_u.fd_read" name="subscription_u.fd_read"></a> `fd_read`: [`subscription_fd_readwrite`](#subscription_fd_readwrite)

- <a href="#subscription_u.fd_write" name="subscription_u.fd_write"></a> `fd_write`: [`subscription_fd_readwrite`](#subscription_fd_readwrite)

## <a href="#subscription" name="subscription"></a> `subscription`: Struct
Subscription to an event.

Size: 48

Alignment: 8

### Struct members
- <a href="#subscription.userdata" name="subscription.userdata"></a> `userdata`: [`userdata`](#userdata)
User-provided value that is attached to the subscription in the
implementation and returned through [`event::userdata`](#event.userdata).

Offset: 0

- <a href="#subscription.u" name="subscription.u"></a> `u`: [`subscription_u`](#subscription_u)
The type of the event to which to subscribe, and the contents of the subscription.

Offset: 8

## <a href="#exitcode" name="exitcode"></a> `exitcode`: `u32`
Exit code generated by a process when exiting.

Size: 4

Alignment: 4

## <a href="#riflags" name="riflags"></a> `riflags`: Flags(`u16`)
Flags provided to [`sock_recv`](#sock_recv).

Size: 2

Alignment: 2

### Flags
- <a href="#riflags.recv_peek" name="riflags.recv_peek"></a> `recv_peek`
Returns the message without removing it from the socket's receive queue.

- <a href="#riflags.recv_waitall" name="riflags.recv_waitall"></a> `recv_waitall`
On byte-stream sockets, block until the full amount of data can be returned.

## <a href="#roflags" name="roflags"></a> `roflags`: Flags(`u16`)
Flags returned by [`sock_recv`](#sock_recv).

Size: 2

Alignment: 2

### Flags
- <a href="#roflags.recv_data_truncated" name="roflags.recv_data_truncated"></a> `recv_data_truncated`
Returned by [`sock_recv`](#sock_recv): Message data has been truncated.

## <a href="#sock_type" name="sock_type"></a> `sock_type`: Enum(`u8`)
Socket type

Size: 1

Alignment: 1

### Variants
- <a href="#sock_type.socket_dgram" name="sock_type.socket_dgram"></a> `socket_dgram`
The file descriptor or file refers to a datagram socket.

- <a href="#sock_type.socket_stream" name="sock_type.socket_stream"></a> `socket_stream`
The file descriptor or file refers to a byte-stream socket.

## <a href="#ip_port" name="ip_port"></a> `ip_port`: `u16`
IP port number

Size: 2

Alignment: 2

## <a href="#addr_type" name="addr_type"></a> `addr_type`: Enum(`u8`)
Address type

Size: 1

Alignment: 1

### Variants
- <a href="#addr_type.ip4" name="addr_type.ip4"></a> `ip4`
IPv4 address

- <a href="#addr_type.ip6" name="addr_type.ip6"></a> `ip6`
IPv6 address

## <a href="#addr_ip4" name="addr_ip4"></a> `addr_ip4`: Struct
An IPv4 address is a 32-bit number that uniquely identifies a network interface on a machine.

Size: 4

Alignment: 1

### Struct members
- <a href="#addr_ip4.n0" name="addr_ip4.n0"></a> `n0`: `u8`

Offset: 0

- <a href="#addr_ip4.n1" name="addr_ip4.n1"></a> `n1`: `u8`

Offset: 1

- <a href="#addr_ip4.h0" name="addr_ip4.h0"></a> `h0`: `u8`

Offset: 2

- <a href="#addr_ip4.h1" name="addr_ip4.h1"></a> `h1`: `u8`

Offset: 3

## <a href="#addr_ip4_port" name="addr_ip4_port"></a> `addr_ip4_port`: Struct
An IPv4 address with a port number

Size: 6

Alignment: 2

### Struct members
- <a href="#addr_ip4_port.addr" name="addr_ip4_port.addr"></a> `addr`: [`addr_ip4`](#addr_ip4)

Offset: 0

- <a href="#addr_ip4_port.port" name="addr_ip4_port.port"></a> `port`: [`ip_port`](#ip_port)

Offset: 4

## <a href="#addr_ip6" name="addr_ip6"></a> `addr_ip6`: Struct
An IPv6 address is a 128-bit number that uniquely identifies a network interface on a machine.

Size: 16

Alignment: 2

### Struct members
- <a href="#addr_ip6.n0" name="addr_ip6.n0"></a> `n0`: `u16`

Offset: 0

- <a href="#addr_ip6.n1" name="addr_ip6.n1"></a> `n1`: `u16`

Offset: 2

- <a href="#addr_ip6.n2" name="addr_ip6.n2"></a> `n2`: `u16`

Offset: 4

- <a href="#addr_ip6.n3" name="addr_ip6.n3"></a> `n3`: `u16`

Offset: 6

- <a href="#addr_ip6.h0" name="addr_ip6.h0"></a> `h0`: `u16`

Offset: 8

- <a href="#addr_ip6.h1" name="addr_ip6.h1"></a> `h1`: `u16`

Offset: 10

- <a href="#addr_ip6.h2" name="addr_ip6.h2"></a> `h2`: `u16`

Offset: 12

- <a href="#addr_ip6.h3" name="addr_ip6.h3"></a> `h3`: `u16`

Offset: 14

## <a href="#addr_ip6_port" name="addr_ip6_port"></a> `addr_ip6_port`: Struct
An IPv6 address with a port number

Size: 18

Alignment: 2

### Struct members
- <a href="#addr_ip6_port.addr" name="addr_ip6_port.addr"></a> `addr`: [`addr_ip6`](#addr_ip6)

Offset: 0

- <a href="#addr_ip6_port.port" name="addr_ip6_port.port"></a> `port`: [`ip_port`](#ip_port)

Offset: 16

## <a href="#addr" name="addr"></a> `addr`: Union
Union of all possible addresses type

Size: 20

Alignment: 2

### Union Layout
- tag_size: 1
- tag_align: 1
- contents_offset: 2
- contents_size: 18
- contents_align: 2
### Union variants
- <a href="#addr.ip4" name="addr.ip4"></a> `ip4`: [`addr_ip4_port`](#addr_ip4_port)

- <a href="#addr.ip6" name="addr.ip6"></a> `ip6`: [`addr_ip6_port`](#addr_ip6_port)

## <a href="#address_family" name="address_family"></a> `address_family`: Enum(`u8`)
Address family

Size: 1

Alignment: 1

### Variants
- <a href="#address_family.inet4" name="address_family.inet4"></a> `inet4`
IP v4

- <a href="#address_family.inet6" name="address_family.inet6"></a> `inet6`
IP v6

## <a href="#siflags" name="siflags"></a> `siflags`: `u16`
Flags provided to [`sock_send`](#sock_send). As there are currently no flags
defined, it must be set to zero.

Size: 2

Alignment: 2

## <a href="#sdflags" name="sdflags"></a> `sdflags`: Flags(`u8`)
Which channels on a socket to shut down.

Size: 1

Alignment: 1

### Flags
- <a href="#sdflags.rd" name="sdflags.rd"></a> `rd`
Disables further receive operations.

- <a href="#sdflags.wr" name="sdflags.wr"></a> `wr`
Disables further send operations.

## <a href="#preopentype" name="preopentype"></a> `preopentype`: Enum(`u8`)
Identifiers for preopened capabilities.

Size: 1

Alignment: 1

### Variants
- <a href="#preopentype.dir" name="preopentype.dir"></a> `dir`
A pre-opened directory.

## <a href="#prestat_dir" name="prestat_dir"></a> `prestat_dir`: Struct
The contents of a [`prestat`](#prestat) when its type is [`preopentype::dir`](#preopentype.dir).

Size: 4

Alignment: 4

### Struct members
- <a href="#prestat_dir.pr_name_len" name="prestat_dir.pr_name_len"></a> `pr_name_len`: [`size`](#size)
The length of the directory name for use with `fd_prestat_dir_name`.

Offset: 0

## <a href="#prestat" name="prestat"></a> `prestat`: Union
Information about a pre-opened capability.

Size: 8

Alignment: 4

### Union Layout
- tag_size: 1
- tag_align: 1
- contents_offset: 4
- contents_size: 4
- contents_align: 4
### Union variants
- <a href="#prestat.dir" name="prestat.dir"></a> `dir`: [`prestat_dir`](#prestat_dir)
When type is [`preopentype::dir`](#preopentype.dir):

# Modules
## <a href="#wasi_ephemeral_args" name="wasi_ephemeral_args"></a> wasi_ephemeral_args
### Imports
#### Memory
### Functions

---

#### <a href="#get" name="get"></a> `get(argv: Pointer<Pointer<char8>>, argv_buf: Pointer<char8>) -> errno`
Read command-line argument data.
The size of the array should match that returned by [`sizes_get`](#sizes_get)

##### Params
- <a href="#get.argv" name="get.argv"></a> `argv`: `Pointer<Pointer<char8>>`

- <a href="#get.argv_buf" name="get.argv_buf"></a> `argv_buf`: `Pointer<char8>`

##### Results
- <a href="#get.error" name="get.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sizes_get" name="sizes_get"></a> `sizes_get() -> (errno, size, size)`
Return command-line argument data sizes.

##### Params
##### Results
- <a href="#sizes_get.error" name="sizes_get.error"></a> `error`: [`errno`](#errno)

- <a href="#sizes_get.argc" name="sizes_get.argc"></a> `argc`: [`size`](#size)
The number of arguments.

- <a href="#sizes_get.argv_buf_size" name="sizes_get.argv_buf_size"></a> `argv_buf_size`: [`size`](#size)
The size of the argument string data.

## <a href="#wasi_ephemeral_clock" name="wasi_ephemeral_clock"></a> wasi_ephemeral_clock
### Imports
#### Memory
### Functions

---

#### <a href="#res_get" name="res_get"></a> `res_get(id: clockid) -> (errno, timestamp)`
Return the resolution of a clock.
Implementations are required to provide a non-zero value for supported clocks. For unsupported clocks,
return [`errno::inval`](#errno.inval).
Note: This is similar to `clock_getres` in POSIX.

##### Params
- <a href="#res_get.id" name="res_get.id"></a> `id`: [`clockid`](#clockid)
The clock for which to return the resolution.

##### Results
- <a href="#res_get.error" name="res_get.error"></a> `error`: [`errno`](#errno)

- <a href="#res_get.resolution" name="res_get.resolution"></a> `resolution`: [`timestamp`](#timestamp)
The resolution of the clock.


---

#### <a href="#time_get" name="time_get"></a> `time_get(id: clockid, precision: timestamp) -> (errno, timestamp)`
Return the time value of a clock.
Note: This is similar to `clock_gettime` in POSIX.

##### Params
- <a href="#time_get.id" name="time_get.id"></a> `id`: [`clockid`](#clockid)
The clock for which to return the time.

- <a href="#time_get.precision" name="time_get.precision"></a> `precision`: [`timestamp`](#timestamp)
The maximum lag (exclusive) that the returned time value may have, compared to its actual value.

##### Results
- <a href="#time_get.error" name="time_get.error"></a> `error`: [`errno`](#errno)

- <a href="#time_get.time" name="time_get.time"></a> `time`: [`timestamp`](#timestamp)
The time value of the clock.

## <a href="#wasi_ephemeral_environ" name="wasi_ephemeral_environ"></a> wasi_ephemeral_environ
### Imports
#### Memory
### Functions

---

#### <a href="#get" name="get"></a> `get(environ: Pointer<Pointer<char8>>, environ_buf: Pointer<char8>) -> errno`
Read environment variable data.
The sizes of the buffers should match that returned by [`sizes_get`](#sizes_get).

##### Params
- <a href="#get.environ" name="get.environ"></a> `environ`: `Pointer<Pointer<char8>>`

- <a href="#get.environ_buf" name="get.environ_buf"></a> `environ_buf`: `Pointer<char8>`

##### Results
- <a href="#get.error" name="get.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sizes_get" name="sizes_get"></a> `sizes_get() -> (errno, size, size)`
Return environment variable data sizes.

##### Params
##### Results
- <a href="#sizes_get.error" name="sizes_get.error"></a> `error`: [`errno`](#errno)

- <a href="#sizes_get.environc" name="sizes_get.environc"></a> `environc`: [`size`](#size)
The number of environment variable arguments.

- <a href="#sizes_get.environ_buf_size" name="sizes_get.environ_buf_size"></a> `environ_buf_size`: [`size`](#size)
The size of the environment variable data.

## <a href="#wasi_ephemeral_fd" name="wasi_ephemeral_fd"></a> wasi_ephemeral_fd
### Imports
#### Memory
### Functions

---

#### <a href="#advise" name="advise"></a> `advise(fd: fd, offset: filesize, len: filesize, advice: advice) -> errno`
Provide file advisory information on a file descriptor.
Note: This is similar to `posix_fadvise` in POSIX.

##### Params
- <a href="#advise.fd" name="advise.fd"></a> `fd`: [`fd`](#fd)

- <a href="#advise.offset" name="advise.offset"></a> `offset`: [`filesize`](#filesize)
The offset within the file to which the advisory applies.

- <a href="#advise.len" name="advise.len"></a> `len`: [`filesize`](#filesize)
The length of the region to which the advisory applies.

- <a href="#advise.advice" name="advise.advice"></a> `advice`: [`advice`](#advice)
The advice.

##### Results
- <a href="#advise.error" name="advise.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#allocate" name="allocate"></a> `allocate(fd: fd, offset: filesize, len: filesize) -> errno`
Force the allocation of space in a file.
Note: This is similar to `posix_fallocate` in POSIX.

##### Params
- <a href="#allocate.fd" name="allocate.fd"></a> `fd`: [`fd`](#fd)

- <a href="#allocate.offset" name="allocate.offset"></a> `offset`: [`filesize`](#filesize)
The offset at which to start the allocation.

- <a href="#allocate.len" name="allocate.len"></a> `len`: [`filesize`](#filesize)
The length of the area that is allocated.

##### Results
- <a href="#allocate.error" name="allocate.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#close" name="close"></a> `close(fd: fd) -> errno`
Close a file descriptor.
Note: This is similar to [`close`](#close) in POSIX.

##### Params
- <a href="#close.fd" name="close.fd"></a> `fd`: [`fd`](#fd)

##### Results
- <a href="#close.error" name="close.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#datasync" name="datasync"></a> `datasync(fd: fd) -> errno`
Synchronize the data of a file to disk.
Note: This is similar to `fdatasync` in POSIX.

##### Params
- <a href="#datasync.fd" name="datasync.fd"></a> `fd`: [`fd`](#fd)

##### Results
- <a href="#datasync.error" name="datasync.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#fdstat_get" name="fdstat_get"></a> `fdstat_get(fd: fd) -> (errno, fdstat)`
Get the attributes of a file descriptor.
Note: This returns similar flags to `fsync(fd, F_GETFL)` in POSIX, as well as additional fields.

##### Params
- <a href="#fdstat_get.fd" name="fdstat_get.fd"></a> `fd`: [`fd`](#fd)

##### Results
- <a href="#fdstat_get.error" name="fdstat_get.error"></a> `error`: [`errno`](#errno)

- <a href="#fdstat_get.stat" name="fdstat_get.stat"></a> `stat`: [`fdstat`](#fdstat)
The buffer where the file descriptor's attributes are stored.


---

#### <a href="#fdstat_set_flags" name="fdstat_set_flags"></a> `fdstat_set_flags(fd: fd, flags: fdflags) -> errno`
Adjust the flags associated with a file descriptor.
Note: This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX.

##### Params
- <a href="#fdstat_set_flags.fd" name="fdstat_set_flags.fd"></a> `fd`: [`fd`](#fd)

- <a href="#fdstat_set_flags.flags" name="fdstat_set_flags.flags"></a> `flags`: [`fdflags`](#fdflags)
The desired values of the file descriptor flags.

##### Results
- <a href="#fdstat_set_flags.error" name="fdstat_set_flags.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#fdstat_set_rights" name="fdstat_set_rights"></a> `fdstat_set_rights(fd: fd, fs_rights_base: rights, fs_rights_inheriting: rights) -> errno`
Adjust the rights associated with a file descriptor.
This can only be used to remove rights, and returns [`errno::notcapable`](#errno.notcapable) if called in a way that would attempt to add rights

##### Params
- <a href="#fdstat_set_rights.fd" name="fdstat_set_rights.fd"></a> `fd`: [`fd`](#fd)

- <a href="#fdstat_set_rights.fs_rights_base" name="fdstat_set_rights.fs_rights_base"></a> `fs_rights_base`: [`rights`](#rights)
The desired rights of the file descriptor.

- <a href="#fdstat_set_rights.fs_rights_inheriting" name="fdstat_set_rights.fs_rights_inheriting"></a> `fs_rights_inheriting`: [`rights`](#rights)

##### Results
- <a href="#fdstat_set_rights.error" name="fdstat_set_rights.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#filestat_get" name="filestat_get"></a> `filestat_get(fd: fd) -> (errno, filestat)`
Return the attributes of an open file.

##### Params
- <a href="#filestat_get.fd" name="filestat_get.fd"></a> `fd`: [`fd`](#fd)

##### Results
- <a href="#filestat_get.error" name="filestat_get.error"></a> `error`: [`errno`](#errno)

- <a href="#filestat_get.buf" name="filestat_get.buf"></a> `buf`: [`filestat`](#filestat)
The buffer where the file's attributes are stored.


---

#### <a href="#filestat_set_size" name="filestat_set_size"></a> `filestat_set_size(fd: fd, size: filesize) -> errno`
Adjust the size of an open file. If this increases the file's size, the extra bytes are filled with zeros.
Note: This is similar to `ftruncate` in POSIX.

##### Params
- <a href="#filestat_set_size.fd" name="filestat_set_size.fd"></a> `fd`: [`fd`](#fd)

- <a href="#filestat_set_size.size" name="filestat_set_size.size"></a> `size`: [`filesize`](#filesize)
The desired file size.

##### Results
- <a href="#filestat_set_size.error" name="filestat_set_size.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#filestat_set_times" name="filestat_set_times"></a> `filestat_set_times(fd: fd, atim: timestamp, mtim: timestamp, fst_flags: fstflags) -> errno`
Adjust the timestamps of an open file or directory.
Note: This is similar to `futimens` in POSIX.

##### Params
- <a href="#filestat_set_times.fd" name="filestat_set_times.fd"></a> `fd`: [`fd`](#fd)

- <a href="#filestat_set_times.atim" name="filestat_set_times.atim"></a> `atim`: [`timestamp`](#timestamp)
The desired values of the data access timestamp.

- <a href="#filestat_set_times.mtim" name="filestat_set_times.mtim"></a> `mtim`: [`timestamp`](#timestamp)
The desired values of the data modification timestamp.

- <a href="#filestat_set_times.fst_flags" name="filestat_set_times.fst_flags"></a> `fst_flags`: [`fstflags`](#fstflags)
A bitmask indicating which timestamps to adjust.

##### Results
- <a href="#filestat_set_times.error" name="filestat_set_times.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#permissions_set" name="permissions_set"></a> `permissions_set(fd: fd, permissions: permissions) -> errno`
Set the permissions of a file or directory.

This sets the permissions associated with a file or directory in
a filesystem at the time it is called. The ability to actually access
a file or directory may depend on additional permissions not reflected
here.

Note: This is similar `fchmod` in POSIX.

Unlike POSIX, this doesn't expose a user/group/other distinction;
implementations in POSIX environments are suggested to consult the
umask to determine which of the user/group/other flags to modify.

##### Params
- <a href="#permissions_set.fd" name="permissions_set.fd"></a> `fd`: [`fd`](#fd)

- <a href="#permissions_set.permissions" name="permissions_set.permissions"></a> `permissions`: [`permissions`](#permissions)
The permissions associated with the file.

##### Results
- <a href="#permissions_set.error" name="permissions_set.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#pread" name="pread"></a> `pread(fd: fd, iovs: iovec_array, offset: filesize) -> (errno, size)`
Read from a file descriptor, without using and updating the file descriptor's offset.
Note: This is similar to `preadv` in Linux (and other Unix-es).

##### Params
- <a href="#pread.fd" name="pread.fd"></a> `fd`: [`fd`](#fd)

- <a href="#pread.iovs" name="pread.iovs"></a> `iovs`: [`iovec_array`](#iovec_array)
List of scatter/gather vectors in which to store data.

- <a href="#pread.offset" name="pread.offset"></a> `offset`: [`filesize`](#filesize)
The offset within the file at which to read.

##### Results
- <a href="#pread.error" name="pread.error"></a> `error`: [`errno`](#errno)

- <a href="#pread.nread" name="pread.nread"></a> `nread`: [`size`](#size)
The number of bytes read.


---

#### <a href="#prestat_get" name="prestat_get"></a> `prestat_get(fd: fd) -> (errno, prestat)`
Return a description of the given preopened file descriptor.

##### Params
- <a href="#prestat_get.fd" name="prestat_get.fd"></a> `fd`: [`fd`](#fd)

##### Results
- <a href="#prestat_get.error" name="prestat_get.error"></a> `error`: [`errno`](#errno)

- <a href="#prestat_get.buf" name="prestat_get.buf"></a> `buf`: [`prestat`](#prestat)
The buffer where the description is stored.


---

#### <a href="#prestat_dir_name" name="prestat_dir_name"></a> `prestat_dir_name(fd: fd, path: Pointer<char8>, path_len: size) -> errno`
Return a description of the given preopened file descriptor.

##### Params
- <a href="#prestat_dir_name.fd" name="prestat_dir_name.fd"></a> `fd`: [`fd`](#fd)

- <a href="#prestat_dir_name.path" name="prestat_dir_name.path"></a> `path`: `Pointer<char8>`
A buffer into which to write the preopened directory name.

- <a href="#prestat_dir_name.path_len" name="prestat_dir_name.path_len"></a> `path_len`: [`size`](#size)

##### Results
- <a href="#prestat_dir_name.error" name="prestat_dir_name.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#pwrite" name="pwrite"></a> `pwrite(fd: fd, iovs: ciovec_array, offset: filesize) -> (errno, size)`
Write to a file descriptor, without using and updating the file descriptor's offset.
Note: This is similar to `pwritev` in Linux (and other Unix-es).

Like Linux (and other Unix-es), any calls of [`pwrite`](#pwrite) (and other
functions to read or write) for a regular file by other threads in the
WASI process should not be interleaved while [`pwrite`](#pwrite) is executed.

##### Params
- <a href="#pwrite.fd" name="pwrite.fd"></a> `fd`: [`fd`](#fd)

- <a href="#pwrite.iovs" name="pwrite.iovs"></a> `iovs`: [`ciovec_array`](#ciovec_array)
List of scatter/gather vectors from which to retrieve data.

- <a href="#pwrite.offset" name="pwrite.offset"></a> `offset`: [`filesize`](#filesize)
The offset within the file at which to write.

##### Results
- <a href="#pwrite.error" name="pwrite.error"></a> `error`: [`errno`](#errno)

- <a href="#pwrite.nwritten" name="pwrite.nwritten"></a> `nwritten`: [`size`](#size)
The number of bytes written.


---

#### <a href="#read" name="read"></a> `read(fd: fd, iovs: iovec_array) -> (errno, size)`
Read from a file descriptor.
Note: This is similar to `readv` in POSIX.

##### Params
- <a href="#read.fd" name="read.fd"></a> `fd`: [`fd`](#fd)

- <a href="#read.iovs" name="read.iovs"></a> `iovs`: [`iovec_array`](#iovec_array)
List of scatter/gather vectors to which to store data.

##### Results
- <a href="#read.error" name="read.error"></a> `error`: [`errno`](#errno)

- <a href="#read.nread" name="read.nread"></a> `nread`: [`size`](#size)
The number of bytes read.


---

#### <a href="#readdir" name="readdir"></a> `readdir(fd: fd, buf: Pointer<u8>, buf_len: size, cookie: dircookie) -> (errno, size)`
Read directory entries from a directory.
When successful, the contents of the output buffer consist of a sequence of
directory entries. Each directory entry consists of a [`dirent`](#dirent) object,
followed by [`dirent::d_namlen`](#dirent.d_namlen) bytes holding the name of the directory
entry.
This function fills the output buffer as much as possible, potentially
truncating the last directory entry. This allows the caller to grow its
read buffer size in case it's too small to fit a single large directory
entry, or skip the oversized directory entry.

##### Params
- <a href="#readdir.fd" name="readdir.fd"></a> `fd`: [`fd`](#fd)

- <a href="#readdir.buf" name="readdir.buf"></a> `buf`: `Pointer<u8>`
The buffer where directory entries are stored

- <a href="#readdir.buf_len" name="readdir.buf_len"></a> `buf_len`: [`size`](#size)

- <a href="#readdir.cookie" name="readdir.cookie"></a> `cookie`: [`dircookie`](#dircookie)
The location within the directory to start reading

##### Results
- <a href="#readdir.error" name="readdir.error"></a> `error`: [`errno`](#errno)

- <a href="#readdir.bufused" name="readdir.bufused"></a> `bufused`: [`size`](#size)
The number of bytes stored in the read buffer. If less than the size of the read buffer, the end of the directory has been reached.


---

#### <a href="#renumber" name="renumber"></a> `renumber(fd: fd, to: fd) -> errno`
Atomically replace a file descriptor by renumbering another file descriptor.
Due to the strong focus on thread safety, this environment does not provide
a mechanism to duplicate or renumber a file descriptor to an arbitrary
number, like `dup2()`. This would be prone to race conditions, as an actual
file descriptor with the same number could be allocated by a different
thread at the same time.
This function provides a way to atomically renumber file descriptors, which
would disappear if `dup2()` were to be removed entirely.

##### Params
- <a href="#renumber.fd" name="renumber.fd"></a> `fd`: [`fd`](#fd)

- <a href="#renumber.to" name="renumber.to"></a> `to`: [`fd`](#fd)
The file descriptor to overwrite.

##### Results
- <a href="#renumber.error" name="renumber.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#seek" name="seek"></a> `seek(fd: fd, offset: filedelta, whence: whence) -> (errno, filesize)`
Move the offset of a file descriptor.
Note: This is similar to `lseek` in POSIX.

##### Params
- <a href="#seek.fd" name="seek.fd"></a> `fd`: [`fd`](#fd)

- <a href="#seek.offset" name="seek.offset"></a> `offset`: [`filedelta`](#filedelta)
The number of bytes to move.

- <a href="#seek.whence" name="seek.whence"></a> `whence`: [`whence`](#whence)
The base from which the offset is relative.

##### Results
- <a href="#seek.error" name="seek.error"></a> `error`: [`errno`](#errno)

- <a href="#seek.newoffset" name="seek.newoffset"></a> `newoffset`: [`filesize`](#filesize)
The new offset of the file descriptor, relative to the start of the file.


---

#### <a href="#sync" name="sync"></a> `sync(fd: fd) -> errno`
Synchronize the data and metadata of a file to disk.
Note: This is similar to `fsync` in POSIX.

##### Params
- <a href="#sync.fd" name="sync.fd"></a> `fd`: [`fd`](#fd)

##### Results
- <a href="#sync.error" name="sync.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#tell" name="tell"></a> `tell(fd: fd) -> (errno, filesize)`
Return the current offset of a file descriptor.
Note: This is similar to `lseek(fd, 0, SEEK_CUR)` in POSIX.

##### Params
- <a href="#tell.fd" name="tell.fd"></a> `fd`: [`fd`](#fd)

##### Results
- <a href="#tell.error" name="tell.error"></a> `error`: [`errno`](#errno)

- <a href="#tell.offset" name="tell.offset"></a> `offset`: [`filesize`](#filesize)
The current offset of the file descriptor, relative to the start of the file.


---

#### <a href="#write" name="write"></a> `write(fd: fd, iovs: ciovec_array) -> (errno, size)`
Write to a file descriptor.
Note: This is similar to `writev` in POSIX.

Like POSIX, any calls of [`write`](#write) (and other functions to read or write)
for a regular file by other threads in the WASI process should not be
interleaved while [`write`](#write) is executed.

##### Params
- <a href="#write.fd" name="write.fd"></a> `fd`: [`fd`](#fd)

- <a href="#write.iovs" name="write.iovs"></a> `iovs`: [`ciovec_array`](#ciovec_array)
List of scatter/gather vectors from which to retrieve data.

##### Results
- <a href="#write.error" name="write.error"></a> `error`: [`errno`](#errno)

- <a href="#write.nwritten" name="write.nwritten"></a> `nwritten`: [`size`](#size)
The number of bytes written.

## <a href="#wasi_ephemeral_path" name="wasi_ephemeral_path"></a> wasi_ephemeral_path
### Imports
#### Memory
### Functions

---

#### <a href="#create_directory" name="create_directory"></a> `create_directory(fd: fd, path: string) -> errno`
Create a directory.
Note: This is similar to `mkdirat` in POSIX.

##### Params
- <a href="#create_directory.fd" name="create_directory.fd"></a> `fd`: [`fd`](#fd)

- <a href="#create_directory.path" name="create_directory.path"></a> `path`: `string`
The path at which to create the directory.

##### Results
- <a href="#create_directory.error" name="create_directory.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#filestat_get" name="filestat_get"></a> `filestat_get(fd: fd, flags: lookupflags, path: string) -> (errno, filestat)`
Return the attributes of a file or directory.
Note: This is similar to `stat` in POSIX.

##### Params
- <a href="#filestat_get.fd" name="filestat_get.fd"></a> `fd`: [`fd`](#fd)

- <a href="#filestat_get.flags" name="filestat_get.flags"></a> `flags`: [`lookupflags`](#lookupflags)
Flags determining the method of how the path is resolved.

- <a href="#filestat_get.path" name="filestat_get.path"></a> `path`: `string`
The path of the file or directory to inspect.

##### Results
- <a href="#filestat_get.error" name="filestat_get.error"></a> `error`: [`errno`](#errno)

- <a href="#filestat_get.buf" name="filestat_get.buf"></a> `buf`: [`filestat`](#filestat)
The buffer where the file's attributes are stored.


---

#### <a href="#filestat_set_times" name="filestat_set_times"></a> `filestat_set_times(fd: fd, flags: lookupflags, path: string, atim: timestamp, mtim: timestamp, fst_flags: fstflags) -> errno`
Adjust the timestamps of a file or directory.
Note: This is similar to `utimensat` in POSIX.

##### Params
- <a href="#filestat_set_times.fd" name="filestat_set_times.fd"></a> `fd`: [`fd`](#fd)

- <a href="#filestat_set_times.flags" name="filestat_set_times.flags"></a> `flags`: [`lookupflags`](#lookupflags)
Flags determining the method of how the path is resolved.

- <a href="#filestat_set_times.path" name="filestat_set_times.path"></a> `path`: `string`
The path of the file or directory to operate on.

- <a href="#filestat_set_times.atim" name="filestat_set_times.atim"></a> `atim`: [`timestamp`](#timestamp)
The desired values of the data access timestamp.

- <a href="#filestat_set_times.mtim" name="filestat_set_times.mtim"></a> `mtim`: [`timestamp`](#timestamp)
The desired values of the data modification timestamp.

- <a href="#filestat_set_times.fst_flags" name="filestat_set_times.fst_flags"></a> `fst_flags`: [`fstflags`](#fstflags)
A bitmask indicating which timestamps to adjust.

##### Results
- <a href="#filestat_set_times.error" name="filestat_set_times.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#permissions_set" name="permissions_set"></a> `permissions_set(fd: fd, flags: lookupflags, path: string, permissions: permissions) -> errno`
Set the permissions of a file or directory.

This sets the permissions associated with a file or directory in
a filesystem at the time it is called. The ability to actually access
a file or directory may depend on additional permissions not reflected
here.

Note: This is similar to `fchmodat` in POSIX.

Unlike POSIX, this doesn't expose a user/group/other distinction;
implementations in POSIX environments are suggested to consult the
umask to determine which of the user/group/other flags to modify.

##### Params
- <a href="#permissions_set.fd" name="permissions_set.fd"></a> `fd`: [`fd`](#fd)

- <a href="#permissions_set.flags" name="permissions_set.flags"></a> `flags`: [`lookupflags`](#lookupflags)
Flags determining the method of how the path is resolved.

- <a href="#permissions_set.path" name="permissions_set.path"></a> `path`: `string`
The path to a file to query.

- <a href="#permissions_set.permissions" name="permissions_set.permissions"></a> `permissions`: [`permissions`](#permissions)
The permissions to associate with the file.

##### Results
- <a href="#permissions_set.error" name="permissions_set.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#link" name="link"></a> `link(old_fd: fd, old_flags: lookupflags, old_path: string, new_fd: fd, new_path: string) -> errno`
Create a hard link.
Note: This is similar to `linkat` in POSIX.

##### Params
- <a href="#link.old_fd" name="link.old_fd"></a> `old_fd`: [`fd`](#fd)

- <a href="#link.old_flags" name="link.old_flags"></a> `old_flags`: [`lookupflags`](#lookupflags)
Flags determining the method of how the path is resolved.

- <a href="#link.old_path" name="link.old_path"></a> `old_path`: `string`
The source path from which to link.

- <a href="#link.new_fd" name="link.new_fd"></a> `new_fd`: [`fd`](#fd)
The working directory at which the resolution of the new path starts.

- <a href="#link.new_path" name="link.new_path"></a> `new_path`: `string`
The destination path at which to create the hard link.

##### Results
- <a href="#link.error" name="link.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#open" name="open"></a> `open(fd: fd, dirflags: lookupflags, path: string, oflags: oflags, fs_rights_base: rights, fs_rights_inherting: rights, fdflags: fdflags, permissions: permissions) -> (errno, fd)`
Open a file or directory.
The returned file descriptor is not guaranteed to be the lowest-numbered
file descriptor not currently open; it is randomized to prevent
applications from depending on making assumptions about indexes, since this
is error-prone in multi-threaded contexts. The returned file descriptor is
guaranteed to be less than 2**31.
Note: This is similar to `openat` in POSIX.

##### Params
- <a href="#open.fd" name="open.fd"></a> `fd`: [`fd`](#fd)

- <a href="#open.dirflags" name="open.dirflags"></a> `dirflags`: [`lookupflags`](#lookupflags)
Flags determining the method of how the path is resolved.

- <a href="#open.path" name="open.path"></a> `path`: `string`
The relative path of the file or directory to open, relative to the
[`fd`](#fd) directory.

- <a href="#open.oflags" name="open.oflags"></a> `oflags`: [`oflags`](#oflags)
The method by which to open the file.

- <a href="#open.fs_rights_base" name="open.fs_rights_base"></a> `fs_rights_base`: [`rights`](#rights)
The initial rights of the newly created file descriptor. The
implementation is allowed to return a file descriptor with fewer rights
than specified, if and only if those rights do not apply to the type of
file being opened.
The *base* rights are rights that will apply to operations using the file
descriptor itself, while the *inheriting* rights are rights that apply to
file descriptors derived from it.

- <a href="#open.fs_rights_inherting" name="open.fs_rights_inherting"></a> `fs_rights_inherting`: [`rights`](#rights)

- <a href="#open.fdflags" name="open.fdflags"></a> `fdflags`: [`fdflags`](#fdflags)

- <a href="#open.permissions" name="open.permissions"></a> `permissions`: [`permissions`](#permissions)
If a file is created, the filesystem permissions to associate with it.

##### Results
- <a href="#open.error" name="open.error"></a> `error`: [`errno`](#errno)

- <a href="#open.opened_fd" name="open.opened_fd"></a> `opened_fd`: [`fd`](#fd)
The file descriptor of the file that has been opened.


---

#### <a href="#readlink" name="readlink"></a> `readlink(fd: fd, path: string, buf: Pointer<char8>, buf_len: size) -> (errno, size)`
Read the contents of a symbolic link.
Note: This is similar to `readlinkat` in POSIX.

##### Params
- <a href="#readlink.fd" name="readlink.fd"></a> `fd`: [`fd`](#fd)

- <a href="#readlink.path" name="readlink.path"></a> `path`: `string`
The path of the symbolic link from which to read.

- <a href="#readlink.buf" name="readlink.buf"></a> `buf`: `Pointer<char8>`
The buffer to which to write the contents of the symbolic link.

- <a href="#readlink.buf_len" name="readlink.buf_len"></a> `buf_len`: [`size`](#size)

##### Results
- <a href="#readlink.error" name="readlink.error"></a> `error`: [`errno`](#errno)

- <a href="#readlink.bufused" name="readlink.bufused"></a> `bufused`: [`size`](#size)
The number of bytes placed in the buffer.


---

#### <a href="#remove_directory" name="remove_directory"></a> `remove_directory(fd: fd, path: string) -> errno`
Remove a directory.
Return [`errno::notempty`](#errno.notempty) if the directory is not empty.
Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.

##### Params
- <a href="#remove_directory.fd" name="remove_directory.fd"></a> `fd`: [`fd`](#fd)

- <a href="#remove_directory.path" name="remove_directory.path"></a> `path`: `string`
The path to a directory to remove.

##### Results
- <a href="#remove_directory.error" name="remove_directory.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#rename" name="rename"></a> `rename(fd: fd, old_path: string, new_fd: fd, new_path: string) -> errno`
Rename a file or directory.
Note: This is similar to `renameat` in POSIX.

##### Params
- <a href="#rename.fd" name="rename.fd"></a> `fd`: [`fd`](#fd)

- <a href="#rename.old_path" name="rename.old_path"></a> `old_path`: `string`
The source path of the file or directory to rename.

- <a href="#rename.new_fd" name="rename.new_fd"></a> `new_fd`: [`fd`](#fd)
The working directory at which the resolution of the new path starts.

- <a href="#rename.new_path" name="rename.new_path"></a> `new_path`: `string`
The destination path to which to rename the file or directory.

##### Results
- <a href="#rename.error" name="rename.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#symlink" name="symlink"></a> `symlink(old_path: string, fd: fd, new_path: string) -> errno`
Create a symbolic link.
Note: This is similar to `symlinkat` in POSIX.

##### Params
- <a href="#symlink.old_path" name="symlink.old_path"></a> `old_path`: `string`
The contents of the symbolic link.

- <a href="#symlink.fd" name="symlink.fd"></a> `fd`: [`fd`](#fd)

- <a href="#symlink.new_path" name="symlink.new_path"></a> `new_path`: `string`
The destination path at which to create the symbolic link.

##### Results
- <a href="#symlink.error" name="symlink.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#unlink_file" name="unlink_file"></a> `unlink_file(fd: fd, path: string) -> errno`
Unlink a file.
Return [`errno::isdir`](#errno.isdir) if the path refers to a directory.
Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.

##### Params
- <a href="#unlink_file.fd" name="unlink_file.fd"></a> `fd`: [`fd`](#fd)

- <a href="#unlink_file.path" name="unlink_file.path"></a> `path`: `string`
The path to a file to unlink.

##### Results
- <a href="#unlink_file.error" name="unlink_file.error"></a> `error`: [`errno`](#errno)

## <a href="#wasi_ephemeral_poll" name="wasi_ephemeral_poll"></a> wasi_ephemeral_poll
### Imports
#### Memory
### Functions

---

#### <a href="#oneoff" name="oneoff"></a> `oneoff(in: ConstPointer<subscription>, out: Pointer<event>, nsubscriptions: size) -> (errno, size)`
Concurrently poll for the occurrence of a set of events.

If `nsubscriptions` is 0, returns [`errno::inval`](#errno.inval).

##### Params
- <a href="#oneoff.in" name="oneoff.in"></a> `in`: `ConstPointer<subscription>`
The events to which to subscribe.

- <a href="#oneoff.out" name="oneoff.out"></a> `out`: `Pointer<event>`
The events that have occurred.

- <a href="#oneoff.nsubscriptions" name="oneoff.nsubscriptions"></a> `nsubscriptions`: [`size`](#size)
Both the number of subscriptions and events.

##### Results
- <a href="#oneoff.error" name="oneoff.error"></a> `error`: [`errno`](#errno)

- <a href="#oneoff.nevents" name="oneoff.nevents"></a> `nevents`: [`size`](#size)
The number of events stored.

## <a href="#wasi_ephemeral_proc" name="wasi_ephemeral_proc"></a> wasi_ephemeral_proc
### Imports
### Functions

---

#### <a href="#exit" name="exit"></a> `exit(rval: exitcode)`
Terminate the process normally. An exit code of 0 indicates successful
termination of the program. The meanings of other values is dependent on
the environment.

##### Params
- <a href="#exit.rval" name="exit.rval"></a> `rval`: [`exitcode`](#exitcode)
The exit code returned by the process.

##### Results
## <a href="#wasi_ephemeral_random" name="wasi_ephemeral_random"></a> wasi_ephemeral_random
### Imports
#### Memory
### Functions

---

#### <a href="#get" name="get"></a> `get(buf: Pointer<u8>, buf_len: size) -> errno`
Write high-quality random data into a buffer.
This function blocks when the implementation is unable to immediately
provide sufficient high-quality random data.
This function may execute slowly, so when large mounts of random data are
required, it's advisable to use this function to seed a pseudo-random
number generator, rather than to provide the random data directly.

##### Params
- <a href="#get.buf" name="get.buf"></a> `buf`: `Pointer<u8>`
The buffer to fill with random data.

- <a href="#get.buf_len" name="get.buf_len"></a> `buf_len`: [`size`](#size)

##### Results
- <a href="#get.error" name="get.error"></a> `error`: [`errno`](#errno)

## <a href="#wasi_ephemeral_sched" name="wasi_ephemeral_sched"></a> wasi_ephemeral_sched
### Imports
### Functions

---

#### <a href="#yield" name="yield"></a> `yield() -> errno`
Temporarily yield execution of the calling thread.
Note: This is similar to [`yield`](#yield) in POSIX.

##### Params
##### Results
- <a href="#yield.error" name="yield.error"></a> `error`: [`errno`](#errno)

## <a href="#wasi_ephemeral_sock" name="wasi_ephemeral_sock"></a> wasi_ephemeral_sock
### Imports
#### Memory
### Functions

---

#### <a href="#addr_resolve" name="addr_resolve"></a> `addr_resolve(pool: fd, host: string, port: ip_port, buf: Pointer<u8>, buf_len: size) -> (errno, size)`
Resolves a hostname and a port to one or more IP addresses. Port is optional
and you can pass 0 (zero) in most cases, it is used a hint for protocol.

Note: This is similar to `getaddrinfo` in POSIX

When successful, the contents of the output buffer consist of a sequence of
IPv4 and/or IPv6 addresses. Each address entry consists of a addr_t object.
This function fills the output buffer as much as possible, potentially
truncating the last address entry. It is advisable that the buffer is

##### Params
- <a href="#addr_resolve.pool" name="addr_resolve.pool"></a> `pool`: [`fd`](#fd)
Address pool file descriptor

- <a href="#addr_resolve.host" name="addr_resolve.host"></a> `host`: `string`
Host to resolve

- <a href="#addr_resolve.port" name="addr_resolve.port"></a> `port`: [`ip_port`](#ip_port)
Port number

- <a href="#addr_resolve.buf" name="addr_resolve.buf"></a> `buf`: `Pointer<u8>`
The buffer where IP addresses will be stored

- <a href="#addr_resolve.buf_len" name="addr_resolve.buf_len"></a> `buf_len`: [`size`](#size)

##### Results
- <a href="#addr_resolve.error" name="addr_resolve.error"></a> `error`: [`errno`](#errno)

- <a href="#addr_resolve.bufused" name="addr_resolve.bufused"></a> `bufused`: [`size`](#size)
The number of bytes stored in the buffer. If less than the size of the buffer, no
more IP addresses are available.


---

#### <a href="#sock_addr_local" name="sock_addr_local"></a> `sock_addr_local(fd: fd, buf: Pointer<u8>, buf_len: size) -> errno`
Returns the local address to which the socket is bound.

Note: This is similar to `getsockname` in POSIX

When successful, the contents of the output buffer consist of an IP address,
either IP4 or IP6.

##### Params
- <a href="#sock_addr_local.fd" name="sock_addr_local.fd"></a> `fd`: [`fd`](#fd)
Host to resolve

- <a href="#sock_addr_local.buf" name="sock_addr_local.buf"></a> `buf`: `Pointer<u8>`
The buffer where IP addresses will be stored

- <a href="#sock_addr_local.buf_len" name="sock_addr_local.buf_len"></a> `buf_len`: [`size`](#size)

##### Results
- <a href="#sock_addr_local.error" name="sock_addr_local.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sock_addr_remote" name="sock_addr_remote"></a> `sock_addr_remote(fd: fd, buf: Pointer<u8>, buf_len: size) -> errno`
Returns the remote address to which the socket is connected to.

Note: This is similar to `getpeername` in POSIX

When successful, the contents of the output buffer consist of an IP address,
either IP4 or IP6.

##### Params
- <a href="#sock_addr_remote.fd" name="sock_addr_remote.fd"></a> `fd`: [`fd`](#fd)
Host to resolve

- <a href="#sock_addr_remote.buf" name="sock_addr_remote.buf"></a> `buf`: `Pointer<u8>`
The buffer where IP addresses will be stored

- <a href="#sock_addr_remote.buf_len" name="sock_addr_remote.buf_len"></a> `buf_len`: [`size`](#size)

##### Results
- <a href="#sock_addr_remote.error" name="sock_addr_remote.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sock_open" name="sock_open"></a> `sock_open(pool: fd, af: address_family, socktype: sock_type) -> (errno, fd)`
Open a socket

The first argument to this function is a handle to an
address pool. The address pool determines what actions can
be performed and at which addresses they can be performed to.

The address pool cannot be re-assigned. You will need to close
the socket and open a new one to use a different address pool.

Note: This is similar to `socket` in POSIX using PF_INET

##### Params
- <a href="#sock_open.pool" name="sock_open.pool"></a> `pool`: [`fd`](#fd)
Address pool file descriptor

- <a href="#sock_open.af" name="sock_open.af"></a> `af`: [`address_family`](#address_family)
Address family

- <a href="#sock_open.socktype" name="sock_open.socktype"></a> `socktype`: [`sock_type`](#sock_type)
Socket type, either datagram or stream

##### Results
- <a href="#sock_open.error" name="sock_open.error"></a> `error`: [`errno`](#errno)

- <a href="#sock_open.fd" name="sock_open.fd"></a> `fd`: [`fd`](#fd)
The opened socket


---

#### <a href="#sock_close" name="sock_close"></a> `sock_close(fd: fd) -> errno`
Close a socket (this is an alias for `fd_close`)
Note: This is similar to [`close`](#close) in POSIX.

##### Params
- <a href="#sock_close.fd" name="sock_close.fd"></a> `fd`: [`fd`](#fd)
Socket descriptor

##### Results
- <a href="#sock_close.error" name="sock_close.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sock_set_reuse_addr" name="sock_set_reuse_addr"></a> `sock_set_reuse_addr(fd: fd, reuse: u8) -> errno`
Enable/disable address reuse on a socket
Note: This is similar to `setsockopt` in POSIX for SO_REUSEADDR

##### Params
- <a href="#sock_set_reuse_addr.fd" name="sock_set_reuse_addr.fd"></a> `fd`: [`fd`](#fd)
Socket descriptor

- <a href="#sock_set_reuse_addr.reuse" name="sock_set_reuse_addr.reuse"></a> `reuse`: `u8`
1 to enable, 0 to disable

##### Results
- <a href="#sock_set_reuse_addr.error" name="sock_set_reuse_addr.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sock_get_reuse_addr" name="sock_get_reuse_addr"></a> `sock_get_reuse_addr(fd: fd) -> (errno, u8)`
Retrieve status of address reuse on a socket
Note: This is similar to `getsockopt` in POSIX for SO_REUSEADDR

##### Params
- <a href="#sock_get_reuse_addr.fd" name="sock_get_reuse_addr.fd"></a> `fd`: [`fd`](#fd)
Socket descriptor

##### Results
- <a href="#sock_get_reuse_addr.error" name="sock_get_reuse_addr.error"></a> `error`: [`errno`](#errno)

- <a href="#sock_get_reuse_addr.reuse" name="sock_get_reuse_addr.reuse"></a> `reuse`: `u8`


---

#### <a href="#sock_set_reuse_port" name="sock_set_reuse_port"></a> `sock_set_reuse_port(fd: fd, reuse: u8) -> errno`
Enable port reuse on a socket
Note: This is similar to `setsockopt` in POSIX for SO_REUSEPORT

##### Params
- <a href="#sock_set_reuse_port.fd" name="sock_set_reuse_port.fd"></a> `fd`: [`fd`](#fd)
Socket descriptor

- <a href="#sock_set_reuse_port.reuse" name="sock_set_reuse_port.reuse"></a> `reuse`: `u8`
1 to enable, 0 to disable

##### Results
- <a href="#sock_set_reuse_port.error" name="sock_set_reuse_port.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sock_get_reuse_port" name="sock_get_reuse_port"></a> `sock_get_reuse_port(fd: fd) -> (errno, u8)`
Retrieve status of port reuse on a socket
Note: This is similar to `getsockopt` in POSIX for SO_REUSEPORT

##### Params
- <a href="#sock_get_reuse_port.fd" name="sock_get_reuse_port.fd"></a> `fd`: [`fd`](#fd)
Socket descriptor

##### Results
- <a href="#sock_get_reuse_port.error" name="sock_get_reuse_port.error"></a> `error`: [`errno`](#errno)

- <a href="#sock_get_reuse_port.reuse" name="sock_get_reuse_port.reuse"></a> `reuse`: `u8`


---

#### <a href="#sock_set_recv_buf_size" name="sock_set_recv_buf_size"></a> `sock_set_recv_buf_size(fd: fd, size: size) -> errno`
Set size of receive buffer
Note: This is similar to `setsockopt` in POSIX for SO_RCVBUF

##### Params
- <a href="#sock_set_recv_buf_size.fd" name="sock_set_recv_buf_size.fd"></a> `fd`: [`fd`](#fd)
Socket descriptor

- <a href="#sock_set_recv_buf_size.size" name="sock_set_recv_buf_size.size"></a> `size`: [`size`](#size)
Buffer size

##### Results
- <a href="#sock_set_recv_buf_size.error" name="sock_set_recv_buf_size.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sock_get_recv_buf_size" name="sock_get_recv_buf_size"></a> `sock_get_recv_buf_size(fd: fd) -> (errno, size)`
Retrieve the size of the receive buffer
Note: This is similar to `getsockopt` in POSIX for SO_RCVBUF

##### Params
- <a href="#sock_get_recv_buf_size.fd" name="sock_get_recv_buf_size.fd"></a> `fd`: [`fd`](#fd)
Socket descriptor

##### Results
- <a href="#sock_get_recv_buf_size.error" name="sock_get_recv_buf_size.error"></a> `error`: [`errno`](#errno)

- <a href="#sock_get_recv_buf_size.size" name="sock_get_recv_buf_size.size"></a> `size`: [`size`](#size)


---

#### <a href="#sock_set_send_buf_size" name="sock_set_send_buf_size"></a> `sock_set_send_buf_size(fd: fd, size: size) -> errno`
Set size of send buffer
Note: This is similar to `setsockopt` in POSIX for SO_SNDBUF

##### Params
- <a href="#sock_set_send_buf_size.fd" name="sock_set_send_buf_size.fd"></a> `fd`: [`fd`](#fd)
Socket descriptor

- <a href="#sock_set_send_buf_size.size" name="sock_set_send_buf_size.size"></a> `size`: [`size`](#size)
Buffer size

##### Results
- <a href="#sock_set_send_buf_size.error" name="sock_set_send_buf_size.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sock_get_send_buf_size" name="sock_get_send_buf_size"></a> `sock_get_send_buf_size(fd: fd) -> (errno, size)`
Retrieve the size of the send buffer
Note: This is similar to `getsockopt` in POSIX for SO_SNDBUF

##### Params
- <a href="#sock_get_send_buf_size.fd" name="sock_get_send_buf_size.fd"></a> `fd`: [`fd`](#fd)
Socket descriptor

##### Results
- <a href="#sock_get_send_buf_size.error" name="sock_get_send_buf_size.error"></a> `error`: [`errno`](#errno)

- <a href="#sock_get_send_buf_size.size" name="sock_get_send_buf_size.size"></a> `size`: [`size`](#size)


---

#### <a href="#sock_bind" name="sock_bind"></a> `sock_bind(fd: fd, addr: Pointer<addr>) -> errno`
Bind a socket
Note: This is similar to `bind` in POSIX using PF_INET

##### Params
- <a href="#sock_bind.fd" name="sock_bind.fd"></a> `fd`: [`fd`](#fd)
File descriptor of the socket to be bind

- <a href="#sock_bind.addr" name="sock_bind.addr"></a> `addr`: `Pointer<addr>`
Address to bind the socket to

##### Results
- <a href="#sock_bind.error" name="sock_bind.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sock_listen" name="sock_listen"></a> `sock_listen(fd: fd, backlog: size) -> errno`
Listen for connections on a socket
Note: This is similar to `listen`

##### Params
- <a href="#sock_listen.fd" name="sock_listen.fd"></a> `fd`: [`fd`](#fd)
File descriptor of the socket to be bind

- <a href="#sock_listen.backlog" name="sock_listen.backlog"></a> `backlog`: [`size`](#size)
Maximum size of the queue for pending connections

##### Results
- <a href="#sock_listen.error" name="sock_listen.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sock_accept" name="sock_accept"></a> `sock_accept(fd: fd) -> (errno, fd)`
Accept a connection on a socket
Note: This is similar to `accept`

##### Params
- <a href="#sock_accept.fd" name="sock_accept.fd"></a> `fd`: [`fd`](#fd)
File descriptor of the socket to be bind

##### Results
- <a href="#sock_accept.error" name="sock_accept.error"></a> `error`: [`errno`](#errno)

- <a href="#sock_accept.childfd" name="sock_accept.childfd"></a> `childfd`: [`fd`](#fd)


---

#### <a href="#sock_connect" name="sock_connect"></a> `sock_connect(fd: fd, addr: Pointer<addr>) -> errno`
Initiate a connection on a socket to the specified address
Note: This is similar to `connect` in POSIX

##### Params
- <a href="#sock_connect.fd" name="sock_connect.fd"></a> `fd`: [`fd`](#fd)
Socket descriptor

- <a href="#sock_connect.addr" name="sock_connect.addr"></a> `addr`: `Pointer<addr>`
Address of the socket to connect to

##### Results
- <a href="#sock_connect.error" name="sock_connect.error"></a> `error`: [`errno`](#errno)


---

#### <a href="#sock_recv" name="sock_recv"></a> `sock_recv(fd: fd, buf: Pointer<u8>, buf_len: size, flags: riflags) -> (errno, size)`
Receive a message from a socket.
Note: This is similar to `recv` in POSIX.

##### Params
- <a href="#sock_recv.fd" name="sock_recv.fd"></a> `fd`: [`fd`](#fd)

- <a href="#sock_recv.buf" name="sock_recv.buf"></a> `buf`: `Pointer<u8>`
The buffer where data will be stored

- <a href="#sock_recv.buf_len" name="sock_recv.buf_len"></a> `buf_len`: [`size`](#size)

- <a href="#sock_recv.flags" name="sock_recv.flags"></a> `flags`: [`riflags`](#riflags)
Message flags.

##### Results
- <a href="#sock_recv.error" name="sock_recv.error"></a> `error`: [`errno`](#errno)

- <a href="#sock_recv.bufused" name="sock_recv.bufused"></a> `bufused`: [`size`](#size)


---

#### <a href="#sock_recv_from" name="sock_recv_from"></a> `sock_recv_from(fd: fd, buf: Pointer<u8>, buf_len: size, addr_buf: Pointer<u8>, addr_buf_len: size, flags: riflags) -> (errno, size)`
Receive a message from a socket.

The address buffer must be at least the size of addr_t.

Note: This is similar to `recvfrom` in POSIX.

##### Params
- <a href="#sock_recv_from.fd" name="sock_recv_from.fd"></a> `fd`: [`fd`](#fd)

- <a href="#sock_recv_from.buf" name="sock_recv_from.buf"></a> `buf`: `Pointer<u8>`
The buffer where data will be stored

- <a href="#sock_recv_from.buf_len" name="sock_recv_from.buf_len"></a> `buf_len`: [`size`](#size)

- <a href="#sock_recv_from.addr_buf" name="sock_recv_from.addr_buf"></a> `addr_buf`: `Pointer<u8>`
The address of origin for the message

- <a href="#sock_recv_from.addr_buf_len" name="sock_recv_from.addr_buf_len"></a> `addr_buf_len`: [`size`](#size)

- <a href="#sock_recv_from.flags" name="sock_recv_from.flags"></a> `flags`: [`riflags`](#riflags)
Message flags.

##### Results
- <a href="#sock_recv_from.error" name="sock_recv_from.error"></a> `error`: [`errno`](#errno)

- <a href="#sock_recv_from.bufused" name="sock_recv_from.bufused"></a> `bufused`: [`size`](#size)


---

#### <a href="#sock_send" name="sock_send"></a> `sock_send(fd: fd, buf: Pointer<u8>, buf_len: size, flags: siflags) -> (errno, size)`
Send a message on a socket.
Note: This is similar to `send` in POSIX.

##### Params
- <a href="#sock_send.fd" name="sock_send.fd"></a> `fd`: [`fd`](#fd)

- <a href="#sock_send.buf" name="sock_send.buf"></a> `buf`: `Pointer<u8>`
The buffer where data will be stored

- <a href="#sock_send.buf_len" name="sock_send.buf_len"></a> `buf_len`: [`size`](#size)

- <a href="#sock_send.flags" name="sock_send.flags"></a> `flags`: [`siflags`](#siflags)
Message flags.

##### Results
- <a href="#sock_send.error" name="sock_send.error"></a> `error`: [`errno`](#errno)

- <a href="#sock_send.bufused" name="sock_send.bufused"></a> `bufused`: [`size`](#size)
Number of bytes transmitted.


---

#### <a href="#sock_send_to" name="sock_send_to"></a> `sock_send_to(fd: fd, buf: Pointer<u8>, buf_len: size, addr: Pointer<addr>, flags: siflags) -> (errno, size)`
Send a message on a socket.
Note: This is similar to `sendto` in POSIX.

##### Params
- <a href="#sock_send_to.fd" name="sock_send_to.fd"></a> `fd`: [`fd`](#fd)

- <a href="#sock_send_to.buf" name="sock_send_to.buf"></a> `buf`: `Pointer<u8>`
The buffer where data will be stored

- <a href="#sock_send_to.buf_len" name="sock_send_to.buf_len"></a> `buf_len`: [`size`](#size)

- <a href="#sock_send_to.addr" name="sock_send_to.addr"></a> `addr`: `Pointer<addr>`
Address of the socket to send message to

- <a href="#sock_send_to.flags" name="sock_send_to.flags"></a> `flags`: [`siflags`](#siflags)
Message flags.

##### Results
- <a href="#sock_send_to.error" name="sock_send_to.error"></a> `error`: [`errno`](#errno)

- <a href="#sock_send_to.bufused" name="sock_send_to.bufused"></a> `bufused`: [`size`](#size)
Number of bytes transmitted.


---

#### <a href="#sock_shutdown" name="sock_shutdown"></a> `sock_shutdown(fd: fd, how: sdflags) -> errno`
Shut down socket send and receive channels.
Note: This is similar to `shutdown` in POSIX.

##### Params
- <a href="#sock_shutdown.fd" name="sock_shutdown.fd"></a> `fd`: [`fd`](#fd)

- <a href="#sock_shutdown.how" name="sock_shutdown.how"></a> `how`: [`sdflags`](#sdflags)
Which channels on the socket to shut down.

##### Results
- <a href="#sock_shutdown.error" name="sock_shutdown.error"></a> `error`: [`errno`](#errno)

