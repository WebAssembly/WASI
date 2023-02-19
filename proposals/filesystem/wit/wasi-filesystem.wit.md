# WASI Filesystem API

## `wasi-filesystem`
```wit
/// WASI filesystem is a filesystem API primarily intended to let users run WASI
/// programs that access their files on their existing filesystems, without
/// significant overhead.
///
/// It is intended to be roughly portable between Unix-family platforms and
/// Windows, though it does not hide many of the major differences.
///
/// Paths are passed as interface-type `string`s, meaning they must consist of
/// a sequence of Unicode Scalar Values (USVs). Some filesystems may contain paths
/// which are not accessible by this API.
default interface wasi-filesystem {
```

# Imports
```wit
use pkg.wasi-io.{input-stream, output-stream}
use pkg.wasi-wall-clock.{datetime}
```

## `filesize`
```wit
/// File size or length of a region within a file.
type filesize = u64
```

## `descriptor-type`
```wit
/// The type of a filesystem object referenced by a descriptor.
///
/// Note: This was called `filetype` in earlier versions of WASI.
enum descriptor-type {
    /// The type of the descriptor or file is unknown or is different from
    /// any of the other types specified.
    unknown,
    /// The descriptor refers to a block device inode.
    block-device,
    /// The descriptor refers to a character device inode.
    character-device,
    /// The descriptor refers to a directory inode.
    directory,
    /// The descriptor refers to a named pipe.
    fifo,
    /// The file refers to a symbolic link inode.
    symbolic-link,
    /// The descriptor refers to a regular file inode.
    regular-file,
    /// The descriptor refers to a socket.
    socket,
}
```

## `descriptor-flags`
```wit
/// Descriptor flags.
///
/// Note: This was called `fdflags` in earlier versions of WASI.
flags descriptor-flags {
    /// Read mode: Data can be read.
    read,
    /// Write mode: Data can be written to.
    write,
    /// Requests non-blocking operation.
    ///
    /// When this flag is enabled, functions may return immediately with an
    /// `error-code::would-block` error code in situations where they would otherwise
    /// block. However, this non-blocking behavior is not required.
    /// Implementations are permitted to ignore this flag and block.
    non-blocking,
    /// Request that writes be performed according to synchronized I/O file
    /// integrity completion. The data stored in the file and the file's
    /// metadata are synchronized.
    ///
    /// The precise semantics of this operation have not yet been defined for
    /// WASI. At this time, it should be interpreted as a request, and not a
    /// requirement.
    file-integrity-sync,
    /// Request that writes be performed according to synchronized I/O data
    /// integrity completion. Only the data stored in the file is
    /// synchronized.
    ///
    /// The precise semantics of this operation have not yet been defined for
    /// WASI. At this time, it should be interpreted as a request, and not a
    /// requirement.
    data-integrity-sync,
    /// Requests that reads be performed at the same level of integrety
    /// requested for writes.
    ///
    /// The precise semantics of this operation have not yet been defined for
    /// WASI. At this time, it should be interpreted as a request, and not a
    /// requirement.
    requested-write-sync,
    /// Mutating directories mode: Directory contents may be mutated.
    ///
    /// When this flag is unset on a descriptor, operations using the
    /// descriptor which would create, rename, delete, modify the data or
    /// metadata of filesystem objects, or obtain another handle which
    /// would permit any of those, shall fail with `error-code::read-only` if
    /// they would otherwise succeed.
    ///
    /// This may only be set on directories.
    mutate-directory,
}
```

## `descriptor-stat`
```wit
/// File attributes.
/// 
/// Note: This was called `filestat` in earlier versions of WASI.
record descriptor-stat {
    /// Device ID of device containing the file.
    device: device,
    /// File serial number.
    inode: inode,
    /// File type.
    %type: descriptor-type,
    /// Number of hard links to the file.
    link-count: link-count,
    /// For regular files, the file size in bytes. For symbolic links, the length
    /// in bytes of the pathname contained in the symbolic link.
    size: filesize,
    /// Last data access timestamp.
    data-access-timestamp: datetime,
    /// Last data modification timestamp.
    data-modification-timestamp: datetime,
    /// Last file status change timestamp.
    status-change-timestamp: datetime,
}
```

