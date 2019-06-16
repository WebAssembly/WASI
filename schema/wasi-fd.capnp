@0xf8a5c786b8463cf0;

using import "wasi-common.capnp".Errno;
using import "wasi-common.capnp".Timestamp;
using import "wasi-common.capnp".Size;

using Filesize = UInt64; # Non-negative file size or length of a region within a file.
using Iovec = Data; # A region of memory for scatter/gather reads.

# The type of a file descriptor or file.
enum Filetype {
  unknown @0; # The type of the file descriptor or file is unknown or is different from any of the other types specified.
  blockDevices @1; # The file descriptor or file refers to a block device inode.
  characterDevice @2; # The file descriptor or file refers to a character device inode.
  directory @3; # The file descriptor or file refers to a directory inode.
  regularFile @4; # The file descriptor or file refers to a regular file inode.
  socketDgram @5; # The file descriptor or file refers to a datagram socket.
  socketStream @6; # The file descriptor or file refers to a byte-stream socket.
  symbolicLink @7; # The file refers to a symbolic link inode.
}

# File descriptor rights, determining which actions may be performed.
struct Rights {
  fdDatasync @0 :Bool; # The right to invoke __wasi_fd_datasync(). If __WASI_RIGHT_PATH_OPEN is set, includes the right to invoke __wasi_path_open() with __WASI_FDFLAG_DSYNC.
  fdRead @1 :Bool; # The right to invoke __wasi_fd_read() and __wasi_sock_recv(). If __WASI_RIGHT_FD_SEEK is set, includes the right to invoke __wasi_fd_pread().
  fdSeek @2 :Bool; # The right to invoke __wasi_fd_seek(). This flag implies __WASI_RIGHT_FD_TELL.
  fdFdstatSetFlags @3 :Bool; # The right to invoke __wasi_fd_fdstat_set_flags().
  fdSync @4 :Bool; # The right to invoke __wasi_fd_sync(). If __WASI_RIGHT_PATH_OPEN is set, includes the right to invoke __wasi_path_open() with __WASI_FDFLAG_RSYNC and __WASI_FDFLAG_DSYNC.
  fdTell @5 :Bool; # The right to invoke __wasi_fd_seek() in such a way that the file offset remains unaltered (i.e., __WASI_WHENCE_CUR with offset zero), or to invoke __wasi_fd_tell().
  fdWrite @6 :Bool; # The right to invoke __wasi_fd_write() and __wasi_sock_send(). If __WASI_RIGHT_FD_SEEK is set, includes the right to invoke __wasi_fd_pwrite().
  fdAdvise @7 :Bool; # The right to invoke __wasi_fd_advise().
  fdAllocate @8 :Bool; # The right to invoke __wasi_fd_allocate().
  pathCreateDirectory @9 :Bool; # The right to invoke __wasi_path_create_directory().
  pathCreateFile @10 :Bool; # If __WASI_RIGHT_PATH_OPEN is set, the right to invoke __wasi_path_open() with __WASI_O_CREAT.
  pathLinkSource @11 :Bool; # The right to invoke __wasi_path_link() with the file descriptor as the source directory.
  pathLinkTarget @12 :Bool; # The right to invoke __wasi_path_link() with the file descriptor as the target directory.
  pathOpen @13 :Bool; # The right to invoke __wasi_path_open().
  fdReaddir @14 :Bool; # The right to invoke __wasi_fd_readdir().
  pathReadlink @15 :Bool; # The right to invoke __wasi_path_readlink().
  pathRenameSource @16 :Bool; # The right to invoke __wasi_path_rename() with the file descriptor as the source directory.
  pathRenameTarget @17 :Bool; # The right to invoke __wasi_path_rename() with the file descriptor as the target directory.
  pathFilestatGet @18 :Bool; # The right to invoke __wasi_path_filestat_get().
  # The right to change a file's size (there is no __wasi_path_filestat_set_size()). 
  # If __WASI_RIGHT_PATH_OPEN is set, includes the right to invoke __wasi_path_open with __WASI_O_TRUNC.
  pathFilestatSetSize @19 :Bool; 
  pathFilestatSetTimes @20 :Bool; # The right to invoke __wasi_path_filestat_set_times().
  fdFilestatGet @21 :Bool; # The right to invoke __wasi_fd_filestat_get().
  fdFilestatSetSize @22 :Bool; # The right to invoke __wasi_fd_filestat_set_size().
  fdFilestatSetTimes @23 :Bool; # The right to invoke __wasi_fd_filestat_set_times().
  pathSymlink @24 :Bool; # The right to invoke __wasi_path_symlink().
  pathUnlinkFile @25 :Bool; # The right to invoke __wasi_path_unlink_file().
  pathRemoveDirectory @26 :Bool; # The right to invoke __wasi_path_remove_directory().
  # If __WASI_RIGHT_FD_READ is set, includes the right to invoke __wasi_poll_oneoff() to subscribe to __WASI_EVENTTYPE_FD_READ.
  # If __WASI_RIGHT_FD_WRITE is set, includes the right to invoke __wasi_poll_oneoff() to subscribe to __WASI_EVENTTYPE_FD_WRITE.
  pollFdReadwrite @27 :Bool; 
  sockShutdown @28 :Bool;  # The right to invoke __wasi_sock_shutdown().
}

