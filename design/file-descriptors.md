# File Descriptors
File Descriptors ([witx type `fd`](https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#-fd)) are an abstract way of representing access to a resource. More often than not, they are processes (WIP), sockets (WIP), files, or even the console.

# Null File Descriptor
A File Descriptor of 0 is a null file descriptor. It's used whenever there is *no* file descriptor. Passing this to a method is a no-no, and receiving one from a method must be handled.

# The Console
When a program is initiated, `stdout`, `stdin`, and `stderr` (respectively `fd` ids `1`, `2`, and `3`) are opened. To print to these, simply call `fd_write` with the corresponding arguments. An simple "Hello World!" program written in `wat` is shown below:

```wat
(module
  ;; TODO: small hello world?
)
```

# Files
Files can be opened and closed with the method [`path_open`](https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#-path_openfd-fd-dirflags-lookupflags-path-string-oflags-oflags-fs_rights_base-rights-fs_rights_inherting-rights-fdflags-fdflags---errno-fd), and simply drop the returned `fd` to close it. These commands will fail if the WASM environment did not give the program permission to execute these.