## `path-flags`
```wit
/// Flags determining the method of how paths are resolved.
flags path-flags {
    /// As long as the resolved path corresponds to a symbolic link, it is expanded.
    symlink-follow,
}
```

## `open-flags`
```wit
/// Open flags used by `open-at`.
flags open-flags {
    /// Create file if it does not exist.
    create,
    /// Fail if not a directory.
    directory,
    /// Fail if file already exists.
    exclusive,
    /// Truncate file to size 0.
    truncate,
}
```

## `modes`
```wit
/// Permissions mode used by `open-at`, `change-file-permissions-at`, and
/// similar.
flags modes {
    /// True if the resource is considered readable by the containing
    /// filesystem.
    readable,
    /// True if the resource is considered writeable by the containing
    /// filesystem.
    writeable,
    /// True if the resource is considered executable by the containing
    /// filesystem. This does not apply to directories.
    executable,
}
```

## `link-count`
```wit
/// Number of hard links to an inode.
type link-count = u64
```

## `device`
```wit
/// Identifier for a device containing a file system. Can be used in combination
/// with `inode` to uniquely identify a file or directory in the filesystem.
type device = u64
```

## `inode`
```wit
/// Filesystem object serial number that is unique within its file system.
type inode = u64
```

## `new-timestamp`
```wit
/// When setting a timestamp, this gives the value to set it to.
variant new-timestamp {
    /// Leave the timestamp set to its previous value.
    no-change,
    /// Set the timestamp to the current time of the system clock associated
    /// with the filesystem.
    now,
    /// Set the timestamp to the given value.
    timestamp(datetime),
}
```

## `directory-entry`
```wit
/// A directory entry. 
record directory-entry {
    /// The serial number of the object referred to by this directory entry.
    /// May be none if the inode value is not known.
    ///
    /// When this is none, libc implementations might do an extra `stat-at`
    /// call to retrieve the inode number to fill their `d_ino` fields, so
    /// implementations which can set this to a non-none value should do so.
    inode: option<inode>,

    /// The type of the file referred to by this directory entry.
    %type: descriptor-type,

    /// The name of the object.
    name: string,
}
```

## `error-code`
```wit
/// Error codes returned by functions.
/// Not all of these error codes are returned by the functions provided by this
/// API; some are used in higher-level library layers, and others are provided
/// merely for alignment with POSIX.
enum error-code {
    /// Permission denied.
    access,
    /// Resource unavailable, or operation would block.
    would-block,
    /// Connection already in progress.
    already,
    /// Bad descriptor.
    bad-descriptor,
    /// Device or resource busy.
    busy,
    /// Resource deadlock would occur.
    deadlock,
    /// Storage quota exceeded.
    quota,
    /// File exists.
    exist,
    /// File too large.
    file-too-large,
    /// Illegal byte sequence.
    Illegal-byte-sequence,
    /// Operation in progress.
    in-progress,
    /// Interrupted function.
    interrupted,
    /// Invalid argument.
    invalid,
    /// I/O error.
    io,
    /// Is a directory.
    is-directory,
    /// Too many levels of symbolic links.
    loop,
    /// Too many links.
    too-many-links,
    /// Message too large.
    message-size,
    /// Filename too long.
    name-too-long,
    /// No such device.
    no-device,
    /// No such file or directory.
    no-entry,
    /// No locks available.
    no-lock,
    /// Not enough space.
    insufficient-memory,
    /// No space left on device.
    insufficient-space,
    /// Not a directory or a symbolic link to a directory.
    not-directory,
    /// Directory not empty.
    not-empty,
    /// State not recoverable.
    not-recoverable,
    /// Not supported
    unsupported,
    /// Inappropriate I/O control operation.
    no-tty,
    /// No such device or address.
    no-such-device,
    /// Value too large to be stored in data type.
    overflow,
    /// Operation not permitted.
    not-permitted,
    /// Broken pipe.
    pipe,
    /// Read-only file system.
    read-only,
    /// Invalid seek.
    invalid-seek,
    /// Text file busy.
    text-file-busy,
    /// Cross-device link.
    cross-device,
}
```

