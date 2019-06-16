@0xf8a5c786b8463cf1;

using Timestamp = UInt64; # Timestamp in nanoseconds.
using Size = UInt64;


# Error codes returned by functions.
# Not all of these error codes are returned by the functions provided by this API; some are used in higher-level library layers, and others are provided merely for alignment with POSIX.
enum Errno {
  esuccess @0; # No error occurred. System call completed successfully.ErrnoT
  e2big @1; # Argument list too long.
  eacces @2; # Permission denied.
  eaddrinuse @3; # Address in use.
  eaddrnotavail @4; # Address not available.
  eafnosupport @5; # Address family not supported.
  eagain @6; # Resource unavailable, or operation would block.
  ealready @7; # Connection already in progress.
  ebadf @8; # Bad file descriptor.
  ebadmsg @9; # Bad message.
  ebusy @10; # Device or resource busy.
  ecanceled @11; # Operation canceled.
  eechild @12; # No child processes.
  econnaborted @13; # Connection aborted.
  econnrefused @14; # Connection refused.
  econnreset @15; # Connection reset.
  edeadlk @16; # Resource deadlock would occur.
  edestaddrreq @17; # Destination address required.
  edom @18; # Mathematics argument out of domain of function.
  edquot @19; # Reserved.
  eexist @20; # File exists.
  efault @21; # Bad address.
  efbig @22; # File too large.
  ehostunreach @23; # Host is unreachable.
  eidrm @24; # Identifier removed.
  eilseq @25; # Illegal byte sequence.
  einprogress @26; # Operation in progress.
  eintr @27; # Interrupted function.
  einval @28; # Invalid argument.
  eio @29; # I/O error.
  eisconn @30; # Socket is connected.
  eisdir @31; # Is a directory.
  eloop @32; # Too many levels of symbolic links.
  emfile @33; # File descriptor value too large.
  emlink @34; # Too many links.
  emsgsize @35; # Message too large.
  emultihop @36; # Reserved.
  enametoolong @37; # Filename too long.
  enetdown @38; # Network is down.
  enetreset @39; # Connection aborted by network.
  enetunreach @40; # Network unreachable.
  enfile @41; # Too many files open in system.
  enobufs @42; # No buffer space available.
  enodev @43; # No such device.
  enoent @44; # No such file or directory.
  enoexec @45; # Executable file format error.
  enolck @46; # No locks available.
  enolink @47; # Reserved.
  enomem @48; # Not enough space.
  enomsg @49; # No message of the desired type.
  enoprotoopt @50; # Protocol not available.
  enospc @51; # No space left on device.
  enosys @52; # Function not supported.
  enotconn @53; # The socket is not connected.
  enotdir @54; # Not a directory or a symbolic link to a directory.
  enotempty @55; # Directory not empty.
  enotrecoverable @56; # State not recoverable.
  enotsock @57; # Not a socket.
  enotsup @58; # Not supported, or operation not supported on socket.
  enotty @59; # Inappropriate I/O control operation.
  enxio @60; # No such device or address.
  eoverflow @61; # Value too large to be stored in data type.
  eownerdead @62; # Previous owner died.
  eperm @63; # Operation not permitted.
  epipe @64; # Broken pipe.
  eproto @65; # Protocol error.
  eprotonosupport @66; # Protocol not supported.
  eprototype @67; # Protocol wrong type for socket.
  erange @68; # Result too large.
  erofs @69; # Read-only file system.
  espipe @70; # Invalid seek.
  esrch @71; # No such process. 
  estale @72; # Reserved
  etimedout @73; # Connection timed out.
  etxtbsy @74; # Text file busy.
  exdev @75; # Cross-device link.
  enotcapable @76; # Extension: Capabilities insufficient.
}

