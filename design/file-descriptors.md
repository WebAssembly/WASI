# File Descriptors (a.k.a handles)
File Descriptors ([witx type `fd`](https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#-fd)), often referred to as "handles" interchangeably, are an abstract way of representing access to a resource. More often than not, they are processes (WIP), sockets (WIP), files, or even the console.

## The Console
When a program is initiated, `stdout`, `stdin`, and `stderr` (respectively `fd` ids `0`, `1`, and `2`) are opened. To print to these, simply call `fd_write` with the corresponding arguments. An simple "Hello World!" program written in `wat` is shown below, [sourced from the wasmtime test suite](https://github.com/bytecodealliance/wasmtime/blob/main/tests/wasm/hello_wasi_snapshot1.wat):

```wat
(module
  (import "wasi_snapshot_preview1" "proc_exit"
    (func $__wasi_proc_exit (param i32)))
  (import "wasi_snapshot_preview1" "fd_write"
    (func $__wasi_fd_write (param i32 i32 i32 i32) (result i32)))
  (func $_start
    (i32.store (i32.const 24) (i32.const 14))
    (i32.store (i32.const 20) (i32.const 0))
    (block
      (br_if 0
        (call $__wasi_fd_write
          (i32.const 1)
          (i32.const 20)
          (i32.const 1)
          (i32.const 16)))
      (br_if 0 (i32.ne (i32.load (i32.const 16)) (i32.const 14)))
      (br 1)
    )
    (call $__wasi_proc_exit (i32.const 1))
  )
  (memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (data (i32.const 0) "Hello, world!\0a")
)
```

## Files
Files can be opened and closed with the method [`path_open`](https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#path_open), and simply drop the returned `fd` to close it. These commands will fail if the WASM environment did not give the program permission to execute these.