## `advice`
```wit
/// File or memory access pattern advisory information.
enum advice {
    /// The application has no advice to give on its behavior with respect to the specified data.
    normal,
    /// The application expects to access the specified data sequentially from lower offsets to higher offsets.
    sequential,
    /// The application expects to access the specified data in a random order.
    random,
    /// The application expects to access the specified data in the near future.
    will-need,
    /// The application expects that it will not access the specified data in the near future.
    dont-need,
    /// The application expects to access the specified data once and then not reuse it thereafter.
    no-reuse,
}
```

## `descriptor`
```wit
/// A descriptor is a reference to a filesystem object, which may be a file,
/// directory, named pipe, special file, or other object on which filesystem
/// calls may be made.
// TODO(resource descriptor {)
type descriptor = u32
```

## `read-via-stream`
```wit
/// Return a stream for reading from a file.
///
/// Note: This allows using `read-stream`, which is similar to `read` in POSIX.
read-via-stream: func(
    this: descriptor,
    /// The offset within the file at which to start reading.
    offset: filesize,
) -> result<input-stream, error-code>
```

## `write-via-stream`
```wit
/// Return a stream for writing to a file.
///
/// Note: This allows using `write-stream`, which is similar to `write` in POSIX.
write-via-stream: func(
    this: descriptor,
    /// The offset within the file at which to start writing.
    offset: filesize,
) -> result<output-stream, error-code>
```

## `append-via-stream`
```wit
/// Return a stream for appending to a file.
///
/// Note: This allows using `write-stream`, which is similar to `write` with
/// `O_APPEND` in in POSIX.
append-via-stream: func(
    this: descriptor,
    /// The resource to operate on.
    fd: descriptor,
) -> result<output-stream, error-code>
```

## `advise`
```wit
/// Provide file advisory information on a descriptor.
///
/// This is similar to `posix_fadvise` in POSIX.
advise: func(
    this: descriptor,
    /// The offset within the file to which the advisory applies.
    offset: filesize,
    /// The length of the region to which the advisory applies.
    length: filesize,
    /// The advice.
    advice: advice
) -> result<_, error-code>
```

## `sync-data`
```wit
/// Synchronize the data of a file to disk.
///
/// This function succeeds with no effect if the file descriptor is not
/// opened for writing.
///
/// Note: This is similar to `fdatasync` in POSIX.
sync-data: func(this: descriptor) -> result<_, error-code>
```

## `get-flags`
```wit
/// Get flags associated with a descriptor.
///
/// Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX.
///
/// Note: This returns the value that was the `fs_flags` value returned
/// from `fdstat_get` in earlier versions of WASI.
get-flags: func(this: descriptor) -> result<descriptor-flags, error-code>
```

## `get-type`
```wit
/// Get the dynamic type of a descriptor.
///
/// Note: This returns the same value as the `type` field of the `fd-stat`
/// returned by `stat`, `stat-at` and similar.
///
/// Note: This returns similar flags to the `st_mode & S_IFMT` value provided
/// by `fstat` in POSIX.
///
/// Note: This returns the value that was the `fs_filetype` value returned
/// from `fdstat_get` in earlier versions of WASI.
get-type: func(this: descriptor) -> result<descriptor-type, error-code>
```

## `set-flags`
```wit
/// Set status flags associated with a descriptor.
///
/// This function may only change the `non-blocking` flag.
///
/// Note: This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX.
///
/// Note: This was called `fd_fdstat_set_flags` in earlier versions of WASI.
set-flags: func(this: descriptor, %flags: descriptor-flags) -> result<_, error-code>
```

## `set-size`
```wit
/// Adjust the size of an open file. If this increases the file's size, the
/// extra bytes are filled with zeros.
///
/// Note: This was called `fd_filestat_set_size` in earlier versions of WASI.
set-size: func(this: descriptor, size: filesize) -> result<_, error-code>
```

## `set-times`
```wit
/// Adjust the timestamps of an open file or directory.
///
/// Note: This is similar to `futimens` in POSIX.
///
/// Note: This was called `fd_filestat_set_times` in earlier versions of WASI.
set-times: func(
    this: descriptor,
    /// The desired values of the data access timestamp.
    data-access-timestamp: new-timestamp,
    /// The desired values of the data modification timestamp.
    data-modification-timestamp: new-timestamp,
) -> result<_, error-code>
```

