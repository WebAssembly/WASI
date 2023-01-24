# WASI I/O

WASI I/O is an I/O abstraction API which is currently focused on providing
stream types.

In the future, the component model is expected to add built in stream types;
when it does, they are expected to subsume this API.

## `stream-error`
```wit
/// An error type returned from a stream operation. Currently this
/// doesn't provide any additional information.
record stream-error {}
```

## `input-stream`
```wit
resource input-stream {
```

## `read`
```wit
    /// Read bytes from a stream.
    read: func(
        /// The maximum number of bytes to read
        len: u64
    ) -> result<tuple<list<u8>, bool>, stream-error>
```

## `skip`
```wit
    /// Skip bytes from a stream.
    skip: func(
        /// The maximum number of bytes to skip.
        len: u64,
    ) -> result<tuple<u64, bool>, stream-error>
```

```wit
}
```

## `output-stream`
```wit
resource output-stream {
```

## `write`
```wit
    /// Write bytes to a stream.
    write: func(
        /// Data to write
        buf: list<u8>
    ) -> result<u64, stream-error>
```

## `write-repeated`
```wit
    /// Write bytes to a stream.
    write-repeated: func(
        /// The byte to write
        byte: u8,
        /// The number of times to write it
        len: u64
    ) -> result<u64, stream-error>
```

## `splice`
```wit
    /// Read from one stream and write to another.
    splice-stream: func(
        /// The stream to read from.
        src: input-stream,
        /// The number of bytes to splice.
        len: u64,
    ) -> result<tuple<u64, bool>, stream-error>
```

```wit
}
```