# File descriptor flags.
struct Fdflags {
  append @0 :Bool; # Append mode: Data written to the file is always appended to the file's end.
  dsync @1 :Bool; # Write according to synchronized I/O data integrity completion. Only the data stored in the file is synchronized.
  nonblock @2 :Bool; # Non-blocking mode.
  rsync @3 :Bool; # Synchronized read I/O operations.
  sync @4 :Bool; # Write according to synchronized I/O file integrity completion. In addition to synchronizing the data stored in the file, the implementation may also synchronously update the file's metadata.
}


interface FileDescriptor { 
  # File or memory access pattern advisory information.
  enum Advice {
    dontneed @0; # The application expects that it will not access the specified data in the near future.
    noreuse @1; # The application expects to access the specified data once and then not reuse it thereafter.
    normal @2; # The application has no advice to give on its behavior with respect to the specified data.
    random @3; # The application expects to access the specified data in a random order.
    sequential @4; # The application expects to access the specified data sequentially from lower offsets to higher offsets.
    willneed @5; # The application expects to access the specified data in the near future.
  }

  # File descriptor attributes.
  struct FdStat {
    fsFiletype @0 :Filetype; # File type.
    fsFlags @1 :Fdflags; # File descriptor flags.
    fsRightsBase @2 :Rights; # Rights that apply to this file descriptor.
    fsRightsInheriting @3 :Rights; # Maximum set of rights that may be installed on new file descriptors that are created through this file descriptor, e.g., through __wasi_path_open().
  }

  # Which file time attributes to adjust.
  struct Fstflags {
    filestatSetAtim @0 :Bool; # Adjust the last data access timestamp to the value stored in __wasi_filestat_t::st_atim.
    filestatSetAtimNow @1 :Bool; # Adjust the last data access timestamp to the time of clock __WASI_CLOCK_REALTIME.
    filestatSetMtim @2 :Bool; # Adjust the last data modification timestamp to the value stored in __wasi_filestat_t::st_mtim.
    filestatSetMtimNow @3 :Bool; # Adjust the last data modification timestamp to the time of clock __WASI_CLOCK_REALTIME.
  }

  # Provide file advisory information on a file descriptor.
  # Note: This is similar to posix_fadvise in POSIX. It can also operate on folders https://lwn.net/Articles/578207/
  advise @0 (
    offset :Filesize, # The offset within the file to which the advisory applies.
    len :Filesize, # The length of the region to which the advisory applies.
    advice :Advice # The advice.
  ) -> (
    error :Errno
  );

  # Close a file descriptor.
  # Note: This is similar to close in POSIX.
  close @1 () -> (
    error :Errno
  );

  # Synchronize the data of a file to disk.
  # Note: This is similar to fdatasync in POSIX.
  datasync @2 () -> (
    error :Errno
  );
  
  # Atomically replace a file descriptor by renumbering another file descriptor.
  # Due to the strong focus on thread safety, this environment does not provide a mechanism to duplicate or renumber a file descriptor to an arbitrary number, like dup2(). This would be prone to race conditions, as an actual file descriptor with the same number could be allocated by a different thread at the same time.
  # This function provides a way to atomically renumber file descriptors, which would disappear if dup2() were to be removed entirely.
  renumber @3 (
    to :FileDescriptor # The file descriptor to overwrite.  
  ) -> (
    error :Errno
  );

  # Get the attributes of a file descriptor.
  # Note: This returns similar flags to fsync(fd, F_GETFL) in POSIX, as well as additional fields.
  fdstatGet @4 () -> (
    buf :FdStat, # The buffer where the file descriptor's attributes are stored.
    error :Errno
  );