## `read`
```wit
/// Read from a descriptor, without using and updating the descriptor's offset.
///
/// This function returns a list of bytes containing the data that was
/// read, along with a bool which, when true, indicates that the end of the
/// file was reached. The returned list will contain up to `length` bytes; it
/// may return fewer than requested, if the end of the file is reached or
/// if the I/O operation is interrupted.
///
/// Note: This is similar to `pread` in POSIX.
// TODO(stream<u8, error-code>)
read: func(
    this: descriptor,
    /// The maximum number of bytes to read.
    length: filesize,
    /// The offset within the file at which to read.
    offset: filesize,
) -> result<tuple<list<u8>, bool>, errno>
```

## `write`
```wit
/// Write to a descriptor, without using and updating the descriptor's offset.
///
/// It is valid to write past the end of a file; the file is extended to the
/// extent of the write, with bytes between the previous end and the start of
/// the write set to zero.
///
/// Note: This is similar to `pwrite` in POSIX.
// TODO(stream<u8, error-code>)
write: func(
    this: descriptor,
    /// Data to write
    buffer: list<u8>,
    /// The offset within the file at which to write.
    offset: filesize,
) -> result<filesize, error-code>
```

## `read-directory`
```wit
/// Read directory entries from a directory.
///
/// On filesystems where directories contain entries referring to themselves
/// and their parents, often named `.` and `..` respectively, these entries
/// are omitted.
///
/// This always returns a new stream which starts at the beginning of the
/// directory.
read-directory: func(this: descriptor) -> result<directory-entry-stream, error-code>
```

## `sync`
```wit
/// Synchronize the data and metadata of a file to disk.
///
/// This function succeeds with no effect if the file descriptor is not
/// opened for writing.
///
/// Note: This is similar to `fsync` in POSIX.
sync: func(this: descriptor) -> result<_, error-code>
```

## `create-directory-at`
```wit
/// Create a directory.
///
/// Note: This is similar to `mkdirat` in POSIX.
create-directory-at: func(
    this: descriptor,
    /// The relative path at which to create the directory.
    path: string,
) -> result<_, error-code>
```

## `stat`
```wit
/// Return the attributes of an open file or directory.
///
/// Note: This is similar to `fstat` in POSIX.
///
/// Note: This was called `fd_filestat_get` in earlier versions of WASI.
stat: func(this: descriptor) -> result<descriptor-stat, error-code>
```

## `stat-at`
```wit
/// Return the attributes of a file or directory.
///
/// Note: This is similar to `fstatat` in POSIX.
///
/// Note: This was called `path_filestat_get` in earlier versions of WASI.
stat-at: func(
    this: descriptor,
    /// Flags determining the method of how the path is resolved.
    path-flags: path-flags,
    /// The relative path of the file or directory to inspect.
    path: string,
) -> result<descriptor-stat, error-code>
```

## `set-times-at`
```wit
/// Adjust the timestamps of a file or directory.
///
/// Note: This is similar to `utimensat` in POSIX.
///
/// Note: This was called `path_filestat_set_times` in earlier versions of WASI.
set-times-at: func(
    this: descriptor,
    /// Flags determining the method of how the path is resolved.
    path-flags: path-flags,
    /// The relative path of the file or directory to operate on.
    path: string,
    /// The desired values of the data access timestamp.
    data-access-timestamp: new-timestamp,
    /// The desired values of the data modification timestamp.
    data-modification-timestamp: new-timestamp,
) -> result<_, error-code>
```

## `link-at`
```wit
/// Create a hard link.
///
/// Note: This is similar to `linkat` in POSIX.
link-at: func(
    this: descriptor,
    /// Flags determining the method of how the path is resolved.
    old-path-flags: path-flags,
    /// The relative source path from which to link.
    old-path: string,
    /// The base directory for `new-path`.
    new-descriptor: descriptor,
    /// The relative destination path at which to create the hard link.
    new-path: string,
) -> result<_, error-code>
```

