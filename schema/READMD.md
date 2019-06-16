## WASI Interface Definitions

The interface definition are written in [Capn Proto schema](https://capnproto.org/language.html).

### Files
- [wasi-common.capnp](./wasi-common.capnproto)
- [wasi-fd.capnp](./wasi-fd.capnp) The interfaces for file descriptors, directors, files and sockets.
- [wasi-env.capnp](./wasi-env.capnp) The interface for enviromental variables.
- [wasi-args.capnp](./wasi-args.capnp) The interface for command line arguments
- [wasi-clock.capnp](./wasi-clock.capnp) The interface for clocks.
- [wasi-events.capnp](./wasi-events.capnp) The interface events.
- [wasi-proc.capnp](./wasi-proc.capnp) The for proc related items.


## Cap'n Proto Wasm Generation

## Multiple Returns
Multiple returns in Cap'n Proto schema should be translated into pointer arguments that is appended to the end of a function arguments. For example 

```
returnTwoInts @0 (
  someInput :Uint64
) -> (
  first :UInt64,
  second :Uint64
);

```

Would get compiled to the following signature

```
(func $returnTwoInts 
  (param $someInput i64)
  (param $ptr_fist i32)
  (param $ptr_second i32)
)
```

## Returned Text
Text as an return value is turned into two arguments a pointer to write the text and an i32 representing the length of memory allocated for storing the text.

```
returnString @0 (
  someInput :Uint64
) -> (
  path :Text
);

```

Would get compiled to the following signature

```
(func $returnString 
  (param $someInput i64)
  (param $ptr_path i32)
  (param $path_len i32)
)
```

## Lists
Lists are turned into a pointer to a buffer of pointers

```
get @0 (
  args :List(Text) # the environment variables string data.
) -> ()
```

Would get compiled to the following signature

```
(func $returnString 
  (param $argv i32)
  (param $argv_buf i32)
)
```