  # Adjust the flags associated with a file descriptor.
  # Note: This is similar to fcntl(fd, F_SETFL, flags) in POSIX.
  fdstatSetFlags @5 (
    flags :Fdflags # The desired values of the file descriptor flags.
  ) -> (
    error :Errno
  );

  # Adjust the rights associated with a file descriptor.
  # This can only be used to remove rights, and returns __WASI_ENOTCAPABLE if called in a way that would attempt to add rights.
  fdstatSetRights @6 (
    fsRightsBase :Rights, # The desired rights of the file descriptor. 
    fsRightsInheriting :Rights
  ) -> (
    error :Errno
  );

  # Return the attributes of an open file.
  filestatGet @7 () -> (
    buf :FdStat, #the file descriptor's attributes.
    error :Errno
  );

  # Adjust the timestamps of an open file or directory.
  # Note: This is similar to futimens in POSIX.
  filestatSetTimes @8 (
    stAtim :Timestamp, # The desired values of the data access timestamp.
    stMtim :Timestamp, # The desired values of the data modification timestamp.
    fstFlags :Fstflags # A bitmask indicating which timestamps to adjust.
  ) -> (
    error :Errno
  );

  # Return a description of the given preopened file descriptor.
  prestatGet @9 () -> (
    #buf :__wasi_prestat_t  # The buffer where the description is stored.
    error :Errno
  );

  # Return a description of the given preopened file descriptor.
  prestatDirName @10 () -> (
    error :Errno,
    path :Text # A buffer into which to write the preopened directory name.
  );

  # Synchronize the data and metadata of a file to disk.
  # Note: This is similar to fsync in POSIX.
  sync @11 () -> (
    error :Errno
  );
}

interface File extends (FileDescriptor) {
  using Filedelta = Int64; # File serial number that is unique within its file system.

  # The position relative to which to set the offset of the file descriptor.
  enum Whence {
    cur @0; # Seek relative to current position.
    end @1; # Seek relative to end-of-file.
    set @2; # Seek relative to start-of-file.
  }

  # Force the allocation of space in a file. 
  # Note: This is similar to posix_fallocate in POSIX.
  allocate @0 (
    offset :Filesize, # The offset at which to start the allocation.
    len :Filesize # The length of the area that is allocated.
  ) -> (
    error :Errno
  );


  # Adjust the size of an open file. If this increases the file's size, the extra bytes are filled with zeros.
  # Note: This is similar to ftruncate in POSIX.
  filestatSetSize @1 (
    stSize :Filesize # The desired file size.
  ) -> (
    error :Errno
  );

  # Read from a file descriptor, without using and updating the file descriptor's offset.
  # Note: This is similar to preadv in POSIX.
  pread @2 (
    offset :Filesize
  ) -> (
    iovs :List(Iovec), # List of scatter/gather vectors to which to store data.
    nread :Size, # The number of bytes read.
    error :Errno
  );

  # Write to a file descriptor, without using and updating the file descriptor's offset.
  # Note: This is similar to pwritev in POSIX
  pwrite @3 (
    iovs :List(Iovec), # List of scatter/gather vectors from which to retrieve data.
    offset :Filesize # The offset within the file at which to write.
  ) -> (
    error :Errno,
    nwritten :Size # The number of bytes written.
  );

  # Read from a file descriptor.
  # Note: This is similar to readv in POSIX.
  read @4 (
    iovs :List(Iovec), # List of scatter/gather vectors to which to store data.
  ) -> (
    error :Errno,
    nread :Size
  );

  # Move the offset of a file descriptor.
  # Note: This is similar to lseek in POSIX.
  seek @5 (
    offset :Filedelta, # The number of bytes to move.
    whence :Whence # The base from which the offset is relative.
  ) -> (
    error :Errno,
    newoffset :Size # The new offset of the file descriptor, relative to the start of the file.
  );

  # Return the current offset of a file descriptor.
  # Note: This is similar to lseek(fd, 0, SEEK_CUR) in POSIX.
  tell @6 () -> (
    error :Errno,
    offset :Filesize # The current offset of the file descriptor, relative to the start of the file.
  );

  # Write to a file descriptor.
  # Note: This is similar to writev in POSIX.
  write @7 (
    iovs :List(Iovec), # List of scatter/gather vectors from which to retrieve data.
  ) -> (
    error :Errno,
    nwritten :Size # The number of bytes written.
  );
}


interface Directory extends (FileDescriptor) {
  using Dircookie = UInt64; # A reference to the offset of a directory entry.
  using Inode = UInt64; # File serial number that is unique within its file system.
  using Device = UInt64; # Identifier for a device containing a file system. Can be used in combination with __wasi_inode_t to uniquely identify a file or directory in the filesystem.
  using Linkcount = UInt32; # Number of hard links to an inode.