## `open-at`
```wit
/// Open a file or directory.
///
/// The returned descriptor is not guaranteed to be the lowest-numbered
/// descriptor not currently open/ it is randomized to prevent applications
/// from depending on making assumptions about indexes, since this is
/// error-prone in multi-threaded contexts. The returned descriptor is
/// guaranteed to be less than 2**31.
///
/// If `flags` contains `descriptor-flags::mutate-directory`, and the base
/// descriptor doesn't have `descriptor-flags::mutate-directory` set,
/// `open-at` fails with `error-code::read-only`.
///
/// If `flags` contains `write`, or `open-flags` contains `truncate`
/// or `create`, and the base descriptor doesn't have
/// `descriptor-flags::mutate-directory` set, `open-at` fails with
/// `error-code::read-only`.
///
/// Note: This is similar to `openat` in POSIX.
open-at: func(
    this: descriptor,
    /// Flags determining the method of how the path is resolved.
    path-flags: path-flags,
    /// The relative path of the object to open.
    path: string,
    /// The method by which to open the file.
    open-flags: open-flags,
    /// Flags to use for the resulting descriptor.
    %flags: descriptor-flags,
    /// Permissions to use when creating a new file.
    modes: modes
) -> result<descriptor, error-code>
```

## `readlink-at`
```wit
/// Read the contents of a symbolic link.
///
/// Note: This is similar to `readlinkat` in POSIX.
readlink-at: func(
    this: descriptor,
    /// The relative path of the symbolic link from which to read.
    path: string,
) -> result<string, error-code>
```

## `remove-directory-at`
```wit
/// Remove a directory.
///
/// Return `error-code::not-empty` if the directory is not empty.
///
/// Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
remove-directory-at: func(
    this: descriptor,
    /// The relative path to a directory to remove.
    path: string,
) -> result<_, error-code>
```

## `rename-at`
```wit
/// Rename a filesystem object.
///
/// Note: This is similar to `renameat` in POSIX.
rename-at: func(
    this: descriptor,
    /// The relative source path of the file or directory to rename.
    old-path: string,
    /// The base directory for `new-path`.
    new-descriptor: descriptor,
    /// The relative destination path to which to rename the file or directory.
    new-path: string,
) -> result<_, error-code>
```

## `symlink-at`
```wit
/// Create a symbolic link.
///
/// Note: This is similar to `symlinkat` in POSIX.
symlink-at: func(
    this: descriptor,
    /// The contents of the symbolic link.
    old-path: string,
    /// The relative destination path at which to create the symbolic link.
    new-path: string,
) -> result<_, error-code>
```

## `unlink-file-at`
```wit
/// Unlink a filesystem object that is not a directory.
///
/// Return `error-code::is-directory` if the path refers to a directory.
/// Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.
unlink-file-at: func(
    this: descriptor,
    /// The relative path to a file to unlink.
    path: string,
) -> result<_, error-code>
```

## `change-file-permissions-at`
```wit
/// Change the permissions of a filesystem object that is not a directory.
///
/// Note that the ultimate meanings of these permissions is
/// filesystem-specific.
///
/// Note: This is similar to `fchmodat` in POSIX.
change-file-permissions-at: func(
    this: descriptor,
    /// Flags determining the method of how the path is resolved.
    path-flags: path-flags,
    /// The relative path to operate on.
    path: string,
    /// The new permissions for the filesystem object.
    modes: modes,
) -> result<_, error-code>
```

## `change-dir-permissions-at`
```wit
/// Change the permissions of a directory.
///
/// Note that the ultimate meanings of these permissions is
/// filesystem-specific.
///
/// Unlike in POSIX, the `executable` flag is not reinterpreted as a "search"
/// flag. `read` on a directory implies readability and searchability, and
/// `execute` is not valid for directories.
///
/// Note: This is similar to `fchmodat` in POSIX.
change-directory-permissions-at: func(
    this: descriptor,
    /// Flags determining the method of how the path is resolved.
    path-flags: path-flags,
    /// The relative path to operate on.
    path: string,
    /// The new permissions for the directory.
    modes: modes,
) -> result<_, error-code>
```

## `lock-shared`
```wit
/// Request a shared advisory lock for an open file.
///
/// This requests a *shared* lock; more than one shared lock can be held for
/// a file at the same time.
///
/// If the open file has an exclusive lock, this function downgrades the lock
/// to a shared lock. If it has a shared lock, this function has no effect.
///
/// This requests an *advisory* lock, meaning that the file could be accessed
/// by other programs that don't hold the lock.
///
/// It is unspecified how shared locks interact with locks acquired by
/// non-WASI programs.
///
/// This function blocks until the lock can be acquired.
///
/// Not all filesystems support locking; on filesystems which don't support
/// locking, this function returns `error-code::unsupported`.
///
/// Note: This is similar to `flock(fd, LOCK_SH)` in Unix.
lock-shared: func(this: descriptor) -> result<_, error-code>
```

## `lock-exclusive`
```wit
/// Request an exclusive advisory lock for an open file.
///
/// This requests an *exclusive* lock; no other locks may be held for the
/// file while an exclusive lock is held.
///
/// If the open file has a shared lock and there are no exclusive locks held
/// for the file, this function upgrades the lock to an exclusive lock. If the
/// open file already has an exclusive lock, this function has no effect.
///
/// This requests an *advisory* lock, meaning that the file could be accessed
/// by other programs that don't hold the lock.
///
/// It is unspecified whether this function succeeds if the file descriptor
/// is not opened for writing. It is unspecified how exclusive locks interact
/// with locks acquired by non-WASI programs.
///
/// This function blocks until the lock can be acquired.
///
/// Not all filesystems support locking; on filesystems which don't support
/// locking, this function returns `error-code::unsupported`.
///
/// Note: This is similar to `flock(fd, LOCK_EX)` in Unix.
lock-exclusive: func(this: descriptor) -> result<_, error-code>
```

## `try-lock-shared`
```wit
/// Request a shared advisory lock for an open file.
///
/// This requests a *shared* lock; more than one shared lock can be held for
/// a file at the same time.
///
/// If the open file has an exclusive lock, this function downgrades the lock
/// to a shared lock. If it has a shared lock, this function has no effect.
///
/// This requests an *advisory* lock, meaning that the file could be accessed
/// by other programs that don't hold the lock.
///
/// It is unspecified how shared locks interact with locks acquired by
/// non-WASI programs.
///
/// This function returns `error-code::would-block` if the lock cannot be acquired.
///
/// Not all filesystems support locking; on filesystems which don't support
/// locking, this function returns `error-code::unsupported`.
///
/// Note: This is similar to `flock(fd, LOCK_SH | LOCK_NB)` in Unix.
try-lock-shared: func(this: descriptor) -> result<_, error-code>
```

## `try-lock-exclusive`
```wit
/// Request an exclusive advisory lock for an open file.
///
/// This requests an *exclusive* lock; no other locks may be held for the
/// file while an exclusive lock is held.
///
/// If the open file has a shared lock and there are no exclusive locks held
/// for the file, this function upgrades the lock to an exclusive lock. If the
/// open file already has an exclusive lock, this function has no effect.
///
/// This requests an *advisory* lock, meaning that the file could be accessed
/// by other programs that don't hold the lock.
///
/// It is unspecified whether this function succeeds if the file descriptor
/// is not opened for writing. It is unspecified how exclusive locks interact
/// with locks acquired by non-WASI programs.
///
/// This function returns `error-code::would-block` if the lock cannot be acquired.
///
/// Not all filesystems support locking; on filesystems which don't support
/// locking, this function returns `error-code::unsupported`.
///
/// Note: This is similar to `flock(fd, LOCK_EX | LOCK_NB)` in Unix.
try-lock-exclusive: func(this: descriptor) -> result<_, error-code>
```

## `unlock`
```wit
/// Release a shared or exclusive lock on an open file.
///
/// Note: This is similar to `flock(fd, LOCK_UN)` in Unix.
unlock: func(this: descriptor) -> result<_, error-code>
```

# `drop-descriptor`
```wit
/// Dispose of the specified `descriptor`, after which it may no longer
/// be used.
// TODO(} /* resource descriptor */)
drop-descriptor: func(this: descriptor)
```

## `directory-entry-stream`
```wit
/// A stream of directory entries.
// TODO(resource directory-entry-stream {)
// TODO(stream<directory-entry, error-code>)
type directory-entry-stream = u32
```

## `read-directory-entry`
```wit
/// Read a single directory entry from a `directory-entry-stream`.
read-directory-entry: func(this: directory-entry-stream) -> result<option<directory-entry>, error-code>
```

# `drop-directory-entry-stream`
```wit
/// Dispose of the specified `directory-entry-stream`, after which it may no longer
/// be used.
// TODO(} /* resource directory-entry-stream */)
drop-directory-entry-stream: func(this: directory-entry-stream)
```

```wit
}
```