  # Flags determining the method of how paths are resolved.
  enum LookupFlags {
    lookupSymlinkFollow @0; # As long as the resolved path corresponds to a symbolic link, it is expanded.
  }

  # Open flags
  enum Oflags {
    creat @0; # Create file if it does not exist.
    directory @1; # Fail if not a directory.
    excl @2; # Fail if file already exists.
    trunc @3; # Truncate file to size 0.
  }

  struct Filestat {
    dev @0 :Device; # Device ID of device containing the file.
    ino @1 :Inode; # File serial number.
    filetype @2 :Filetype; # File type.
    nlink @3 :Linkcount; # Number of hard links to the file.
    size @4 :Filesize; # For regular files, the file size in bytes. For symbolic links, the length in bytes of the pathname contained in the symbolic link.
    atim @5 :Timestamp; # Last data access timestamp.
    mtim @6 :Timestamp; # Last data modification timestamp.
    ctim @7 :Timestamp; # Last file status change timestamp.
  }
  
  # Which file time attributes to adjust.
  struct Fstflags {
    filestatSetAtim @0 :Bool; # Adjust the last data access timestamp to the value stored in __wasi_filestat_t::st_atim.
    filestatSetAtimNow @1 :Bool; # Adjust the last data access timestamp to the time of clock __WASI_CLOCK_REALTIME.
    filestatSetMtim @2 :Bool; # Adjust the last data modification timestamp to the value stored in __wasi_filestat_t::st_mtim.
    filestatSetMtimNow @3 :Bool; # Adjust the last data modification timestamp to the time of clock __WASI_CLOCK_REALTIME.
  }

  # A directory entry.
  struct Dirent {
    next @0 :Dircookie; # The offset of the next directory entry stored in this directory.
    no @1 :Inode; # The serial number of the file referred to by this directory entry.
    namlen @2 :UInt32; # The length of the name of the directory entry.
    type @3 :Filetype;
    name @4 :Data; # the name of the directory entry
  }

  # Read directory entries from a directory.
  # When successful, the contents of the output buffer consist of a sequence of directory entries. Each directory entry consists of a __wasi_dirent_t object, followed by __wasi_dirent_t::d_namlen bytes holding the name of the directory entry.
  # This function fills the output buffer as much as possible, potentially truncating the last directory entry. This allows the caller to grow its read buffer size in case it's too small to fit a single large directory entry, or skip the oversized directory entry.
  readdir @0 (
    bufLen :Size, # the number of bytes to read
    cookie :Dircookie, # The location within the directory to start reading. 
  ) -> (
    error :Errno,
    entries :List(Dirent), # the entries that where read
    bufused :Size # The number of bytes stored in the read buffer. If less than the size of the read buffer, the end of the directory has been reached.
  );

  # Create a directory.
  # Note: This is similar to mkdirat in POSIX.
  pathCreateDirectory @1 (
    path :Text # The path at which to create the directory.
  ) -> (
    error :Errno
  );

  # Return the attributes of a file or directory.
  # Note: This is similar to stat in POSIX.
  pathFilestatGet @2 (
    flags :LookupFlags, # Flags determining the method of how the path is resolved.
    path :Text #The path of the file or directory to inspect.     
  ) -> (
    error :Errno,
    buf :Filestat
  );

  # Adjust the timestamps of a file or directory.
  # Note: This is similar to utimensat in POSIX.
  pathFilestatSetTimes @3 (
    flags :LookupFlags, # Flags determining the method of how the path is resolved.
    path :Text, # The path of the file or directory to operate on.
    stAtim :Timestamp, # The desired values of the data access timestamp.
    stMtim :Timestamp, # The desired values of the data modification timestamp.
    fstFlags :Fstflags # A bitmask indicating which timestamps to adjust.
  ) -> (
    error :Errno
  );

  # Create a hard link.
  # Note: This is similar to linkat in POSIX.
  pathLink @4 (
    oldFlags :LookupFlags, # Flags determining the method of how the path is resolved.
    oldPath :Text, # The source path from which to link.
    newFd :Directory, # The working directory at which the resolution of the new path starts.
    newPath :Text # The destination path at which to create the hard link.
  ) -> (
    error :Errno
  );

  # Open a file or directory.
  # The returned file descriptor is not guaranteed to be the lowest-numbered file descriptor not currently open; it is randomized to prevent applications from depending on making assumptions about indexes, since this is error-prone in multi-threaded contexts. The returned file descriptor is guaranteed to be less than 231.
  # Note: This is similar to openat in POSIX.
  pathOpen @5 (
    dirflags :LookupFlags, # Flags determining the method of how the path is resolved.
    path :Text, # The relative path of the file or directory to open, relative to the dirfd directory.
    oFlags :Oflags, # The method by which to open the file.
    fsRightsBase :Rights, # The initial rights of the newly created file descriptor. The implementation is allowed to return a file descriptor with fewer rights than specified, if and only if those rights do not apply to the type of file being opened.
    fsRightsInheriting :Rights, # The base rights are rights that will apply to operations using the file descriptor itself, while the inheriting rights are rights that apply to file descriptors derived from it. 
    fsFlags :Fdflags, # The initial flags of the file descriptor.
  ) -> (
    error :Errno,
    fd :FileDescriptor  # The file descriptor of the file that has been opened.
  );

  # Read the contents of a symbolic link.
  # Note: This is similar to readlinkat in POSIX.
  pathReadlink @6 (
    path :Text, # The path of the symbolic link from which to read.
    bufLen :Size
  ) -> (
    error :Errno,
    buf :Text, # The buffer to which to write the contents of the symbolic link.
    bufused :Size # The number of bytes placed in the buffer.
  );

  # Remove a directory.
  # Return __WASI_ENOTEMPTY if the directory is not empty.
  # Note: This is similar to unlinkat(fd, path, AT_REMOVEDIR) in POSIX.
  pathRemoveDirectory @7 (
    path :Text, # The path to a directory to remove.
  ) -> (
    error :Errno  
  );

  # Rename a file or directory.
  # Note: This is similar to renameat in POSIX.
  pathRename @8 (
    oldPath :Text, # The source path of the file or directory to rename.
    newFd :Directory, # The working directory at which the resolution of the new path starts.
    newPath :Text # The destination path to which to rename the file or directory.
  ) -> (
    error :Errno
  );

  # Create a symbolic link.
  # Note: This is similar to symlinkat in POSIX.
  pathSymlink @9 (
    oldPath :Text, # The contents of the symbolic link.
    newPath :Text # The destination path at which to create the symbolic link.
  ) -> (
    error :Errno  
  );

  # Unlink a file.
  # Return __WASI_EISDIR if the path refers to a directory.
  # Note: This is similar to unlinkat(fd, path, 0) in POSIX.
  pathUnlinkFile @10 (
    path :Text # The path to a file to unlink.
  ) -> (
    error :Errno
  );

interface Socket extends (FileDescriptor) {
  # Flags provided to __wasi_sock_recv().
  struct Riflags {
    sockRecvPeek @0 :Bool; # Returns the message without removing it from the socket's receive queue.
    sockRecvWaitall @1 :Bool; # On byte-stream sockets, block until the full amount of data can be returned.
  }

  # Flags returned by __wasi_sock_recv().
  struct RoFlags {
    sockRecvDataTruncated @0 :Bool; # Returned by __wasi_sock_recv(): Message data has been truncated.
  }

    # Which channels on a socket to shut down.
  struct Sdflags {
    shutRd @0 :Bool; # Disables further receive operations.
    shutWr @1 :Bool; # Disables further send operations.
  }

  # Flags provided to __wasi_sock_send(). As there are currently no flags defined, it must be set to zero.
  enum Siflags {
  }
  

  # Receive a message from a socket.
  # Note: This is similar to recv in POSIX, though it also supports reading the data into multiple buffers in the manner of readv.
  sockRecv @0 (
    riDataLen :Size, 
    riFlags :Riflags # Message flags.
  ) -> (
    error :Errno,
    riData :List(Iovec), # List of scatter/gather vectors to which store the data.
    roFlags :RoFlags # Message flags.
  );

  # Send a message on a socket.
  # Note: This is similar to send in POSIX, though it also supports writing the data from multiple buffers in the manner of writev.
  sockSend @1 (
    siData :List(Iovec), # List of scatter/gather vectors to which to retrieve data
    siFlags :Siflags, # Message flags.
  ) -> (
    error :Errno,
    soDataLen :Size # Number of bytes transmitted.  
  );
   
  # Shut down socket send and receive channels.
  # Note: This is similar to shutdown in POSIX.
  sockShutdown @2 (
    how :Sdflags # Which channels on the socket to shut down.
  ) -> (
    error :Errno    
  );
  }
}

